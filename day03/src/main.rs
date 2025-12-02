use common::read_input;

// ============================================================================
// Parsing
// ============================================================================

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

// ============================================================================
// Solutions
// ============================================================================

fn part1(input: &str) -> usize {
    let _data = parse(input);
    todo!()
}

fn part2(input: &str) -> usize {
    let _data = parse(input);
    todo!()
}

fn main() {
    let input = read_input(3);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
