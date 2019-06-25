use crate::prelude::*;
use crate::schema::validate_codes;
use crate::model::validate_code::ValidateCodeInsert;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types::*;
use rand::{thread_rng, Rng};

/// 校验代码创建条件
pub enum CreateCondition {
    /// 在指定时间（秒）后
    AfterSec(u32),
    /// 总是（无视任何条件）
    Always,
}

/// 创建一个校验代码
pub struct CreateValidateCode<'i> {
    /// 校验通道
    pub channel: i16,
    /// 校验目标
    pub target: &'i str,
    /// 校验代码过期时间（若为空则表示不会过期）
    pub expired_at: Option<NaiveDateTime>,
    /// 如果已存在未被验证的代码，是否强制创建
    pub force_create: CreateCondition,
    /// 指定被验证的时间（默认为当前系统时间）
    pub current_time: Option<NaiveDateTime>,
}

impl Default for CreateValidateCode<'static> {
    fn default() -> Self {
        CreateValidateCode {
            channel: 0,
            target: "",
            expired_at: None,
            force_create: CreateCondition::AfterSec(60),
            current_time: None,
        }
    }
}

impl<'i> ActiveModel for CreateValidateCode<'i> {
    type Result = ActionResult<(i64, String)>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        if let CreateCondition::AfterSec(sec) = self.force_create {
            let current_time = self.current_time.unwrap_or(Utc::now().naive_local());

            let created: ActionResult<Option<NaiveDateTime>> = validate_codes::table
                .select(validate_codes::columns::created_at)
                .filter(
                    validate_codes::columns::is_validated.eq(true)
                        .and(validate_codes::columns::validate_channel.eq(self.channel))
                        .and(validate_codes::columns::validate_target.eq(self.target))
                        .and(validate_codes::columns::status.eq(true))
                        .and(validate_codes::columns::expired_at.is_null().or(validate_codes::columns::expired_at.gt(self.current_time.as_ref())))
                )
                .order((validate_codes::columns::created_at.desc(), validate_codes::columns::id.desc()))
                .get_result(conn)
                .optional()
                .map_err(From::from);

            let created = created?;

            // 存在未验证的校验代码
            if let Some(v) = created {
                if current_time - v < time::Duration::seconds(sec as i64) {
                    return Err(utils::biz_err(result::ValidateCodeExistsError));
                }
            }
        }

        let mut rng = thread_rng();
        let code = format!("{:06}", rng.gen_range(0, 999999));

        let i = ValidateCodeInsert {
            code: code.as_str(),
            validate_target: self.target,
            validate_channel: self.channel,
            expired_at: self.expired_at,
        };

        let id: ActionResult<usize> = diesel::insert_into(validate_codes::table)
            .values(&i)
            .returning(validate_codes::id)
            .execute(conn)
            .map_err(From::from);

        let id = id?;

        Ok((id as i64, code))
    }
}

/// 验证校验代码
#[derive(Default)]
pub struct ValidateCode<'i> {
    /// 校验通道
    pub channel: i16,
    /// 校验目标
    pub target: &'i str,
    /// 校验代码
    pub code: &'i str,
    /// 指定被验证的时间（默认为当前系统时间）
    pub current_time: Option<NaiveDateTime>,
    /// 若有关闭未被验证的（统一通道和目标的）代码，是否 <u>**不**</u> 关闭
    pub not_close_other: bool,
}

impl<'i> ActiveModel for ValidateCode<'i> {
    type Result = ActionResult<bool>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let current_time = self.current_time.unwrap_or(Utc::now().naive_local());

        let result = diesel::update(validate_codes::table)
            .filter(validate_codes::columns::code.eq(self.code)
                .and(validate_codes::columns::validate_target.eq(self.target))
                .and(validate_codes::columns::validate_channel.eq(self.channel))
                .and(validate_codes::columns::status.eq(true))
                .and(validate_codes::columns::expired_at.is_null().or(validate_codes::columns::expired_at.gt(self.current_time.as_ref())))
                .and(validate_codes::columns::is_validated.eq(false))
            )
            .set(validate_codes::columns::is_validated.eq(true))
            .execute(conn);

        match result {
            Ok(effect) => if effect > 0 {
                // 同时关闭所有统一校验通道、校验目标的未被验证的校验代码，用于放置
                if !self.not_close_other {
                    diesel::update(validate_codes::table)
                        .filter(validate_codes::columns::validate_target.eq(self.target)
                            .and(validate_codes::columns::validate_channel.eq(self.channel))
                            .and(validate_codes::columns::status.eq(true))
                        )
                        .set(validate_codes::columns::status.eq(false))
                        .execute(conn);
                }

                Ok(true)
            } else {
                Ok(false)
            },
            Err(err) => Err(err.into())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use chrono::prelude::*;
    use super::ValidateCode;
    use time::Duration;

    #[test]
    fn test_validate_code() {
        auto_clear_base_environment(|conn| {
            let v = ValidateCode {
                channel: 1,
                target: "+86-18000000005",
                code: "151010",
                current_time: Some(Utc.datetime_from_str("2019-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_local()),
                ..Default::default()
            };

            let r = v.activate(conn).unwrap();
            assert_eq!(false, r);

            let v = ValidateCode {
                channel: 1,
                target: "+86-18000000005",
                code: "151010",
                current_time: Some(Utc.datetime_from_str("2019-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_local() - Duration::seconds(1)),
                ..Default::default()
            };

            let r = v.activate(conn).unwrap();
            assert_eq!(true, r);
        })
    }
}