use itertools::{self, Itertools};
advent_of_code::solution!(3);

fn find_first_number(line: &str, total_numbers: usize) -> (usize, char) {
    *line[..(line.len() - (total_numbers - 1))]
        .char_indices()
        .max_set_by(|x, y| x.1.cmp(&y.1))
        .iter()
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
}

fn find_second_number(line: &str, first_number_index: usize) -> (usize, char) {
    *line[(first_number_index + 1)..]
        .char_indices()
        .max_set_by(|x, y| x.1.cmp(&y.1))
        .iter()
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
}

fn find_biggest_number(line: &str, start_index: usize, remaining_numbers: usize) -> (usize, char) {
    line[(start_index + 1)..(line.len() - remaining_numbers)]
        .char_indices()
        .max_set_by(|x, y| x.1.cmp(&y.1))
        .iter()
        .min_by(|x, y| x.0.cmp(&y.0))
        .map(|x| (x.0 + start_index + 1, x.1))
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                print!("line is {line} : ");
                let (i, a) = find_first_number(line, 2);
                let (_j, b) = find_second_number(line, i);
                let mut final_number = a.to_string();
                final_number.push(b);
                println!("max is {final_number}");
                final_number.parse::<u64>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let total_numbers = 12;
                let mut final_string = "".to_owned();
                let (i_first, first_number) = find_first_number(line, total_numbers);
                let mut current_index = i_first;
                final_string.push(first_number);
                for n in (0..(total_numbers - 1)).rev() {
                    let (i_new, new_number) = find_biggest_number(line, current_index, n);
                    current_index = i_new;
                    final_string.push(new_number);
                }
                final_string.parse::<u64>().unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
