use crossbeam::channel;
use std::thread;
use std::time::Duration;

// 定义游戏角色
struct Monster {
    id: u32,
}

struct Hero {
    id: u32,
}

#[test]
fn test_cross() {
    let (monster_sender, monster_receiver) = channel::unbounded();

    // 创建一个生产者线程来生成怪物
    let producer = thread::spawn(move || {
        for id in 0..10 {
            let monster = Monster { id };
            monster_sender.send(monster).unwrap();
            thread::sleep(Duration::from_millis(100)); // 每100毫秒生成一个怪物
        }
    });

    // 创建一个消费者线程来让英雄消灭怪物
    let consumer = thread::spawn(move || {
        let hero = Hero { id: 1 };
        for monster in monster_receiver.iter() {
            println!("Hero {} defeated Monster {}", hero.id, monster.id);
            thread::sleep(Duration::from_millis(50)); // 每50毫秒消灭一个怪物
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}


//Si半导体制造PN结，PN结和逻辑门造基本电路，基本电路组合成CPU，内存，硬盘
#[derive(Debug)]
struct Si5Electron {
    id: u32,
    vec: Vec<u32>,
}

#[derive(Debug)]
struct Si3Empty {
    id: u32,
    vec: Vec<u32>,
}

#[derive(Debug)]
struct PN {
    id: u32,
    p: Si5Electron,
    n: Si3Empty,
}

#[derive(Debug)]
struct LogicGate {
    id: u32,
    pn: PN,
}

#[derive(Debug)]
struct BasicCircuit {
    id: u32,
    pn:PN,
    logic_gate: LogicGate,
}

#[derive(Debug)]
struct CPU {
    id: u32,
    basic_circuit: BasicCircuit,
}

struct Memory {
    id: u32,
    basic_circuit: BasicCircuit,
}

struct HardDisk {
    id: u32,
    basic_circuit: BasicCircuit,
}

#[test]
fn test_si() {
    //实体化CPU
    let cpu = CPU {
        id: 1,
        basic_circuit: BasicCircuit {
            id: 1,
            pn: PN {
                id: 1,
                p: Si5Electron {
                    id: 1,
                    vec: vec![1, 2, 3, 4, 5],
                },
                n: Si3Empty {
                    id: 1,
                    vec: vec![],
                },
            },
            logic_gate: LogicGate {
                id: 1,
                pn: PN {
                    id: 1,
                    p: Si5Electron {
                        id: 1,
                        vec: vec![1, 2, 3, 4, 5],
                    },
                    n: Si3Empty {
                        id: 1,
                        vec: vec![],
                    },
                },
            },
        },
    };

    println!("{:?}", cpu);
}

//人生的重大事件函数
fn f1(i:i32) -> i32 {
    i*2
}

fn f2(i:i32) -> i32 {
    i*3
}

fn f3(i:i32) -> i32 {
    i*4
}

#[test]
fn test_lifetime() {
    //人生的5个阶段
    let vec = vec![1,2,3,4,5];
    //使用vec的map不断的应用重大事件
    vec.iter().map(|&x|f1(x))
        .map(|x|f2(x))
        .map(|x|f3(x))
        .for_each(|x|println!("{}",x));
}
