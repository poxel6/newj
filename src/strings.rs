pub trait Strings {
    fn to_capitilize(&self) -> String;
    fn to_snake_case(&self) -> String;
}

impl Strings for String {
    fn to_capitilize(&self) -> String {
        let mut result = self.to_string();
        let first = self.chars().nth(0).unwrap();
        if !first.is_uppercase() {
            result.replace_range(..1, first.to_uppercase().to_string().as_str());
        }
        result
    }

    fn to_snake_case(&self) -> String {
        let mut result = String::from("");
        for c in self.chars() {
            if c.is_uppercase() && c.is_alphabetic() {
                result.push_str("_");
                result.push_str(c.to_string().to_lowercase().as_str());
            } else {
                result.push_str(c.to_string().as_str());
            }
        }
        if result.starts_with("_") {
            result.replace_range(0..1, "");
        }
        result
    }
}
