use utils::SnekCase;

pub fn create(name: &str, context_name: &str) -> String {
  templatify! { "use context::{"; context_name ;"};
use fanta::{MiddlewareChain};

use super::"; &name.to_snek_case() ;"_service;
use models::"; &name.to_snek_case() ;"::{New"; name ;", "; name ;"};

pub fn create_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> "; context_name ;" {
  context
}

pub fn get_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> "; context_name ;" {
  context
}

pub fn update_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> "; context_name ;" {
  context
}

pub fn delete_"; &name.to_snek_case() ;"(mut context: "; context_name ;", _chain: &MiddlewareChain<"; context_name ;">) -> "; context_name ;" {
  context
}
" }
}
