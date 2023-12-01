pub fn part_1(input: String) {
    let input = input.lines().collect::<Vec<&str>>();
    let calibration_sum: u32 = input.iter()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|l| format!("{}{}", l.first().unwrap(), l.last().unwrap()).parse::<u8>().unwrap())
        .fold(0u32, |acc, line_value| acc + u32::from(line_value));

    println!("Calibration Sum: {}", calibration_sum)
}

pub fn part_2(input: String) {
    
}
