const EXAMPLE_INPUT: &str = include_str!("example_input");
const EXAMPLE_SOLUTION_1: u64 = 8;
const INPUT: &str = include_str!("input");

fn main() {
    assert_eq!(part1(EXAMPLE_INPUT, 5), EXAMPLE_SOLUTION_1);

    let p1_sol = part1(INPUT, 140);
    println!("part 1: {}", p1_sol);
}

fn part1(input: &str, dimensions: usize) -> u64 {
    let maze = Maze::new(input, dimensions);

    let (mut x, mut y, mut tile) = maze.starting_position();
    let mut counter = 0;
    let mut direction = tile.to_directions().expect("tile is a pipe").0;
    loop {
        (x, y) = direction
            .walk(x, y)
            .expect("pipes will not go across the border");
        tile = maze.tile_at(x, y);
        counter += 1;
        if let Some(next_dir) = tile.other_pipe_end(direction) {
            direction = next_dir;
        } else {
            break;
        }
    }

    counter / 2
}

struct Maze<'a> {
    maze: &'a [u8],
    dimensions: usize,
}

impl<'a> Maze<'a> {
    fn new(input: &'a str, dimensions: usize) -> Self {
        Self {
            maze: input.as_bytes(),
            dimensions,
        }
    }

    fn tile_at(&self, x: usize, y: usize) -> Tile {
        let slice_index = self.coords_to_slice_index(x, y);
        Tile::parse(self.maze[slice_index])
    }

    fn starting_position(&self) -> (usize, usize, Tile) {
        let slice_index = self
            .maze
            .into_iter()
            .position(|&tile| (tile != b'\n') && (Tile::parse(tile) == Tile::Start))
            .expect("maze contains a starting tile");
        let (x, y) = self.slice_index_to_coords(slice_index);

        let starting_directions: Vec<_> = Direction::DIRECTIONS
            .into_iter()
            .filter(|dir| {
                if let Some((x_next, y_next)) = dir.walk(x, y) {
                    self.tile_at(x_next, y_next).other_pipe_end(*dir).is_some()
                } else {
                    false
                }
            })
            .collect();

        debug_assert_eq!(starting_directions.len(), 2);

        let starting_tile =
            Tile::from_directions((starting_directions[0], starting_directions[1])).unwrap();

        (x, y, starting_tile)
    }

    fn slice_index_to_coords(&self, index: usize) -> (usize, usize) {
        debug_assert!(index < (self.dimensions + 1) * self.dimensions);

        let x = index % (self.dimensions + 1);
        let y = index / (self.dimensions + 1);

        debug_assert!(x != self.dimensions);

        (x, y)
    }

    fn coords_to_slice_index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.dimensions);
        debug_assert!(y < self.dimensions);

        y * (self.dimensions + 1) + x
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn parse(tile: u8) -> Self {
        match tile {
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'L' => Self::NorthEast,
            b'J' => Self::NorthWest,
            b'7' => Self::SouthWest,
            b'F' => Self::SouthEast,
            b'.' => Self::Ground,
            b'S' => Self::Start,
            _ => panic!("unknown tile: \"{}\"", char::from(tile)),
        }
    }

    fn from_directions(mut directions: (Direction, Direction)) -> Option<Self> {
        if directions.0 > directions.1 {
            std::mem::swap(&mut directions.0, &mut directions.1);
        }

        match directions {
            (Direction::North, Direction::South) => Some(Self::Vertical),
            (Direction::East, Direction::West) => Some(Self::Horizontal),
            (Direction::North, Direction::East) => Some(Self::NorthEast),
            (Direction::North, Direction::West) => Some(Self::NorthWest),
            (Direction::East, Direction::South) => Some(Self::SouthEast),
            (Direction::South, Direction::West) => Some(Self::SouthWest),
            _ => None,
        }
    }

    fn to_directions(self) -> Option<(Direction, Direction)> {
        match self {
            Self::Vertical => Some((Direction::North, Direction::South)),
            Self::Horizontal => Some((Direction::East, Direction::West)),
            Self::NorthEast => Some((Direction::North, Direction::East)),
            Self::NorthWest => Some((Direction::North, Direction::West)),
            Self::SouthWest => Some((Direction::South, Direction::West)),
            Self::SouthEast => Some((Direction::South, Direction::East)),
            _ => None,
        }
    }

    fn other_pipe_end(self, direction: Direction) -> Option<Direction> {
        let (dir1, dir2) = self.to_directions()?;

        let direction = direction.opposite();
        if direction == dir1 {
            Some(dir2)
        } else if direction == dir2 {
            Some(dir1)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    const DIRECTIONS: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    fn walk(self, x: usize, y: usize) -> Option<(usize, usize)> {
        Some(match self {
            Self::North => (x, y.checked_sub(1)?),
            Self::East => (x.checked_add(1)?, y),
            Self::South => (x, y.checked_add(1)?),
            Self::West => (x.checked_sub(1)?, y),
        })
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}
