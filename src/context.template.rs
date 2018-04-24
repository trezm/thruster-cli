use std::collections::{HashMap};
use tokio::reactor::Handle;
use thruster::{Context, Request, Response};

pub struct Ctx {
  pub body: String,
  pub method: String,
  pub path: String,
  pub request_body: String,
  pub request_headers: HashMap<String, String>,
  pub params: HashMap<String, String>,
  pub headers: HashMap<String, String>,
  pub status_code: u16
}

impl Ctx {
  pub fn get_request_body(&self) -> String {
    self.request_body.clone()
  }

  pub fn get_requset_header(&self, header: &str) -> Option<&String> {
    self.request_headers.get(header)
  }
}

impl Context for Ctx {
  fn get_response(&self) -> Response<String> {
    let mut response_builder = Response::builder();
    response_builder.status(self.status_code);

    for header_pair in &self.headers {
      let name_ref: &str = &header_pair.0;
      let value_ref: &str = &header_pair.1;
      response_builder.header(name_ref, value_ref);
    }

    response_builder.body(self.body.clone()).unwrap()
  }

  fn set_body(&mut self, body: String) {
    self.body = body;
  }
}

pub fn generate_context(request: &Request) -> Ctx {
  Ctx {
    body: "".to_owned(),
    method: request.method().to_owned(),
    path: request.path().to_owned(),
    params: request.params().clone(),
    request_body: request.raw_body().to_owned(),
    request_headers: request.headers(),
    headers: HashMap::new(),
    status_code: 200
  }
}
