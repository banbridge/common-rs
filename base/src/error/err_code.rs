use derive::BizError;
use log::warn;

pub type AppResult<T> = Result<T, AppErrorBuilt>;

#[derive(Copy, Clone, Debug, BizError)]

pub enum AppError {
    // ===== 通用基础错误（100xxxx）=====
    #[detail(code = 0, http_status = 200, message_zh = "操作成功", message_en = "Success")]
    Ok,
    #[detail(
        code = 1000000,
        http_status = 500,
        message_zh = "服务器开小差了，请稍后再试",
        message_en = "Internal server error, please try again later"
    )]
    Internal,
    #[detail(
        code = 1000001,
        http_status = 400,
        message_zh = "您输入的参数有误，请检查后重新提交",
        message_en = "Invalid parameters, please check and try again"
    )]
    InvalidParam,
    #[detail(
        code = 1000002,
        http_status = 500,
        message_zh = "遇到了未知问题，请稍后再试",
        message_en = "Unknown error, please try again later"
    )]
    UnknownAnyhow,
    #[detail(
        code = 1000003,
        http_status = 404,
        message_zh = "您访问的页面不存在",
        message_en = "The requested resource was not found"
    )]
    RequestNotFound,
    #[detail(
        code = 1000004,
        http_status = 408,
        message_zh = "请求超时，请检查网络后重试",
        message_en = "Request timeout, please check your network and try again"
    )]
    RequestTimeout,
    #[detail(
        code = 1000005,
        http_status = 400,
        message_zh = "缺少必填项，请填写完整后提交",
        message_en = "Required fields are missing, please fill in all required fields"
    )]
    UninitializedFieldError,
    #[detail(
        code = 1000006,
        http_status = 400,
        message_zh = "部分信息不符合要求，请核对后重新提交",
        message_en = "Parameter validation failed, please check the highlighted fields"
    )]
    ValidateParamFailed,
    #[detail(
        code = 1000007,
        http_status = 405,
        message_zh = "该操作暂不支持",
        message_en = "Method not allowed"
    )]
    MethodNotAllowed,
    #[detail(
        code = 1000008,
        http_status = 415,
        message_zh = "不支持的数据格式，请检查后重试",
        message_en = "Unsupported media type"
    )]
    UnsupportedMediaType,
    #[detail(
        code = 1000009,
        http_status = 413,
        message_zh = "请求内容过大，请精简后重试",
        message_en = "Payload too large"
    )]
    PayloadTooLarge,
    #[detail(
        code = 1000010,
        http_status = 400,
        message_zh = "请勿重复提交，请稍后再试",
        message_en = "Duplicate submission, please try again later"
    )]
    DuplicateSubmission,
    #[detail(
        code = 1000011,
        http_status = 400,
        message_zh = "请求过于频繁，请稍候再试",
        message_en = "Too many requests, please try again later"
    )]
    TooManyRequests,
    #[detail(
        code = 1000012,
        http_status = 400,
        message_zh = "缺少必要的请求头信息",
        message_en = "Missing required request headers"
    )]
    MissingHeader,

    // ===== 参数解析错误（101xxxx）=====
    #[detail(
        code = 1010000,
        http_status = 400,
        message_zh = "数据解析失败，请检查输入格式",
        message_en = "Failed to parse request parameters, please check the format"
    )]
    ParamBind,
    #[detail(
        code = 1010001,
        http_status = 400,
        message_zh = "数据格式有误，请检查 JSON 格式",
        message_en = "JSON parse error, please check the JSON format"
    )]
    JsonParse,
    #[detail(
        code = 1010002,
        http_status = 500,
        message_zh = "数据处理失败，请稍后重试",
        message_en = "JSON serialization failed, please try again"
    )]
    JsonSerde,
    #[detail(
        code = 1010003,
        http_status = 500,
        message_zh = "加密处理失败，请稍后重试",
        message_en = "BCrypt encryption failed, please try again"
    )]
    BcryptFailed,
    #[detail(
        code = 1010004,
        http_status = 400,
        message_zh = "日期格式有误，请重新选择",
        message_en = "Invalid date format, please reselect"
    )]
    InvalidDateFormat,
    #[detail(
        code = 1010005,
        http_status = 400,
        message_zh = "邮箱格式不正确",
        message_en = "Invalid email format"
    )]
    InvalidEmailFormat,
    #[detail(
        code = 1010006,
        http_status = 400,
        message_zh = "手机号格式不正确",
        message_en = "Invalid phone number format"
    )]
    InvalidPhoneFormat,
    #[detail(
        code = 1010007,
        http_status = 400,
        message_zh = "身份证号格式不正确",
        message_en = "Invalid ID card format"
    )]
    InvalidIdCardFormat,
    #[detail(
        code = 1010008,
        http_status = 400,
        message_zh = "URL 格式不正确",
        message_en = "Invalid URL format"
    )]
    InvalidUrlFormat,

    // ===== 认证授权相关错误（102xxxx）=====
    #[detail(
        code = 1020000,
        http_status = 401,
        message_zh = "登录状态已失效，请重新登录",
        message_en = "Invalid token, please log in again"
    )]
    JwtInvalidToken,
    #[detail(
        code = 1020001,
        http_status = 401,
        message_zh = "请先登录后再操作",
        message_en = "Please log in to continue"
    )]
    Unauthenticated,
    #[detail(
        code = 1020002,
        http_status = 401,
        message_zh = "登录已过期，请重新登录",
        message_en = "Token expired or invalid, please log in again"
    )]
    JwtDecode,
    #[detail(
        code = 1020003,
        http_status = 500,
        message_zh = "登录处理失败，请稍后重试",
        message_en = "Token encode failed, please try again"
    )]
    JwtEncode,
    #[detail(
        code = 1020004,
        http_status = 401,
        message_zh = "账号或密码错误",
        message_en = "Invalid username or password"
    )]
    PasswordInvalid,
    #[detail(
        code = 1020005,
        http_status = 501,
        message_zh = "权限服务暂时不可用，请稍后重试",
        message_en = "Permission service unavailable, please try again"
    )]
    CasbinFailed,
    #[detail(
        code = 1020006,
        http_status = 403,
        message_zh = "您没有权限执行该操作",
        message_en = "You do not have permission to access this resource"
    )]
    NoAuthResource,
    #[detail(
        code = 1020007,
        http_status = 401,
        message_zh = "登录已过期，请重新登录",
        message_en = "Token expired, please log in again"
    )]
    JwtTokenExpired,
    #[detail(
        code = 1020008,
        http_status = 403,
        message_zh = "该账号已被禁止登录",
        message_en = "This account is banned"
    )]
    AccountBanned,
    #[detail(
        code = 1020009,
        http_status = 403,
        message_zh = "该 IP 已被封禁，请联系客服",
        message_en = "This IP address is blocked, please contact support"
    )]
    IpBlocked,
    #[detail(
        code = 1020010,
        http_status = 401,
        message_zh = "刷新凭证已失效，请重新登录",
        message_en = "Refresh token invalid, please log in again"
    )]
    RefreshTokenInvalid,
    #[detail(
        code = 1020011,
        http_status = 403,
        message_zh = "角色权限不足",
        message_en = "Insufficient role permissions"
    )]
    InsufficientRolePermission,
    #[detail(
        code = 1020012,
        http_status = 403,
        message_zh = "该功能需要管理员权限",
        message_en = "Admin privileges required"
    )]
    AdminPrivilegeRequired,
    #[detail(
        code = 1020013,
        http_status = 401,
        message_zh = "双因素认证失败",
        message_en = "Two-factor authentication failed"
    )]
    TwoFactorAuthFailed,
    #[detail(
        code = 1020014,
        http_status = 401,
        message_zh = "双因素认证码错误",
        message_en = "Invalid two-factor authentication code"
    )]
    TwoFactorCodeInvalid,
    #[detail(
        code = 1020015,
        http_status = 401,
        message_zh = "OAuth 认证失败",
        message_en = "OAuth authentication failed"
    )]
    OAuthFailed,

    // ===== 登录流程错误（10201xx）=====
    #[detail(
        code = 1020100,
        http_status = 401,
        message_zh = "登录失败，请检查账号密码",
        message_en = "Login failed, please check your credentials"
    )]
    LoginFailed,
    #[detail(
        code = 1020101,
        http_status = 401,
        message_zh = "该账号不存在",
        message_en = "Account not found"
    )]
    AccountNotFound,
    #[detail(
        code = 1020102,
        http_status = 401,
        message_zh = "账号已被禁用，请联系管理员",
        message_en = "Account disabled, please contact administrator"
    )]
    AccountDisabled,
    #[detail(
        code = 1020103,
        http_status = 401,
        message_zh = "账号已被锁定，请稍后再试或联系管理员解锁",
        message_en = "Account locked, please try again later or contact support"
    )]
    AccountLocked,
    #[detail(
        code = 1020104,
        http_status = 401,
        message_zh = "账号尚未激活，请先激活您的账号",
        message_en = "Account not activated, please activate your account first"
    )]
    AccountInactive,
    #[detail(
        code = 1020105,
        http_status = 401,
        message_zh = "登录失败次数过多，请 30 分钟后重试",
        message_en = "Too many login attempts, please try again in 30 minutes"
    )]
    LoginAttemptsExceeded,
    #[detail(
        code = 1020106,
        http_status = 401,
        message_zh = "验证码错误，请重新输入",
        message_en = "Invalid captcha code, please try again"
    )]
    CaptchaInvalid,
    #[detail(
        code = 1020107,
        http_status = 401,
        message_zh = "验证码已过期，请重新获取",
        message_en = "Captcha expired, please request a new one"
    )]
    CaptchaExpired,
    #[detail(
        code = 1020108,
        http_status = 401,
        message_zh = "当前设备未授权，请使用常用设备登录",
        message_en = "Device mismatch, please use a trusted device"
    )]
    DeviceMismatch,
    #[detail(
        code = 1020109,
        http_status = 401,
        message_zh = "检测到异地登录，请验证身份",
        message_en = "Unusual login location detected, please verify your identity"
    )]
    RiskyLoginLocation,
    #[detail(
        code = 1020110,
        http_status = 401,
        message_zh = "密码已过期，请修改密码后重新登录",
        message_en = "Password expired, please change your password"
    )]
    PasswordExpired,
    #[detail(
        code = 1020111,
        http_status = 400,
        message_zh = "密码强度不足，建议包含大小写字母、数字和特殊字符",
        message_en = "Password too weak, include uppercase, lowercase, numbers and special characters"
    )]
    PasswordStrengthInsufficient,
    #[detail(
        code = 1020112,
        http_status = 400,
        message_zh = "原密码输入错误",
        message_en = "Old password is incorrect"
    )]
    OldPasswordIncorrect,
    #[detail(
        code = 1020113,
        http_status = 400,
        message_zh = "新密码不能与旧密码相同",
        message_en = "New password cannot be the same as the old one"
    )]
    PasswordSameAsOld,
    #[detail(
        code = 1020114,
        http_status = 401,
        message_zh = "账号已在其他设备登录，您已被迫下线",
        message_en = "Account logged in elsewhere, you have been signed out"
    )]
    AccountLoggedInElsewhere,
    #[detail(
        code = 1020115,
        http_status = 401,
        message_zh = "登录超时，请重新操作",
        message_en = "Login timeout, please try again"
    )]
    LoginTimeout,
    #[detail(
        code = 1020116,
        http_status = 401,
        message_zh = "您已成功退出登录",
        message_en = "You have been logged out successfully"
    )]
    AlreadyLoggedOut,

    // ===== 用户注册相关（10202xx）=====
    #[detail(
        code = 1020200,
        http_status = 400,
        message_zh = "注册失败，请稍后重试",
        message_en = "Registration failed, please try again"
    )]
    RegistrationFailed,
    #[detail(
        code = 1020201,
        http_status = 409,
        message_zh = "该用户名已被使用，请换一个再试",
        message_en = "Username already taken, please try another one"
    )]
    UsernameAlreadyExists,
    #[detail(
        code = 1020202,
        http_status = 409,
        message_zh = "该邮箱已被注册，请直接登录或使用其他邮箱",
        message_en = "Email already registered, please log in or use another email"
    )]
    EmailAlreadyExists,
    #[detail(
        code = 1020203,
        http_status = 409,
        message_zh = "该手机号已被注册，请直接登录或使用其他号码",
        message_en = "Phone number already registered, please log in or use another number"
    )]
    PhoneAlreadyExists,
    #[detail(
        code = 1020204,
        http_status = 400,
        message_zh = "两次输入的密码不一致",
        message_en = "Passwords do not match"
    )]
    PasswordMismatch,
    #[detail(
        code = 1020205,
        http_status = 400,
        message_zh = "注册邀请码无效或已使用",
        message_en = "Invalid or used invitation code"
    )]
    InvalidInvitationCode,
    #[detail(
        code = 1020206,
        http_status = 400,
        message_zh = "请先同意用户协议和隐私政策",
        message_en = "Please accept the terms of service and privacy policy"
    )]
    TermsNotAccepted,
    #[detail(
        code = 1020207,
        http_status = 400,
        message_zh = "当前注册通道暂时关闭",
        message_en = "Registration is currently closed"
    )]
    RegistrationClosed,
    #[detail(
        code = 1020208,
        http_status = 400,
        message_zh = "激活链接无效或已过期",
        message_en = "Activation link invalid or expired"
    )]
    ActivationLinkInvalid,
    #[detail(
        code = 1020209,
        http_status = 400,
        message_zh = "账号激活失败，请稍后重试",
        message_en = "Account activation failed, please try again"
    )]
    ActivationFailed,
    #[detail(
        code = 1020210,
        http_status = 400,
        message_zh = "注册验证码错误",
        message_en = "Invalid registration verification code"
    )]
    RegistrationCodeInvalid,
    #[detail(
        code = 1020211,
        http_status = 429,
        message_zh = "验证码发送过于频繁，请稍后再试",
        message_en = "Verification code sent too frequently, please try again later"
    )]
    VerificationCodeThrottled,
    #[detail(
        code = 1020212,
        http_status = 500,
        message_zh = "验证码发送失败，请稍后重试",
        message_en = "Failed to send verification code, please try again"
    )]
    VerificationCodeSendFailed,

    // ===== 数据库相关错误（103xxxx）=====
    #[detail(
        code = 1030000,
        http_status = 500,
        message_zh = "数据处理失败，请稍后重试",
        message_en = "Database error, please try again later"
    )]
    DBCommon,
    #[detail(
        code = 1030001,
        http_status = 404,
        message_zh = "您查找的信息不存在",
        message_en = "Record not found"
    )]
    DBNotFound,
    #[detail(
        code = 1030002,
        http_status = 500,
        message_zh = "数据查询失败，请稍后重试",
        message_en = "Query failed, please try again later"
    )]
    DBQueryFailed,
    #[detail(
        code = 1030003,
        http_status = 500,
        message_zh = "数据更新失败，请稍后重试",
        message_en = "Update failed, please try again later"
    )]
    DBUpdateFailed,
    #[detail(
        code = 1030004,
        http_status = 500,
        message_zh = "数据保存失败，请稍后重试",
        message_en = "Insert failed, please try again later"
    )]
    DBInsertFailed,
    #[detail(
        code = 1030005,
        http_status = 500,
        message_zh = "数据删除失败，请稍后重试",
        message_en = "Delete failed, please try again later"
    )]
    DBDeleteFailed,
    #[detail(
        code = 1030006,
        http_status = 500,
        message_zh = "数据库连接失败，请稍后重试",
        message_en = "Database connection failed, please try again later"
    )]
    DBConnectionFailed,
    #[detail(
        code = 1030007,
        http_status = 500,
        message_zh = "操作执行失败，请稍后重试",
        message_en = "Transaction begin failed, please try again later"
    )]
    DBTransactionBeginFailed,
    #[detail(
        code = 1030008,
        http_status = 500,
        message_zh = "操作提交失败，请稍后重试",
        message_en = "Transaction commit failed, please try again later"
    )]
    DBTransactionCommitFailed,
    #[detail(
        code = 1030009,
        http_status = 409,
        message_zh = "数据已存在，请勿重复添加",
        message_en = "Duplicate entry, data already exists"
    )]
    DBDuplicateEntry,
    #[detail(
        code = 1030010,
        http_status = 500,
        message_zh = "数据迁移失败，请联系管理员",
        message_en = "Data migration failed, please contact administrator"
    )]
    DBDataMigrationFailed,
    #[detail(
        code = 1030011,
        http_status = 400,
        message_zh = "部分数据被其他内容引用，无法删除",
        message_en = "Cannot delete: data is referenced by other records"
    )]
    DBForeignKeyConstraint,
    #[detail(
        code = 1030012,
        http_status = 400,
        message_zh = "批量操作数量超出限制",
        message_en = "Batch operation size exceeds limit"
    )]
    DBBatchSizeExceeded,
    #[detail(
        code = 1030013,
        http_status = 500,
        message_zh = "数据回滚失败，请联系管理员",
        message_en = "Transaction rollback failed, please contact administrator"
    )]
    DBTransactionRollbackFailed,

    // ===== 缓存相关错误（104xxxx）=====
    #[detail(
        code = 1040000,
        http_status = 500,
        message_zh = "缓存服务暂时不可用",
        message_en = "Cache service error"
    )]
    CacheCommon,
    #[detail(
        code = 1040001,
        http_status = 500,
        message_zh = "数据加载失败，请稍后重试",
        message_en = "Cache query failed, please try again"
    )]
    CacheQueryFailed,
    #[detail(
        code = 1040002,
        http_status = 500,
        message_zh = "数据保存失败，请稍后重试",
        message_en = "Cache set failed, please try again"
    )]
    CacheSetFailed,
    #[detail(
        code = 1040003,
        http_status = 500,
        message_zh = "数据清理失败，请稍后重试",
        message_en = "Cache delete failed, please try again"
    )]
    CacheDeleteFailed,
    #[detail(
        code = 1040004,
        http_status = 500,
        message_zh = "缓存服务连接失败，请稍后重试",
        message_en = "Cache connection failed, please try again"
    )]
    CacheConnectionFailed,
    #[detail(
        code = 1040005,
        http_status = 404,
        message_zh = "缓存内容已过期，请刷新重试",
        message_en = "Cache key not found or expired"
    )]
    CacheKeyNotFound,

    // ===== LLM / Agent 相关错误（105xxxx）=====
    #[detail(
        code = 1050000,
        http_status = 500,
        message_zh = "AI 服务暂时不可用，请稍后重试",
        message_en = "LLM service unavailable, please try again"
    )]
    LLMError,
    #[detail(
        code = 1050001,
        http_status = 500,
        message_zh = "AI 服务初始化失败，请稍后重试",
        message_en = "LLM client build error, please try again"
    )]
    LLMClientBuildError,
    #[detail(
        code = 1050002,
        http_status = 500,
        message_zh = "AI 工具加载失败，请稍后重试",
        message_en = "LLM model load tool error"
    )]
    LLMModelLoadToolError,
    #[detail(
        code = 1050003,
        http_status = 429,
        message_zh = "AI 服务使用过于频繁，请稍后再试",
        message_en = "LLM rate limit exceeded, please try again later"
    )]
    LLMRateLimitExceeded,
    #[detail(
        code = 1050004,
        http_status = 504,
        message_zh = "AI 处理超时，请稍后重试或简化问题",
        message_en = "LLM request timeout, please try again"
    )]
    LLMTimeout,
    #[detail(
        code = 1050005,
        http_status = 400,
        message_zh = "对话内容过长，请精简后重试",
        message_en = "LLM content length exceeded limit"
    )]
    LLMContextLengthExceeded,
    #[detail(
        code = 1050006,
        http_status = 400,
        message_zh = "输入内容包含敏感信息，请调整后重试",
        message_en = "LLM content flagged as unsafe"
    )]
    LLMContentModeration,
    #[detail(
        code = 1050007,
        http_status = 500,
        message_zh = "当前 AI 模型暂不可用，请切换模型后重试",
        message_en = "Specified LLM model unavailable"
    )]
    LLMModelUnavailable,

    // ===== 命令执行相关错误（106xxxx）=====
    #[detail(
        code = 1060000,
        http_status = 500,
        message_zh = "操作执行失败，请稍后重试",
        message_en = "Command execution failed"
    )]
    CommandExecuteError,
    #[detail(
        code = 1060001,
        http_status = 504,
        message_zh = "操作执行超时，请稍后重试",
        message_en = "Command execution timeout"
    )]
    CommandExecuteTimeout,
    #[detail(
        code = 1060002,
        http_status = 403,
        message_zh = "该命令暂不允许执行",
        message_en = "Command execution not allowed"
    )]
    CommandNotAllowed,

    // ===== 文件存储相关错误（107xxxx）=====
    #[detail(
        code = 1070000,
        http_status = 500,
        message_zh = "文件服务暂时不可用",
        message_en = "Storage service error"
    )]
    StorageCommon,
    #[detail(
        code = 1070001,
        http_status = 500,
        message_zh = "文件上传失败，请稍后重试",
        message_en = "File upload failed, please try again"
    )]
    FileUploadFailed,
    #[detail(
        code = 1070002,
        http_status = 500,
        message_zh = "文件下载失败，请稍后重试",
        message_en = "File download failed, please try again"
    )]
    FileDownloadFailed,
    #[detail(
        code = 1070003,
        http_status = 404,
        message_zh = "文件不存在或已被删除",
        message_en = "File not found or deleted"
    )]
    FileNotFound,
    #[detail(
        code = 1070004,
        http_status = 400,
        message_zh = "不支持该文件格式，请上传其他格式",
        message_en = "Unsupported file type, please upload another format"
    )]
    FileTypeNotSupported,
    #[detail(
        code = 1070005,
        http_status = 400,
        message_zh = "文件过大，请压缩后重新上传",
        message_en = "File size exceeded limit, please compress and try again"
    )]
    FileSizeExceed,
    #[detail(
        code = 1070006,
        http_status = 500,
        message_zh = "存储服务连接失败，请稍后重试",
        message_en = "S3 connection failed, please try again"
    )]
    S3ConnectionFailed,
    #[detail(
        code = 1070007,
        http_status = 500,
        message_zh = "存储操作失败，请稍后重试",
        message_en = "S3 operation failed, please try again"
    )]
    S3OperationFailed,
    #[detail(
        code = 1070008,
        http_status = 400,
        message_zh = "文件名包含非法字符，请修改后重试",
        message_en = "Invalid filename, contains illegal characters"
    )]
    InvalidFileName,
    #[detail(
        code = 1070009,
        http_status = 400,
        message_zh = "上传内容为空，请选择文件后重试",
        message_en = "Uploaded file is empty"
    )]
    FileEmpty,
    #[detail(
        code = 1070010,
        http_status = 409,
        message_zh = "同名文件已存在",
        message_en = "File with the same name already exists"
    )]
    FileAlreadyExists,
    #[detail(
        code = 1070011,
        http_status = 400,
        message_zh = "图片尺寸超出限制",
        message_en = "Image dimensions exceeded limit"
    )]
    ImageDimensionExceeded,
    #[detail(
        code = 1070012,
        http_status = 400,
        message_zh = "文件损坏或格式不完整",
        message_en = "File corrupted or invalid format"
    )]
    FileCorrupted,
    #[detail(
        code = 1070013,
        http_status = 403,
        message_zh = "您没有权限下载此文件",
        message_en = "No permission to download this file"
    )]
    FileDownloadForbidden,

    // ===== 网络/RPC 相关错误（108xxxx）=====
    #[detail(
        code = 1080000,
        http_status = 500,
        message_zh = "网络请求失败，请检查网络后重试",
        message_en = "Network request failed, please check your connection"
    )]
    NetworkError,
    #[detail(
        code = 1080001,
        http_status = 500,
        message_zh = "服务调用失败，请稍后重试",
        message_en = "RPC call failed, please try again"
    )]
    RpcCallFailed,
    #[detail(
        code = 1080002,
        http_status = 500,
        message_zh = "服务调用失败，请稍后重试",
        message_en = "gRPC call failed, please try again"
    )]
    GrpcCallFailed,
    #[detail(
        code = 1080003,
        http_status = 504,
        message_zh = "服务响应超时，请稍后重试",
        message_en = "RPC call timeout, please try again"
    )]
    RpcCallTimeout,
    #[detail(
        code = 1080004,
        http_status = 502,
        message_zh = "网关响应异常，请稍后重试",
        message_en = "Bad gateway, please try again"
    )]
    GatewayResponseError,
    #[detail(
        code = 1080005,
        http_status = 502,
        message_zh = "服务暂时不可用，请稍后重试",
        message_en = "Upstream service unavailable"
    )]
    UpstreamUnavailable,
    #[detail(
        code = 1080006,
        http_status = 503,
        message_zh = "服务正在维护中，请稍后再试",
        message_en = "Service under maintenance, please try again later"
    )]
    ServiceUnderMaintenance,

    // ===== 消息队列相关错误（109xxxx）=====
    #[detail(
        code = 1090000,
        http_status = 500,
        message_zh = "消息服务暂时不可用",
        message_en = "Message queue service error"
    )]
    MQCommon,
    #[detail(
        code = 1090001,
        http_status = 500,
        message_zh = "消息发送失败，请稍后重试",
        message_en = "Kafka message send failed"
    )]
    KafkaSendFailed,
    #[detail(
        code = 1090002,
        http_status = 500,
        message_zh = "消息处理失败，请稍后重试",
        message_en = "Kafka message consume failed"
    )]
    KafkaConsumeFailed,
    #[detail(
        code = 1090003,
        http_status = 500,
        message_zh = "消息服务连接失败，请稍后重试",
        message_en = "RabbitMQ connection failed"
    )]
    RabbitMQConnectionFailed,
    #[detail(
        code = 1090004,
        http_status = 500,
        message_zh = "消息发送失败，请稍后重试",
        message_en = "RabbitMQ message send failed"
    )]
    RabbitMQSendFailed,
    #[detail(
        code = 1090005,
        http_status = 500,
        message_zh = "消息确认失败，请稍后重试",
        message_en = "Message ack failed"
    )]
    MQAckFailed,

    // ===== GraphQL 相关错误（110xxxx）=====
    #[detail(
        code = 1100000,
        http_status = 400,
        message_zh = "查询格式有误，请检查后重试",
        message_en = "GraphQL query error"
    )]
    GraphQLQueryError,
    #[detail(
        code = 1100001,
        http_status = 400,
        message_zh = "查询解析失败，请检查语法",
        message_en = "GraphQL parse error"
    )]
    GraphQLParseError,
    #[detail(
        code = 1100002,
        http_status = 400,
        message_zh = "查询参数校验失败",
        message_en = "GraphQL validation error"
    )]
    GraphQLValidationError,

    // ===== WebSocket 相关错误（111xxxx）=====
    #[detail(
        code = 1110000,
        http_status = 500,
        message_zh = "实时连接失败，请刷新页面重试",
        message_en = "WebSocket connection error"
    )]
    WebSocketConnectError,
    #[detail(
        code = 1110001,
        http_status = 500,
        message_zh = "消息发送失败，请稍后重试",
        message_en = "WebSocket message send failed"
    )]
    WebSocketSendFailed,
    #[detail(
        code = 1110002,
        http_status = 504,
        message_zh = "连接已超时，请刷新页面重试",
        message_en = "WebSocket timeout, please refresh the page"
    )]
    WebSocketTimeout,
    #[detail(
        code = 1110003,
        http_status = 404,
        message_zh = "连接已断开，请刷新页面重试",
        message_en = "WebSocket session not found"
    )]
    WebSocketSessionNotFound,

    // ===== 限流相关错误（112xxxx）=====
    #[detail(
        code = 1120000,
        http_status = 429,
        message_zh = "您操作太快啦，喝口水休息一下吧",
        message_en = "Rate limit exceeded, please slow down"
    )]
    RateLimitExceeded,
    #[detail(
        code = 1120001,
        http_status = 429,
        message_zh = "当前网络环境访问受限，请稍后再试",
        message_en = "IP rate limit exceeded"
    )]
    IPRateLimitExceeded,
    #[detail(
        code = 1120002,
        http_status = 429,
        message_zh = "您的操作过于频繁，请稍后再试",
        message_en = "User rate limit exceeded"
    )]
    UserRateLimitExceeded,
    #[detail(
        code = 1120003,
        http_status = 429,
        message_zh = "当前接口繁忙，请稍后再试",
        message_en = "API rate limit exceeded"
    )]
    ApiRateLimitExceeded,
    #[detail(
        code = 1120004,
        http_status = 429,
        message_zh = "并发请求过多，请稍后再试",
        message_en = "Concurrency limit exceeded"
    )]
    ConcurrencyLimitExceeded,

    // ===== 配置相关错误（113xxxx）=====
    #[detail(
        code = 1130000,
        http_status = 500,
        message_zh = "系统配置加载失败，请联系管理员",
        message_en = "Config load error, contact administrator"
    )]
    ConfigLoadError,
    #[detail(
        code = 1130001,
        http_status = 500,
        message_zh = "系统配置解析失败，请联系管理员",
        message_en = "Config parse error, contact administrator"
    )]
    ConfigParseError,
    #[detail(
        code = 1130002,
        http_status = 500,
        message_zh = "系统配置缺失，请联系管理员",
        message_en = "Required config item missing, contact administrator"
    )]
    ConfigItemMissing,
    #[detail(
        code = 1130003,
        http_status = 500,
        message_zh = "环境配置错误，请联系管理员",
        message_en = "Environment variable error, contact administrator"
    )]
    EnvVarError,
    #[detail(
        code = 1130004,
        http_status = 400,
        message_zh = "该功能尚未开启，请先在设置中启用",
        message_en = "Feature not enabled, please enable it in settings"
    )]
    FeatureNotEnabled,

    // ===== 业务校验相关错误（114xxxx）=====
    #[detail(
        code = 1140000,
        http_status = 400,
        message_zh = "当前操作不符合业务规则，请检查后重试",
        message_en = "Business rule violation"
    )]
    BusinessRuleViolation,
    #[detail(
        code = 1140001,
        http_status = 400,
        message_zh = "数据状态异常，请刷新后重试",
        message_en = "Data state error, please refresh and try again"
    )]
    DataStateError,
    #[detail(
        code = 1140002,
        http_status = 409,
        message_zh = "操作冲突，该数据正在被其他人处理",
        message_en = "Operation conflict, data is being modified by others"
    )]
    OperationConflict,
    #[detail(
        code = 1140003,
        http_status = 409,
        message_zh = "该名称已被使用，请换一个再试",
        message_en = "Name already in use, please try another"
    )]
    DataAlreadyOccupied,
    #[detail(
        code = 1140004,
        http_status = 400,
        message_zh = "关联数据检查失败，请检查相关数据",
        message_en = "Data relation validation failed"
    )]
    DataRelationError,
    #[detail(
        code = 1140005,
        http_status = 400,
        message_zh = "状态流转不合法，请检查当前状态",
        message_en = "Invalid state transition"
    )]
    InvalidStateTransition,
    #[detail(
        code = 1140006,
        http_status = 400,
        message_zh = "该数据已被其他人修改，请刷新后重试",
        message_en = "Data version mismatch, please refresh and try again"
    )]
    OptimisticLockFailed,
    #[detail(
        code = 1140007,
        http_status = 400,
        message_zh = "数据已过期，请刷新后重试",
        message_en = "Data expired, please refresh and try again"
    )]
    DataExpired,
    #[detail(
        code = 1140008,
        http_status = 400,
        message_zh = "请勿删除包含子内容的项目",
        message_en = "Cannot delete item that still has children"
    )]
    CannotDeleteWithChildren,
    #[detail(
        code = 1140009,
        http_status = 400,
        message_zh = "当前状态下无法执行此操作",
        message_en = "Operation not allowed in current state"
    )]
    OperationNotAllowedInState,
    #[detail(
        code = 1140010,
        http_status = 400,
        message_zh = "操作已过期或已被处理",
        message_en = "Operation already processed or expired"
    )]
    OperationAlreadyProcessed,
    #[detail(
        code = 1140011,
        http_status = 400,
        message_zh = "日期范围无效，结束日期不能早于开始日期",
        message_en = "Invalid date range: end date before start date"
    )]
    InvalidDateRange,
    #[detail(
        code = 1140012,
        http_status = 400,
        message_zh = "数值超出允许范围",
        message_en = "Numeric value out of allowed range"
    )]
    NumericValueOutOfRange,

    // ===== 邮件/短信通知相关错误（115xxxx）=====
    #[detail(
        code = 1150000,
        http_status = 500,
        message_zh = "邮件发送失败，请稍后重试",
        message_en = "Email send failed, please try again"
    )]
    EmailSendFailed,
    #[detail(
        code = 1150001,
        http_status = 500,
        message_zh = "短信发送失败，请稍后重试",
        message_en = "SMS send failed, please try again"
    )]
    SMSSendFailed,
    #[detail(
        code = 1150002,
        http_status = 503,
        message_zh = "通知服务暂时不可用，请稍后重试",
        message_en = "Notification service unavailable"
    )]
    NotificationServiceUnavailable,
    #[detail(
        code = 1150003,
        http_status = 400,
        message_zh = "短信验证码错误",
        message_en = "Invalid SMS verification code"
    )]
    SMSCodeInvalid,
    #[detail(
        code = 1150004,
        http_status = 400,
        message_zh = "邮箱验证码错误",
        message_en = "Invalid email verification code"
    )]
    EmailCodeInvalid,
    #[detail(
        code = 1150005,
        http_status = 400,
        message_zh = "短信验证码已过期，请重新获取",
        message_en = "SMS verification code expired"
    )]
    SMSCodeExpired,
    #[detail(
        code = 1150006,
        http_status = 400,
        message_zh = "邮箱验证码已过期，请重新获取",
        message_en = "Email verification code expired"
    )]
    EmailCodeExpired,
    #[detail(
        code = 1150007,
        http_status = 400,
        message_zh = "推送通知发送失败，请稍后重试",
        message_en = "Push notification send failed"
    )]
    PushNotificationFailed,
    #[detail(
        code = 1150008,
        http_status = 403,
        message_zh = "该用户未开启消息通知",
        message_en = "User has disabled notifications"
    )]
    NotificationDisabled,

    // ===== 支付相关错误（116xxxx）=====
    #[detail(
        code = 1160000,
        http_status = 500,
        message_zh = "支付服务暂时不可用，请稍后重试",
        message_en = "Payment service error"
    )]
    PaymentError,
    #[detail(
        code = 1160001,
        http_status = 400,
        message_zh = "支付信息有误，请检查后重新提交",
        message_en = "Invalid payment parameters"
    )]
    PaymentParamError,
    #[detail(
        code = 1160002,
        http_status = 503,
        message_zh = "当前支付渠道维护中，请选择其他方式",
        message_en = "Payment channel unavailable"
    )]
    PaymentChannelUnavailable,
    #[detail(
        code = 1160003,
        http_status = 400,
        message_zh = "支付金额不正确，请检查后重试",
        message_en = "Invalid payment amount"
    )]
    PaymentAmountInvalid,
    #[detail(
        code = 1160004,
        http_status = 409,
        message_zh = "该订单已支付，请勿重复付款",
        message_en = "Order already paid"
    )]
    OrderAlreadyPaid,
    #[detail(
        code = 1160005,
        http_status = 408,
        message_zh = "支付超时，请重新发起支付",
        message_en = "Order payment timeout"
    )]
    OrderPaymentTimeout,
    #[detail(
        code = 1160006,
        http_status = 400,
        message_zh = "订单不存在",
        message_en = "Order not found"
    )]
    OrderNotFound,
    #[detail(
        code = 1160007,
        http_status = 400,
        message_zh = "订单已取消，无法继续支付",
        message_en = "Order cancelled, cannot proceed with payment"
    )]
    OrderCancelled,
    #[detail(
        code = 1160008,
        http_status = 400,
        message_zh = "余额不足，请充值或更换支付方式",
        message_en = "Insufficient balance"
    )]
    InsufficientBalance,
    #[detail(
        code = 1160009,
        http_status = 400,
        message_zh = "退款失败，请稍后重试",
        message_en = "Refund failed, please try again"
    )]
    RefundFailed,
    #[detail(
        code = 1160010,
        http_status = 409,
        message_zh = "该订单已申请退款",
        message_en = "Refund already requested for this order"
    )]
    RefundAlreadyRequested,

    // ===== 外部 API 调用相关错误（117xxxx）=====
    #[detail(
        code = 1170000,
        http_status = 502,
        message_zh = "第三方服务调用失败，请稍后重试",
        message_en = "External API call failed, please try again"
    )]
    ExternalApiCallFailed,
    #[detail(
        code = 1170001,
        http_status = 504,
        message_zh = "第三方服务响应超时，请稍后重试",
        message_en = "External API timeout, please try again"
    )]
    ExternalApiTimeout,
    #[detail(
        code = 1170002,
        http_status = 502,
        message_zh = "第三方服务暂时不可用，请稍后重试",
        message_en = "Third party service unavailable"
    )]
    ThirdPartyServiceUnavailable,
    #[detail(
        code = 1170003,
        http_status = 502,
        message_zh = "第三方返回数据异常，请稍后重试",
        message_en = "Third party returned invalid data"
    )]
    ThirdPartyDataFormatError,
    #[detail(
        code = 1170004,
        http_status = 401,
        message_zh = "第三方服务授权失败，请重新授权",
        message_en = "Third party authentication failed"
    )]
    ThirdPartyAuthFailed,

    // ===== 数据解析相关错误（118xxxx）=====
    #[detail(
        code = 1180000,
        http_status = 400,
        message_zh = "Excel 解析失败，请检查文件格式是否正确",
        message_en = "Excel parse error, please check the file format"
    )]
    ExcelParseError,
    #[detail(
        code = 1180001,
        http_status = 400,
        message_zh = "CSV 解析失败，请检查文件格式是否正确",
        message_en = "CSV parse error, please check the file format"
    )]
    CsvParseError,
    #[detail(
        code = 1180002,
        http_status = 400,
        message_zh = "XML 解析失败，请检查文件格式是否正确",
        message_en = "XML parse error, please check the file format"
    )]
    XmlParseError,
    #[detail(
        code = 1180003,
        http_status = 400,
        message_zh = "数据转换失败，请检查输入内容",
        message_en = "Data transform error, please check the input"
    )]
    DataTransformError,
    #[detail(
        code = 1180004,
        http_status = 400,
        message_zh = "数据导入失败，请检查导入文件内容",
        message_en = "Data import failed, please check the import file"
    )]
    DataImportFailed,
    #[detail(
        code = 1180005,
        http_status = 500,
        message_zh = "数据导出失败，请稍后重试",
        message_en = "Data export failed, please try again"
    )]
    DataExportFailed,
    #[detail(
        code = 1180006,
        http_status = 400,
        message_zh = "导入的列名与模板不匹配，请下载最新模板",
        message_en = "Import header mismatch, please download the latest template"
    )]
    ImportHeaderMismatch,
    #[detail(
        code = 1180007,
        http_status = 400,
        message_zh = "导入数据为空，请填写数据后重新上传",
        message_en = "Import data is empty"
    )]
    ImportEmptyData,
    #[detail(
        code = 1180008,
        http_status = 400,
        message_zh = "部分行导入失败，请检查导入报告",
        message_en = "Some rows failed to import, please check the report"
    )]
    ImportPartialFailed,
    #[detail(
        code = 1180009,
        http_status = 400,
        message_zh = "导出数量超出限制，请筛选后重试",
        message_en = "Export amount exceeded limit, please filter first"
    )]
    ExportSizeExceeded,

    // ===== 定时任务相关错误（119xxxx）=====
    #[detail(
        code = 1190000,
        http_status = 500,
        message_zh = "定时任务执行失败，请联系管理员",
        message_en = "Cron task error"
    )]
    CronTaskError,
    #[detail(
        code = 1190001,
        http_status = 500,
        message_zh = "任务调度失败，请稍后重试",
        message_en = "Cron schedule failed"
    )]
    CronScheduleFailed,
    #[detail(
        code = 1190002,
        http_status = 500,
        message_zh = "任务执行失败，请稍后重试",
        message_en = "Cron execute failed"
    )]
    CronExecuteFailed,
    #[detail(
        code = 1190003,
        http_status = 400,
        message_zh = "任务配置有误，请检查后重试",
        message_en = "Cron config error"
    )]
    CronConfigError,
    #[detail(
        code = 1190004,
        http_status = 409,
        message_zh = "任务正在执行中，请勿重复操作",
        message_en = "Task already running"
    )]
    TaskAlreadyRunning,
    #[detail(
        code = 1190005,
        http_status = 400,
        message_zh = "该任务暂不支持手动触发",
        message_en = "Manual task trigger not allowed"
    )]
    ManualTriggerNotAllowed,

    // ===== 分布式锁相关错误（120xxxx）=====
    #[detail(
        code = 1200000,
        http_status = 500,
        message_zh = "操作处理失败，请稍后重试",
        message_en = "Distributed lock error"
    )]
    DistributedLockError,
    #[detail(
        code = 1200001,
        http_status = 409,
        message_zh = "该资源正在被处理，请稍后再试",
        message_en = "Failed to acquire lock, please try again"
    )]
    LockAcquireFailed,
    #[detail(
        code = 1200002,
        http_status = 500,
        message_zh = "操作释放失败，请稍后重试",
        message_en = "Lock release failed"
    )]
    LockReleaseFailed,
    #[detail(
        code = 1200003,
        http_status = 409,
        message_zh = "该内容正在编辑中，请稍后再试",
        message_en = "Lock already occupied"
    )]
    LockAlreadyOccupied,
    #[detail(
        code = 1200004,
        http_status = 408,
        message_zh = "等待处理超时，请稍后再试",
        message_en = "Lock wait timeout"
    )]
    LockWaitTimeout,

    // ===== 分布式 ID 生成相关错误（121xxxx）=====
    #[detail(
        code = 1210000,
        http_status = 500,
        message_zh = "标识生成失败，请稍后重试",
        message_en = "ID generation error"
    )]
    IdGenerateError,
    #[detail(
        code = 1210001,
        http_status = 500,
        message_zh = "标识生成失败，请稍后重试",
        message_en = "Snowflake ID generation failed"
    )]
    SnowflakeIdFailed,
    #[detail(
        code = 1210002,
        http_status = 500,
        message_zh = "唯一标识生成失败，请稍后重试",
        message_en = "UUID generation failed"
    )]
    UUIDGenerateFailed,

    // ===== 加密解密相关错误（122xxxx）=====
    #[detail(
        code = 1220000,
        http_status = 500,
        message_zh = "数据加密处理失败，请稍后重试",
        message_en = "Crypto error"
    )]
    CryptoError,
    #[detail(
        code = 1220001,
        http_status = 500,
        message_zh = "数据加密失败，请稍后重试",
        message_en = "AES encryption failed"
    )]
    AesEncryptFailed,
    #[detail(
        code = 1220002,
        http_status = 500,
        message_zh = "数据解密失败，请检查数据是否完整",
        message_en = "AES decryption failed"
    )]
    AesDecryptFailed,
    #[detail(
        code = 1220003,
        http_status = 500,
        message_zh = "数据加密失败，请稍后重试",
        message_en = "RSA encryption failed"
    )]
    RsaEncryptFailed,
    #[detail(
        code = 1220004,
        http_status = 500,
        message_zh = "数据解密失败，请检查数据是否完整",
        message_en = "RSA decryption failed"
    )]
    RsaDecryptFailed,
    #[detail(
        code = 1220005,
        http_status = 400,
        message_zh = "签名验证失败，数据可能已被篡改",
        message_en = "Signature verification failed, data may be tampered"
    )]
    SignatureVerifyFailed,
    #[detail(
        code = 1220006,
        http_status = 400,
        message_zh = "密钥无效，请检查后重试",
        message_en = "Invalid key"
    )]
    InvalidKey,
    #[detail(
        code = 1220007,
        http_status = 400,
        message_zh = "Token 签名无效",
        message_en = "Invalid token signature"
    )]
    InvalidTokenSignature,

    // ===== 熔断降级相关错误（123xxxx）=====
    #[detail(
        code = 1230000,
        http_status = 503,
        message_zh = "服务繁忙，请稍后再试",
        message_en = "Service temporarily unavailable (circuit breaker open)"
    )]
    CircuitBreakerOpen,
    #[detail(
        code = 1230001,
        http_status = 503,
        message_zh = "系统繁忙，部分功能暂时不可用，请稍后再试",
        message_en = "Service degraded, please try again later"
    )]
    ServiceDegraded,
    #[detail(
        code = 1230002,
        http_status = 503,
        message_zh = "服务暂时不可用，请稍后再试",
        message_en = "Service unavailable"
    )]
    ServiceUnavailable,

    // ===== 链路追踪相关错误（124xxxx）=====
    #[detail(
        code = 1240000,
        http_status = 500,
        message_zh = "链路追踪服务异常",
        message_en = "Tracing service error"
    )]
    TracingError,
    #[detail(
        code = 1240001,
        http_status = 500,
        message_zh = "链路追踪创建失败",
        message_en = "Span create failed"
    )]
    SpanCreateFailed,
    #[detail(
        code = 1240002,
        http_status = 500,
        message_zh = "链路追踪上报失败",
        message_en = "Trace report failed"
    )]
    TraceReportFailed,

    // ===== 日志相关错误（125xxxx）=====
    #[detail(
        code = 1250000,
        http_status = 500,
        message_zh = "日志写入失败",
        message_en = "Log write error"
    )]
    LogWriteError,
    #[detail(
        code = 1250001,
        http_status = 500,
        message_zh = "日志配置错误",
        message_en = "Log config error"
    )]
    LogConfigError,

    // ===== 国际化相关错误（126xxxx）=====
    #[detail(
        code = 1260000,
        http_status = 500,
        message_zh = "国际化服务错误",
        message_en = "I18n error"
    )]
    I18nError,
    #[detail(
        code = 1260001,
        http_status = 400,
        message_zh = "语言包加载失败",
        message_en = "Locale load failed"
    )]
    LocaleLoadFailed,
    #[detail(
        code = 1260002,
        http_status = 400,
        message_zh = "翻译内容缺失",
        message_en = "Translation key not found"
    )]
    TranslationKeyNotFound,

    // ===== 连接池相关错误（127xxxx）=====
    #[detail(
        code = 1270000,
        http_status = 500,
        message_zh = "连接池服务错误，请稍后重试",
        message_en = "Connection pool error"
    )]
    ConnectionPoolError,
    #[detail(
        code = 1270001,
        http_status = 503,
        message_zh = "系统繁忙，请稍后再试",
        message_en = "Connection pool exhausted"
    )]
    ConnectionPoolExhausted,
    #[detail(
        code = 1270002,
        http_status = 504,
        message_zh = "获取连接超时，请稍后重试",
        message_en = "Connection acquire timeout"
    )]
    ConnectionAcquireTimeout,

    // ===== 灰度发布相关错误（128xxxx）=====
    #[detail(
        code = 1280000,
        http_status = 500,
        message_zh = "灰度发布错误",
        message_en = "Gray release error"
    )]
    GrayReleaseError,
    #[detail(
        code = 1280001,
        http_status = 400,
        message_zh = "灰度规则配置错误",
        message_en = "Gray rule config error"
    )]
    GrayRuleConfigError,
    #[detail(
        code = 1280002,
        http_status = 500,
        message_zh = "灰度分流失败",
        message_en = "Gray route failed"
    )]
    GrayRouteFailed,

    // ===== 审计日志相关错误（129xxxx）=====
    #[detail(
        code = 1290000,
        http_status = 500,
        message_zh = "审计日志错误",
        message_en = "Audit log error"
    )]
    AuditLogError,
    #[detail(
        code = 1290001,
        http_status = 500,
        message_zh = "审计日志记录失败",
        message_en = "Audit log record failed"
    )]
    AuditLogRecordFailed,

    // ===== 资源配额相关错误（130xxxx）=====
    #[detail(
        code = 1300000,
        http_status = 400,
        message_zh = "资源配额错误",
        message_en = "Resource quota error"
    )]
    ResourceQuotaError,
    #[detail(
        code = 1300001,
        http_status = 400,
        message_zh = "资源已用完，请升级套餐或删除部分内容",
        message_en = "Resource quota exceeded"
    )]
    ResourceQuotaExceeded,
    #[detail(
        code = 1300002,
        http_status = 400,
        message_zh = "已达上限，请升级套餐后再试",
        message_en = "Resource limit reached"
    )]
    ResourceLimitReached,
    #[detail(
        code = 1300003,
        http_status = 400,
        message_zh = "存储空间已满，请清理后重试",
        message_en = "Storage space full, please clean up and try again"
    )]
    StorageSpaceFull,
    #[detail(
        code = 1300004,
        http_status = 400,
        message_zh = "成员数量已达上限",
        message_en = "Member count limit reached"
    )]
    MemberLimitReached,

    // ===== 租户/组织相关错误（131xxxx）=====
    #[detail(
        code = 1310000,
        http_status = 404,
        message_zh = "组织不存在或已解散",
        message_en = "Tenant not found or disbanded"
    )]
    TenantNotFound,
    #[detail(
        code = 1310001,
        http_status = 409,
        message_zh = "该组织名称已被使用",
        message_en = "Tenant name already exists"
    )]
    TenantNameExists,
    #[detail(
        code = 1310002,
        http_status = 403,
        message_zh = "您不是该组织成员",
        message_en = "You are not a member of this tenant"
    )]
    NotTenantMember,
    #[detail(
        code = 1310003,
        http_status = 403,
        message_zh = "您已被移出该组织",
        message_en = "You have been removed from this tenant"
    )]
    TenantMembershipRevoked,
    #[detail(
        code = 1310004,
        http_status = 400,
        message_zh = "该成员已在组织中",
        message_en = "Member already in tenant"
    )]
    MemberAlreadyExists,
    #[detail(
        code = 1310005,
        http_status = 400,
        message_zh = "邀请链接无效或已过期",
        message_en = "Invitation link invalid or expired"
    )]
    InvitationInvalid,
    #[detail(
        code = 1310006,
        http_status = 400,
        message_zh = "不能移除组织创建者",
        message_en = "Cannot remove the tenant owner"
    )]
    CannotRemoveTenantOwner,
    #[detail(
        code = 1310007,
        http_status = 400,
        message_zh = "组织名称不能为空",
        message_en = "Tenant name cannot be empty"
    )]
    TenantNameEmpty,
    #[detail(
        code = 1310008,
        http_status = 403,
        message_zh = "当前组织已停用，请联系管理员",
        message_en = "Tenant disabled, please contact administrator"
    )]
    TenantDisabled,
    #[detail(
        code = 1310009,
        http_status = 400,
        message_zh = "您已加入的组织数量已达上限",
        message_en = "Tenant membership limit reached"
    )]
    TenantMembershipLimit,
    #[detail(
        code = 1310010,
        http_status = 409,
        message_zh = "您的入组申请正在审核中，请勿重复提交",
        message_en = "Join request already pending"
    )]
    JoinRequestAlreadyPending,
    #[detail(
        code = 1310011,
        http_status = 400,
        message_zh = "角色不存在",
        message_en = "Role not found"
    )]
    RoleNotFound,
    #[detail(
        code = 1310012,
        http_status = 409,
        message_zh = "角色名称已存在",
        message_en = "Role name already exists"
    )]
    RoleNameAlreadyExists,
    #[detail(
        code = 1310013,
        http_status = 400,
        message_zh = "部门不存在",
        message_en = "Department not found"
    )]
    DepartmentNotFound,
    #[detail(
        code = 1310014,
        http_status = 400,
        message_zh = "不能将部门移动到其子部门下",
        message_en = "Cannot move department to its own child"
    )]
    InvalidDepartmentHierarchy,

    // ===== 工作流/审批相关错误（132xxxx）=====
    #[detail(
        code = 1320000,
        http_status = 404,
        message_zh = "流程不存在",
        message_en = "Workflow not found"
    )]
    WorkflowNotFound,
    #[detail(
        code = 1320001,
        http_status = 400,
        message_zh = "流程已结束，无法继续操作",
        message_en = "Workflow already completed"
    )]
    WorkflowCompleted,
    #[detail(
        code = 1320002,
        http_status = 400,
        message_zh = "当前无需您审批",
        message_en = "Not your turn to approve"
    )]
    NotYourApprovalTurn,
    #[detail(
        code = 1320003,
        http_status = 400,
        message_zh = "流程已被撤回",
        message_en = "Workflow withdrawn"
    )]
    WorkflowWithdrawn,
    #[detail(
        code = 1320004,
        http_status = 400,
        message_zh = "该流程状态下不允许撤回",
        message_en = "Withdrawal not allowed in current state"
    )]
    WithdrawalNotAllowed,
    #[detail(
        code = 1320005,
        http_status = 400,
        message_zh = "审批意见不能为空",
        message_en = "Approval comment cannot be empty"
    )]
    ApprovalCommentEmpty,
    #[detail(
        code = 1320006,
        http_status = 400,
        message_zh = "未找到审批配置，请联系管理员",
        message_en = "Approval flow not configured"
    )]
    ApprovalFlowNotConfigured,
    #[detail(
        code = 1320007,
        http_status = 400,
        message_zh = "该流程已被其他人处理",
        message_en = "Workflow already processed by others"
    )]
    WorkflowAlreadyProcessed,
    #[detail(
        code = 1320008,
        http_status = 409,
        message_zh = "您已提交过审批，请勿重复操作",
        message_en = "Approval already submitted"
    )]
    ApprovalAlreadySubmitted,

    // ===== 社交/好友/关注相关错误（133xxxx）=====
    #[detail(
        code = 1330000,
        http_status = 404,
        message_zh = "用户不存在",
        message_en = "User not found"
    )]
    UserNotFound,
    #[detail(
        code = 1330001,
        http_status = 409,
        message_zh = "你们已经是好友了",
        message_en = "You are already friends"
    )]
    AlreadyFriends,
    #[detail(
        code = 1330002,
        http_status = 400,
        message_zh = "不能添加自己为好友",
        message_en = "Cannot add yourself as friend"
    )]
    CannotAddSelfAsFriend,
    #[detail(
        code = 1330003,
        http_status = 409,
        message_zh = "好友申请已发送，请耐心等待",
        message_en = "Friend request already sent"
    )]
    FriendRequestAlreadySent,
    #[detail(
        code = 1330004,
        http_status = 403,
        message_zh = "对方已拒绝接受好友请求",
        message_en = "Friend request blocked by user"
    )]
    FriendRequestBlocked,
    #[detail(
        code = 1330005,
        http_status = 400,
        message_zh = "您已关注该用户",
        message_en = "Already following this user"
    )]
    AlreadyFollowing,
    #[detail(
        code = 1330006,
        http_status = 400,
        message_zh = "您还未关注该用户",
        message_en = "Not following this user"
    )]
    NotFollowing,
    #[detail(
        code = 1330007,
        http_status = 403,
        message_zh = "对方已将您拉黑",
        message_en = "You have been blocked by this user"
    )]
    BlockedByUser,
    #[detail(
        code = 1330008,
        http_status = 400,
        message_zh = "不能关注自己",
        message_en = "Cannot follow yourself"
    )]
    CannotFollowSelf,
    #[detail(
        code = 1330009,
        http_status = 403,
        message_zh = "该用户已设置为私密账号",
        message_en = "This user has a private account"
    )]
    UserAccountPrivate,

    // ===== 内容管理相关错误（134xxxx）=====
    #[detail(
        code = 1340000,
        http_status = 404,
        message_zh = "内容不存在或已被删除",
        message_en = "Content not found or deleted"
    )]
    ContentNotFound,
    #[detail(
        code = 1340001,
        http_status = 403,
        message_zh = "您没有权限编辑此内容",
        message_en = "No permission to edit this content"
    )]
    ContentEditForbidden,
    #[detail(
        code = 1340002,
        http_status = 403,
        message_zh = "该内容已发布，无法继续编辑",
        message_en = "Content already published, cannot edit"
    )]
    ContentAlreadyPublished,
    #[detail(
        code = 1340003,
        http_status = 400,
        message_zh = "内容审核未通过，请修改后重新提交",
        message_en = "Content rejected by moderation"
    )]
    ContentRejected,
    #[detail(
        code = 1340004,
        http_status = 400,
        message_zh = "内容正在审核中，请耐心等待",
        message_en = "Content under review"
    )]
    ContentUnderReview,
    #[detail(
        code = 1340005,
        http_status = 400,
        message_zh = "标题不能为空",
        message_en = "Title cannot be empty"
    )]
    TitleEmpty,
    #[detail(
        code = 1340006,
        http_status = 400,
        message_zh = "内容包含敏感词，请修改后重试",
        message_en = "Content contains sensitive words, please revise"
    )]
    ContentContainsSensitiveWords,
    #[detail(
        code = 1340007,
        http_status = 400,
        message_zh = "内容分类不存在",
        message_en = "Content category not found"
    )]
    CategoryNotFound,
    #[detail(
        code = 1340008,
        http_status = 400,
        message_zh = "标签不存在",
        message_en = "Tag not found"
    )]
    TagNotFound,
    #[detail(
        code = 1340009,
        http_status = 400,
        message_zh = "文章标题过长，请精简后重试",
        message_en = "Title too long"
    )]
    TitleTooLong,
    #[detail(
        code = 1340010,
        http_status = 400,
        message_zh = "评论内容不能为空",
        message_en = "Comment cannot be empty"
    )]
    CommentEmpty,
    #[detail(
        code = 1340011,
        http_status = 403,
        message_zh = "评论功能已关闭",
        message_en = "Comments disabled"
    )]
    CommentsDisabled,
    #[detail(
        code = 1340012,
        http_status = 400,
        message_zh = "您已点赞，请勿重复操作",
        message_en = "Already liked"
    )]
    AlreadyLiked,
    #[detail(
        code = 1340013,
        http_status = 400,
        message_zh = "您还未点赞",
        message_en = "Not liked yet"
    )]
    NotLikedYet,
    #[detail(
        code = 1340014,
        http_status = 400,
        message_zh = "请勿重复收藏",
        message_en = "Already favorited"
    )]
    AlreadyFavorited,
    #[detail(
        code = 1340015,
        http_status = 400,
        message_zh = "您还未收藏",
        message_en = "Not favorited yet"
    )]
    NotFavoritedYet,
    #[detail(
        code = 1340016,
        http_status = 403,
        message_zh = "该内容仅作者可见",
        message_en = "Content visible to author only"
    )]
    ContentAuthorOnly,
    #[detail(
        code = 1340017,
        http_status = 403,
        message_zh = "该内容仅成员可见",
        message_en = "Content visible to members only"
    )]
    ContentMembersOnly,

    // ===== 版本控制/更新相关错误（135xxxx）=====
    #[detail(
        code = 1350000,
        http_status = 404,
        message_zh = "版本记录不存在",
        message_en = "Version not found"
    )]
    VersionNotFound,
    #[detail(
        code = 1350001,
        http_status = 409,
        message_zh = "版本号冲突，请刷新后重试",
        message_en = "Version conflict, please refresh and try again"
    )]
    VersionConflict,
    #[detail(
        code = 1350002,
        http_status = 400,
        message_zh = "请更新至最新版本后重试",
        message_en = "Please update to the latest version"
    )]
    VersionOutdated,
    #[detail(
        code = 1350003,
        http_status = 400,
        message_zh = "当前已是最新版本",
        message_en = "Already on the latest version"
    )]
    AlreadyLatestVersion,
    #[detail(
        code = 1350004,
        http_status = 400,
        message_zh = "版本恢复失败",
        message_en = "Version restore failed"
    )]
    VersionRestoreFailed,
    #[detail(
        code = 1350005,
        http_status = 400,
        message_zh = "当前版本不支持此功能",
        message_en = "Feature not supported in current version"
    )]
    FeatureNotSupportedInVersion,

    // ===== 设备管理相关错误（136xxxx）=====
    #[detail(
        code = 1360000,
        http_status = 404,
        message_zh = "设备不存在",
        message_en = "Device not found"
    )]
    DeviceNotFound,
    #[detail(
        code = 1360001,
        http_status = 409,
        message_zh = "该设备已绑定其他账号",
        message_en = "Device already bound to another account"
    )]
    DeviceAlreadyBound,
    #[detail(
        code = 1360002,
        http_status = 400,
        message_zh = "设备未绑定账号",
        message_en = "Device not bound to any account"
    )]
    DeviceNotBound,
    #[detail(
        code = 1360003,
        http_status = 400,
        message_zh = "设备授权码无效或已过期",
        message_en = "Device activation code invalid or expired"
    )]
    DeviceActivationCodeInvalid,
    #[detail(
        code = 1360004,
        http_status = 403,
        message_zh = "设备已被禁用，请联系管理员",
        message_en = "Device disabled, please contact administrator"
    )]
    DeviceDisabled,
    #[detail(
        code = 1360005,
        http_status = 400,
        message_zh = "设备离线，请检查连接后重试",
        message_en = "Device offline, please check connection"
    )]
    DeviceOffline,
    #[detail(
        code = 1360006,
        http_status = 400,
        message_zh = "绑定设备数量已达上限",
        message_en = "Device binding limit reached"
    )]
    DeviceBindingLimitReached,

    // ===== 通知/消息中心相关错误（137xxxx）=====
    #[detail(
        code = 1370000,
        http_status = 404,
        message_zh = "消息不存在",
        message_en = "Message not found"
    )]
    MessageNotFound,
    #[detail(
        code = 1370001,
        http_status = 400,
        message_zh = "不能给自己发消息",
        message_en = "Cannot send message to yourself"
    )]
    CannotSendToSelf,
    #[detail(
        code = 1370002,
        http_status = 403,
        message_zh = "消息发送过于频繁，请稍后再试",
        message_en = "Message sending too frequent"
    )]
    MessageSendThrottled,
    #[detail(
        code = 1370003,
        http_status = 400,
        message_zh = "消息内容为空或过长",
        message_en = "Message content empty or too long"
    )]
    MessageContentInvalid,
    #[detail(
        code = 1370004,
        http_status = 403,
        message_zh = "对方拒收您的消息",
        message_en = "Recipient blocked your messages"
    )]
    RecipientBlockedMessages,
    #[detail(
        code = 1370005,
        http_status = 400,
        message_zh = "收件箱已满，请清理后重试",
        message_en = "Inbox full, please clean up"
    )]
    InboxFull,
    #[detail(
        code = 1370006,
        http_status = 400,
        message_zh = "请勿重复发送相同内容",
        message_en = "Duplicate message content detected"
    )]
    DuplicateMessage,

    // ===== 反馈/工单相关错误（138xxxx）=====
    #[detail(
        code = 1380000,
        http_status = 404,
        message_zh = "工单不存在",
        message_en = "Ticket not found"
    )]
    TicketNotFound,
    #[detail(
        code = 1380001,
        http_status = 400,
        message_zh = "工单已关闭，无法继续回复",
        message_en = "Ticket closed, cannot reply"
    )]
    TicketClosed,
    #[detail(
        code = 1380002,
        http_status = 400,
        message_zh = "反馈内容不能为空",
        message_en = "Feedback cannot be empty"
    )]
    FeedbackEmpty,
    #[detail(
        code = 1380003,
        http_status = 400,
        message_zh = "请选择问题分类后提交",
        message_en = "Please select a feedback category"
    )]
    FeedbackCategoryMissing,

    // ===== 活动/营销相关错误（139xxxx）=====
    #[detail(
        code = 1390000,
        http_status = 404,
        message_zh = "活动不存在或已下线",
        message_en = "Activity not found or ended"
    )]
    ActivityNotFound,
    #[detail(
        code = 1390001,
        http_status = 400,
        message_zh = "活动尚未开始",
        message_en = "Activity not started yet"
    )]
    ActivityNotStarted,
    #[detail(
        code = 1390002,
        http_status = 400,
        message_zh = "活动已结束",
        message_en = "Activity already ended"
    )]
    ActivityEnded,
    #[detail(
        code = 1390003,
        http_status = 400,
        message_zh = "优惠券无效或已过期",
        message_en = "Coupon invalid or expired"
    )]
    CouponInvalid,
    #[detail(
        code = 1390004,
        http_status = 400,
        message_zh = "您已领取过该优惠券",
        message_en = "Coupon already claimed"
    )]
    CouponAlreadyClaimed,
    #[detail(
        code = 1390005,
        http_status = 400,
        message_zh = "该优惠券已被领完",
        message_en = "Coupon out of stock"
    )]
    CouponOutOfStock,
    #[detail(
        code = 1390006,
        http_status = 400,
        message_zh = "不满足优惠券使用条件",
        message_en = "Coupon requirements not met"
    )]
    CouponRequirementsNotMet,
    #[detail(
        code = 1390007,
        http_status = 400,
        message_zh = "您已参与过该活动",
        message_en = "Already participated in this activity"
    )]
    ActivityAlreadyParticipated,
    #[detail(
        code = 1390008,
        http_status = 400,
        message_zh = "兑换码无效或已使用",
        message_en = "Redemption code invalid or used"
    )]
    RedemptionCodeInvalid,
    #[detail(
        code = 1390009,
        http_status = 400,
        message_zh = "奖品已被抢光，下次早点来哦",
        message_en = "Prize out of stock"
    )]
    PrizeOutOfStock,
    #[detail(
        code = 1390010,
        http_status = 400,
        message_zh = "抽奖机会已用完",
        message_en = "No more lottery chances"
    )]
    NoMoreLotteryChances,

    // ===== 搜索相关错误（140xxxx）=====
    #[detail(
        code = 1400000,
        http_status = 500,
        message_zh = "搜索服务暂时不可用，请稍后重试",
        message_en = "Search service unavailable"
    )]
    SearchServiceError,
    #[detail(
        code = 1400001,
        http_status = 400,
        message_zh = "请输入搜索关键词",
        message_en = "Please enter search keywords"
    )]
    SearchKeywordEmpty,
    #[detail(
        code = 1400002,
        http_status = 400,
        message_zh = "搜索关键词过长，请精简后重试",
        message_en = "Search keyword too long"
    )]
    SearchKeywordTooLong,
    #[detail(
        code = 1400003,
        http_status = 500,
        message_zh = "索引更新失败，请稍后重试",
        message_en = "Search index update failed"
    )]
    SearchIndexUpdateFailed,

    // ===== Listing / 商品采集相关错误（141xxxx）=====
    #[detail(
        code = 1410000,
        http_status = 404,
        message_zh = "商品信息不存在",
        message_en = "Listing not found"
    )]
    ListingNotFound,
    #[detail(
        code = 1410001,
        http_status = 400,
        message_zh = "商品信息校验失败，请检查后重新提交",
        message_en = "Listing validation failed"
    )]
    ListingValidationFailed,
    #[detail(
        code = 1410002,
        http_status = 400,
        message_zh = "该平台暂不支持，请选择其他平台",
        message_en = "Platform not supported"
    )]
    PlatformNotSupported,
    #[detail(
        code = 1410003,
        http_status = 404,
        message_zh = "草稿不存在或已被删除",
        message_en = "Draft not found"
    )]
    DraftNotFound,
    #[detail(
        code = 1410004,
        http_status = 400,
        message_zh = "草稿ID列表不能为空",
        message_en = "Draft ID list cannot be empty"
    )]
    DraftListEmpty,
    #[detail(
        code = 1410005,
        http_status = 400,
        message_zh = "批量处理数量超出限制，请减少批量数量后重试",
        message_en = "Batch operation size exceeded limit"
    )]
    BatchOperationLimitExceeded,
    #[detail(
        code = 1410006,
        http_status = 404,
        message_zh = "未找到符合条件的商品",
        message_en = "No matching products found"
    )]
    NoMatchingProducts,
    #[detail(
        code = 1410007,
        http_status = 404,
        message_zh = "采集商品不存在",
        message_en = "Collected product not found"
    )]
    CollectedProductNotFound,
    #[detail(
        code = 1410008,
        http_status = 400,
        message_zh = "采集商品状态无效",
        message_en = "Invalid collected product state"
    )]
    CollectedProductInvalidState,
    #[detail(
        code = 1410009,
        http_status = 500,
        message_zh = "采集数据解析失败，请稍后重试",
        message_en = "Failed to parse collected data"
    )]
    CollectedDataParseFailed,
    #[detail(
        code = 1410010,
        http_status = 500,
        message_zh = "批量标准化失败，请稍后重试",
        message_en = "Batch standardization failed"
    )]
    BatchStandardizationFailed,
    #[detail(
        code = 1410011,
        http_status = 500,
        message_zh = "批量评分失败，请稍后重试",
        message_en = "Batch scoring failed"
    )]
    BatchScoringFailed,
    #[detail(
        code = 1410012,
        http_status = 400,
        message_zh = "源平台不能为空，请在请求中指定",
        message_en = "Source platform is required"
    )]
    SourcePlatformRequired,
    #[detail(
        code = 1410013,
        http_status = 400,
        message_zh = "源类目ID不能为空",
        message_en = "Source category ID is required"
    )]
    SourceCategoryIdRequired,
    #[detail(
        code = 1410014,
        http_status = 404,
        message_zh = "类目映射不存在",
        message_en = "Category mapping not found"
    )]
    CategoryMappingNotFound,
    #[detail(
        code = 1410015,
        http_status = 404,
        message_zh = "平台类目不存在",
        message_en = "Platform category not found"
    )]
    PlatformCategoryNotFound,
    #[detail(
        code = 1410016,
        http_status = 404,
        message_zh = "Prompt模板不存在",
        message_en = "Prompt template not found"
    )]
    PromptTemplateNotFound,
    #[detail(
        code = 1410017,
        http_status = 400,
        message_zh = "无效的步骤类型",
        message_en = "Invalid step type"
    )]
    InvalidStepType,
    #[detail(
        code = 1410018,
        http_status = 404,
        message_zh = "选品结果不存在",
        message_en = "Selection result not found"
    )]
    SelectionResultNotFound,
    #[detail(
        code = 1410019,
        http_status = 404,
        message_zh = "AI任务不存在",
        message_en = "AI task not found"
    )]
    AiTaskNotFound,
    #[detail(
        code = 1410020,
        http_status = 400,
        message_zh = "AI任务步骤不存在",
        message_en = "AI task step not found"
    )]
    AiTaskStepNotFound,
    #[detail(
        code = 1410021,
        http_status = 500,
        message_zh = "素材落库失败，请稍后重试",
        message_en = "Creative asset persist failed"
    )]
    CreativeAssetPersistFailed,
    #[detail(
        code = 1410022,
        http_status = 500,
        message_zh = "Listing 提交失败，请稍后重试",
        message_en = "Listing submission failed"
    )]
    ListingSubmitFailed,
    #[detail(
        code = 1410023,
        http_status = 500,
        message_zh = "成本记录失败，请稍后重试",
        message_en = "Cost log record failed"
    )]
    CostLogRecordFailed,
    #[detail(
        code = 1410024,
        http_status = 404,
        message_zh = "创意素材不存在",
        message_en = "Creative asset not found"
    )]
    CreativeAssetNotFound,

    // ===== AI / Agent 业务扩展（105xxxx 已有基础，此处为业务层扩展 10501xx）=====
    #[detail(
        code = 1050100,
        http_status = 403,
        message_zh = "AI 预算不足，无法执行任务",
        message_en = "AI budget exceeded, cannot execute task"
    )]
    AiBudgetExceeded,
    #[detail(
        code = 1050101,
        http_status = 500,
        message_zh = "AI 返回数据解析失败，请稍后重试",
        message_en = "AI response parse failed"
    )]
    AiResponseParseFailed,
    #[detail(
        code = 1050102,
        http_status = 500,
        message_zh = "AI 服务未配置，请联系管理员",
        message_en = "AI provider not configured"
    )]
    AiProviderNotConfigured,
    #[detail(
        code = 1050103,
        http_status = 500,
        message_zh = "AI 调用失败且无备用方案，请稍后重试",
        message_en = "AI call failed with no fallback available"
    )]
    AiFallbackFailed,
    #[detail(
        code = 1050104,
        http_status = 500,
        message_zh = "AI Agent 构建失败，请稍后重试",
        message_en = "AI agent build failed"
    )]
    AiAgentBuildFailed,
    #[detail(
        code = 1050105,
        http_status = 500,
        message_zh = "AI Agent 执行失败，请稍后重试",
        message_en = "AI agent execution failed"
    )]
    AiAgentExecutionFailed,
    #[detail(
        code = 1050106,
        http_status = 400,
        message_zh = "AI 任务参数错误，请检查后重试",
        message_en = "Invalid AI task parameters"
    )]
    AiTaskParamError,

    // ===== OAuth / 平台账号相关错误（142xxxx）=====
    #[detail(
        code = 1420000,
        http_status = 400,
        message_zh = "OAuth state 参数无效",
        message_en = "Invalid OAuth state parameter"
    )]
    OAuthStateInvalid,
    #[detail(
        code = 1420001,
        http_status = 400,
        message_zh = "OAuth 授权已过期，请重新授权",
        message_en = "OAuth state expired"
    )]
    OAuthStateExpired,
    #[detail(
        code = 1420002,
        http_status = 400,
        message_zh = "OAuth 签名验证失败",
        message_en = "OAuth signature verification failed"
    )]
    OAuthSignatureFailed,
    #[detail(
        code = 1420003,
        http_status = 500,
        message_zh = "OAuth 未配置，请联系管理员",
        message_en = "OAuth not configured"
    )]
    OAuthNotConfigured,
    #[detail(
        code = 1420004,
        http_status = 500,
        message_zh = "OAuth Token 刷新失败，请重新授权",
        message_en = "OAuth token refresh failed"
    )]
    OAuthTokenRefreshFailed,
    #[detail(
        code = 1420005,
        http_status = 500,
        message_zh = "OAuth 授权回调失败，请稍后重试",
        message_en = "OAuth callback failed"
    )]
    OAuthCallbackFailed,
    #[detail(
        code = 1420006,
        http_status = 404,
        message_zh = "平台账号不存在",
        message_en = "Platform account not found"
    )]
    PlatformAccountNotFound,
    #[detail(
        code = 1420007,
        http_status = 400,
        message_zh = "平台账号未激活，请先完成授权",
        message_en = "Platform account not activated"
    )]
    PlatformAccountNotActivated,
    #[detail(
        code = 1420008,
        http_status = 400,
        message_zh = "平台账号未启用",
        message_en = "Platform account not enabled"
    )]
    PlatformAccountNotEnabled,
    #[detail(
        code = 1420009,
        http_status = 400,
        message_zh = "平台账号未配置访问凭证",
        message_en = "Platform account token missing"
    )]
    PlatformAccountTokenMissing,
    #[detail(
        code = 1420010,
        http_status = 500,
        message_zh = "凭证加密失败，请稍后重试",
        message_en = "Token encryption failed"
    )]
    TokenEncryptFailed,
    #[detail(
        code = 1420011,
        http_status = 500,
        message_zh = "凭证解密失败，请检查密钥配置",
        message_en = "Token decryption failed"
    )]
    TokenDecryptFailed,
    #[detail(
        code = 1420012,
        http_status = 500,
        message_zh = "服务器未配置 RSA 私钥，无法解密凭证",
        message_en = "RSA private key not configured"
    )]
    RsaKeyNotConfigured,
    #[detail(
        code = 1420013,
        http_status = 500,
        message_zh = "OAuth 响应数据过大，可能存在异常",
        message_en = "OAuth response size exceeded limit"
    )]
    OAuthResponseTooLarge,

    // ===== 汇率服务相关错误（143xxxx）=====
    #[detail(
        code = 1430000,
        http_status = 500,
        message_zh = "汇率服务未配置，请联系管理员",
        message_en = "Forex service not configured"
    )]
    ForexServiceNotConfigured,
    #[detail(
        code = 1430001,
        http_status = 500,
        message_zh = "汇率查询失败，请稍后重试",
        message_en = "Forex query failed"
    )]
    ForexQueryFailed,
    #[detail(
        code = 1430002,
        http_status = 400,
        message_zh = "不支持的汇率转换方向",
        message_en = "Unsupported forex conversion"
    )]
    UnsupportedForexConversion,
    #[detail(
        code = 1430003,
        http_status = 500,
        message_zh = "汇率数据获取失败，请稍后重试",
        message_en = "Forex data fetch failed"
    )]
    ForexDataFetchFailed,

    // ===== 任务 / Worker 队列相关错误（144xxxx）=====
    #[detail(
        code = 1440000,
        http_status = 500,
        message_zh = "任务队列连接失败，请稍后重试",
        message_en = "Job queue connection failed"
    )]
    JobQueueConnectionFailed,
    #[detail(
        code = 1440001,
        http_status = 500,
        message_zh = "任务入队失败，请稍后重试",
        message_en = "Job enqueue failed"
    )]
    JobEnqueueFailed,
    #[detail(
        code = 1440002,
        http_status = 400,
        message_zh = "任务缺少必要字段",
        message_en = "Task required field missing"
    )]
    TaskFieldMissing,
    #[detail(
        code = 1440003,
        http_status = 409,
        message_zh = "任务已在处理中，请勿重复操作",
        message_en = "Task already in progress"
    )]
    TaskAlreadyInProgress,
    #[detail(
        code = 1440004,
        http_status = 500,
        message_zh = "任务回收失败，请稍后重试",
        message_en = "Task reaping failed"
    )]
    TaskReapingFailed,
    #[detail(
        code = 1440005,
        http_status = 500,
        message_zh = "死信任务归档失败，请联系管理员",
        message_en = "Dead letter task archive failed"
    )]
    DeadLetterArchiveFailed,
    #[detail(
        code = 1440006,
        http_status = 500,
        message_zh = "任务事件发布失败",
        message_en = "Task event publish failed"
    )]
    TaskEventPublishFailed,

    // ===== ERP / 第三方电商服务相关错误（145xxxx）=====
    #[detail(
        code = 1450000,
        http_status = 500,
        message_zh = "ERP 服务调用失败，请稍后重试",
        message_en = "ERP service call failed"
    )]
    ErpServiceCallFailed,
    #[detail(
        code = 1450001,
        http_status = 500,
        message_zh = "ERP 服务未配置，请联系管理员",
        message_en = "ERP service not configured"
    )]
    ErpNotConfigured,
    #[detail(
        code = 1450002,
        http_status = 401,
        message_zh = "ERP 授权失败，请检查配置",
        message_en = "ERP authentication failed"
    )]
    ErpAuthFailed,
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
