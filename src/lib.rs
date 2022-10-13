pub fn who_are_you() -> String {
    "HAPI".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = who_are_you();
        assert_eq!(result, "HAPI".to_string());
    }
}
