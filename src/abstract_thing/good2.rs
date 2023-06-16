use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

const SIZE: usize = 21;  // 定义迷宫的大小，注意大小必须为奇数

type Cell = bool;
struct Mi(Vec<Vec<Cell>>);

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_to(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let (x, y) = (x as isize, y as isize);
        match *self {
            Direction::North => {
                if x >= 2 {
                    Some(((x - 2) as usize, y as usize))
                } else {
                    None
                }
            }
            Direction::East => Some((x as usize, (y + 2) as usize)),
            Direction::South => Some(((x + 2) as usize, y as usize)),
            Direction::West => {
                if y >= 2 {
                    Some((x as usize, (y - 2) as usize))
                } else {
                    None
                }
            }
        }
    }
}


fn generate_maze() -> Mi {
    let mut maze = vec![vec![false; SIZE]; SIZE];
    for i in (0..SIZE).step_by(2) {
        for j in (0..SIZE).step_by(2) {
            maze[i][j] = true;
        }
    }
    dfs(&mut maze, 0, 0);
    Mi(maze)
}


fn dfs(maze: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let mut directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let mut rng = thread_rng();
    directions.shuffle(&mut rng);

    for &direction  in &directions {
        let Some((nx, ny)) = direction.move_to(x, y) else { panic!("wrong")  };
        if nx > 0 && nx < SIZE && ny > 0 && ny < SIZE && !maze[nx][ny] {
            maze[(x + nx) / 2][(y + ny) / 2] = true;
            maze[nx][ny] = true;
            dfs(maze, nx, ny);
        }
    }
}

impl fmt::Display for Mi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &cell in row {
                write!(f, "{}", if cell { "  " } else { "##" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_mi() {
    let maze = generate_maze();
    println!("{}", maze);
}
