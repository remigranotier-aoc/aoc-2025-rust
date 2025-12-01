advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut dial = 50;
    let mut zero_counter = 0;
    input.split('\n').for_each(|line| {
        if line.is_empty() {
            return;
        }

        let number = line[1..].to_owned().parse::<i64>().unwrap();
        if line.starts_with('R') {
            dial = (dial + number).rem_euclid(100)
        } else {
            dial = (dial - number).rem_euclid(100)
        }
        if dial == 0 {
            zero_counter += 1;
        }
    });
    Some(zero_counter)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut dial = 50;
    let mut zero_counter = 0;
    input.split('\n').for_each(|line| {
        if line.is_empty() {
            return;
        }

        let number = line[1..].to_owned().parse::<i64>().unwrap();
        if line.starts_with('R') {
            if dial + number > 99 {
                zero_counter += (dial + number) / 100;
            }
            dial = (dial + number).rem_euclid(100);
        } else {
            if dial - number <= 0 {
                zero_counter += ((dial - number) / 100).abs() + 1 - (if dial == 0 { 1 } else { 0 });
            }
            dial = (dial - number).rem_euclid(100);
        }
    });
    Some(zero_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
