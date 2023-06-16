use std::sync::Arc;
use std::thread;
use std::time::Duration;
use object_pool::Pool;

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Activity {
    start: i32,
    finish: i32,
}

fn select_activities(mut activities: Vec<Activity>) -> Vec<Activity> {
    activities.sort_by(|a, b| a.finish.cmp(&b.finish));

    let mut result = vec![activities[0]];
    let mut current = activities[0];

    for activity in activities.iter().skip(1) {
        if activity.start >= current.finish {
            result.push(*activity);
            current = *activity;
        }
    }

    result
}

//安排会议的贪心算法
#[test]
fn test_confrence() {
    let activities = vec![
        Activity { start: 1, finish: 4 },
        Activity { start: 3, finish: 5 },
        Activity { start: 0, finish: 6 },
        Activity { start: 5, finish: 7 },
        Activity { start: 8, finish: 9 },
        Activity { start: 5, finish: 9 },
    ];
    let result = select_activities(activities);
    for activity in &result {
        println!("Activity: start={}, finish={}", activity.start, activity.finish);
    }
}


//找零问题
fn coin_change(coins: &Vec<i32>, mut amount: i32) -> i32 {
    let mut count = 0;
    let mut coins = coins.clone();
    coins.sort();

    while let Some(coin) = coins.pop() {
        while amount >= coin {
            amount -= coin;
            count += 1;
        }
        if amount == 0 {
            return count;
        }
    }

    if amount > 0 { -1 } else { count }
}

#[test]
fn test_ling() {
    let coins = vec![1, 5, 10, 25];
    let amount = 63;
    println!("{}", coin_change(&coins, amount));  // 输出：6
}


//简单电梯算法
struct Elevator {
    current_floor: i32,
    target_floors: Vec<i32>,
}

impl Elevator {
    fn new(start_floor: i32) -> Elevator {
        Elevator {
            current_floor: start_floor,
            target_floors: Vec::new(),
        }
    }

    fn request_floor(&mut self, floor: i32) {
        self.target_floors.push(floor);
        self.target_floors.sort_unstable();
    }

    fn move_to_next_floor(&mut self) {
        if let Some(&next_floor) = self.target_floors.first() {
            if self.current_floor < next_floor {
                self.current_floor += 1;
            } else if self.current_floor > next_floor {
                self.current_floor -= 1;
            } else { //到达的楼层就移除
                println!("Arrived at floor {}", next_floor);
                self.target_floors.remove(0);
            }
        }
    }

    fn run(&mut self) {
        while !self.target_floors.is_empty() {
            self.move_to_next_floor();
            println!("Current floor: {}", self.current_floor);
        }
    }
}

#[test]
fn test_floor() {
    let mut elevator = Elevator::new(0);

    elevator.request_floor(8);
    elevator.request_floor(5);
    elevator.request_floor(1);

    elevator.run();
}


//递归创造嵌套对象
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, next: Option<Box<Node>>) -> Node {
        Node { value, next }
    }

    // 递归函数来创建嵌套对象
    fn create_nested(count: i32, current_value: i32) -> Node {
        if count == 0 {
            Node::new(current_value, None)
        } else {
            Node::new(
                current_value,
                Some(Box::new(Node::create_nested(count - 1, current_value + 1)))
            )
        }
    }
}

#[test]
fn test_recursive() {
    let nested_node = Node::create_nested(5, 1);
    println!("{:?}", nested_node); // 输出：1
}

//生成多个struct
#[derive(Debug, Clone, Copy)]
struct Cpu {
    value: i32,
}

impl Cpu {
    fn new(value: i32) -> Cpu {
        Cpu { value }
    }
}

#[derive(Debug, Clone, Copy)]
struct Memory {
    value: i32,
}

impl Memory {
    fn new(value: i32) -> Memory {
        Memory { value }
    }
}

#[derive(Debug, Clone, Copy)]
struct Storage {
    value: i32,
}

impl Storage {
    fn new(value: i32) -> Storage {
        Storage { value }
    }
}

#[derive(Debug, Clone, Copy)]
struct Input {
    value: i32,
}

impl Input {
    fn new(value: i32) -> Input {
        Input { value }
    }
}

#[derive(Debug, Clone, Copy)]
struct Output {
    value: i32,
}

impl Output {
    fn new(value: i32) -> Output {
        Output { value }
    }
}

#[derive(Debug,Copy,Clone)]
struct Machine {
    cpu: Cpu,
    memory: Memory,
    storage: Storage,
    input: Input,
    output: Output,
}

impl Machine {
    fn new(cpu: Cpu, memory: Memory, storage: Storage, input: Input, output: Output) -> Machine {
        Machine { cpu, memory, storage, input, output }
    }
}

//沙箱机制
#[test]
fn test_struct() {
    let cpu = Cpu { value: 1 };
    let memory = Memory { value: 2 };
    let storage = Storage { value: 3 };
    let input = Input { value: 4 };
    let output = Output { value: 5 };
    let machine = Machine::new(cpu, memory, storage, input, output);
    println!("{:?}", machine); // 输出：Machine { cpu: Cpu { value: 1 }, memory: Memory { value: 2 }, storage: Storage { value: 3 }, input: Input { value: 4 }, output: Output { value: 5 } }

    let pool: Pool<Machine> = Pool::new(3, || machine);

    //把machine作为虚拟的资源供给多个线程使用，每个线程都有这个,从对象池里取对象
    let (pool, mut machine1)= pool.try_pull().unwrap().detach();
    let (pool, mut machine2)= pool.try_pull().unwrap().detach();

    //每个线程拥有自己的machine，沙箱机制
    thread::spawn(move || {
        println!("Thread 1: {:?}", machine1);
    });

    thread::spawn(move || {
        println!("Thread 2: {:?}", machine2);
    });

    thread::sleep(Duration::from_millis(1000));
}


//抽象的抽象,函数是一切的抽象，函数是人类思维对世界的一种认识，如何表达人类思维呢,就是一个二阶递归
trait TwoRecurve {
    fn humanbeing_understand(f: fn());
}

struct Person{}

impl TwoRecurve for Person {
    fn humanbeing_understand(f: fn()) {
        f();
    }
}

#[test]
fn test_trait() {
    // fn humanbeing_understand(f: fn()) {
    //     f();
    // }
    // fn print() {
    //     println!("Hello, world!");
    // }
    // humanbeing_understand(print);

    let person = Person{};
    fn print() {
        println!("Hello, world!");
    }
    Person::humanbeing_understand(print);
}
