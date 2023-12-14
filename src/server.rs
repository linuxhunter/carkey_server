use std::time::{Duration, Instant};
use actix::prelude::*;
use actix::Actor;
use actix_web_actors::ws;
use crate::{events, update_vehicle_address};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Event {
    pub data: events::EventMessage,
}

pub struct MyWebSocket {
    hb: Instant,
}

impl MyWebSocket {
    pub fn new() -> Self {
        MyWebSocket {
            hb: Instant::now(),
        }
    }
    pub fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let vehicle_address = ctx.address().recipient();
        update_vehicle_address(vehicle_address);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Text(msg)) => {
                println!("Message from client: {}", msg);
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<Event> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: Event, ctx: &mut Self::Context) -> Self::Result {
        ctx.binary(msg.data.serialize())
    }
}