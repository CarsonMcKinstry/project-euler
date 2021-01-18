use std::convert::From;
use std::vec::Vec;

#[derive(PartialEq, Copy, Clone)]
pub(crate) enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    pub(crate) fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::South,
            Direction::SouthEast,
            Direction::East,
            Direction::NorthEast,
        ]
        .iter()
        .copied()
    }
}

pub(crate) struct Grid {
    pub(crate) grid: Vec<i64>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl From<&str> for Grid {
    fn from(grid: &str) -> Self {
        let height = grid.lines().count();

        let grid = grid
            .lines()
            .map(|line| {
                line.split(" ")
                    .filter_map(|line_size| match line_size.parse::<i64>() {
                        Ok(line_size) => Some(line_size),
                        _ => None,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<i64>>>();
        let width = grid[0].len();

        let grid: Vec<i64> = grid.into_iter().flatten().collect();
        assert_eq!(grid.len(), width * height);

        Self {
            grid,
            width,
            height,
        }
    }
}

impl Grid {
    pub(crate) fn size(&self) -> usize {
        self.grid.len()
    }

    pub(crate) fn at(&self, pos: (usize, usize)) -> Option<i64> {
        self.get_index(pos).map(|i| self.grid[i])
    }

    pub(crate) fn get_index(&self, pos: (usize, usize)) -> Option<usize> {
        let (row, col) = pos;

        if row >= self.height {
            None
        } else if col >= self.width {
            None
        } else {
            Some(row * self.width + col)
        }
    }

    pub(crate) fn get_position(&self, i: usize) -> (usize, usize) {
        let col = i % self.width;
        let row = i / self.width;

        (row, col)
    }

    pub(crate) fn neighbors(&self, pos: (usize, usize)) -> Vec<Option<(usize, usize)>> {
        let (row, col) = pos;

        let neighbors_raw = vec![
            (row - 1, col),     // line_size
            (row - 1, col + 1), // ne
            (row, col + 1),     // e
            (row + 1, col + 1), // se
            (row + 1, col),     // s
            (row + 1, col - 1), // sw
            (row, col - 1),     // w
            (row - 1, col - 1), //nw
        ];

        let mut neighbors: Vec<Option<(usize, usize)>> = vec![];

        for neighbor in neighbors_raw {
            neighbors.push(match self.get_index(neighbor) {
                Some(_) => Some(neighbor),
                _ => None,
            })
        }

        neighbors
    }

    pub(crate) fn get_values_in_direction(
        &self,
        pos: (usize, usize),
        dir: Direction,
        line_size: usize,
    ) -> Option<Vec<i64>> {
        if line_size > self.width || line_size > self.height {
            panic!(
                "Line size too big. Must be <= {}",
                self.width.max(self.height)
            )
        }

        let (row, col) = pos;

        match dir {
            Direction::North if row < line_size => return None,
            Direction::NorthWest if row < line_size || col > self.width - line_size => return None,
            Direction::West if col > self.width - line_size => return None,
            Direction::SouthWest
                if row >= self.height - line_size || col > self.width - line_size =>
            {
                return None
            }
            Direction::South if row >= self.height - line_size => return None,
            Direction::SouthEast if col < line_size || row >= self.height - line_size => {
                return None
            }
            Direction::East if col < line_size => return None,
            Direction::NorthEast if row < line_size || col < line_size => return None,
            _ => (),
        };

        Some(
            (0..line_size)
                .filter_map(|i| {
                    let new_pos = match dir {
                        Direction::North => (row - i, col),
                        Direction::NorthWest => (row - i, col + i),
                        Direction::West => (row, col + i),
                        Direction::SouthWest => (row + i, col + i),
                        Direction::South => (row + i, col),
                        Direction::SouthEast => (row + i, col - i),
                        Direction::East => (row, col - i),
                        Direction::NorthEast => (row - i, col - i),
                    };
                    self.at(new_pos)
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> Grid {
        Grid::from(
            "1 2 3
        4 5 6
        7 8 9",
        )
    }

    #[test]
    fn grid_works() {
        let grid = test_grid();

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    fn grid_get_position() {
        let grid = test_grid();

        assert_eq!(grid.get_position(0), (0, 0));
        assert_eq!(grid.get_position(4), (1, 1));
    }

    #[test]
    fn grid_get_index() {
        let grid = test_grid();

        assert_eq!(grid.get_index((0, 0)), Some(0));
        assert_eq!(grid.get_index((1, 1)), Some(4));
        assert_eq!(grid.get_index((5, 5)), None);
    }

    #[test]
    fn grid_get_values_in_line() {
        let grid = test_grid();
        let expected: Vec<i64> = vec![1, 2, 3];
        assert_eq!(
            grid.get_values_in_direction((0, 0), Direction::West, 3),
            Some(expected)
        );
    }
}
