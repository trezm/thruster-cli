#![feature(await_macro, async_await, futures_api, proc_macro_hygiene)]
#[macro_use] extern crate thruster;
extern crate futures;

use futures::future;
use std::boxed::Box;
use std::pin::Pin;

use thruster::{MiddlewareNext, MiddlewareReturnValue};
use thruster::{App, BasicContext as Ctx, Request};
use thruster::server::Server;
use thruster::ThrusterServer;
use thruster::thruster_proc::{async_middleware, middleware_fn};

#[middleware_fn]
async fn ping(context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
  let val = "pong";
  context.body(val);

  context
}

fn main() {
  println!("Starting server...");

  let mut app = App::<Request, Ctx>::new_basic();

  app.get("/ping", async_middleware!(Ctx, [plaintext]));

  let server = Server::new(app);
  server.start("0.0.0.0", 4321);
}
