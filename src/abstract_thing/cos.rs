use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use syslog::ResultExt;

type SF<T,U,Y> = fn(T,U) -> Y;
struct COS<T,U,Y> {
    sf:Box<dyn Fn(T,U) -> Y>
}

impl<T: 'static,U: 'static,Y: 'static> COS<T,U,Y> {
    fn new(sf:SF<T,U,Y>) -> Self{
        COS {
            sf: Box::new(sf)
        }
    }

    fn deal(&self,a:T,b:U) -> Y {
        (self.sf)(a,b)
    }
}

#[test]
pub fn test_aos1() {
    let a1 = COS::new(add);
    let r1 = a1.deal(2,3);
    println!("r1 = {}",r1);

    let a2 = COS::new(sub);
    let r2 = a2.deal(2.0,3.1);
    println!("r2 = {}",r2);

    let a3 = COS::new(sf1);
    let r3 = a3.deal(6,3);
    println!("r3 = {}",r3);

    let a4 = COS::new(sf2);
    let r4 = a4.deal(6,true);
     println!("r4 = {}",r4.unwrap());

}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sub<T: std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

//为什么T的约束要加这些类型，因为参数是范型T没有算数运算规则。
fn sf1<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> +  std::marker::Copy>(a:T,b:T) -> T {
    (a+b)*(a-b)
}

//做其他事情的应用，那么每新增一个应用就新定义一个函数，这个应用处理它自己要处理的参数,参数类型可以不同，返回类型都是Result（便于管理错误）
fn sf2<T : Display,U: Display>(a:T,b:U) -> Result<T, ()> {
    println!("parameter1 {},parameter2 {}",a,b);
    self::sf_common();
    Ok(a)
}

fn sf_common(){
    println!("公共函数")
}