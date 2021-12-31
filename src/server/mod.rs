pub mod actix;
pub mod auth;
pub mod test_handling;

pub async fn server_run() {
    test_handling::loop_handler().await;
    actix::run_actix().await;
}