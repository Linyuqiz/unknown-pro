use super::Error;

// user err: 10000
pub const ERR_INTERNAL_SERVER: Error = Error(10001, "服务内部错误");
pub const ERR_INVALID_PARAM: Error = Error(10002, "请求参数无效");
