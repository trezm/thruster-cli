use utils::SnekCase;

pub fn create(name: &str, context_name: &str) -> String {
  templatify! { "use context::{"; context_name ;"};
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use super::"; &name.to_snek_case() ;"_service;
use models::"; &name.to_snek_case() ;"s::{New"; name ;", "; name ;"};
use futures::future;
use std::boxed::Box;

pub fn create_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> MiddlewareReturnValue<"; context_name ;"> {
  Box::new(future::ok(context))
}

pub fn get_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> MiddlewareReturnValue<"; context_name ;"> {
  Box::new(future::ok(context))
}

pub fn update_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> MiddlewareReturnValue<"; context_name ;"> {
  Box::new(future::ok(context))
}

pub fn delete_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> MiddlewareReturnValue<"; context_name ;"> {
  Box::new(future::ok(context))
}
" }
}
