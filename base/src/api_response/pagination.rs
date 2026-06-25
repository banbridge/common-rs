use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug, Clone, PartialEq, Eq)]
pub struct PaginationQuery {
    #[validate(range(min = 1))]
    pub page_num: i64,
    #[validate(range(min = 1, max = 200))]
    pub page_size: i64,
}

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        (self.page_num - 1) * self.page_size
    }

    pub fn limit(&self) -> i64 {
        self.page_size
    }
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page_num: 1,
            page_size: 20,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page_num: i64,
    pub page_size: i64,
}

impl<T> PaginatedData<T> {
    pub fn new(items: Vec<T>, total: i64, page_num: i64, page_size: i64) -> Self {
        Self {
            items,
            total,
            page_num,
            page_size,
        }
    }

    pub fn total_pages(&self) -> i64 {
        if self.page_size == 0 {
            return 0;
        }
        (self.total + self.page_size - 1) / self.page_size
    }

    pub fn has_next(&self) -> bool {
        self.page_num < self.total_pages()
    }

    pub fn has_prev(&self) -> bool {
        self.page_num > 1
    }
}
