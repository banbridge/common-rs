use derive::BizError;
use std::error::Error;

#[derive(BizError, Debug)]
enum BanbridgeError {
    #[detail(code = 1234, http_status = 400, message_zh = "邮箱格式错误")]
    InvalidEmail,
    #[detail(code = 1235, http_status = 400)]
    InvalidPassword,
}

fn main() {
    let code = BanbridgeError::InvalidEmail.get_code();

    println!("{}", code);

    let c = BanbridgeErrorBuilt::invalid_email("sa".to_string());

    let c = c.with_context("key".to_string(), "value".to_string());

    println!("{:?}", c.source());
}

// fn print_stack() {
//     let a = backtrace::Backtrace::new();
//     println!("{:?}", a);
// }
