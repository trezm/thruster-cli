#![feature(test)]
extern crate test;
extern crate regex;
extern crate rand;
extern crate chrono;

#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use test::Bencher;

use std::env;
use std::process::Command;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use chrono::Utc;

#[macro_use]
mod templatify;
mod context_template;
mod main_template;
mod mod_template;
mod controller_template;
mod model_template;
mod service_template;
mod util_template;
mod utils;

use utils::SnekCase;

static TIMESTAMP_FORMAT: &str = "%Y-%m-%d-%H%M%S";

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

    if unwrapped_arg == "init" {
      init(&args.next().expect("init needs an argument, form 'fanta init [project-name]'"));
    }

    if unwrapped_arg == "migrate" {
      migrate();
    }

    arg = args.next();
  }
}

fn migrate() {
  Command::new("diesel")
    .arg("migration")
    .arg("run")
    .output()
    .expect("failed to run migrations");

  Command::new("sh")
    .arg("-c")
    .arg("diesel print-schema > src/schema.rs")
    .output()
    .expect("failed to create schema");
}

fn create_component(name: &str) {
  create_dir(format!("src/{}s", &name.to_snek_case()))
    .expect("failed to create component directory");

  Command::new("mkdir")
    .arg("-p")
    .arg("src/models")
    .output()
    .expect("failed to create models directory");

  Command::new("sh")
    .arg("-c")
    .arg(format!("echo 'pub mod {}s;\n' >> src/models/mod.rs", name.to_snek_case()))
    .output()
    .expect("failed to create models directory");

  let mut controller_file = File::create(format!("src/{}s/{}_controller.rs", &name.to_snek_case(), &name.to_snek_case()))
    .expect("Could not create controller");
  controller_file.write_all(controller_template::create(name, "Ctx").as_bytes())
    .expect("Could not write controller to file");

  let mut service_file = File::create(format!("src/{}s/{}_service.rs", &name.to_snek_case(), &name.to_snek_case()))
    .expect("Could not create service");
  service_file.write_all(service_template::create(name).as_bytes())
    .expect("Could not write service to file");

  let mut mod_file = File::create(format!("src/{}s/mod.rs", &name.to_snek_case()))
    .expect("Could not create mod file");
  mod_file.write_all(mod_template::create(name, "Ctx").as_bytes())
    .expect("Could not write mod to file");

  let mut model_file = File::create(format!("src/models/{}s.rs", name.to_snek_case()))
    .expect("Could not create model file");
  model_file.write_all(model_template::create(name).as_bytes())
    .expect("Could not write model file");

  let migration_folder = format!("migrations/{}_create_{}", Utc::now().format(TIMESTAMP_FORMAT), name.to_snek_case());
  create_dir(&migration_folder)
    .expect("failed to create migration folder");

  let mut up_file = File::create(format!("{}/up.sql", migration_folder))
    .expect("Could not create up migration file");
  up_file.write_all(format!("CREATE TABLE {}s (
  id SERIAL PRIMARY KEY,
  some_field TEXT
);
", name.to_snek_case()).as_bytes())
    .expect("Could not create up migration file");

  let mut down_file = File::create(format!("{}/down.sql", migration_folder))
    .expect("Could not create up migration file");
  down_file.write_all(format!("DROP TABLE {}s
", name.to_snek_case()).as_bytes())
    .expect("Could not create up migration file");

  let message = templatify! { "Almost there! Your new component isn't linked up to a route, so just add something like the following to your App:

mod "; &name.to_snek_case() ;"s;

...

use "; &name.to_snek_case() ;"s::{init as "; &name.to_snek_case() ;"_routes};

...

lazy_static! {
  static ref APP: App<Ctx> = {
    let mut _app = App::<Ctx>::create(generate_context);

    ....

    _app.use_sub_app(\"/"; &name.to_snek_case() ;"s\", &"; &name.to_snek_case() ;"_routes());
  }
}
" };

  println!("{}", message);
}

fn init(name: &str) {
  Command::new("mkdir")
    .arg(name)
    .output()
    .expect("failed to create project directory");

  Command::new("cargo")
    .arg("init")
    .arg("--bin")
    .current_dir(name)
    .output()
    .expect("failed to initialize rust");


  let dependencies = "'diesel = { version = \"1.0.0-rc1\", features = [\"postgres\", \"uuid\"] }
dotenv = \"0.9.0\"
fanta = \"0.1.5\"
lazy_static = \"0.2\"
serde = \"1.0.24\"
serde_json = \"1.0.8\"
serde_derive = \"1.0.24\"
time = \"0.1.38\"
tokio-proto = \"0.1\"
tokio-service = \"0.1\"
env_logger = { version = \"0.3.4\", default-features = false }
'";

  Command::new("sh")
    .arg("-c")
    .arg(format!("echo {} >> Cargo.toml", dependencies))
    .current_dir(name)
    .output()
    .expect("failed to add dependencies");

  let database_file = format!("DATABASE_URL=postgres://postgres@localhost/{}", name);
  Command::new("sh")
    .arg("-c")
    .arg(format!("echo {} > .env", database_file))
    .current_dir(name)
    .output()
    .expect("failed to create databse file");

  Command::new("diesel")
    .arg("setup")
    .current_dir(name)
    .output()
    .expect("failed to setup diesel");

  let mut main_file = File::create(format!("{}/src/main.rs", name))
    .expect("Could not create main file");
  main_file.write_all(main_template::create().as_bytes())
    .expect("Could not write main file");

  Command::new("mkdir")
    .arg(format!("{}/src/models", name))
    .output()
    .expect("failed to create models directory");

  let mut models_mod_file = File::create(format!("{}/src/models/mod.rs", name))
    .expect("Could not create models/mod file");
  models_mod_file.write_all("// Models
".as_bytes())
    .expect("Could not write models/mod file");

  let mut context_file = File::create(format!("{}/src/context.rs", name))
    .expect("Could not create context file");
  context_file.write_all(context_template::create().as_bytes())
    .expect("Could not write context file");

  let mut context_file = File::create(format!("{}/src/util.rs", name))
    .expect("Could not create util file");
  context_file.write_all(util_template::create().as_bytes())
    .expect("Could not write util file");
}

// #[bench]
// fn test_format(b: &mut Bencher) {
//   let index = "test_idx".to_string();
//   let name = rand::thread_rng()
//         .gen_ascii_chars()
//         .take(10)
//         .collect::<String>().to_string();

//   b.iter(|| {
//     format!("The {} fox {} the fence, awesome!", index, name);
//   });
// }

// #[bench]
// fn test_push_str(b: &mut Bencher) {
//     let index = "test_idx".to_string();
//     let name = rand::thread_rng()
//         .gen_ascii_chars()
//         .take(10)
//         .collect::<String>().to_string();

//     b.iter(|| {
//         let mut url = String::with_capacity("The  fox  the fence, awesome!".len()
//             + index.len() + name.len());
//         url.push_str("The ");
//         url.push_str(&index);
//         url.push_str(" fox ");
//         url.push_str(&name);
//         url.push_str(" the fence, awesome! ");
//         url
//     });
// }

// #[bench]
// fn test_templatify(b: &mut Bencher) {
//   let index = "test_idx";
//   let name = &rand::thread_rng()
//         .gen_ascii_chars()
//         .take(10)
//         .collect::<String>();

//   b.iter(|| {
//     templatify! {
//       "The "; index ;" fox "; name ;" the fence, awesome!"
//     }
//   });
// }
