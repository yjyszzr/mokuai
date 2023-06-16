
use log::{debug, error, info, warn};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

pub fn test_query() -> Result<()> {
    // 打开数据库连接
    let conn = Connection::open("test.db")?;

    // 创建表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL
                  )",
        [],
    )?;

    // 插入数据
    let person = Person {
        id: 1,
        name: "Alice".to_string(),
        age: 20,
    };
    conn.execute(
        "INSERT INTO person (id, name, age) VALUES (?1, ?2, ?3)",
        params![person.id, person.name, person.age],
    )?;

    // 查询数据
    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}


pub fn test_tr() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

    tx.commit()
}


fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

pub fn test_log() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        error!("Failed to execute query: {}", err);
    }
}


use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};


static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn test_info() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    info!("hello log");
    warn!("warning");
    error!("oops");
    Ok(())
}


use syslog::{Facility};

pub fn test_syslog() -> Result<(), syslog::Error> {
    syslog::init(Facility::LOG_USER,
                 log::LevelFilter::Debug,
                 Some("My app name"))?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}



use ndarray::Array;

pub fn test_nd() {
    let a = ndarray::Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let b = ndarray::Array::from_vec(vec![5., 4., 3., 2., 1.]);
    let mut c = ndarray::Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let mut d = ndarray::Array::from_vec(vec![5., 4., 3., 2., 1.]);

    let z = &a + &b;
    let w =  &c + &d;

    let epsilon = 1e-8;
    for elem in z.iter() {
        let diff: f32 = *elem - 6.;
        assert!(diff.abs() < epsilon);
    }

    println!("a = {}", a);
    // c[0] = 10.;
    // d[1] = 10.;
    //
    // for elem in w.iter() {
    //     let diff: f32 = *elem - 6.;
    //     assert!(diff.abs() < epsilon);
    // }

}

use ndarray::{Array1, ArrayView1};

fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

pub fn test_norm() {
    let x = ndarray::Array::from_vec(vec![1., 2., 3., 4., 5.]);
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}


use ndarray::arr2;
pub fn test_add() {
    let a = arr2(&[[1, 2, 3],
        [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4],
        [3, 2, 1]]);

    println!("Sum: {}", a + b);
}


pub fn test_mul() {
    let a = arr2(&[[1, 2, 3],
        [4, 5, 6]]);

    let b = arr2(&[[6, 3],
        [5, 2],
        [4, 1]]);

    println!("{}", a.dot(&b));
}


use ndarray::{arr1};

pub fn test_scale_vec() {
    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6],
        [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}


pub fn test_sin() {
    let angle: f64 = 2.0;
    let side_length = 80.0;
    let hypotenuse = side_length/angle.sin();
    println!("Hypotenuse: {}", hypotenuse);

    let x: f64 = 6.0;
    let a = x.tan();
    let b = x.sin()/x.cos();
    assert_eq!(a, b);
}


pub fn test_complex() {
    let complex_num1 = num::complex::Complex::new(10.0, 20.0); // Must use floats
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);
    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}

pub fn test_data_ave() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum /count as f32),
        _ => None
    };

    println!("Mean of the data is {:?}", mean);
}

use std::collections::{HashMap, HashSet};
use std::io::Error;

pub fn test_data() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is {:?}", mode);
}

use num::bigint::{BigInt, ToBigInt};

fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..(x+1) {
            factorial = factorial * i;
        }
        factorial
    }
    else {
        panic!("Failed to calculate factorial!");
    }
}

pub fn test_fac() {
    println!("{}! equals {}", 100, factorial(100));
}



pub fn test_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    let mapped = vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
    assert_eq!(mapped, vec![2, 4, 6, 8, 10]);

    let vec = vec![1, 2, 3, 4, 5];
    let filtered = vec.iter().filter(|&x| x % 2 == 0).collect::<Vec<&i32>>();
    assert_eq!(filtered, vec![&2, &4]);

    let vec = vec![1, 2, 3, 4, 5];
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 15);

    let vec = vec![1, 2, 3, 4, 5];
    let max = vec.iter().max().unwrap();
    assert_eq!(*max, 5);

    let vec = vec![1, 2, 3, 4, 5];
    let min = vec.iter().min().unwrap();
    assert_eq!(*min, 1);

    let v = vec!["apple", "banana", "cherry"];
    for (i, s) in v.iter().enumerate() {
        println!("{}: {}", i, s);
    }

    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    for (n, l) in numbers.iter().zip(letters.iter()) {
        println!("{} is {}", n, l);
    }
}


pub fn test_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let new_scores = vec![
        (String::from("Blue"), 20),
        (String::from("Yellow"), 60),
        (String::from("Red"), 30),
    ];

    scores.extend(new_scores);

    println!("{:?}", scores);
}

pub fn test_m1() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let score = scores.entry("Blue").or_insert(0);
    *score += 1;

    println!("{:?}", scores);

    // HashSet 之间的操作
    let set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = vec![2, 3, 4].into_iter().collect();

    // difference
    // let set3 = set1.difference(&set2).collect::<HashSet<_>>();
    // assert_eq!(set3, vec![1].into_iter().collect());
    //
    // // intersection
    // let set4 = set1.intersection(&set2).collect::<HashSet<_>>();
    // assert_eq!(set4, vec![2, 3].into_iter().collect());
    //
    // // union
    // let set5 = set1.union(&set2).collect::<HashSet<_>>();
    // assert_eq!(set5, vec![1, 2, 3, 4].into_iter().collect());

}

use serde_json::{ Value};

pub fn untyped_example() -> serde_json::Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Student {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn typed_example() -> Result<(),Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Student = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}


use serde_json::{Deserializer};
use tar::Header;

pub fn test_stream() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";

    let stream = Deserializer::from_str(data).into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }
}




#[derive(Debug, Serialize, Deserialize)]
struct Boy {
    name: String,
    age: u8,
}



pub fn de_list() -> Result<(),Error> {
    let json_str = r#"
    [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25},
        {"name": "Charlie", "age": 40}
    ]
    "#;

    let people: Vec<Boy> = serde_json::from_str(json_str)?;
    println!("{:?}", people);

    Ok(())
}



#[test]
pub fn test_sn() {
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    assert_eq!(printed, "128");
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

//#[test]
//pub fn test_cl() {
//    let token = encode(&jsonwebtoken::Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
//    println!("token:{}",token)
//}