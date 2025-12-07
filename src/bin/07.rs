use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

fn add_hit_splitters(
    map: &[Vec<char>],
    position: (usize, usize),
    splitters_hit: &mut HashSet<(usize, usize)>,
) {
    let (mut i, j) = position;
    i += 1;
    while i < map.len() && map[i][j] != '^' {
        i += 1
    }

    if i >= map.len() {
        return;
    }

    if map[i][j] == '^' {
        let new_insert = splitters_hit.insert((i, j));
        if new_insert {
            add_hit_splitters(map, (i, j - 1), splitters_hit);
            add_hit_splitters(map, (i, j + 1), splitters_hit);
        }
    }
}

fn count_timelines(
    map: &[Vec<char>],
    position: (usize, usize),
    counted_timelines: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&value) = counted_timelines.get(&position) {
        return value;
    }

    let (mut i, j) = position;
    i += 1;
    while i < map.len() && map[i][j] != '^' {
        i += 1
    }

    if i >= map.len() {
        return 1;
    }

    let total = count_timelines(map, (i, j - 1), counted_timelines)
        + count_timelines(map, (i, j + 1), counted_timelines);

    counted_timelines.insert(position, total);

    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut initial_position: (usize, usize) = (0, 0);
    let mut splitters: Vec<(usize, usize)> = vec![];

    for (i, line) in map.iter().enumerate() {
        for (j, &char) in line.iter().enumerate() {
            if char == 'S' {
                initial_position = (i, j);
                continue;
            }

            if char == '^' {
                splitters.push((i, j));
            }
        }
    }

    let mut hit_splitters: HashSet<(usize, usize)> = HashSet::new();
    add_hit_splitters(&map, initial_position, &mut hit_splitters);
    Some(hit_splitters.len())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut initial_position: (usize, usize) = (0, 0);
    let mut splitters: Vec<(usize, usize)> = vec![];

    for (i, line) in map.iter().enumerate() {
        for (j, &char) in line.iter().enumerate() {
            if char == 'S' {
                initial_position = (i, j);
                continue;
            }

            if char == '^' {
                splitters.push((i, j));
            }
        }
    }

    let mut counted_timelines: HashMap<(usize, usize), u64> = HashMap::new();
    Some(count_timelines(
        &map,
        initial_position,
        &mut counted_timelines,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
