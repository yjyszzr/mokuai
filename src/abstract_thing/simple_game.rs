
use smallvec::SmallVec;

struct Character {
    name: String,
    health: u32,
}

impl Character {
    fn new(name: &str, health: u32) -> Self {
        Self {
            name: name.to_string(),
            health,
        }
    }
}

#[test]
fn test_fn() {
    let mut v: SmallVec<[Character; 6]> = SmallVec::new();
    v.push(Character::new("A", 100));
    v.push(Character::new("B", 101));
    v.push(Character::new("C", 102));
    v.push(Character::new("D", 103));

    //让v的元素互相掉血之后剩下最右一个赢的元素
    while v.len() > 1 {
        let mut i = 0;
        while i < v.len() {
            v[i].health -= 1;
            if v[i].health == 0 {
                v.remove(i);
            } else {
                i += 1;
            }
        }
    }

    println!("winner is {}", v[0].name);
}

struct Hero {
    name: String,
    hp: i32,
    attack: i32,
}

impl Hero {
    fn new(name:String,hp: i32, attack: i32) -> Hero {
        Hero { name, hp, attack }
    }

    fn take_damage(&mut self, amount: i32) {
        println!("{} was took {} damage",self.name,amount);
        self.hp -= amount;
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

#[test]
fn test_attck() {
    let mut hero1 = Hero::new("hero1".into(),10, 2);
    let mut hero2 = Hero::new("hero2".into(),10, 3);

    loop {
        hero2.take_damage(hero1.attack);
        if hero2.is_dead() {
            println!("Hero 1 wins!");
            break;
        }

        hero1.take_damage(hero2.attack);
        if hero1.is_dead() {
            println!("Hero 2 wins!");
            break;
        }
    }
}

use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, WriteStorage};
use specs_derive::Component;
use specs::Join;

// 组件定义

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

// 系统定义

struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

// main 函数，游戏的启动点

#[test]
fn testBird() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    // 创建100只小鸟
    for _ in 0..100 {
        world
            .create_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .with(Velocity { x: 0.1, y: 0.2 })
            .build();
    }

    // 创建并设置系统
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(Movement, "movement", &[])
        .build();

    dispatcher.setup(&mut world);

    // 更新世界状态，将所有小鸟向前移动
    for _ in 0..100 {
        dispatcher.dispatch(&world);
        world.maintain();
    }
}
