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
            Move::Left(n) => 100 - *n,
            Move::Right(n) => *n,
        })
        .scan(50, |acc, x| {
            *acc = (*acc + x) % 100;
            Some(*acc)
        })
        .filter(|x| *x == 0)
        .count()
}

fn part2(_input: &str) -> usize {
    todo!()
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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(EXAMPLE), 0);
    // }
}
