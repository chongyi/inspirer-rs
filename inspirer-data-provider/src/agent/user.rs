use crate::model::user::*;
use crate::utils;
use chrono::prelude::*;
use crate::prelude::*;
use crate::schema::users;

pub struct CreateUser<'i> {
    pub invitor_uuid: &'i str,
    pub status: Option<i16>,
    pub password: Option<&'i str>,
}

/// 手机号注册模型
pub struct MobilePhoneRegister<'i> {
    pub mobile_phone: &'i str,
    pub country_code: Option<&'i str>,
    pub password: Option<&'i str>,
    pub nickname: Option<&'i str>,
    pub status: Option<i16>,
    pub invitor_uuid: Option<&'i str>,
}

impl<'i> ActiveModel for MobilePhoneRegister<'i> {
    type Result = ActionResult<(i64, String)>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        // 创建 UUID Buffer
        let mut uuid_buffer = [0; 32];

        let insert = MobilePhoneRegisterUser {
            user_uuid: utils::generate_uuid(&mut uuid_buffer),
            mobile_phone: self.mobile_phone,
            country_code: self.country_code.unwrap_or("86"),
            password: self.password.map(utils::password_hash),
            status: self.status.unwrap_or(0),
            nickname: self.nickname.unwrap_or(self.mobile_phone),
            invitor_uuid: self.invitor_uuid,
        };

        diesel::insert_into(users::table)
            .values(&insert)
            .returning((users::columns::id, users::columns::user_uuid))
            .get_result(conn).map_err(From::from)
    }
}

/// 邮箱注册模型
pub struct EmailRegister<'i> {
    pub email: &'i str,
    pub password: Option<&'i str>,
    pub nickname: Option<&'i str>,
    pub status: Option<i16>,
    pub invitor_uuid: Option<&'i str>,
}

impl<'i> ActiveModel for EmailRegister<'i> {
    type Result = ActionResult<(i64, String)>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        // 创建 UUID Buffer
        let mut uuid_buffer = [0; 32];

        let insert = EmailRegisterUser {
            user_uuid: utils::generate_uuid(&mut uuid_buffer),
            email: self.email,
            password: self.password.map(utils::password_hash),
            status: self.status.unwrap_or(0),
            nickname: self.nickname.unwrap_or(self.email),
            invitor_uuid: self.invitor_uuid,
        };

        diesel::insert_into(users::table)
            .values(&insert)
            .returning((users::columns::id, users::columns::user_uuid))
            .get_result(conn).map_err(From::from)
    }
}

/// 用户登录触发模型
#[derive(Default)]
pub struct UserLoginTrigger<'i> {
    pub user_uuid: &'i str,
    pub ip: Option<&'i str>,
    pub event_time: Option<NaiveDateTime>,
}

impl<'i> ActiveModel for UserLoginTrigger<'i> {
    type Result = ActionResult<()>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let target = users::table.filter(users::columns::user_uuid.eq(self.user_uuid));
        diesel::update(target)
            .set((&UpdateUserLastLogin {
                last_login_ip: self.ip,
                last_login: self.event_time.unwrap_or(Utc::now().naive_utc()),
            }, users::columns::login_count.eq(users::columns::login_count + 1)))
            .execute(conn)
            .map(|_| ()).map_err(From::from)
    }
}

/// 用户活跃状态触发模型
pub struct UserActiveTrigger<'i> {
    pub user_uuid: &'i str,
    pub event_time: Option<NaiveDateTime>,
}

impl<'i> ActiveModel for UserActiveTrigger<'i> {
    type Result = ActionResult<()>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let target = users::table.filter(users::columns::user_uuid.eq(self.user_uuid));
        diesel::update(target)
            .set(&UpdateUserActivatedTime {
                activated_at: self.event_time.unwrap_or(Utc::now().naive_utc()),
            })
            .execute(conn)
            .map(|_| ()).map_err(From::from)
    }
}