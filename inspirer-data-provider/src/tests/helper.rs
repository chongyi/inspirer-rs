use crate::prelude::*;
use diesel::connection::SimpleConnection;

pub fn create_conn() -> PooledConn {
    let cpm = ConnPoolManager::builder().build();
    cpm.pool().get().unwrap()
}

pub fn auto_clear_base_environment(f: impl FnOnce(&PooledConn)) {
    let conn = create_conn();
    let _: QueryResult<()> = conn.transaction(|| {
        conn.batch_execute(include_str!("test_data_clear.sql"))?;
        conn.batch_execute(include_str!("test_data_build.sql"))?;

        f(&conn);
        conn.batch_execute(include_str!("test_data_clear.sql"))?;
        Ok(())
    });
}