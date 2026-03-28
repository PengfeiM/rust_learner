pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn internal() {
        let result = add_two(4);
        assert_eq!(result, 6);
    }
}
