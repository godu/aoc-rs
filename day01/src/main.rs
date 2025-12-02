use alga::general::{
    AbstractGroup, AbstractLoop, AbstractMagma, AbstractMonoid, AbstractQuasigroup,
    AbstractSemigroup, Additive, Identity, TwoSidedInverse,
};
use common::read_input;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};

// ============================================================================
// Algebraic Traits
// ============================================================================

/// A group G acts on a set X by translation
trait GroupAction<X> {
    fn act(self, x: X) -> X;
}

// ============================================================================
// Dial: The cyclic group Z₁₀₀
// ============================================================================

/// Dial position in Z₁₀₀ (cyclic group of order 100)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Dial(isize);

impl Dial {
    const MODULUS: isize = 100;
    const ZERO: Dial = Dial(0);
    const START: Dial = Dial(50);

    fn new(value: isize) -> Self {
        Dial(value.rem_euclid(Self::MODULUS))
    }

    fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl AbstractMagma<Additive> for Dial {
    fn operate(&self, right: &Self) -> Self {
        Dial::new(self.0 + right.0)
    }
}

impl Identity<Additive> for Dial {
    fn identity() -> Self {
        Dial::ZERO
    }
}

impl TwoSidedInverse<Additive> for Dial {
    fn two_sided_inverse(&self) -> Self {
        Dial::new(-self.0)
    }
}

impl AbstractSemigroup<Additive> for Dial {}
impl AbstractMonoid<Additive> for Dial {}
impl AbstractQuasigroup<Additive> for Dial {}
impl AbstractLoop<Additive> for Dial {}
impl AbstractGroup<Additive> for Dial {}

// ============================================================================
// Rotation: Left or Right rotation by a given amount
// ============================================================================

/// A rotation (element of Z acting on Z₁₀₀ by translation)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rotation {
    Left(isize),
    Right(isize),
}

impl Rotation {
    /// Count how many times the dial points at 0 during this rotation
    fn zero_crossings(self, from: Dial) -> isize {
        let d = from.0;

        match self {
            Rotation::Right(0) | Rotation::Left(0) => 0,
            Rotation::Right(n) => (d + n).div_euclid(Dial::MODULUS),
            Rotation::Left(n) => {
                let start = if d == 0 { Dial::MODULUS } else { d };
                if n >= start {
                    1 + (n - start).div_euclid(Dial::MODULUS)
                } else {
                    0
                }
            }
        }
    }
}

/// Z acts on Z₁₀₀ by translation
impl GroupAction<Dial> for Rotation {
    fn act(self, dial: Dial) -> Dial {
        let d = dial.0;
        match self {
            Rotation::Right(n) => Dial::new(d + n),
            Rotation::Left(n) => Dial::new(d - n),
        }
    }
}

// ============================================================================
// Trajectory: The orbit under successive rotations
// ============================================================================

/// Iterator adapter that yields (from, rotation, to) triples
fn trajectory(
    rotations: impl Iterator<Item = Rotation>,
) -> impl Iterator<Item = (Dial, Rotation, Dial)> {
    rotations.scan(Dial::START, |state, rot| {
        let from = *state;
        let to = rot.act(from);
        *state = to;
        Some((from, rot, to))
    })
}

// ============================================================================
// Parsing
// ============================================================================

fn rotation_parser(input: &str) -> IResult<&str, Rotation> {
    alt((
        map(preceded(char('L'), u32), |n| Rotation::Left(n as isize)),
        map(preceded(char('R'), u32), |n| Rotation::Right(n as isize)),
    ))
    .parse(input)
}

fn parse(input: &str) -> Vec<Rotation> {
    separated_list1(newline, rotation_parser)
        .parse(input)
        .unwrap()
        .1
}

// ============================================================================
// Solutions
// ============================================================================

fn part1(input: &str) -> usize {
    let rotations = parse(input);
    trajectory(rotations.into_iter())
        .filter(|(_, _, to)| to.is_zero())
        .count()
}

fn part2(input: &str) -> isize {
    let rotations = parse(input);
    trajectory(rotations.into_iter())
        .map(|(from, rot, _)| rot.zero_crossings(from))
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
        assert_eq!(part2("R1000"), 10);
    }
}
