#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let expected = std::fs::read_to_string("resources/test/input.txt").expect("Invalid file");
        println!("{}", expected);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
