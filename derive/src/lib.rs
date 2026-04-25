mod biz_err_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(BizError, attributes(detail))]

pub fn biz_error_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    biz_err_derive::my_error_derive_impl(input).into()
}
