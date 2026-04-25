use darling::{FromDeriveInput, FromVariant};

#[allow(unused)]
// derive FromDeriveInput, 表示这个结构体可以用 `syn::DeriveInput` 转换过来
#[derive(Debug, FromDeriveInput)]
// darling 自身的配置，接受 `detail` attr的数据，只允许 enum 的结构体，struct 报错。
#[darling(attributes(detail), supports(enum_any))]

pub(crate) struct DetailErrorEnum {
    // enum 的名称
    pub(crate) ident: syn::Ident,
    // enum 的枚举成员格式化成 DetailErrorVariant
    pub(crate) data: darling::ast::Data<DetailErrorVariant, darling::util::Ignored>,
}

#[allow(unused)]
#[derive(Debug, FromVariant)]
#[darling(attributes(detail))]

pub(crate) struct DetailErrorVariant {
    pub(crate) ident: syn::Ident,
    // fields 的数据， 指的是 `InvalidEmail(String)` 里面的 `String`
    pub(crate) fields: darling::ast::Fields<syn::Field>,
    // 这里表示从 `FromMeta` 中取数据，这里特指 `#[detail(code=400)]`
    #[darling(default)]
    pub(crate) code: u64,
    #[darling(default)]
    pub(crate) http_status: u16,
    // 这里表示从 `FromMeta` 中取数据，这里特指 `#[detail(message="detail message")]`
    #[darling(default)]
    pub(crate) message_zh: Option<String>,
}
