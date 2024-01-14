use super::Error;

// user err: 11000
pub const ERR_LOGIN_USER: Error = Error(11001, "用户登陆失败");
pub const ERR_LOGOUT_USER: Error = Error(11002, "用户登出失败");
pub const ERR_LIST_USER: Error = Error(11003, "获取用户列表失败");
pub const ERR_GET_USER: Error = Error(11004, "获取用户信息失败");
pub const ERR_ADD_USER: Error = Error(11005, "添加用户信息失败");
pub const ERR_UPDATE_USER: Error = Error(11006, "更新用户信息失败");
pub const ERR_DELETE_USER: Error = Error(11007, "删除用户信息失败");
pub const ERR_USERNAME_OR_PASSWORD: Error = Error(11008, "用户名或密码错误");
