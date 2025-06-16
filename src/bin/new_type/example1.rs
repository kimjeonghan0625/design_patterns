use std::fmt::Display;

struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let unsecured_password = "secret".to_string();
        let password = Password(unsecured_password.clone());
        assert_eq!(format!("{}", unsecured_password), "secret");
        assert_eq!(format!("{}", password), "****************");
    }
}
