struct Summer {
    name: String,
}

impl Summer {
    fn sum_computer() {
        Self::compute1(Compose {
            inner1:String::from("a"),
        });

        Self::compute2(Compose {
            inner1:String::from("b"),
        });

        Self::compute3(Compose {
            inner1:String::from("c"),
        });
    }

}

#[derive(Debug,Clone)]
struct Compose<T> {
    inner1:T,
}

impl Unit1<Compose<String>> for Summer {
    fn compute1(c: Compose<String>) {
        println!("unit 1 {:?}",c)
    }
}

impl Unit2<Compose<String>> for Summer {
    fn compute2(c: Compose<String>) {
        println!("unit 2 {:?}",c)
    }
}

impl Unit3<Compose<String>> for Summer {
    fn compute3(c: Compose<String>) {
        println!("unit 3 {:?}",c)
    }
}

pub trait Unit1<Compose> {
    fn compute1(c:Compose);
}

pub trait Unit2<Compose> {
    fn compute2(c:Compose);
}

pub trait Unit3<Compose> {
    fn compute3(c:Compose);
}

#[test]
fn test_control() {
    Summer::sum_computer();
}
