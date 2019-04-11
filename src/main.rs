#[macro_use] extern crate fuel_line;
#[macro_use] extern crate fuel_line_derive;

use std::env;

#[macro_use]
mod templatify;
mod utils;
mod generator;

use crate::generator::{create_component, migrate, init};

enum Command {
  NewComponent(String, bool),
  Init(String, bool),
  Migrate,
  None
}

fn main() {
  let mut args = env::args();
  args.next();

  let mut arg = args.next();

  let mut command = Command::None;

  while arg.is_some() {
    let unwrapped_arg = arg.unwrap();

    if unwrapped_arg == "c" ||
      unwrapped_arg == "component" {

      let mut component_name = "".to_string();
      let mut is_async = false;

      arg = args.next();
      while arg.is_some() {
        let arg_clone = arg.clone();
        component_name = arg_clone.unwrap().clone();

        match arg.unwrap().as_ref() {
          "-a" | "--async" => is_async = true,
          _ => ()
        };

        arg = args.next();
      }

      command = Command::NewComponent(component_name, is_async);
    }

    if unwrapped_arg == "-h" ||
      unwrapped_arg == "--help" {
      println!("Useage: thruster [command]

commands:
    init [project-name]   Generate a new project with the given project name. Good project names are of the form 'test-project'
    migrate               Run migrations
    component [name]      Create a new component with the given name. Components should be PascalCase, i.e. SessionToken.
")
    }

    if unwrapped_arg == "init" {
      // let project_name = args.next().expect("init needs an argument, form 'thruster init [project-name]'");
      // command = Command::Init(project_name, false);
      let mut project_name = "".to_string();
      let mut is_async = false;

      arg = args.next();
      while arg.is_some() {
        let arg_clone = arg.clone();
        project_name = arg_clone.unwrap().clone();

        match arg.unwrap().as_ref() {
          "-a" | "--async" => is_async = true,
          _ => ()
        };

        arg = args.next();
      }

      command = Command::Init(project_name, is_async);
    }

    if unwrapped_arg == "migrate" {
      command = Command::Migrate
    }

    arg = args.next();
  }

  match command {
    Command::NewComponent(name, is_async) => create_component(&name, is_async),
    Command::Init(name, is_async) => init(&name, is_async),
    Command::Migrate => migrate(),
    Command::None => ()
  };
}
