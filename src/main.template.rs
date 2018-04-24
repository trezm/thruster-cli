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

use thruster::{App, MiddlewareChain, MiddlewareReturnValue};
use time::Duration;
use context::{generate_context, Ctx};

fn profiling(context: Ctx, chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let start_time = time::now();

  let ctx_future = chain.next(context)
      .and_then(move |ctx| {
        let elapsed_time: Duration = time::now() - start_time;
        println!("[{}μs] {} -- {}",
          elapsed_time.num_microseconds().unwrap(),
          ctx.method.clone(),
          ctx.path.clone());

        future::ok(ctx)
      });

  Box::new(ctx_future)
}

fn ping(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let val = "pong".to_owned();
  context.body = val;

  Box::new(future::ok(context))
}

fn main() {
  dotenv().ok();

  let mut app = App::<Ctx>::create(generate_context);

  app.use_middleware("/", profiling);
  app.get("/ping", vec![ping]);

  let host = env::var("HOST")
    .unwrap_or("0.0.0.0".to_string());
  let port = env::var("PORT")
    .unwrap_or("4321".to_string());

  println!("Running on {}:{}", &host, &port);
  App::start(app, host, port);
}
