/// Amazing calculation that combines addition and multiplication
pub fn my_calc(a: i64, b: i64, c: i64) -> String {
    ((a + b) * c).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_calc() {
        assert_eq!(my_calc(1, 2, 3), "9");
    }
}
