use std::cmp::max;

advent_of_code::solution!(2);

fn get_max_cubes(color: &str) -> u32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game_results: u32 = input
        .lines()
        .map(|game| {
            let mut game_round_string = game.split(':');

            let game_id = game_round_string
                .next()
                .unwrap()
                .split("Game ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap_or(0);

            let rounds = game_round_string.next().unwrap().split(';');

            let round_results = rounds
                .map(|round| {
                    round.trim().split(", ").all(|number_and_color| {
                        let mut splitted = number_and_color.split(' ');
                        let number = splitted.next();
                        let number = number.expect("invalid number").parse::<u32>().unwrap_or(0);

                        let color = splitted.next().expect("no color found").trim();

                        number <= get_max_cubes(color)
                    })
                })
                .all(|round_result| round_result);

            return if round_results { game_id } else { 0 };
        })
        .sum();

    Some(game_results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let game_results: u32 = input
        .lines()
        .map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            let rounds = game.split(':').nth(1).unwrap().split(';');

            rounds.for_each(|round| {
                round.trim().split(", ").for_each(|number_and_color| {
                    let mut splitted = number_and_color.split(' ');
                    let number = splitted
                        .next()
                        .expect("invalid number")
                        .parse::<u32>()
                        .unwrap_or(0);

                    let color = splitted.next().expect("no color found").trim();

                    match color {
                        "red" => min_red = max(number, min_red),
                        "green" => min_green = max(number, min_green),
                        "blue" => min_blue = max(number, min_blue),
                        _ => (),
                    };
                });
            });

            min_red * min_green * min_blue
        })
        .sum();

    Some(game_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
