use fancy_regex::Regex;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .filter(|range| !range.is_empty())
        .map(|range| {
            let mut numbers_map = range.split('-').map(|n| n.parse::<u64>().unwrap());
            let first_number = numbers_map.next().unwrap();
            let second_number = numbers_map.next().unwrap();
            println!("{first_number}, {second_number}");
            (first_number, second_number)
        })
        .collect::<Vec<_>>();

    let regex = Regex::new(r"^([0-9]+)\1$").unwrap();
    let mut id_sum = 0;

    for range in ranges {
        for number in range.0..(range.1 + 1) {
            if regex.is_match(&number.to_string()).unwrap() {
                id_sum += number;
            }
        }
    }

    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .filter(|range| !range.is_empty())
        .map(|range| {
            let mut numbers_map = range.split('-').map(|n| n.parse::<u64>().unwrap());
            let first_number = numbers_map.next().unwrap();
            let second_number = numbers_map.next().unwrap();
            println!("{first_number}, {second_number}");
            (first_number, second_number)
        })
        .collect::<Vec<_>>();

    let regex = Regex::new(r"^([0-9]+)\1+$").unwrap();
    let mut id_sum = 0;

    for range in ranges {
        for number in range.0..(range.1 + 1) {
            if regex.is_match(&number.to_string()).unwrap() {
                id_sum += number;
            }
        }
    }

    Some(id_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
