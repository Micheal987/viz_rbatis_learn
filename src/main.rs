use std::net::SocketAddr;
use tokio::net::TcpListener;
use viz::types::State;
use viz::{ Request, RequestExt, Result, Router, serve };
use viz_rbatis_learn::entity::UserActivity;
use viz_rbatis_learn::state::AppState;
use viz_rbatis_learn::db::rbatis_init;

async fn index(mut req: Request) -> Result<&'static str> {
    let db = &req.extract::<State<AppState>>().await.expect("sql失败").into_inner();
    // 创建数据
    let activity: UserActivity = UserActivity {
        id: None,
        name: String::from("fengfengzhidao"),
        age: 25 as i32,
    };
    let data: rbatis::rbdc::db::ExecResult = UserActivity::insert(db.db(), &activity).await.expect(
        "数据库插入失败"
    );
    println!("{:?}", data);
    Ok("Hello, Viz!")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let state: AppState = AppState::new(rbatis_init().await.expect("数据库连接失败"));
    tracing::info!("listening on http://{addr}");
    let app = Router::new().get("/", index).with(State(state));
    if let Err(e) = serve(listener, app).await {
        tracing::debug!("{e}");
    }

    Ok(())
}
