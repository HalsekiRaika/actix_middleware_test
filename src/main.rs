mod server;
mod logger;

// このコードはCtrl+Cで停止できません。
// タスクマネージャーなどを使用してプロセスを強制終了してください。
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    println!("Hello, world!");

    server::server_run().await;
    Ok(())
}