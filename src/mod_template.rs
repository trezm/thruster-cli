use utils::SnekCase;

pub fn create(name: &str, context_name: &str) -> String {
  templatify! { "mod "; &name.to_snek_case() ;"_controller;
mod "; &name.to_snek_case() ;"_service;

use thruster::{App};
use context::{generate_context, "; context_name ;"};
use self::"; &name.to_snek_case() ;"_controller::{create_"; &name.to_snek_case() ;", get_"; &name.to_snek_case() ;", update_"; &name.to_snek_case() ;", delete_"; &name.to_snek_case() ;"};

pub fn init() -> App<"; context_name ;"> {
  let mut subroute = App::<"; context_name ;">::create(generate_context);

  subroute.post(\"/\", vec![create_"; &name.to_snek_case() ;"]);
  subroute.get(\"/:id\", vec![get_"; &name.to_snek_case() ;"]);
  subroute.put(\"/:id\", vec![update_"; &name.to_snek_case() ;",]);
  subroute.delete(\"/:id\", vec![delete_"; &name.to_snek_case() ;"]);

  subroute
}
" }
}
