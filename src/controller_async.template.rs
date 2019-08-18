use std::boxed::Box;
use uuid::Uuid;
use thruster::{MiddlewareNext, MiddlewareReturnValue};
use thruster::thruster_proc::{middleware_fn};

use crate::{{ snek_case_plural }}::{{ snek_case }}_service;
use crate::models::{{ snek_case_plural }}::{ New{{ name }}, {{ name }} };
use crate::context::{ {{ ctx }} };

#[middleware_fn]
pub async fn create_{{ snek_case }}(mut context: {{ ctx }}, _next: MiddlewareNext<{{ ctx }}>) -> {{ ctx }} {
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

  context
}

#[middleware_fn]
pub async fn get_{{ snek_case }}(mut context: {{ ctx }}, _next: MiddlewareNext<{{ ctx }}>) -> {{ ctx }} {
  fn error(mut context: Ctx) -> Ctx {
    context.status(400);
    context.body("Could not get {{ name }}");
    context
  }

  let id = match context.params.get("id") {
    Some(_id) => _id,
    None => return error(context)
  };

  let parsed_id = match id.parse::<Uuid>() {
    Ok(_parsed_id) => _parsed_id,
    Err(_) => return error(context)
  };

  let fetched_result = match {{ snek_case }}_service::get_{{ snek_case }}(parsed_id) {
    Ok(_fetched_result) => _fetched_result,
    Err(_) => return error(context)
  };

  match serde_json::to_string(&fetched_result) {
    Ok(result) => context.body(&result),
    Err(_) => return error(context)
  };

  context
}

#[middleware_fn]
pub async fn update_{{ snek_case }}(context: {{ ctx }}, _next: MiddlewareNext<{{ ctx }}>) -> {{ ctx }} {
  context
}

#[middleware_fn]
pub async fn delete_{{ snek_case }}(context: {{ ctx }}, _next: MiddlewareNext<{{ ctx }}>) -> {{ ctx }} {
  context
}
