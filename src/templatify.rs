macro_rules! templatify {
  ( $head_template:expr $(;$key:expr; $template:expr)* ) => {
    {
      let mut total_length = 0;
      total_length = total_length + $head_template.len();

      $(
        total_length = total_length + $key.len() + $template.len();
      )*

      let mut output_string = String::with_capacity(total_length);
      output_string.push_str($head_template);

      $(
        output_string.push_str($key);
        output_string.push_str($template);
      )*

      output_string
    }
  }
}
