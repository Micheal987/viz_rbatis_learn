use anyhow::Ok;
use rbatis::{ RBatis, DefaultPool };
use rbdc_pg::{ options::PgConnectOptions, PostgresDriver };

pub async fn rbatis_init() -> anyhow::Result<RBatis> {
    // 创建 Rbatis 实例
    let rb: RBatis = RBatis::new();
    let opts = PgConnectOptions::new()
        .host("127.0.0.1")
        .port(5433)
        .username("postgres")
        .password("123456")
        .database("rust_test");
    // 连接数据库
    rb.init_option::<PostgresDriver, PgConnectOptions, DefaultPool>(PostgresDriver {}, opts)?;
    //rb.init(PostgresDriver {}, "postgres://postgres:123456@localhost:5433/rust_test").unwrap();
    // 查询版本
    sql_version(&rb).await;
    Ok(rb)
}
async fn sql_version(rb: &RBatis) {
    let version: String = rb.query_decode("SELECT VERSION()", vec![]).await.expect("查询版本失败");
    tracing::info!("pg version is :{}", version);
}
