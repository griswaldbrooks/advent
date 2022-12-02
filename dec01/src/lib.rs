#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = std::fs::read_to_string("resources/test/input.txt").expect("Invalid file");
        let calories = input.split("\n\n").map(|groceries| {
            groceries
                .split('\n')
                .flat_map(|calorie| calorie.parse::<i32>())
                .sum()
        });
        assert_eq!(calories.max(), Some(66616));
    }
}
