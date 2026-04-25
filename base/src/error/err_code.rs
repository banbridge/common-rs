use derive::BizError;
use log::warn;

pub type AppResult<T> = Result<T, AppErrorBuilt>;

#[derive(Copy, Clone, Debug, BizError)]

pub enum AppError {
    // http请求就相关错误
    #[detail(code = 0, http_status = 200)]
    Ok,
    #[detail(code = 1000000, http_status = 500, message_zh = "内部通用错误")]
    Internal,
    #[detail(
        code = 1000001,
        http_status = 400,
        message_zh = "请求参数错误，请仔细核对参数有效性"
    )]
    InvalidParam,
    #[detail(code = 1000002, http_status = 500, message_zh = "未知错误")]
    UnknownAnyhow,
    #[detail(code = 1000003, http_status = 404, message_zh = "请求资源不存在")]
    RequestNotFound,
    #[detail(code = 1000004, http_status = 408, message_zh = "请求超时")]
    RequestTimeout,
    #[detail(
        code = 1000005,
        http_status = 400,
        message_zh = "必传参数未设置，请检查请求参数"
    )]
    UninitializedFieldError,
    #[detail(
        code = 1000006,
        http_status = 400,
        message_zh = "参数验证失败，请检查对应参数"
    )]
    ValidateParamFailed,

    // 参数解析错误码
    #[detail(
        code = 1010000,
        http_status = 400,
        message_zh = "从请求中解析参数错误，请检查对应参数"
    )]
    ParamBind,
    #[detail(
        code = 1010001,
        http_status = 400,
        message_zh = "JSON解析出错，请检查json格式"
    )]
    JsonParse,
    #[detail(
        code = 1010002,
        http_status = 400,
        message_zh = "JSON序列化错误，请检查对应参数"
    )]
    JsonSerde,
    #[detail(code = 1010003, http_status = 500, message_zh = "BCrypt 加密失败")]
    BcryptFailed,

    // auth 相关错误码
    #[detail(code = 1020000, http_status = 401, message_zh = "JWT Token 无效")]
    JwtInvalidToken, // auth 无效
    #[detail(
        code = 1020001,
        http_status = 401,
        message_zh = "用户未登录或Token失效"
    )]
    Unauthenticated, // 未授权
    #[detail(code = 1020002, http_status = 401, message_zh = "Token无效或已过期")]
    JwtDecode,
    #[detail(
        code = 1020003,
        http_status = 401,
        message_zh = "Token编码失败，请检查对应参数"
    )]
    JwtEncode,
    #[detail(code = 1020004, http_status = 401, message_zh = "用户名或密码错误")]
    PasswordInvalid,
    #[detail(code = 1020005, http_status = 501, message_zh = "访问权限服务失败")]
    CasbinFailed,
    #[detail(code = 1020006, http_status = 403, message_zh = "用户无权限访问该资源")]
    NoAuthResource,

    // 数据库相关错误吗
    #[detail(code = 1030000, http_status = 500, message_zh = "数据库错误")]
    DBCommon, // 数据库错误
    #[detail(code = 1030001, http_status = 404, message_zh = "数据库记录未找到")]
    DBNotFound, // 数据库未找到
    #[detail(code = 1030002, http_status = 500, message_zh = "数据库查询失败")]
    DBQueryFailed, // 数据库查询失败
    #[detail(code = 1030003, http_status = 500, message_zh = "数据库更新失败")]
    DBUpdateFailed, // 数据库更新失败
    #[detail(code = 1030004, http_status = 500, message_zh = "数据库插入失败")]
    DBInsertFailed, // 插入数据库失败
    #[detail(code = 1030005, http_status = 500, message_zh = "数据库删除失败")]
    DBDeleteFailed, // 删除数据库失败
    #[detail(code = 1030006, http_status = 500, message_zh = "数据库连接失败")]
    DBConnectionFailed,
    #[detail(code = 1030007, http_status = 500, message_zh = "数据库事务开启")]
    DBTransactionBeginFailed,
    #[detail(code = 1030008, http_status = 500, message_zh = "数据库事务提交失败")]
    DBTransactionCommitFailed,

    #[detail(code = 1040000, http_status = 400, message_zh = "缓存错误")]
    CacheCommon,
    #[detail(code = 1040001, http_status = 500, message_zh = "redis查询错误")]
    CacheQueryFailed,
    #[detail(code = 1040002, http_status = 500, message_zh = "redis设置错误")]
    CacheSetFailed,
    #[detail(code = 1040003, http_status = 500, message_zh = "redis删除错误")]
    CacheDeleteFailed,
    #[detail(code = 1040004, http_status = 500, message_zh = "获取redis连接错误")]
    CacheConnectionFailed,

    // agent 相关错误码
    #[detail(code = 1050000, http_status = 500, message_zh = "LLM 错误")]
    LLMError,
    #[detail(code = 1050001, http_status = 400, message_zh = "LLM客户端构建失败")]
    LLMClientBuildError,
    #[detail(code = 1050002, http_status = 400, message_zh = "LLM模型加载Tool失败")]
    LLMModelLoadToolError,

    // command 相关错误码
    #[detail(code = 1060000, http_status = 500, message_zh = "命令执行失败")]
    CommandExecuteError,
    #[detail(code = 1060001, http_status = 500, message_zh = "命令执行超时")]
    CommandExecuteTimeout,
}

impl AppErrorBuilt {
    pub fn print_stack(self) -> Self {
        let st = std::backtrace::Backtrace::force_capture();

        let mut result = String::from("\n");

        let mut step = 0;

        // 将backtrace转换为字符串并过滤
        let bt_str = format!("{}", st);

        for line in bt_str.lines() {
            // 跳过标准库内部调用，只保留用户代码

            if line.contains("std::") || line.contains("core::") {
                continue;
            }

            result.push_str(line);

            result.push('\n');

            step += 1;

            if step > 10 {
                break;
            }
        }

        warn!("{:#?} stack:{}", self.get_message(), result);

        self
    }
}
