
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Copy,PartialEq)]
enum Cell {
    Wall,
    Path,
}

struct Maze(Vec<Vec<Cell>>);

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let mut maze = vec![vec![Cell::Wall; height]; width];
        let mut rng = rand::thread_rng();
        let start = (rng.gen_range(0..width), rng.gen_range(0..height));
        Self::carve(&mut maze, start);
        Maze(maze)
    }

    fn carve(maze: &mut Vec<Vec<Cell>>, (x, y): (usize, usize)) {
        let mut rng = rand::thread_rng();
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let shuffled_directions = directions.choose_multiple(&mut rng, 4);

        maze[x][y] = Cell::Path;

        for &(dx, dy) in shuffled_directions {
            let new_x = (x as i32 + 2 * dx) as usize;
            let new_y = (y as i32 + 2 * dy) as usize;

            if new_x < maze.len() && new_y < maze[0].len() && maze[new_x][new_y] == Cell::Wall {
                maze[(x as i32 + dx) as usize][(y as i32 + dy) as usize] = Cell::Path;
                Self::carve(maze, (new_x, new_y));
            }
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &cell in row {
                let ch = match cell {
                    Cell::Wall => "█",
                    Cell::Path => "  ",
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_m() {
    let maze = Maze::new(21, 42);
    println!("{}", maze);
}
