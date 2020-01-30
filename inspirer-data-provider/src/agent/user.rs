use crate::model::user::*;
use crate::utils;
use chrono::prelude::*;
use crate::prelude::*;
use crate::schema::{users, user_base_profiles, user_mobile_phone_credentials, user_email_credentials};
use diesel::r2d2::ConnectionManager;
use crate::model::user_base_profiles::InsertUserBaseProfile;
use crate::model::user_email_credentials::InsertUserEmailCredential;
use crate::model::user_mobile_phone_credentials::InsertUserMobilePhoneCredential;

pub struct CreateUser<'i> {
    pub invitor_uuid: Option<&'i str>,
    pub status: Option<i16>,
    pub password: Option<&'i str>,
}

impl<'i> ActiveModel for CreateUser<'i> {
    type Result = ActionResult<(i64, String)>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        // 创建 UUID Buffer
        let mut uuid_buffer = [0; 32];

        diesel::insert_into(users::table)
            .values(&InsertUser {
                uuid: utils::generate_uuid(&mut uuid_buffer),
                invitor_uuid: self.invitor_uuid,
                password: self.password.map(utils::password_hash),
                status: self.status.unwrap_or(0),
            })
            .returning((users::id, users::uuid))
            .get_result(conn)
            .map_err(From::from)
    }
}

pub struct CreateUserBaseProfile<'i> {
    pub user_uuid: &'i str,
    pub nickname: Option<&'i str>,
    pub avatar: Option<&'i str>,
    pub gender: Option<i16>,
}

impl<'i> ActiveModel for CreateUserBaseProfile<'i> {
    type Result = ActionResult<String>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        diesel::insert_into(user_base_profiles::table)
            .values(&InsertUserBaseProfile {
                user_uuid: self.user_uuid,
                nickname: self.nickname.unwrap_or(""),
                avatar: self.avatar.unwrap_or(""),
                gender: self.gender.unwrap_or(0),
            })
            .returning(user_base_profiles::user_uuid)
            .get_result(conn)
            .map_err(From::from)
    }
}

pub struct CreateUserEmailCredential<'i> {
    pub user_uuid: &'i str,
    pub email: &'i str,
    pub status: Option<i16>,
    pub activated_at: Option<NaiveDateTime>,
}

impl<'i> ActiveModel for CreateUserEmailCredential<'i> {
    type Result = ActionResult<String>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        diesel::insert_into(user_email_credentials::table)
            .values(&InsertUserEmailCredential {
                user_uuid: self.user_uuid,
                email: self.email,
                status: self.status.unwrap_or(0),
                activated_at: self.activated_at,
            })
            .returning(user_email_credentials::user_uuid)
            .get_result(conn)
            .map_err(From::from)
    }
}

pub struct CreateUserMobilePhoneCredential<'i> {
    pub user_uuid: &'i str,
    pub country_code: &'i str,
    pub mobile_phone: &'i str,
    pub status: Option<i16>,
}

impl<'i> ActiveModel for CreateUserMobilePhoneCredential<'i> {
    type Result = ActionResult<String>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        diesel::insert_into(user_mobile_phone_credentials::table)
            .values(&InsertUserMobilePhoneCredential {
                user_uuid: self.user_uuid,
                country_code: self.country_code,
                mobile_phone: self.mobile_phone,
                status: self.status.unwrap_or(0),
            })
            .returning(user_mobile_phone_credentials::user_uuid)
            .get_result(conn)
            .map_err(From::from)
    }
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
        let (mobile_phone, country_code, password, nickname, status, invitor_uuid) = (
            self.mobile_phone,
            self.country_code.unwrap_or("86"),
            self.password,
            self.nickname,
            self.status,
            self.invitor_uuid
        );

        let (avatar, gender) = (None, None);
        let (id, uuid) = CreateUser {invitor_uuid, password, status}.activate(conn)?;
        let user_uuid = uuid.as_str();

        CreateUserBaseProfile { user_uuid, nickname, avatar, gender }.activate(conn)?;
        CreateUserMobilePhoneCredential { user_uuid, country_code, mobile_phone, status }.activate(conn)?;

        Ok((id, uuid))
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
        let (email, password, nickname, status, invitor_uuid) = (
            self.email,
            self.password,
            self.nickname,
            self.status,
            self.invitor_uuid
        );

        let (avatar, gender) = (None, None);
        let (id, uuid) = CreateUser {invitor_uuid, password, status}.activate(conn)?;
        let user_uuid = uuid.as_str();

        CreateUserBaseProfile { user_uuid, nickname, avatar, gender }.activate(conn)?;
        CreateUserEmailCredential { user_uuid, email, status, activated_at: None }.activate(conn)?;

        Ok((id, uuid))
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
        let target = users::table.filter(users::columns::uuid.eq(self.user_uuid));
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
        let target = users::table.filter(users::columns::uuid.eq(self.user_uuid));
        diesel::update(target)
            .set(&UpdateUserActivatedTime {
                activated_at: self.event_time.unwrap_or(Utc::now().naive_utc()),
            })
            .execute(conn)
            .map(|_| ()).map_err(From::from)
    }
}