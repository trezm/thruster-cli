extern crate regex;
extern crate rand;
extern crate chrono;

#[macro_use] extern crate fuel_line;
#[macro_use] extern crate fuel_line_derive;

use std::env;

#[macro_use]
mod templatify;
mod utils;
mod generator;

use generator::{create_component, migrate, init};

fn main() {
  let mut args = env::args();
  args.next();

  let mut arg = args.next();

  while arg.is_some() {
    let unwrapped_arg = arg.unwrap();

    if unwrapped_arg == "c" ||
      unwrapped_arg == "component" {

      let name = args.next().expect("Component flag needs an argument");
      create_component(&name);
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
      init(&args.next().expect("init needs an argument, form 'thruster init [project-name]'"));
    }

    if unwrapped_arg == "migrate" {
      migrate();
    }

    arg = args.next();
  }
}
