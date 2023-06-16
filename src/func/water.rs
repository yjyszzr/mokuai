
pub trait Volume {
    fn compose() -> u64;
}

#[derive(Default)]
struct Water {
    density:f32,
    color:i8,
}

struct Lake<Water> {
    waters:Vec<Water>,
}

struct River<Water> {
    waters:Vec<Water>,
}


#[test]
fn test_water() {
    let water = Water::default();
    let lake = Lake {
        waters:(0..100).map(|x| &water).collect(),
    };

    let river = River {
        waters:(0..10000).map(|x| &water).collect(),
    };
}
