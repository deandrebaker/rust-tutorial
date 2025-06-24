pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn internal_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        let result = internal_add(2, 2);
        assert_eq!(result, 4);
    }
}
