pub fn create() -> String {
  templatify! { "extern crate dotenv;
extern crate env_logger;
extern crate fanta;
extern crate serde;
extern crate serde_json;
extern crate tokio_proto;
extern crate tokio_service;
extern crate time;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;

mod context;
mod util;

use fanta::{App, MiddlewareChain, MiddlewareReturnValue};
use time::Duration;
use context::{generate_context, Ctx};

lazy_static! {
  static ref APP: App<Ctx> = {
    let mut _app = App::<Ctx>::create(generate_context);

    _app.use_middleware(\"/\", profiling);

    _app.set404(vec![not_found_404]);

    _app
  };
}

fn not_found_404(context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let mut context = Ctx::new(context);

  context.body = \"<html>
  ( ͡° ͜ʖ ͡°) What're you looking for here?
</html>\".to_owned();
  context.set_header(\"Content-Type\", \"text/html\");
  context.status_code = 404;

  Box::new(future::ok(context))
}

fn profiling(context: Ctx, chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let start_time = time::now();

  let context = chain.next(context);

  let elapsed_time: Duration = time::now() - start_time;
  println!(\"[{}micros] {} -- {}\",
    elapsed_time.num_microseconds().unwrap(),
    context.method.clone(),
    context.path.clone());

  Box::new(future::ok(context))
}

fn main() {
  println!(\"Starting server...\");

  drop(env_logger::init());
  App::start(&APP, \"0.0.0.0\".to_string(), \"8080\".to_string());
}
" }
}
