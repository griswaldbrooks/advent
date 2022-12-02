#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = std::fs::read_to_string("resources/test/input.txt").expect("Invalid file");
        let food_calories: Vec<&str> = input.split("\n\n").collect();

        let mut max = 0;
        for calories in food_calories {
            let mut sum = 0;
            for s in calories.split('\n') {
                sum += s.parse::<i32>().unwrap();
            }
            if sum > max {
                max = sum;
            }
        }
        assert_eq!(max, 66616);
    }
}
