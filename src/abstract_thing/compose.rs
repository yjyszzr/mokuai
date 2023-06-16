

#[derive(Debug,Clone)]
struct Compose<T> {
    inner1:T,
}

#[test]
fn test_compose() {
    let c1 = Compose {
        inner1:String::from("a"),
    };

    let c2 = Compose {
        inner1:String::from("b"),
    };

    let vec = vec![c1,c2];
    vec.iter().for_each(|x| {
        println!("{:?}",x);
    })

}
