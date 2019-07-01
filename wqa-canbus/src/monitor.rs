#![feature(async_await)]
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use failure::Fallible;
use super::*;
// use super::
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
#[derive(Default)]
pub struct MonitorIO<BUS>
where BUS: {
    contents: Mutex<Vec<Message>>,
    bus: Mutex<Connection>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    author: Option<String>,
    contents: String,
}

impl MonitorIO {
    fn insert(&self, msg: Message) -> usize {
        let mut table = self.contents.lock().unwrap();
        table.push(msg);
        table.len() - 1
    }

    fn get(&self, id: usize) -> Option<Message> {
        self.contents.lock().unwrap().get(id).cloned()
    }

    fn set(&self, id: usize, msg: Message) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(old_msg) = table.get_mut(id) {
            *old_msg = msg;
            true
        } else {
            false
        }
    }

}



async fn new_message(mut cx: Context<MonitorIO>) -> EndpointResult<String> {
    let msg = cx.body_json().await.client_err()?;
    Ok(cx.state().insert(msg).to_string())
}

async fn set_message(mut cx: Context<MonitorIO>) -> EndpointResult<()> {
    let msg = cx.body_json().await.client_err()?;
    let id = cx.param("id").client_err()?;

    if cx.state().set(id, msg) {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn get_message(cx: Context<MonitorIO>) -> EndpointResult {
    let id = cx.param("id").client_err()?;
    if let Some(msg) = cx.state().get(id) {
        Ok(response::json(msg))
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn device_state(cx: Context<MonitorIO>) -> EndpointResult {

    let state:String = "Analyser state".to_owned();
    Ok(response::json(state))

}



pub async fn mio_api_run() -> Fallible<()>{
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    let settings = crate::settings::Settings::new()?;

    let app_config = Configuration::build()
        .address(settings.server_address.as_ref())
        .port(settings.server_port)
        .finalize();
    let mut app = App::with_state(Database::default());
    DBusMod::
    // app.config(app_config);
    app.at("/humidity").post(new_message);
    app.at("/message/:id").get(get_message).post(set_message);
    app.at("/dbus/").get(get)
    app.run("127.0.0.1:7777").unwrap();
}



//
