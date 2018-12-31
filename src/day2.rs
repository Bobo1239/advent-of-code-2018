use std::collections::{HashMap, HashSet};
use std::iter;

use aoc_runner_derive::aoc;

const ID_LEN: usize = 26;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let counts = input.lines().fold((0, 0), |mut acc, id| {
        // Use `u8` (8 bits) instead of `char` (32 bits) for better performance
        let mut chars: Vec<u8> = id.bytes().collect();
        chars.sort_unstable();

        let mut prev = 0;
        let mut count = 0;
        let mut found_2 = false;
        let mut found_3 = false;

        for c in chars.into_iter().chain(iter::once(0)) {
            if c == prev {
                count += 1;
            } else {
                match count {
                    2 => found_2 = true,
                    3 => found_3 = true,
                    _ => {}
                }
                count = 1;
                prev = c;
            }
        }

        if found_2 {
            acc.0 += 1;
        }
        if found_3 {
            acc.1 += 1;
        }
        acc
    });
    counts.0 * counts.1
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    let ids: HashSet<_> = input.lines().collect();
    let (id_1, id_2) = find_ids(ids, 0, ID_LEN).unwrap();
    id_1.chars()
        .zip(id_2.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

fn find_ids(candidates: HashSet<&str>, left: usize, right: usize) -> Option<(&str, &str)> {
    if candidates.len() < 2 {
        return None;
    }

    if right - left == 1 {
        if candidates.len() == 2 {
            let mut candidates = candidates.into_iter();
            let candidate_0 = candidates.next().unwrap();
            let candidate_1 = candidates.next().unwrap();
            if candidate_0
                .chars()
                .zip(candidate_1.chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                return Some((&candidate_0, &candidate_1));
            }
        }
        None
    } else {
        let mid = left + (right - left) / 2;

        let mut hashmap_left = HashMap::new();
        let mut hashmap_right = HashMap::new();
        let mut candidates_left = HashSet::new();
        let mut candidates_right = HashSet::new();

        for id in candidates {
            if let Some(prev_id) = hashmap_left.insert(&id[left..mid], id) {
                candidates_left.insert(prev_id);
                candidates_left.insert(id);
            }
            if let Some(prev_id) = hashmap_right.insert(&id[mid..right], id) {
                candidates_right.insert(prev_id);
                candidates_right.insert(id);
            }
        }

        find_ids(candidates_right, left, mid).or_else(|| find_ids(candidates_left, mid, right))
    }
}
