use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Debug)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Path {
    moves: Vec<Move>,
}

impl Path {
    fn from(input: &str) -> Path {
        let term = ['L', 'R'];
        let moves: Vec<Move> = input.split_inclusive(term).flat_map(|s| {
            let n = s.trim_end_matches(term).parse::<usize>().unwrap();
            let mut a = vec![Move::Forward(n)];
            if s.ends_with(term) {
                a.push(match s.chars().last().unwrap() {
                    'L' => Move::Left,
                    'R' => Move::Right,
                    _ => panic!()
                });
            }
            a.into_iter()
        }).collect();
        Path { moves }
    }
}

#[derive(Clone, Debug)]
struct Row {
    walls: Vec<u8>,
    start: usize,
}

impl Row {
    fn end(&self) -> usize {
        self.start + self.walls.len() - 1
    }

    fn is_valid(&self, x: usize) -> bool {
        x >= self.start && x <= self.end()
    }

    fn has_wall(&self, x: usize) -> bool {
        self.walls[x - self.start] != 0
    }
}

#[derive(Clone, Debug)]
struct Maze {
    rows: Vec<Row>,
}

impl Maze {
    fn from(input: &str) -> Maze {
        let rows: Vec<Row> = input.lines().map(|s| {
            let walls: Vec<u8> = s.trim().chars().map(|c| match c {
                '.' => 0, '#' => 1, _ => panic!()
            }).collect();
            let start = s.len() - s.trim_start().len() + 1;
            Row { walls, start }
        }).collect();
        Maze { rows }
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        y >= 1 && y <= self.rows.len() && self.rows[y - 1].is_valid(x)
    }

    fn iter(&self, path: &Path) -> MazeIter {
        MazeIter {
            maze: self,
            path: path.moves.iter().rev().cloned().collect(),
            pos: (self.rows[0].start, 1),
            dir: Direction::Right,
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            if row.start > 1 {
                write!(f, "{}", " ".repeat(row.start - 1))?;
            }
            writeln!(f, "{}", String::from_iter(row.walls.iter().map(|v|
                match v { 0 => '.', 1 => '#', _ => panic!() }
            )))?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
struct MazeIter<'a> {
    maze: &'a Maze,
    path: Vec<Move>,
    pos: (usize, usize),
    dir: Direction,
}

impl<'a> Iterator for MazeIter<'a> {
    type Item = (usize, usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop().map(|move_| {
            match move_ {
                Move::Forward(n) => {
                    let (x, y) = self.pos;
                    self.pos = match self.dir {
                        Direction::Up | Direction::Down => {
                            let mut ny = if matches!(self.dir, Direction::Up)
                                {y - 1} else {y + 1};
                            if !self.maze.is_valid(x, ny) {
                                let mut it = (1..=self.maze.rows.len()).filter_map(|i| {
                                    if self.maze.is_valid(x, i) {Some(i)} else {None}
                                });
                                ny = (if ny < y { it.last() } else { it.next() }).unwrap();
                            }
                            let row = &self.maze.rows[ny - 1];
                            (x, if !row.has_wall(x) {ny} else {y})
                        },
                        Direction::Left | Direction::Right => {
                            let row = &self.maze.rows[y - 1];
                            let nx = if matches!(self.dir, Direction::Left) {
                                if x > row.start {x - 1} else {row.end()}
                            } else {
                                if x < row.end() {x + 1} else {row.start}
                            };
                            (if !row.has_wall(nx) {nx} else {x}, y)
                        },
                    };
                    if n > 1 && (self.pos.0 != x || self.pos.1 != y) {
                        self.path.push(Move::Forward(n - 1));
                    }
                },
                Move::Left => self.dir = self.dir.rotate().rotate().rotate(),
                Move::Right => self.dir = self.dir.rotate(),
            };
            (self.pos.0, self.pos.1, self.dir)
        })
    }
}

fn score<T>(iter: T) -> u64
where T: Iterator<Item = (usize, usize, Direction)> {
    let (x, y, d) = iter.last().unwrap();
    y as u64 * 1000 + x as u64 * 4 + match d {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    }
}

pub fn run(content: &str) {
    let parts: Vec<&str> = content.trim_end().split("\n\n").collect();
    let maze = Maze::from(parts[0]);
    let path = Path::from(parts[1]);
    println!("{}", score(maze.iter(&path)));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { r#"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.
"#.trim_matches('\n')
    }

    #[test]
    pub fn maze() {
        let maze = super::Maze::from(example());
        let path = super::Path::from("10R5L5R10L4R5L5");
        assert_eq!(super::score(maze.iter(&path)), 6032);
    }
}
