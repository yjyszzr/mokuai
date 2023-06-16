//
// trait Weather {
//     fn drop_thing();
// }
//
// struct Raining<'a> {
//     name : &'a  str,
// }
//
// impl<'a> Raining<'a> {
//     fn watch_weather() {
//         println!("what's weather like today?")
//     }
// }
//
// impl<'a> Weather for Raining<'a> {
//     fn drop_thing() {
//         println!("it's raining today")
//     }
// }
//
// struct Snowing<'a> {
//     name : &'a  str,
// }
//
// impl<'a> Weather for Snowing<'a> {
//     fn drop_thing() {
//         println!("it's snowing today")
//     }
// }
//
// #[test]
// fn test_weather() {
//     // let mut w:Vec<Box<dyn Weather>> = Vec::new();
//     // let rain = Rainning {
//     //     name:"rain",
//     // };
//     // let snow = Snowing {
//     //     name: "snowing",
//     // };
//     //
//     // w.push(Box::new(rain));
//     // w.push(Box::new(snow));
//
//     let rain = Raining {
//         name:"rain",
//     };
//     let snow = Snowing {
//         name: "snowing",
//     };
//     let mut w:Vec<Box<dyn Weather>> = vec![Box::new(rain),Box::new(snow)];
//
// }