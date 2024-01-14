use crate::{
    cerror,
    common::{self, err},
    entity::user::{self, Entity as User},
    utils,
};
use chrono::Utc;
use ntex::web::{
    types::{Json, Query, State},
    HttpResponse,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use tracing::error;

use self::model::{
    AddUserRequest, AddUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse, ListUserResponse, LoginUserRequest, LoginUserResponse, LogoutUserRequest,
    LogoutUserResponse, UpdateUserRequest,
};

pub mod model;

pub async fn login(
    connection: State<Arc<DatabaseConnection>>,
    body: Json<LoginUserRequest>,
) -> HttpResponse {
    if body.username.is_empty() || body.password.is_empty() {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }

    let users = User::find()
        .filter(user::Column::Username.contains(body.username.clone()))
        .all(connection.as_ref())
        .await
        .map_err(|err| error!("get user err: {:?}", err));

    if let Ok(users) = users {
        if users.len() != 1 {
            return common::err::<cerror::Error>(cerror::user::ERR_USERNAME_OR_PASSWORD);
        }
        if users[0].password != body.password {
            return common::err::<cerror::Error>(cerror::user::ERR_USERNAME_OR_PASSWORD);
        }

        return common::ok(LoginUserResponse { success: true });
    } else {
        return common::err::<cerror::Error>(cerror::user::ERR_USERNAME_OR_PASSWORD);
    }
}

pub async fn logout(
    _connection: State<Arc<DatabaseConnection>>,
    body: Json<LogoutUserRequest>,
) -> HttpResponse {
    let _user_id: i64;
    if let Ok(id) = body.id.parse::<i64>() {
        _user_id = id;
    } else {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }

    common::ok(LogoutUserResponse { success: true })
}

pub async fn list_user(connection: State<Arc<DatabaseConnection>>) -> HttpResponse {
    User::find()
        .all(connection.as_ref())
        .await
        .map(|users| common::ok(ListUserResponse { users: users }))
        .map_err(|err| error!("list user err: {:?}", err))
        .unwrap_or(common::err::<cerror::Error>(cerror::user::ERR_LIST_USER))
}

pub async fn get_user(
    connection: State<Arc<DatabaseConnection>>,
    query: Query<GetUserRequest>,
) -> HttpResponse {
    let user_id: i64;
    if let Ok(id) = query.id.parse::<i64>() {
        user_id = id;
    } else {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }

    User::find_by_id(user_id)
        .one(connection.as_ref())
        .await
        .map(|user| common::ok(GetUserResponse { user: user }))
        .map_err(|err| error!("get user err: {:?}", err))
        .unwrap_or(common::err::<cerror::Error>(cerror::user::ERR_GET_USER))
}

pub async fn add_user(
    connection: State<Arc<DatabaseConnection>>,
    body: Json<AddUserRequest>,
) -> HttpResponse {
    if body.username.is_empty() || body.password.is_empty() {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }

    let current_datetime = Utc::now();
    let user = user::ActiveModel {
        id: Set(utils::snowflake::get_snowflake_id()),
        username: Set(body.username.clone()),
        nickname: Set(body.username.clone()),
        password: Set(body.password.clone()),
        mobile: Set("".to_owned()),
        email: Set("".to_owned()),
        enable: Set(1),
        deleted: Set(0),
        created_time: Set(current_datetime.into()),
        updated_time: Set(current_datetime.into()),
    };

    user.insert(connection.as_ref())
        .await
        .map(|_add| common::ok(AddUserResponse { success: true }))
        .map_err(|err| error!("add user err: {:?}", err))
        .unwrap_or(common::err::<cerror::Error>(cerror::user::ERR_ADD_USER))
}

pub async fn update_user(
    connection: State<Arc<DatabaseConnection>>,
    body: Json<UpdateUserRequest>,
) -> HttpResponse {
    let user_id: i64;
    if let Ok(id) = body.id.parse::<i64>() {
        user_id = id;
    } else {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }
    if body.nickname.is_empty() {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }

    let user = User::find_by_id(user_id)
        .one(connection.as_ref())
        .await
        .map_err(|err| error!("get user err: {:?}", err));

    if let Ok(Some(user)) = user {
        let mut user: user::ActiveModel = user.into();
        user.nickname = Set(body.nickname.clone());
        let current_datetime = Utc::now();
        user.updated_time = Set(current_datetime.into());

        user.update(connection.as_ref())
            .await
            .map(|_update| common::ok(DeleteUserResponse { success: true }))
            .map_err(|err| error!("update user err: {:?}", err))
            .unwrap_or(common::err::<cerror::Error>(cerror::user::ERR_UPDATE_USER))
    } else {
        common::err::<cerror::Error>(cerror::user::ERR_UPDATE_USER)
    }
}

pub async fn delete_user(
    connection: State<Arc<DatabaseConnection>>,
    body: Json<DeleteUserRequest>,
) -> HttpResponse {
    let user_id: i64;
    if let Ok(id) = body.id.parse::<i64>() {
        user_id = id;
    } else {
        return err::<cerror::Error>(cerror::common::ERR_INVALID_PARAM);
    }

    let user = User::find_by_id(user_id)
        .one(connection.as_ref())
        .await
        .map_err(|err| error!("get user err: {:?}", err));

    if let Ok(Some(user)) = user {
        let mut user: user::ActiveModel = user.into();
        user.deleted = Set(1);
        let current_datetime = Utc::now();
        user.updated_time = Set(current_datetime.into());

        user.update(connection.as_ref())
            .await
            .map(|_update| common::ok(DeleteUserResponse { success: true }))
            .map_err(|err| error!("delete user err: {:?}", err))
            .unwrap_or(common::err::<cerror::Error>(cerror::user::ERR_DELETE_USER))
    } else {
        common::err::<cerror::Error>(cerror::user::ERR_DELETE_USER)
    }
}
