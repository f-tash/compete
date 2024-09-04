pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn shared_function() {
    println!("This is a shared function2");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
