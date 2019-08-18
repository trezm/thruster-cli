pub mod {{ snek_case }}_controller;
pub mod {{ snek_case }}_service;

use thruster::{App, middleware, MiddlewareChain, MiddlewareReturnValue, Request};
use crate::context::{generate_context, {{ ctx }}};
use crate::{{ snek_case_plural }}::{{ snek_case }}_controller::{
  create_{{ snek_case }},
  get_{{ snek_case }},
  update_{{ snek_case }},
  delete_{{ snek_case }}
};

pub fn init() -> App<Request, {{ ctx }}> {
  let mut subroute = App::<Request, {{ ctx }}>::create(generate_context);

  subroute.post("/", middleware![{{ ctx }} => create_{{ snek_case }}]);
  subroute.get("/:id", middleware![{{ ctx }} => get_{{ snek_case }}]);
  subroute.put("/:id", middleware![{{ ctx }} => update_{{ snek_case }}]);
  subroute.delete("/:id", middleware![{{ ctx }} => delete_{{ snek_case }}]);

  subroute
}
