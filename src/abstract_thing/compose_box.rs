use std::collections::HashMap;
use std::fmt;
use maplit::hashmap;

trait Un {
    fn show(&self) -> i8;
}

struct A {
    name: String,
}

impl Un for A {
    fn show(&self) -> i8 {
        println!("A");
        1
    }
}

struct B {
    name: String,
}

impl Un for B {
    fn show(&self) -> i8 {
        println!("B");
        2
    }
}

//用多个基本单元取组装整个世界
// #[derive(Display)]
struct Compose {
    inner1:Box<dyn Un  + 'static>,
}

impl fmt::Display for Compose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inner: {}", self)
    }
}

impl fmt::Display for dyn Un {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Un: {}", self)
    }
}


#[test]
fn test_compose() {
    let c1 = Compose {
        inner1:Box::new(A {name:String::from("x")}),
    };

    let c2 = Compose {
        inner1:Box::new(B {name:String::from("y")}),
    };

    let vec = vec![c1,c2];
/*    vec.iter().for_each(|x| {
         println!("{}",x.inner1.show());
    });*/


}
