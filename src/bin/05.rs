advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (ranges_raw, ingredients_raw) = (parts[0], parts[1]);

    let mut ranges = vec![];

    for range_raw in ranges_raw.lines() {
        let numbers = range_raw
            .split('-')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let range = (numbers[0], numbers[1]);
        ranges.push(range);
    }

    Some(
        ingredients_raw
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .filter(|&ingredient| {
                ranges
                    .iter()
                    .any(|range| ingredient >= range.0 && ingredient <= range.1)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let ranges_raw = parts[0];

    let mut ranges = vec![];

    for range_raw in ranges_raw.lines() {
        let numbers = range_raw
            .split('-')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let range = (numbers[0], numbers[1]);
        ranges.push(range);
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total = 0;
    let mut max = 0;
    for range in ranges {
        if range.0 > max {
            total += range.1 - range.0 + 1;
            max = range.1;
            continue;
        }

        if range.1 <= max {
            continue;
        }

        total += range.1 - max;
        max = range.1;
    }

    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
