use crate::logger::Logger;

pub async fn loop_handler() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new(Some("loop_handle"));
    tokio::spawn(async move {
        logger.info("Starting loop_handling server!");
        loop {
            ()
        }
    });

    Ok(())
}