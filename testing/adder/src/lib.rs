pub fn add(left: usize, right: usize) -> usize {
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
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }
    
    #[test]
    fn it_fails_for_sure() {
        assert_eq!(2 + 2, 5)
    }
    
    #[test]
    fn it_fails() {
        panic!("Make this test fail")
    }
}
