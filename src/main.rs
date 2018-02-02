#![feature(test)]
extern crate test;
extern crate regex;

use std::fs::File;
use std::io::Read;

use std::env;
use std::collections::HashMap;
use test::Bencher;

mod parsed_file;
#[macro_use]
mod parsed_file_macro;

use parsed_file::ParsedFile;

fn main() {
  println!("Arging it up");
  let mut args = env::args();
  args.next();

  for arg in args {
    println!("arg: {}", arg);
  }

  let template = "Some awesome template {:key}";
  templatify! {
    struct Test {
      key: String,
    }, template
  };

  // Test parsed_file
  // let mut args: HashMap<& 'static str, &str> = HashMap::new();
  // args.insert("adj", "quick");
  // args.insert("verb", "jumped");

  // let parsed_file = ParsedFile::new("The {:adj} fox {:verb} the fence {awesome.}");

  // println!("Parsed File: {}", parsed_file.build(args));
}

#[bench]
fn test_parsed_file(b: &mut Bencher) {
  let index = "test_idx".to_string();
  let name = "test_alias".to_string();

  let parsed_file = ParsedFile::new("The {:adj} fox {:verb} the fence, awesome!");
  b.iter(|| {
    // let mut args: HashMap<& 'static str, &str> = HashMap::new();
    // args.insert("adj", &index);
    // args.insert("verb", &name);

    // parsed_file.build(args)
    parsed_file.build()
  });
}

#[bench]
fn test_format(b: &mut Bencher) {
  let index = "test_idx".to_string();
  let name = "test_alias".to_string();

  b.iter(|| {
    format!("The {} fox {} the fence, awesome!", index, name);
  });
}

#[bench]
fn test_push_str(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        let mut url = String::with_capacity("The  fox  the fence, awesome!".len()
            + index.len() + name.len());
        url.push_str("The ");
        url.push_str(&index);
        url.push_str(" fox ");
        url.push_str(&name);
        url.push_str(" the fence, awesome! ");
        url
    });
}

fn open_file(file_path: &str) -> String {
  let mut f = File::open(file_path)
    .expect(&format!("Couldn't open file: {}", file_path));
  let mut output_string = String::new();

  f.read_to_string(&mut output_string)
    .expect(&format!("Could not read file to string: {}", file_path));

  output_string
}
