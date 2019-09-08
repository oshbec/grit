fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod dummy_tests {

    #[test]
    fn passes() {
        assert!(true);
    }

}
