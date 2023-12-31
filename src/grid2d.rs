use std::fmt::Display;

pub type Point = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2D<T> {
    data: Vec<Vec<T>>,
}

impl<T: PartialEq> Grid2D<T> {
    pub fn get(&self, (x, y): Point) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn get_opt(&self, point: Option<Point>) -> Option<&T> {
        if let Some(point) = point {
            self.get(point)
        } else {
            None
        }
    }

    pub fn get_offset(&self, (x, y): Point, (dx, dy): (isize, isize)) -> Option<&T> {
        if dx < 0 && x < dx.unsigned_abs() {
            return None;
        }
        if dy < 0 && y < dy.unsigned_abs() {
            return None;
        }
        self.get((x + dx as usize, y + dy as usize))
    }

    pub fn set(&mut self, (x, y): Point, value: T) {
        self.data[y][x] = value;
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        self.width() * self.height()
    }

    pub fn find_all(&self, value: T) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.data[y][x] == value {
                    result.push((x, y));
                }
            }
        }
        result
    }

    pub fn get_adjacent(&self, point: Point) -> Vec<&T> {
        [
            self.get_offset(point, Bearing::North.offset()),
            self.get_offset(point, Bearing::East.offset()),
            self.get_offset(point, Bearing::South.offset()),
            self.get_offset(point, Bearing::West.offset()),
        ]
        .iter()
        .filter_map(|x| *x)
        .collect()
    }
}

impl<T: Display> Display for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Grid2D<char> {
    fn from(value: &str) -> Self {
        let data = value
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { data }
    }
}

impl From<&str> for Grid2D<u8> {
    fn from(value: &str) -> Self {
        let data = value
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { data }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

impl Bearing {
    pub fn offset_point(&self, (x, y): Point) -> Option<Point> {
        let (dx, dy) = self.offset();
        if dx < 0 && x == 0 {
            return None;
        }
        if dy < 0 && y == 0 {
            return None;
        }
        Some(((x as isize + dx) as usize, (y as isize + dy) as usize))
    }
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Bearing::North => (0, -1),
            Bearing::East => (1, 0),
            Bearing::South => (0, 1),
            Bearing::West => (-1, 0),
        }
    }
}
