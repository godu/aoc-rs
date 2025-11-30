use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

// Re-export common dependencies for convenience
pub use itertools::Itertools;
pub use rayon::prelude::*;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path))
}

/// Parse input lines into a vector of T
pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

/// Parse input as a grid of characters
pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Cardinal directions (N, E, S, W)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn delta(self) -> (i64, i64) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

/// 2D point with common operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn step(self, dir: Direction) -> Self {
        let (dx, dy) = dir.delta();
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn neighbors4(self) -> impl Iterator<Item = Point> {
        Direction::ALL.into_iter().map(move |d| self.step(d))
    }

    pub fn neighbors8(self) -> impl Iterator<Item = Point> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(move |(dx, dy)| Point::new(self.x + dx, self.y + dy))
    }

    pub fn manhattan_distance(self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// 2D Grid wrapper with bounds checking
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data.first().map_or(0, |row| row.len());
        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        if p.x >= 0 && p.y >= 0 {
            self.data
                .get(p.y as usize)
                .and_then(|row| row.get(p.x as usize))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if p.x >= 0 && p.y >= 0 {
            self.data
                .get_mut(p.y as usize)
                .and_then(|row| row.get_mut(p.x as usize))
        } else {
            None
        }
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && (p.x as usize) < self.width && (p.y as usize) < self.height
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height as i64)
            .cartesian_product(0..self.width as i64)
            .map(|(y, x)| Point::new(x, y))
    }

    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Option<Point> {
        self.iter_points()
            .find(|&p| predicate(self.get(p).unwrap()))
    }
}

impl Grid<char> {
    pub fn parse(input: &str) -> Self {
        Self::new(parse_grid(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_neighbors() {
        let p = Point::new(0, 0);
        assert_eq!(p.neighbors4().count(), 4);
        assert_eq!(p.neighbors8().count(), 8);
    }

    #[test]
    fn test_direction() {
        assert_eq!(Direction::North.turn_right(), Direction::East);
        assert_eq!(Direction::North.opposite(), Direction::South);
    }

    #[test]
    fn test_grid() {
        let grid = Grid::parse("abc\ndef");
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.get(Point::new(0, 0)), Some(&'a'));
        assert_eq!(grid.get(Point::new(2, 1)), Some(&'f'));
        assert_eq!(grid.get(Point::new(-1, 0)), None);
    }
}
