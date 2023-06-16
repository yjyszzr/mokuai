use std::fmt;
use std::fmt::{Formatter, write};


#[derive(Copy, Clone,Debug,PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone,Default,Debug,PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

pub fn test_copy() {
    let p1 = Point{x:1.0,y:2.0};
    let p2 = p1;
    println!("{:?}",p2);
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"({},{})",self.x,self.y)
    }
}

pub fn test_display() {
    let p1 = Point{x:1.0,y:2.0};
    let p2 = p1;
    println!("{}",p2);
}

pub fn test_clone() {
    let p0 = Person::default();
    let p1 = Person{name:"steve".to_string(),age:15};
    let p2 = p1;
    println!("{:?}",p2);
    println!("{:?}",p0);
}

struct MyStruct {
    a: i32,
    b: String,
}

impl Default for MyStruct {
    fn default() -> Self {
        Self {
            a: 42,
            b: String::from("hello"),
        }
    }
}

pub fn test_default() {
    let my_struct = MyStruct::default();
    println!("a = {}, b = {}", my_struct.a, my_struct.b);
}


#[derive(Debug)]
struct MyNumber {
    value: i32,
}

impl PartialEq for MyNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value.abs() == other.value.abs()
    }
}

pub fn test_peq() {
    let num1 = MyNumber { value: 10 };
    let num2 = MyNumber { value: -10 };

    if num1 == num2 {
        println!("The absolute values of num1 and num2 are equal.");
    } else {
        println!("The absolute values of num1 and num2 are different.");
    }
}


impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

pub fn test_pord() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(2.0, 1.0);
    let p3 = Point::new(1.0, 2.0);

    assert!(p1 < p2);
    assert!(p1 <= p3);
    assert!(p1 == p3);
    assert!(p2 > p1);
}

use std::cmp::Ordering;




impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.age.cmp(&other.age))
    }
}

pub fn test_pord_self() {
    let p1 = Person { name: "Alice".to_string(), age: 20 };
    let p2 = Person { name: "Bob".to_string(), age: 30 };
    let p3 = Person { name: "Charlie".to_string(), age: 25 };

    assert!(p1 < p2);
    assert!(p2 > p3);
    //assert!(p1.partial_cmp(&p3).is_none());
}


#[derive(Eq, PartialEq)]
struct Student {
    name: String,
    age: u8,
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn test_stuOrd() {
    let mut students = vec![
        Student { name: "Alice".into(), age: 18 },
        Student { name: "Bob".into(), age: 20 },
        Student { name: "Charlie".into(), age: 15 },
    ];

    students.sort();

    for student in students {
        println!("{} ({})", student.name, student.age);
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub fn test_hash() {
    let person1 = Person {
        age: 1,
        name: "Alice".to_string(),
    };
    let person2 = Person {
        age: 2,
        name: "Bob".to_string(),
    };

    let mut hasher = DefaultHasher::new();
    person1.hash(&mut hasher);
    let hash1 = hasher.finish();
    hasher = DefaultHasher::new();
    person2.hash(&mut hasher);
    let hash2 = hasher.finish();

    println!("Hash of person1: {}", hash1);
    println!("Hash of person2: {}", hash2);
}


struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = std::mem::replace(&mut self.next, new_next);
        Some(self.curr)
    }
}

pub fn test_fib() {
    let fib = Fibonacci { curr: 1, next: 1 };
    for i in fib.take(10) {
        println!("{}", i);
    }
}

pub fn test_iter() {
    let v = vec![1, 2, 3];
    for i in v.into_iter() {
        println!("{}", i);
    }
}



