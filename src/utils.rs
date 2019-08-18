use regex::{Captures, Regex};

pub trait SnekCase {
    fn to_snek_case(&self) -> String;
}

impl SnekCase for str {
    fn to_snek_case(&self) -> String {
        let reg = Regex::new(r"(.)([A-Z])").unwrap();

        let final_value = reg.replace_all(&self, |caps: &Captures| {
            format!("{}_{}", &caps[1], &caps[2])
        });

        final_value.to_lowercase()
    }
}

impl SnekCase for String {
    fn to_snek_case(&self) -> String {
        let reg = Regex::new(r"(.)([A-Z])").unwrap();

        let final_value = reg.replace_all(&self, |caps: &Captures| {
            format!("{}_{}", &caps[1], &caps[2])
        });

        final_value.to_lowercase()
    }
}

#[cfg(test)]
mod test {
    use super::SnekCase;

    #[test]
    fn it_should_insert_underscores() {
        let camel_case = "HelloWorld";

        assert!(camel_case.to_snek_case() == "hello_world");
    }
}
