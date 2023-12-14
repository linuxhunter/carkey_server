mod server;
mod events;

use std::sync::Mutex;
use std::thread;
use actix::Recipient;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use actix_web_actors::ws;
use lazy_static::lazy_static;
use crate::events::EventMessage;
use crate::server::{Event, MyWebSocket};

lazy_static! {
    static ref VEHICLE_ADDR: Mutex<Option<Recipient<Event>>> = Mutex::new(None);
}

pub fn update_vehicle_address(address: Recipient<Event>) {
    let mut vehicle_address = VEHICLE_ADDR.lock().unwrap();
    *vehicle_address = Some(address);
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    thread::spawn(move || loop {
        let mut cmd = String::with_capacity(32);
        if std::io::stdin().read_line(&mut cmd).is_err() {
            return;
        }
        let notification = EventMessage::try_from(cmd.as_ref()).unwrap();
        let vehicle_addr = VEHICLE_ADDR.lock().unwrap();
        vehicle_addr.clone().unwrap().do_send(Event {
            data: notification,
        });
    });

    HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(ws_index))
    })
        .bind(("0.0.0.0", 12345))?
        .run()
        .await
}
