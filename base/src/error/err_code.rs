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

    // 文件存储相关错误码
    #[detail(code = 1070000, http_status = 500, message_zh = "文件存储错误")]
    StorageCommon,
    #[detail(code = 1070001, http_status = 500, message_zh = "文件上传失败")]
    FileUploadFailed,
    #[detail(code = 1070002, http_status = 500, message_zh = "文件下载失败")]
    FileDownloadFailed,
    #[detail(code = 1070003, http_status = 404, message_zh = "文件不存在")]
    FileNotFound,
    #[detail(code = 1070004, http_status = 400, message_zh = "文件类型不支持")]
    FileTypeNotSupported,
    #[detail(code = 1070005, http_status = 400, message_zh = "文件大小超限")]
    FileSizeExceed,
    #[detail(code = 1070006, http_status = 500, message_zh = "S3存储连接失败")]
    S3ConnectionFailed,
    #[detail(code = 1070007, http_status = 500, message_zh = "S3操作失败")]
    S3OperationFailed,

    // 网络/RPC相关错误码
    #[detail(code = 1080000, http_status = 500, message_zh = "网络请求错误")]
    NetworkError,
    #[detail(code = 1080001, http_status = 500, message_zh = "RPC调用失败")]
    RpcCallFailed,
    #[detail(code = 1080002, http_status = 500, message_zh = "gRPC调用失败")]
    GrpcCallFailed,
    #[detail(code = 1080003, http_status = 504, message_zh = "RPC调用超时")]
    RpcCallTimeout,
    #[detail(code = 1080004, http_status = 502, message_zh = "网关响应错误")]
    GatewayResponseError,
    #[detail(code = 1080005, http_status = 502, message_zh = "上游服务不可用")]
    UpstreamUnavailable,

    // 消息队列相关错误码
    #[detail(code = 1090000, http_status = 500, message_zh = "消息队列错误")]
    MQCommon,
    #[detail(code = 1090001, http_status = 500, message_zh = "Kafka消息发送失败")]
    KafkaSendFailed,
    #[detail(code = 1090002, http_status = 500, message_zh = "Kafka消息消费失败")]
    KafkaConsumeFailed,
    #[detail(code = 1090003, http_status = 500, message_zh = "RabbitMQ连接失败")]
    RabbitMQConnectionFailed,
    #[detail(code = 1090004, http_status = 500, message_zh = "RabbitMQ消息发送失败")]
    RabbitMQSnedFailed,
    #[detail(code = 1090005, http_status = 500, message_zh = "消息确认失败")]
    MQAckFailed,

    // GraphQL相关错误码
    #[detail(code = 1100000, http_status = 400, message_zh = "GraphQL查询错误")]
    GraphQLQueryError,
    #[detail(code = 1100001, http_status = 400, message_zh = "GraphQL解析错误")]
    GraphQLParseError,
    #[detail(code = 1100002, http_status = 400, message_zh = "GraphQL验证错误")]
    GraphQLValidationError,

    // WebSocket相关错误码
    #[detail(code = 1110000, http_status = 500, message_zh = "WebSocket连接错误")]
    WebSocketConnectError,
    #[detail(
        code = 1110001,
        http_status = 500,
        message_zh = "WebSocket消息发送失败"
    )]
    WebSocketSendFailed,
    #[detail(code = 1110002, http_status = 500, message_zh = "WebSocket连接超时")]
    WebSocketTimeout,
    #[detail(code = 1110003, http_status = 500, message_zh = "WebSocket会话不存在")]
    WebSocketSessionNotFound,

    // 限流相关错误码
    #[detail(code = 1120000, http_status = 429, message_zh = "请求过于频繁")]
    RateLimitExceeded,
    #[detail(code = 1120001, http_status = 429, message_zh = "IP访问受限")]
    IPRateLimitExceeded,
    #[detail(code = 1120002, http_status = 429, message_zh = "用户访问受限")]
    UserRateLimitExceeded,
    #[detail(code = 1120003, http_status = 429, message_zh = "接口访问受限")]
    ApiRateLimitExceeded,

    // 配置相关错误码
    #[detail(code = 1130000, http_status = 500, message_zh = "配置加载错误")]
    ConfigLoadError,
    #[detail(code = 1130001, http_status = 500, message_zh = "配置解析错误")]
    ConfigParseError,
    #[detail(code = 1130002, http_status = 500, message_zh = "配置项缺失")]
    ConfigItemMissing,
    #[detail(code = 1130003, http_status = 500, message_zh = "环境变量错误")]
    EnvVarError,

    // 业务校验相关错误码
    #[detail(code = 1140000, http_status = 400, message_zh = "业务规则校验失败")]
    BusinessRuleViolation,
    #[detail(code = 1140001, http_status = 400, message_zh = "数据状态异常")]
    DataStateError,
    #[detail(code = 1140002, http_status = 400, message_zh = "操作冲突")]
    OperationConflict,
    #[detail(code = 1140003, http_status = 400, message_zh = "数据已被占用")]
    DataAlreadyOccupied,
    #[detail(code = 1140004, http_status = 400, message_zh = "数据关联检查失败")]
    DataRelationError,

    // 邮件/短信通知相关错误码
    #[detail(code = 1150000, http_status = 500, message_zh = "邮件发送失败")]
    EmailSendFailed,
    #[detail(code = 1150001, http_status = 500, message_zh = "短信发送失败")]
    SMSSendFailed,
    #[detail(code = 1150002, http_status = 500, message_zh = "通知服务不可用")]
    NotificationServiceUnavailable,

    // 支付相关错误码
    #[detail(code = 1160000, http_status = 500, message_zh = "支付服务错误")]
    PaymentError,
    #[detail(code = 1160001, http_status = 400, message_zh = "支付参数错误")]
    PaymentParamError,
    #[detail(code = 1160002, http_status = 500, message_zh = "支付渠道不可用")]
    PaymentChannelUnavailable,
    #[detail(code = 1160003, http_status = 400, message_zh = "支付金额不合法")]
    PaymentAmountInvalid,
    #[detail(code = 1160004, http_status = 400, message_zh = "订单已支付")]
    OrderAlreadyPaid,
    #[detail(code = 1160005, http_status = 400, message_zh = "订单支付超时")]
    OrderPaymentTimeout,

    // 外部API调用相关错误码
    #[detail(code = 1170000, http_status = 502, message_zh = "外部API调用失败")]
    ExternalApiCallFailed,
    #[detail(code = 1170001, http_status = 504, message_zh = "外部API调用超时")]
    ExternalApiTimeout,
    #[detail(code = 1170002, http_status = 502, message_zh = "第三方服务不可用")]
    ThirdPartyServiceUnavailable,
    #[detail(
        code = 1170003,
        http_status = 400,
        message_zh = "第三方返回数据格式错误"
    )]
    ThirdPartyDataFormatError,

    // 数据解析相关错误码
    #[detail(code = 1180000, http_status = 400, message_zh = "Excel解析错误")]
    ExcelParseError,
    #[detail(code = 1180001, http_status = 400, message_zh = "CSV解析错误")]
    CsvParseError,
    #[detail(code = 1180002, http_status = 400, message_zh = "XML解析错误")]
    XmlParseError,
    #[detail(code = 1180003, http_status = 400, message_zh = "数据格式转换错误")]
    DataTransformError,

    // 定时任务相关错误码
    #[detail(code = 1190000, http_status = 500, message_zh = "定时任务错误")]
    CronTaskError,
    #[detail(code = 1190001, http_status = 500, message_zh = "定时任务调度失败")]
    CronScheduleFailed,
    #[detail(code = 1190002, http_status = 500, message_zh = "定时任务执行失败")]
    CronExecuteFailed,
    #[detail(code = 1190003, http_status = 500, message_zh = "定时任务配置错误")]
    CronConfigError,

    // 分布式锁相关错误码
    #[detail(code = 1200000, http_status = 500, message_zh = "分布式锁错误")]
    DistributedLockError,
    #[detail(code = 1200001, http_status = 500, message_zh = "获取分布式锁失败")]
    LockAcquireFailed,
    #[detail(code = 1200002, http_status = 500, message_zh = "释放分布式锁失败")]
    LockReleaseFailed,
    #[detail(code = 1200003, http_status = 400, message_zh = "分布式锁已被占用")]
    LockAlreadyOccupied,

    // 分布式ID生成相关错误码
    #[detail(code = 1210000, http_status = 500, message_zh = "ID生成错误")]
    IdGenerateError,
    #[detail(code = 1210001, http_status = 500, message_zh = "雪花算法ID生成失败")]
    SnowflakeIdFailed,
    #[detail(code = 1210002, http_status = 500, message_zh = "UUID生成失败")]
    UUIDGenerateFailed,

    // 加密解密相关错误码
    #[detail(code = 1220000, http_status = 500, message_zh = "加密解密错误")]
    CryptoError,
    #[detail(code = 1220001, http_status = 500, message_zh = "AES加密失败")]
    AesEncryptFailed,
    #[detail(code = 1220002, http_status = 500, message_zh = "AES解密失败")]
    AesDecryptFailed,
    #[detail(code = 1220003, http_status = 500, message_zh = "RSA加密失败")]
    RsaEncryptFailed,
    #[detail(code = 1220004, http_status = 500, message_zh = "RSA解密失败")]
    RsaDecryptFailed,
    #[detail(code = 1220005, http_status = 500, message_zh = "签名验证失败")]
    SignatureVerifyFailed,
    #[detail(code = 1220006, http_status = 500, message_zh = "密钥无效")]
    InvalidKey,

    // 熔断降级相关错误码
    #[detail(code = 1230000, http_status = 503, message_zh = "服务熔断")]
    CircuitBreakerOpen,
    #[detail(code = 1230001, http_status = 503, message_zh = "服务降级")]
    ServiceDegraded,
    #[detail(code = 1230002, http_status = 503, message_zh = "服务不可用")]
    ServiceUnavailable,

    // 链路追踪相关错误码
    #[detail(code = 1240000, http_status = 500, message_zh = "链路追踪错误")]
    TracingError,
    #[detail(code = 1240001, http_status = 500, message_zh = "Span创建失败")]
    SpanCreateFailed,
    #[detail(code = 1240002, http_status = 500, message_zh = "Trace上报失败")]
    TraceReportFailed,

    // 日志相关错误码
    #[detail(code = 1250000, http_status = 500, message_zh = "日志写入错误")]
    LogWriteError,
    #[detail(code = 1250001, http_status = 500, message_zh = "日志配置错误")]
    LogConfigError,

    // 国际化相关错误码
    #[detail(code = 1260000, http_status = 500, message_zh = "国际化错误")]
    I18nError,
    #[detail(code = 1260001, http_status = 400, message_zh = "语言包加载失败")]
    LocaleLoadFailed,
    #[detail(code = 1260002, http_status = 400, message_zh = "翻译键不存在")]
    TranslationKeyNotFound,

    // 数据库连接池相关错误码
    #[detail(code = 1270000, http_status = 500, message_zh = "连接池错误")]
    ConnectionPoolError,
    #[detail(code = 1270001, http_status = 500, message_zh = "连接池耗尽")]
    ConnectionPoolExhausted,
    #[detail(code = 1270002, http_status = 500, message_zh = "连接获取超时")]
    ConnectionAcquireTimeout,

    // 灰度发布相关错误码
    #[detail(code = 1280000, http_status = 500, message_zh = "灰度发布错误")]
    GrayReleaseError,
    #[detail(code = 1280001, http_status = 400, message_zh = "灰度规则配置错误")]
    GrayRuleConfigError,
    #[detail(code = 1280002, http_status = 500, message_zh = "灰度分流失败")]
    GrayRouteFailed,

    // 审计日志相关错误码
    #[detail(code = 1290000, http_status = 500, message_zh = "审计日志错误")]
    AuditLogError,
    #[detail(code = 1290001, http_status = 500, message_zh = "审计日志记录失败")]
    AuditLogRecordFailed,

    // 资源配额相关错误码
    #[detail(code = 1300000, http_status = 400, message_zh = "资源配额错误")]
    ResourceQuotaError,
    #[detail(code = 1300001, http_status = 400, message_zh = "资源配额不足")]
    ResourceQuotaExceeded,
    #[detail(code = 1300002, http_status = 400, message_zh = "资源已达上限")]
    ResourceLimitReached,
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
