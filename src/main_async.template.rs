#![feature(await_macro, async_await, futures_api, proc_macro_hygiene)]

extern crate dotenv;
extern crate env_logger;
extern crate thruster;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;

mod context;
mod util;

use std::env;
use std::boxed::Box;
use dotenv::dotenv;

use thruster::{MiddlewareNext, MiddlewareReturnValue};
use thruster::{App};
use thruster::server::Server;
use thruster::ThrusterServer;
use thruster::thruster_proc::{async_middleware, middleware_fn};

use std::time::Instant;

use crate::context::{generate_context, Ctx};

#[middleware_fn]
async fn profiling(mut context: Ctx, next: MiddlewareNext<Ctx>) -> Ctx {
  let start_time = Instant::now();

  context = await!(next(context));

  let elapsed_time = start_time.elapsed();
  println!("[{}μs] {} -- {}",
    elapsed_time.as_micros(),
    context.request.method(),
    context.request.path());

  context
}

#[middleware_fn]
async fn ping(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
  let val = "pong";
  context.body(val);

  context
}

#[middleware_fn]
async fn not_found(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
  context.body("Whoops! Nothing here!");
  context.status(404);

  context
}

fn main() {
  dotenv().ok();

  let mut app = App::create(generate_context);

  app.use_middleware("/", async_middleware!(Ctx, [profiling]));
  app.get("/ping", async_middleware!(Ctx, [ping]));

  app.set404(async_middleware!(Ctx, [not_found]));

  let host = env::var("HOST")
    .unwrap_or("0.0.0.0".to_string());
  let port = env::var("PORT")
    .unwrap_or("4321".to_string());

  println!("Running on {}:{}", &host, &port);
  let server = Server::new(app);
  server.start(&host, port.parse::<u16>().unwrap());
}
