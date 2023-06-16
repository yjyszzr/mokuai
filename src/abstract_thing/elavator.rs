
#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Idle,
}

#[derive(Debug)]
struct Elevator {
    current_floor: i32,
    direction: Direction,
}

impl Elevator {
    fn new(c:i32,e:Direction) -> Self {
        Elevator {
            current_floor: c,
            direction: e,
        }
    }

    fn move_to(&mut self, floor: i32) {
        if floor > self.current_floor {
            self.direction = Direction::Up;
        } else if floor < self.current_floor {
            self.direction = Direction::Down;
        } else {
            self.direction = Direction::Idle;
        }
        self.current_floor = floor;
    }
}

#[test]
fn test_elavator() {
    let array: Vec<i32> = (1..=27).collect();
    println!("{:?}", array);

    let elevator_init = Elevator::new(10,Direction::Idle);
    //每次输入3个电梯状态，电梯按照第一个电梯状态移动，然后输出电梯状态
    let  elevators = vec![Elevator::new(11,Direction::Up), Elevator::new(13,Direction::Up), Elevator::new(9,Direction::Down)];

    //按照elevators的顺序，在楼层array 中移动电梯
    let max_layer = elevators.iter().map(|e| e.current_floor).max().unwrap();
    let move_direction = &elevators[0].direction;

}