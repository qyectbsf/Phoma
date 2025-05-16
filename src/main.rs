use background::BackgroundProxy;
// NOTE: When changing this, please also keep `zbus/examples/watch-systemd-jobs.rs` in sync.
use futures_util::stream::StreamExt;
use zbus::{zvariant::OwnedObjectPath, proxy, Connection};
mod background;

fn main() {
    async_io::block_on(watch_running_apps()).expect("Error listening to signal");
}

async fn watch_running_apps() -> zbus::Result<()> {
    println!("here con");
    let connection = Connection::system().await?;

    println!("here proxy");
    let proxy = BackgroundProxy::new(&connection).await?;

    println!("here pre stream");
    let mut stream = proxy.receive_running_applications_changed().await?;
    println!("here post stream");

    while let Some(msg) = stream.next().await {
        print!("here");
    }

    panic!("Stream ended unexpectedly");
}
