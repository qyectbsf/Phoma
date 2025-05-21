use futures_util::stream::StreamExt;
use virtual_keyboard::VirtualKeyboardProxy;
mod virtual_keyboard;

use log::{debug, error, info, log_enabled, Level};

use zbus::message::Type;
use zbus::Connection;
use zbus::MatchRule;
use zbus::MessageStream;

#[tokio::main]
async fn main() -> Result<(), zbus::Error> {
    env_logger::init();

    let connection = Connection::session().await?;

    let virutal_keyboard_proxy = VirtualKeyboardProxy::new(&connection).await?;

    let match_rule = MatchRule::builder()
        .msg_type(Type::Signal)
        //.interface("org.freedesktop.DBus.Properties")
        //.unwrap()
        //.member("PropertiesChanged")
        //.unwrap()
        .build();

    let mut signal_stream = MessageStream::for_match_rule(match_rule, &connection, None).await?;

    while let Some(msg) = signal_stream.next().await {
        match msg {
            Ok(e) => {
                // info!("got dbus signal:\npath: {}\ninterface: {}\nmember: {}\n",
                //            e.header().path().unwrap(),
                //            e.header().interface().unwrap(),
                //            e.header().member().unwrap());

                if e.header().path().unwrap() == "/VirtualKeyboard" {
                    info!("got dbus signal:\nmessage_type: {:?}\npath: {}\ninterface: {}\nmember: {}\nreply_serial: {:?}\ndestination: {:?}\nsender: {}\n",
                          e.header().message_type(),
                          e.header().path().unwrap(),
                          e.header().interface().unwrap(),
                          e.header().member().unwrap(),
                          e.header().reply_serial(),
                          e.header().destination(),
                          e.header().sender().unwrap(),
                    );

                    if e.header().member().unwrap() == "activeChanged" {
                        let result = virutal_keyboard_proxy.active().await?;
                        info!("{}: {}", e.header().member().unwrap(), result);
                    }

                    if e.header().member().unwrap() == "activeClientSupportsTextInputChanged" {
                        let result = virutal_keyboard_proxy.active_client_supports_text_input().await?;
                        info!("{}: {}", e.header().member().unwrap(), result);
                    }

                    if e.header().member().unwrap() == "availableChanged" {
                        let result = virutal_keyboard_proxy.available().await?;
                        info!("{}: {}", e.header().member().unwrap(), result);
                    }

                    if e.header().member().unwrap() == "enabledChanged" {
                        let result = virutal_keyboard_proxy.enabled().await?;
                        info!("{}: {}", e.header().member().unwrap(), result);
                    }

                    if e.header().member().unwrap() == "visibleChanged" {
                        let result = virutal_keyboard_proxy.visible().await?;
                        info!("{}: {}", e.header().member().unwrap(), result);
                    }
                }
            }
            Err(e) => {
                error!("got error {}", e);
            }
        }
    }

    Ok(())
}
