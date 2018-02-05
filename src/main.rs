#![feature(test)]
extern crate test;
extern crate regex;
extern crate rand;

#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use test::Bencher;

use std::env;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
mod templatify;
mod main_template;
mod mod_template;
mod controller_template;
mod model_template;
mod service_template;
mod utils;

use utils::SnekCase;

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
  Command::new("mkdir")
    .arg(format!("src/{}", name))
    .output()
    .expect("failed to create component directory");

  Command::new("mkdir")
    .arg("-p")
    .arg("src/models")
    .output()
    .expect("failed to create models directory");

  Command::new("diesel")
    .arg("migration")
    .arg("generate")
    .arg(format!("create_{}", &name.to_snek_case()))
    .output()
    .expect("failed to create migration");

  let mut controller_file = File::create(format!("src/{}/{}_controller.rs", name, &name.to_snek_case()))
    .expect("Could not create controller");
  controller_file.write_all(controller_template::create(name, "Ctx").as_bytes())
    .expect("Could not write controller to file");

  let mut service_file = File::create(format!("src/{}/{}_service.rs", name, &name.to_snek_case()))
    .expect("Could not create service");
  service_file.write_all(service_template::create(name).as_bytes())
    .expect("Could not write service to file");

  let mut mod_file = File::create(format!("src/{}/mod.rs", name))
    .expect("Could not create mod file");
  mod_file.write_all(mod_template::create(name, "Ctx").as_bytes())
    .expect("Could not write mod to file");

  let mut model_file = File::create(format!("src/models/{}.rs", name))
    .expect("Could not create model file");
  model_file.write_all(model_template::create(name).as_bytes())
    .expect("Could not write model file");

  // Not working yet :(
  // let output = Command::new("ls")
//     .arg(format!("migrations/*create_{}", name.to_snek_case()))
//     .output()
//     .expect("failed to create up migraiton");

//   println!("{}/down.rs", String::from_utf8_lossy(&output.stdout));

//   let mut up_file = File::open(format!("{}/up.rs", String::from_utf8_lossy(&output.stdout)))
//     .expect("Could not create up migration file");
//   up_file.write_all(format!("CREATE TABLE {}s (
//   id SERIAL PRIMARY KEY,
// );
// ", name.to_snek_case()).as_bytes())
//     .expect("Could not create up migration file");

//   let mut down_file = File::open(format!("{}/down.rs", String::from_utf8_lossy(&output.stdout)))
//     .expect("Could not create up migration file");
//   down_file.write_all(format!("DROP TABLE {}s
// ", name.to_snek_case()).as_bytes())
//     .expect("Could not create up migration file");
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
fanta = \"0.1.4\"
lazy_static = \"0.2\"
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

  let database_file = format!("DATABASE_URL=postgres://admin:123456@localhost/{}", name);
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
