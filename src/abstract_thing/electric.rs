use std::collections::VecDeque;
// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     static ref WIRE_LINE:VecDeque<Electric> = VecDeque::new();
// }

#[derive(Clone, Copy)]
struct Electric {
    e_size: i8,
    e_direction: i8,
}

/// time 充电时间
#[test]
 fn test_electric() {
    let mut WIRE_LINE:VecDeque<Electric> = VecDeque::new();
    let mut e_tool = VecDeque::new();
    for i in 0..5 {
        let e = Electric {
            e_size:1,
            e_direction:1,
        };

        for i in 0..5 {
            WIRE_LINE.push_back(e);
        }

        for i in 0..2 {
            unsafe { WIRE_LINE.pop_back(); }
            e_tool.push_back(e);
        }

        println!("当前电线中的电子数量：{}",WIRE_LINE.len());
    }
}