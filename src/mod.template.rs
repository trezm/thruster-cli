mod {{ snek_case }}_controller;
mod {{ snek_case }}_service;

use thruster::{App};
use context::{generate_context, {{ ctx }}};
use crate::{{ snek_case }}_controller::{
  create_{{ snek_case }},
  get_{{ snek_case }},
  update_{{ snek_case }},
  delete_{{ snek_case }}
};

pub fn init() -> App<{{ ctx }}> {
  let mut subroute = App::<{{ ctx }}>::create(generate_context);

  subroute.post("/", vec![create_{{ snek_case }}]);
  subroute.get("/:id", vec![get_{{ snek_case }}]);
  subroute.put("/:id", vec![update_{{ snek_case }},]);
  subroute.delete("/:id", vec![delete_{{ snek_case }}]);

  subroute
}
