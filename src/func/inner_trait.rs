use std::fmt;

struct Person {
    name: String,
    age: i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

pub fn test_to_str() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{}", person);
}

use std::ops::{Deref, DerefMut};

struct MyPointer<T> {
    value: T,
}

impl<T> MyPointer<T> {
    fn new(value: T) -> Self {
        MyPointer { value }
    }
}

impl<T> Deref for MyPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub fn test_def() {
    let ptr = MyPointer::new(5);
    assert_eq!(*ptr, 5);
}


#[derive(Debug)]
struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { data: Vec::new() }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub fn test_def_mut() {
    let mut v = MyVec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    *v.first_mut().unwrap() = 4;
    println!("{:?}", v);
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn write_to_file<P: AsRef<Path>>(file_path: P, text: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn test_deref() -> std::io::Result<()> {
    let file_path = "src/file.txt";
    let text = "Hello, zzr!";
    write_to_file(file_path, text)?;
    Ok(())
}


struct Value {
    s: String,
}

impl AsMut<[u8]> for Value {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            self.s.as_bytes_mut()
        }
    }
}

pub fn test_as_mut() {
    let mut value = Value {
        s: String::from("hello"),
    };
    value.as_mut()[1] = b'o';
    println!("{}", value.s);
}


use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

#[derive(Debug, Clone,PartialEq,Eq)]
struct MyType {
    name: String,
    id: u32,
}

impl Hash for MyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'a> Borrow<u32> for &'a MyType {
    fn borrow(&self) -> &u32 {
        &self.id
    }
}

pub fn test_borrow() {
    let mut map = HashMap::new();
    let obj = MyType { name: "example".to_string(), id: 42 };
    map.insert(&obj, "some value");

    let key = MyType { name: "example".to_string(), id: 42 };
    let value = map.get(&key);
    println!("{:?}", value); // Some("some value")
}



fn count_words(s: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in s.split_whitespace() {
        let count = counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    counts
}

pub fn test_borrow2() {
    let s = "hello world hello Rust Rust";
    let counts = count_words(s);
    for (word, count) in counts.iter() {
        println!("{}: {}", word, count);
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let fields: Vec<_> = s.split(',').collect();
        Person {
            name: fields[0].to_string(),
            age: fields[1].parse().unwrap(),
        }
    }
}

pub fn test_str_to_obj() {
    let p: Person = "John,30".into();
    println!("Name: {}, Age: {}", p.name, p.age);
}

struct MyInt(i32);

impl Into<i32> for MyInt {
    fn into(self) -> i32 {
        self.0
    }
}

pub fn test_into() {
    let my_int = MyInt(5);
    let i: i32 = my_int.into();
    println!("{}", i);
}


use std::ops::Index;

struct Scores {
    data: Vec<u32>,
}

impl Index<usize> for Scores {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub fn test_index() {
    let scores = Scores { data: vec![85, 90, 95, 80, 75] };
    println!("The third score is: {}", scores[2]);
}


fn print_size<T: Sized>(_: &T) {
    println!("size of T: {}", std::mem::size_of::<T>());
}

pub fn test_size() {
    print_size(&42);
    print_size(&"hello, world");
}

