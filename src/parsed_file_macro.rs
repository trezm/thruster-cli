use std::collections::HashMap;
use regex::Regex;

macro_rules! templatify {
  (struct $name:ident {
    $($field_name:ident: $field_type:ty,)*
  }, $template:ident) => {
    struct $name<'a> {
      _processed_template_pieces: Vec<&'a str>,
      _processed_keys: Vec<&'a str>,
      $($field_name: $field_type,)*
    }

    impl<'a> $name<'a> {
      pub fn parse(contents: &str) {
        let re = Regex::new("\\{:[^\\s]+\\}").unwrap();

        let mut last_index = 0;
        for mat in re.find_iter(contents) {
          let unformatted_piece = contents[last_index..mat.start()];
          self._processed_vector.push(unformatted_piece);
          self._processed_keys.push(contents[mat.start() + 2..mat.end() - 1]);
          last_index = mat.end();
          self._string_length_without_keys = self._string_length_without_keys + unformatted_piece.len();
        }

        self._processed_vector.push(contents[last_index..]);
      }

      pub fn to_str(&self) -> &str {

      }

      // This is purely an example—not a good one.
      fn get_field_names() -> Vec<&'static str> {
        vec![$(stringify!($field_name)),*]
      }
    }
  }
}

// #[derive(Debug)]
// pub struct ParsedFile<'a> {
//   unformatted_template: &'a str,
//   _processed_template_pieces: Vec<&'a str>,
//   _processed_keys: Vec<&'a str>,
//   _string_length_without_keys: usize
// }

// impl<'a> ParsedFile<'a> {
//   pub fn new(template: &'a str) -> ParsedFile<'a> {
//     let mut parsed_file = ParsedFile {
//       unformatted_template: template,
//       _processed_vector: Vec::new(),
//       _processed_keys: Vec::new(),
//       _string_length_without_keys: 0
//     };

//     parsed_file._run_slice_contents();

//     parsed_file
//   }

//   pub fn build(&self, parameters: HashMap<& 'static str, &str>) -> String {
//     let mut param_length = 0;
//     for key in &self._processed_keys {
//       let value = parameters.get(key).expect(&format!("Could not find key: {}", key));

//       param_length = param_length + value.len();
//     };

//     let mut output_string = String::with_capacity(self._string_length_without_keys + param_length);
//     let mut unformatted_iter = (&self._processed_vector).into_iter();

//     output_string.push_str(unformatted_iter.next().expect("If we ever get here, panic!"));
//     for key in &self._processed_keys {
//       let value = parameters.get(key).expect(&format!("Could not find key: {}", key));
//       output_string.push_str(value);
//       output_string.push_str(unformatted_iter.next().expect("If we ever get here, panic!"));
//     }

//     output_string
//   }

//   /**
//    * Make the formatting like "Blah blah {:key} blah blah"
//    * Algorithm needs to be:
//    * 1. For each key appearance, slice the string
//    * 2. For each key appearance, add a copy of the key for that key to an array
//    * 3. Add original slice, then the value for the key in order until no more original slices.
//    *
//    * Note that 1 and 2 can be precompiled to work with different inputs.
//    */
//   fn _run_slice_contents(&mut self) {
//     let re = Regex::new("\\{:[^\\s]+\\}").unwrap();

//     let mut last_index = 0;
//     for mat in re.find_iter(&self.unformatted_contents) {
//       let unformatted_piece = &self.unformatted_contents[last_index..mat.start()];
//       self._processed_vector.push(unformatted_piece);
//       self._processed_keys.push(&self.unformatted_contents[mat.start() + 2..mat.end() - 1]);
//       last_index = mat.end();
//       self._string_length_without_keys = self._string_length_without_keys + unformatted_piece.len();
//     }

//     self._processed_vector.push(&self.unformatted_contents[last_index..]);
//   }
// }
