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

    if unwrapped_arg == "init" {
      init(&args.next().expect("init needs an argument, form 'fanta init [project-name]'"));
    }

    if unwrapped_arg == "migrate" {
      migrate();
    }

    arg = args.next();
  }
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
