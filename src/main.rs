use core::error;
use std::any::Any;

//use background::BackgroundProxy;
use futures_util::stream::StreamExt;
//use virtual_keyboard::VirtualKeyboardProxy;
//use zbus::Connection;
//mod background;
//mod virtual_keyboard;

use log::{debug, error, log_enabled, info, Level};

//use futures_util::stream::StreamExt;
use zbus::Connection;
use zbus::MessageStream;
use zbus::MatchRule;
use zbus::message::Type;

#[tokio::main]
async fn main() -> Result<(), zbus::Error> {
    env_logger::init();

    let connection = Connection::session().await?;

    let match_rule = MatchRule::builder()
        .msg_type(Type::Signal)
        //.interface("org.freedesktop.DBus.Properties")
        //.unwrap()
        //.member("PropertiesChanged")
        //.unwrap()
        .build();

    let mut stream = MessageStream::for_match_rule(match_rule, &connection, None).await?;

    while let Some(msg) = stream.next().await {
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
                }
            }
            Err(e) => {
                error!("got error {}", e);
            }

        }
    }

    Ok(())
}



//     let connection = Connection::session().await?;
//     let proxy = VirtualKeyboardProxy::new(&connection).await?;
//     //let mut stream = proxy.receive_running_applications_changed().await?;
//     let mut stream = proxy.receive_active_changed().await?;

//     while let Some(_msg) = stream.next().await {
//         log::info!("active changed signal received!");

//         // Call GetAppState to get the current state
//         match proxy.active().await {
//             Ok(app_state) => {
//                 log::info!("Current application state: {:?}", app_state);
//                 // Process the app_state data
//             }
//             Err(e) => {
//                 log::error!("Error getting application state: {}", e);
//             }
//         }
//     }

//     log::error!("Stream ended unexpectedly");
//     Ok(())
// }
