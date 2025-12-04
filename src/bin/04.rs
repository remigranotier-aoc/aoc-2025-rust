advent_of_code::solution!(4);

pub fn get_neighbors(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 {
        if y > 0 {
            neighbors.push((x - 1, y - 1));
        }
        neighbors.push((x - 1, y));
        if y < width - 1 {
            neighbors.push((x - 1, y + 1));
        }
    }

    if y > 0 {
        neighbors.push((x, y - 1))
    }
    if y < width - 1 {
        neighbors.push((x, y + 1));
    }

    if x < height - 1 {
        if y > 0 {
            neighbors.push((x + 1, y - 1));
        }
        neighbors.push((x + 1, y));
        if y < width - 1 {
            neighbors.push((x + 1, y + 1))
        }
    }
    neighbors
}

pub fn part_one(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let warehouse = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter = 0;

    for i in 0..height {
        for j in 0..width {
            if warehouse[i][j] == '@'
                && get_neighbors(i, j, height, width)
                    .iter()
                    .filter(|neighbor| warehouse[neighbor.0][neighbor.1] == '@')
                    .count()
                    < 4
            {
                counter += 1;
            }
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut warehouse = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter = 0;

    loop {
        let mut new_counter = 0;
        let mut to_remove = vec![];
        for i in 0..height {
            for j in 0..width {
                if warehouse[i][j] == '@'
                    && get_neighbors(i, j, height, width)
                        .iter()
                        .filter(|neighbor| warehouse[neighbor.0][neighbor.1] == '@')
                        .count()
                        < 4
                {
                    to_remove.push((i, j));
                    new_counter += 1;
                }
            }
        }
        for (i, j) in to_remove {
            warehouse[i][j] = '.';
        }
        if new_counter == 0 {
            break;
        }
        counter += new_counter;
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
