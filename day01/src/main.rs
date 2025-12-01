use std::iter::once;

use common::*;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};

enum Move {
    Left(isize),
    Right(isize),
}

fn move_parser(input: &str) -> IResult<&str, Move> {
    alt((
        map(preceded(char('L'), u32), |n| Move::Left(n as isize)),
        map(preceded(char('R'), u32), |n| Move::Right(n as isize)),
    ))
    .parse(input)
}

fn moves_parser(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, move_parser).parse(input)
}

fn part1(input: &str) -> usize {
    let (_, moves) = moves_parser(input).unwrap();
    moves
        .iter()
        .map(|m| match m {
            Move::Left(n) => -*n,
            Move::Right(n) => *n,
        })
        .scan(50, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .filter(|x| x.rem_euclid(100) == 0)
        .count()
}

fn part2(input: &str) -> usize {
    let (_, moves) = moves_parser(input).unwrap();

    once(50isize)
        .chain(
            moves
                .iter()
                .map(|m| match m {
                    Move::Left(n) => -*n,
                    Move::Right(n) => *n,
                })
                .scan(50, |acc, x| {
                    *acc += x;
                    Some(*acc)
                }),
        )
        .tuple_windows::<(isize, isize)>()
        .map(|(a, b)| {
            let (lo, hi) = if a < b { (a, b) } else { (b, a) };
            let in_between = (hi - 1).div_euclid(100) - lo.div_euclid(100);
            let lands_on = if b.rem_euclid(100) == 0 { 1 } else { 0 };
            (in_between + lands_on) as usize
        })
        .sum()
}

fn main() {
    let input = read_input(1);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
