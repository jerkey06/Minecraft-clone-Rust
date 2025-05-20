fn main() {
    println!("Hello, future clone!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_correctly() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn another_test_example() {
        assert_ne!(add(1,1), 3, "1+1 should not be 3");
    }
}