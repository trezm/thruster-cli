use context::{ {{ ctx }}};
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use super::{{ snek_case }}_service;
use models::{{ snek_case }}s::{ New{{ name }}, {{ name }} };
use futures::future;
use std::boxed::Box;

pub fn create_{{ snek_case }}(mut context: {{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  Box::new(future::ok(context))
}

pub fn get_{{ snek_case }}(mut context: {{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  Box::new(future::ok(context))
}

pub fn update_{{ snek_case }}(mut context: {{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  Box::new(future::ok(context))
}

pub fn delete_{{ snek_case }}(mut context:{{ ctx }}, _next: impl Fn({{ ctx }}) -> MiddlewareReturnValue<{{ ctx }}>  + Send + Sync) -> MiddlewareReturnValue<{{ ctx }}> {
  Box::new(future::ok(context))
}
