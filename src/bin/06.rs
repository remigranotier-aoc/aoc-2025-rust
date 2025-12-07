use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let number_of_lines = lines.len();
    let number_of_operations = lines.first().unwrap().len();

    let mut total = 0;

    for i in 0..number_of_operations {
        if lines[number_of_lines - 1][i] == "*" {
            total += lines
                .iter()
                .map(|line| line[i].parse::<u64>().unwrap_or(1))
                .product::<u64>()
        } else {
            total += lines
                .iter()
                .map(|line| line[i].parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let number_of_lines = lines.len();

    let mut total = 0;
    let mut is_multiply = false;
    let mut sub_total = 0;

    for i in 0..lines.first().unwrap().len() {
        let column = lines.iter().map(|line| line[i]).collect::<Vec<char>>();

        // Check empty column
        if column.iter().all(|&char| char == ' ') {
            total += sub_total;
            continue;
        }

        // Change operation
        if column[number_of_lines - 1] == '*' {
            is_multiply = true;
            sub_total = 1;
        }

        if column[number_of_lines - 1] == '+' {
            is_multiply = false;
            sub_total = 0;
        }

        // Perform operation
        let number = column
            .iter()
            .filter(|char| char.is_numeric())
            .join("")
            .parse::<u64>()
            .unwrap();
        if is_multiply {
            sub_total *= number
        } else {
            sub_total += number
        }
    }

    total += sub_total;

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
