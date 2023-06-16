use std::iter;

#[derive(Debug,Clone)]
struct Theory<T> {
    inner:T,
}

impl<T> Theory<T> {
    fn new(t:T) -> Self  {
        Self {
            inner: t,
        }
    }

    fn show(&self){
        println!("12");
    }
}

#[test]
fn test_theory() {
    let colore_x = |x| x*2;
    let colore_y = |x| println!("{}",x);
    let theory_c1 = Theory::new(colore_x(2));
    let theory_c2 = Theory::new(colore_y(5));
    let theory_i = Theory::new(16);

    // theory_c1.show();
    // theory_c2.show();
    // theory_i.show();

    //克隆多个相同的对象
    let objects: Vec<Theory<i32>> = (0..10).map(|_| theory_i.clone()).collect();
    for item in &objects {
        println!("{:?}", item);
    }

}