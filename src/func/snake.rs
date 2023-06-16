use std::{io::stdout, time::Duration};
use std::io::Write;
use std::process::Command;
use std::thread;

extern crate rand;

use rand::Rng;

const ROWS: usize = 20;
const COLS: usize = 30;
const BODY: char = '*';
const FOOD: char = '$';
const EMPTY: char = ' ';

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: Vec<Point>,
    food: Point,
    dir: Direction,
}

impl Game {
    fn new() -> Self {
        let mut snake = Vec::new();
        let head = Point { x: 0, y: 0 };
        snake.push(head);
        let food = Point { x: 5, y: 5 };
        let dir = Direction::Right;
        Game { snake, food, dir }
    }


    fn draw(&self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.snake.iter().any(|p| p.x == col && p.y == row) {
                    print!("{}", BODY);
                } else if self.food.x == col && self.food.y == row {
                    print!("{}", FOOD);
                } else {
                    print!("{}", EMPTY);
                }
            }
            println!("");
        }
    }

    fn input(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "w" => self.dir = Direction::Up,
            "s" => self.dir = Direction::Down,
            "a" => self.dir = Direction::Left,
            "d" => self.dir = Direction::Right,
            _ => (),
        }
    }

    fn update(&mut self) -> bool {
        let head = self.snake[0];
        let mut new_head = head;
        match self.dir {
            Direction::Up => {
                new_head.y -= 1;
            }
            Direction::Down => {
                new_head.y += 1;
            }
            Direction::Left => {
                new_head.x -= 1;
            }
            Direction::Right => {
                new_head.x += 1;
            }
        }

        if self.snake.iter().any(|p| p.x == new_head.x && p.y == new_head.y) {
            return false;
        }

        if new_head.x >= COLS || new_head.y >= ROWS {
            return false;
        }

        self.snake.insert(0, new_head);

        if self.food.x == new_head.x && self.food.y == new_head.y {
            let mut rng = rand::thread_rng();
            self.food.x = rng.gen_range(0..COLS);
            self.food.y = rng.gen_range(0..ROWS);
        } else {
            self.snake.pop();
        }

        true
    }
}

pub fn game_snake() {
    let mut game = Game::new();
    loop {
        game.draw();
        game.input();
        if !game.update() {
            break;
        }
        thread::sleep(Duration::from_millis(500));
        Command::new("clear").status().unwrap();
    }
    println!("Game over!");
}
