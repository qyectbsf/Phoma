use background::BackgroundProxy;
use futures_util::stream::StreamExt;
use zbus::Connection;
mod background;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let connection = Connection::session().await?;
    let proxy = BackgroundProxy::new(&connection).await?;
    let mut stream = proxy.receive_running_applications_changed().await?;

    while let Some(_msg) = stream.next().await { //Note:  We ignore the msg, as it has no data
        log::info!("RunningApplicationsChanged signal received!");

        // Call GetAppState to get the current state
        match proxy.get_app_state().await {
            Ok(app_state) => {
                log::info!("Current application state: {:?}", app_state);
                // Process the app_state data
            }
            Err(e) => {
                log::error!("Error getting application state: {}", e);
            }
        }
    }

    log::error!("Stream ended unexpectedly");
    Ok(())
}
