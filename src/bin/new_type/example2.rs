use std::fmt::Display;

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let wrapper = Wrapper(vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ]);
        assert_eq!(format!("{}", wrapper), "[apple, banana, cherry]");
    }
}
