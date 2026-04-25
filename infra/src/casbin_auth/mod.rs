mod adapter;

use std::sync::Arc;

use adapter::SeaOrmAdapter;
use base::error::{AppErrorBuilt, AppResult};
use casbin::{CoreApi, DefaultModel, Enforcer};
use rust_embed::Embed;
use sea_orm::DatabaseConnection;

#[allow(dead_code)]
#[derive(Clone)]

pub struct CasbinManager {
    enforcer: Arc<Enforcer>,
}

impl CasbinManager {
    pub async fn new(db: DatabaseConnection) -> AppResult<Self> {
        let enforcer = get_enforcer(db).await?;

        Ok(CasbinManager {
            enforcer: Arc::new(enforcer),
        })
    }

    pub fn get_enforcer(&self) -> Arc<Enforcer> {
        self.enforcer.clone()
    }
}

#[allow(dead_code)]
#[derive(Embed)]
#[folder = "src/casbin_auth"]

struct RBAC;

fn get_rbac_model() -> AppResult<String> {
    let rbac_model = RBAC::get("rbac_model.conf")
        .ok_or(AppErrorBuilt::invalid_param(
            "rbac_model.conf not found".to_string(),
        ))?
        .data
        .to_vec();

    String::from_utf8(rbac_model).map_err(|e| AppErrorBuilt::invalid_param(format!("{}", e)))
}

async fn get_enforcer(db: DatabaseConnection) -> AppResult<Enforcer> {
    let policy_str = get_rbac_model()?;

    let m = DefaultModel::from_str(&policy_str)
        .await
        .map_err(|e| AppErrorBuilt::invalid_param(format!("{}", e)))?;

    let a = SeaOrmAdapter::new(db)
        .await
        .map_err(|e| AppErrorBuilt::db_common(format!("{}", e)))?;

    let e = Enforcer::new(m, a)
        .await
        .map_err(|e| AppErrorBuilt::db_common(format!("casbin enforce init failed is {}", e)))?;

    Ok(e)
}

#[cfg(test)]

mod tests {
    use casbin::{MgmtApi, RbacApi};

    use super::*;
    use crate::dbop::conn;

    #[test]

    fn test_get_rbac_model() {
        let rbac_model = get_rbac_model();

        println!("rbac_model: {:?}", rbac_model)
    }

    #[tokio::test]

    async fn test_get_enforcer() {
        dotenv::dotenv().ok();

        let dsn = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let db = conn::get_postgresql_db(dsn.as_str()).await.unwrap();

        let mut e = get_enforcer(db.get_db().clone()).await.unwrap();

        // let rs = e
        //     .add_policy(vec![
        //         "alice".to_string(),
        //         "data1".to_string(),
        //         "read".to_string(),
        //     ])
        //     .await;
        // println!("add policy rs: {:?}", rs);
        //
        // e.add_policy(vec![
        //     "admin".to_string(),
        //     "data2".to_string(),
        //     "read".to_string(),
        // ])
        // .await
        // .unwrap();
        //
        // e.add_roles_for_user("alice", vec!["admin".to_string()], None)
        //     .await
        //     .unwrap();

        let rs = e.get_policy();

        println!("rs: {:?}", rs);

        let ps = e.get_permissions_for_user("alice", None);
        let roles = e.get_roles_for_user("alice", None);

        println!("roles: {:?}", roles);
        println!("ps: {:?}", ps);

        let check = e.enforce(vec![
            "alice".to_string(),
            "data2".to_string(),
            "read".to_string(),
        ]);
        println!("check: {:?}", check);
    }
}
