use crate::context::{ {{ ctx }} };
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::{{ snek_case }}s::{{ snek_case }}_service;
use crate::models::{{ snek_case }}s::{ New{{ name }}, {{ name }} };
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

pub fn create_{{ snek_case }}(mut context: {{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  match serde_json::from_str::<New{{ name }}>(&context.request.body()) {
    Ok(new_{{ snek_case }}) => {
      match {{ snek_case }}_service::create_{{ snek_case }}(new_{{ snek_case }}) {
        Ok({{ snek_case }}) => {
          context.body(&serde_json::to_string(&{{ snek_case }}).unwrap());
        },
        Err(e) => {
          context.status(400);
          context.body("Could not create a new {{ name }}");
        }
      };
    },
    Err(e) => {
      context.status(400);
      context.body("Could not create a new {{ name }}");
    }
  };

  Box::new(future::ok(context))
}

pub fn get_{{ snek_case }}(mut context: {{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
    context.status(400);
    context.body("Could not get {{ name }}");
    Box::new(future::ok(context))
  }

  let id = match context.params.get("id") {
    Some(_id) => _id,
    None => return error(context)
  };

  let id_as_number = match id.parse::<i32>() {
    Ok(_id_as_number) => _id_as_number,
    Err(_) => return error(context)
  };

  let fetched_result = match {{ snek_case }}_service::get_{{ snek_case }}(id_as_number) {
    Ok(_fetched_result) => _fetched_result,
    Err(_) => return error(context)
  };

  match serde_json::to_string(&fetched_result) {
    Ok(result) => context.body(&result),
    Err(_) => return error(context)
  };

  Box::new(future::ok(context))
}

pub fn update_{{ snek_case }}(mut context: {{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  Box::new(future::ok(context))
}

pub fn delete_{{ snek_case }}(mut context:{{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  Box::new(future::ok(context))
}
