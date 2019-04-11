extern crate dotenv;
extern crate env_logger;
extern crate thruster;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_proto;
extern crate tokio_service;
extern crate time;
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
use futures::{future, Future};

use thruster::{middleware, App, MiddlewareChain, MiddlewareReturnValue};
use thruster::server::Server;
use thruster::ThrusterServer;
use time::Duration;

use crate::context::{generate_context, Ctx};

fn profiling(context: Ctx, next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  let start_time = time::now();

  let ctx_future = next(context)
      .and_then(move |ctx| {
        let elapsed_time: Duration = time::now() - start_time;
        println!("[{}μs] {} -- {}",
          elapsed_time.num_microseconds().unwrap(),
          ctx.request.method(),
          ctx.request.path());

        future::ok(ctx)
      });

  Box::new(ctx_future)
}

fn ping(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  let val = "pong";
  context.body(val);

  Box::new(future::ok(context))
}

fn not_found(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  context.body("Whoops! Nothing here!");
  context.status(404);

  Box::new(future::ok(context))
}

fn main() {
  dotenv().ok();

  let mut app = App::create(generate_context);

  app.use_middleware("/", middleware![Ctx => profiling]);
  app.get("/ping", middleware![Ctx => ping]);

  app.set404(middleware![Ctx => not_found]);

  let host = env::var("HOST")
    .unwrap_or("0.0.0.0".to_string());
  let port = env::var("PORT")
    .unwrap_or("4321".to_string());

  println!("Running on {}:{}", &host, &port);
  let server = Server::new(app);
  server.start(&host, port.parse::<u16>().unwrap());
}
