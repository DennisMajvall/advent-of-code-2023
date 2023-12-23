advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mapped = lines.map(|ln| {
        let mut splitted = ln.trim_matches(|c: char| !c.is_digit(10)).chars();
        let first = splitted.next().unwrap_or_else(|| '0');
        let last = splitted.last().unwrap_or_else(|| first);
        let mut result = first.to_string();
        result.push(last);
        result.parse::<u32>().unwrap_or(0)
    });

    Some(mapped.sum())
}

fn get_match(input: &str) -> Option<char> {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result = None;
    for (i, word) in words.iter().enumerate() {
        result = if input.starts_with(word) {
            char::from_digit(i as u32, 10)
        } else {
            None
        };

        if result.is_some() {
            break;
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines_as_numbers = input.lines().map(|ln| {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        let length = ln.len();
        let mut ln_char_iter = ln.chars();

        for i in 0..length {
            let current_char = ln_char_iter.next().unwrap();

            let mut digit: Option<char> = if char::is_digit(current_char, 10) {
                Some(current_char)
            } else {
                None
            };

            let max_index = usize::min(i + 5, length);
            digit = digit.or(get_match(ln.get(i..max_index).unwrap()));

            if digit.is_none() {
                continue;
            };

            if first.is_none() {
                first = digit;
            } else {
                last = digit;
            }
        }

        let first = first.unwrap_or('0');
        let last = last.unwrap_or(first);

        let mut result = first.to_string();
        result.push(last);

        result.parse::<u32>().unwrap_or(0)
    });

    Some(lines_as_numbers.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
