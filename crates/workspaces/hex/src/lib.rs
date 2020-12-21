pub fn hex_string(input: &[u8]) -> String {
    input.as_ref().iter().map(|b| format!("{:x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let data = [149, 40, 84, 13, 85, 18, 116];

        let value = hex_string(&data);

        println!("{:?}", value);

        assert_eq!(value, "952854d551274");
    }
}