#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = std::fs::read_to_string("resources/test/input.txt").expect("Invalid file");
        let calories = input.split("\n\n").map(|groceries| -> i32 {
            groceries.split('\n').map(|calorie| calorie.parse::<i32>().unwrap()).sum::<i32>()    
        });
        assert_eq!(calories.max(), Some(66616));
    }
}
