fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[test]
fn test_server() {
    assert!(1 + 2 > 1);
}
