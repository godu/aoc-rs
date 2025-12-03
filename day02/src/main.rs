use std::ops::{Add, Mul};

use common::read_input;
use nom::{
    IResult, Parser,
    character::complete::{char, u64},
    multi::separated_list1,
};

fn to_id(half: usize, half_digits: usize) -> usize {
    half.mul(10usize.pow(half_digits as u32)).add(half)
}

fn count_digits(value: usize) -> usize {
    (value.ilog10() as usize) + 1
}

fn first_half_in_range(range: &Range) -> usize {
    let start_digits = count_digits(range.0);
    let half_digits = start_digits.div_euclid(2);

    if start_digits.rem_euclid(2) == 1 {
        10usize.pow(half_digits as u32)
    } else {
        let half = range.0.div_euclid(10usize.pow(half_digits as u32));
        let first_id = to_id(half, half_digits);
        if range.0 <= first_id { half } else { half + 1 }
    }
}

fn last_half_in_range(range: &Range) -> usize {
    let end_digits = count_digits(range.1);
    let half_digits = end_digits.div_euclid(2);

    if end_digits.rem_euclid(2) == 1 {
        10usize.pow(half_digits as u32) - 1
    } else {
        let half = range.1.div_euclid(10usize.pow(half_digits as u32));
        let last_id = to_id(half, half_digits);
        if range.1 >= last_id { half } else { half - 1 }
    }
}

fn invalid_ids(range: &Range) -> Vec<usize> {
    let first_half = first_half_in_range(range);
    let last_half = last_half_in_range(range);

    let ids = (first_half..=last_half).map(|half| -> usize { to_id(half, count_digits(half)) });

    ids.collect()
}

// ============================================================================
// Parsing
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range(usize, usize);

fn range_parser(input: &str) -> IResult<&str, Range> {
    let (input, start) = u64(input)?;
    let (input, _) = char('-')(input)?;
    let (input, end) = u64(input)?;
    Ok((input, Range(start as usize, end as usize)))
}

fn parse(input: &str) -> Vec<Range> {
    separated_list1(char(','), range_parser)
        .parse(input)
        .unwrap()
        .1
}

// ============================================================================
// Solutions
// ============================================================================

fn part1(input: &str) -> usize {
    let data = parse(input);
    data.iter().flat_map(invalid_ids).sum()
}

fn part2(input: &str) -> usize {
    let _data = parse(input);
    todo!()
}

fn main() {
    let input = read_input(2);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1("11-22"), 11 + 22);
        assert_eq!(part1("95-115"), 99);
        assert_eq!(part1("998-1012"), 1010);
        assert_eq!(part1("1188511880-1188511890"), 1188511885);
        assert_eq!(part1("222220-222224"), 222222);
        assert_eq!(part1("1698522-1698528"), 0);
        assert_eq!(part1("446443-446449"), 446446);
        assert_eq!(part1("38593856-38593862"), 38593859);
        assert_eq!(part1("565653-565659"), 0);
        assert_eq!(part1("824824821-824824827"), 0);
        assert_eq!(part1("2121212118-2121212124"), 0);
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
