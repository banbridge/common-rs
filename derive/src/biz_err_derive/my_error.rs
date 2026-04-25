use darling::FromDeriveInput;
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::biz_err_derive::DetailErrorEnum;

pub(crate) fn my_error_derive_impl(input: syn::DeriveInput) -> TokenStream {
    let data_error: DetailErrorEnum = DetailErrorEnum::from_derive_input(&input).unwrap();

    let ident = &data_error.ident;

    let enum_quote = gen_error_enum(ident, &data_error);

    let struct_quote = gen_error_struct(ident);

    let struct_methods_quote = gen_error_struct_methods(ident, &data_error);

    let output = quote! {
    #enum_quote

    #struct_quote

    #struct_methods_quote
    };

    // let str = &output;

    // dbg!("output: ", str.to_string());

    output
}

fn gen_error_enum(ident: &Ident, data_error: &DetailErrorEnum) -> TokenStream {
    let enum_data = data_error.data.as_ref().take_enum().unwrap();

    let mut http_status_list: Vec<TokenStream> = vec![];

    let mut biz_message_list: Vec<TokenStream> = vec![];

    let mut message_zh_list: Vec<TokenStream> = vec![];

    let mut code_list: Vec<TokenStream> = vec![];

    enum_data.into_iter().for_each(|item| {
        let item_ident = &item.ident;

        let http_status = item.http_status;

        let code = item.code;

        let message_zh: String = item.message_zh.clone().unwrap_or_default();

        http_status_list.push(quote! {#ident::#item_ident => #http_status});

        biz_message_list.push(quote! {#ident::#item_ident => #ident::#item_ident.to_string()});

        code_list.push(quote! {#ident::#item_ident => #code});

        message_zh_list.push(quote! {#ident::#item_ident => #message_zh.to_string()});
    });

    quote! {
        impl #ident {
            pub fn get_http_status(&self) -> u16 {
                match self {
                    #(#http_status_list,)*
                }
            }

            pub fn get_code(&self)->u64 {
                match self {
                    #(#code_list,)*
                }
            }

            pub fn get_biz_message(&self) -> String {
                match self {
                    #(#biz_message_list,)*
                }
            }

            pub fn get_message_zh(&self) -> String {
                match self {
                    #(#message_zh_list,)*
                }
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    }
}

fn gen_error_struct(ident: &Ident) -> TokenStream {
    let new_ident = get_struct_indent(ident);

    let res = quote! {
        #[derive(Debug, serde::Serialize, thiserror::Error)]
        #[serde(rename_all = "PascalCase")]
        pub struct #new_ident {
            biz_code: u64,
            message: String,
            http_status: u16,
            biz_message: String,
            message_zh: String,
            #[serde(skip)]
            base: Option<anyhow::Error>,
            #[serde(skip)]
            stack: Option<std::backtrace::Backtrace>,
            #[serde(skip)]
            kv: std::collections::HashMap<String, String>,
        }

    };

    res
}

fn gen_error_struct_methods(ident: &Ident, data_error: &DetailErrorEnum) -> TokenStream {
    let new_ident = get_struct_indent(ident);

    let base_method = get_struct_base_method();

    let enum_method = get_enum_method(ident, &new_ident, data_error);

    let res = quote! {
        impl #new_ident {
            #base_method

            #enum_method

        }

        impl std::fmt::Display for #new_ident{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,
            "  http_status: {}, message: {}, biz_code: {}, biz_message: {} base: {:?}",
            self.http_status, self.message, self.biz_code, self.biz_message, self.base)
            }
        }

    };

    res
}

fn get_struct_base_method() -> TokenStream {
    quote! {
        pub fn new(biz_code: u64, message: String, http_status: u16, biz_message: String, message_zh: String) -> Self {
            Self {
                biz_code,
                message,
                http_status,
                biz_message,
                message_zh,
                base: None,
                stack: None,
                kv: std::collections::HashMap::new(),
            }
        }

        pub fn with_base(mut self, base: anyhow::Error) -> Self {
            self.message = format!("{}, base error is {:?}", self.message, base);
            self.base = Some(base);
            self
        }

        pub fn with_stack(mut self) -> Self {
            self.stack = Some(std::backtrace::Backtrace::capture());
            self
        }

        pub fn with_kv(mut self, key: String, value: String) -> Self {
            self.kv.insert(key, value);
            self
        }

        pub fn with_kvs(mut self, kvs: Vec<(String, String)>) -> Self {
            for (k, v) in kvs {
                self.kv.insert(k, v);
            }
            self
        }

        pub fn get_message(&self) -> String {
            let kv_str = self
                .kv
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(",");

            format!("{} {} {}", self.message, kv_str, self.biz_message)
        }

        pub fn get_base(&self) -> Option<&anyhow::Error> {
            self.base.as_ref()
        }

        pub fn get_biz_code(&self) -> u64 {
            self.biz_code
        }

        pub fn get_http_status(&self) -> u16 {
            self.http_status
        }

        pub fn get_biz_message(&self) -> &str {
            self.biz_message.as_str()
        }

    }
}

fn get_enum_method(ident: &Ident, new_ident: &Ident, data_error: &DetailErrorEnum) -> TokenStream {
    let enum_data = data_error.data.as_ref().take_enum().unwrap();

    let mut enum_method_list: Vec<TokenStream> = vec![];

    enum_data.into_iter().for_each(|item| {
        let item_ident = &item.ident;

        let method_name = format_ident!("{}", item_ident.to_string().to_snake_case());

        let method = quote! {

            pub fn #method_name(message: String) -> Self {
                let biz_info = #ident::#item_ident;
                #new_ident::new(
                    biz_info.get_code(),
                    message,
                    biz_info.get_http_status(),
                    biz_info.get_biz_message(),
                    biz_info.get_message_zh(),
                )
            }
        };

        enum_method_list.push(method);
    });

    quote! {
        #(#enum_method_list)*
    }
}

fn get_struct_indent(ident: &Ident) -> Ident {
    format_ident!("{}Built", ident)
}
