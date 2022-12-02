#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = std::fs::read_to_string("resources/test/input.txt").expect("Invalid file");
        let mut calories: Vec<i32> = input.split("\n\n").map(|groceries| {
            groceries
                .split('\n')
                .flat_map(|calorie| calorie.parse::<i32>())
                .sum()
        }).collect::<Vec<i32>>();
        calories.sort();
        let sum: i32 = calories.iter().rev().take(3).sum();
        assert_eq!(sum, 199172);
    }
}
