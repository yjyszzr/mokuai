trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[test]
fn test_dyn() {
    let c: &dyn Shape = &Circle { radius: 1.0 };
    println!("area: {}", c.area()); // 输出 area: 3.141592653589793
}


#[test]
fn test_quicksort() {
   //懒加载








}
