fn calculate_calibration_sum(lines: Vec<&str>) -> u32 {
    lines.iter()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|l| format!("{}{}", l.first().unwrap(), l.last().unwrap()).parse::<u8>().unwrap())
        .fold(0u32, |acc, line_value| acc + u32::from(line_value))
}

pub fn part_1(input: String) {
    let input = input.lines().collect::<Vec<&str>>();
    println!("Calibration Sum: {}", calculate_calibration_sum(input));
}

pub fn part_2(input: String) {
    let spelled_digits = Vec::from([
        ("two", "t2wo"),
        ("one", "on1e"),
        ("seven", "se7en"),
        ("nine", "ni9ne"),
        ("three", "th3ree"),
        ("four", "fo4ur"),
        ("five", "fi5ve"),
        ("six", "s6x"),
        ("eight", "eig8ht")
    ]);
    let input = input.lines().collect::<Vec<&str>>();
    let lines_with_spelled_digits_replaced = input.iter()
        .map(|l| spelled_digits.iter().fold(l.to_string(), |replaced_string, spelled_digit| replaced_string.replace(spelled_digit.0, spelled_digit.1)))
        .collect::<Vec<String>>();

    println!("Calibration Sum: {}", calculate_calibration_sum(lines_with_spelled_digits_replaced.iter().map(AsRef::as_ref).collect()));
}
