pub fn part_1(input: String) {
    let max_red = 12u8;
    let max_green = 13u8;
    let max_blue = 14u8;

    let games = input.lines()
        .map(|line| {
            let mut line = line.strip_prefix("Game ").unwrap().split(": ");
            let game_id: u8 = line.next().unwrap().parse().unwrap();
            let game_is_valid = line.next().unwrap().split("; ").all(|iteration| {
                return iteration.split(", ").all(|count_colour_pair| {
                    let mut split_pair = count_colour_pair.split(" ");
                    let count: u8 = split_pair.next().unwrap().parse().unwrap();
                    match split_pair.next().unwrap() {
                        "red" => count <= max_red,
                        "green" => count <= max_green,
                        "blue" => count <= max_blue,
                        _ => panic!("Unexpected colour")
                    }
                });
            });

            (game_id, game_is_valid)
        });

    let sum_of_valid_games: u32 = games.filter(|g| g.1).map(|g| u32::from(g.0)).sum();

    println!("Sum of all valid game ids: {}", sum_of_valid_games);
}

pub fn part_2(input: String) {
    let games = input.lines()
        .map(|line| {
            line.split(": ").last().unwrap()
                .split(&[',', ';'])
                .fold((0u8, 0u8, 0u8), |highest_counts, l| {
                    let mut split_pair = l.trim().split(" ");
                    let count: u8 = split_pair.next().unwrap().parse().unwrap();
                    match split_pair.next().unwrap() {
                        "red" => (if count > highest_counts.0 { count } else { highest_counts.0 }, highest_counts.1, highest_counts.2),
                        "green" => (highest_counts.0, if count > highest_counts.1 { count } else { highest_counts.1 }, highest_counts.2),
                        "blue" => (highest_counts.0, highest_counts.1, if count > highest_counts.2 { count } else { highest_counts.2 }),
                        c => panic!("Unknown Colour: {}", c)
                    }
                })
        });

    let sum_of_products: u32 = games.map(|g | u32::from(g.0) * u32::from(g.1) * u32::from(g.2)).sum();

    println!("Sum of products of lowest possible games: {}", sum_of_products);
}
