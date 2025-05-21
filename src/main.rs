//use background::BackgroundProxy;
use futures_util::stream::StreamExt;
use kde_virtual_keyboard::VirtualKeyboardProxy;
use zbus::Connection;
//mod background;
mod virtual_keyboard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let connection = Connection::session().await?;
    let proxy = VirtualKeyboardProxy::new(&connection).await?;
    //let mut stream = proxy.receive_running_applications_changed().await?;
    let mut stream = proxy.receive_active_changed().await?;

    while let Some(_msg) = stream.next().await {
        log::info!("active changed signal received!");

        // Call GetAppState to get the current state
        match proxy.active().await {
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
