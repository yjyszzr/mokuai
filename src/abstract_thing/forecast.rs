use std::env;
use std::time::SystemTime;

pub(crate) trait Forecast {
    //获取资料计算
    fn materials_compute();

    //预测计算
    fn forecast_compute()  -> String {
        Self::materials_compute();
        Self::get_data()
    }

    //真实计算
    fn real_compute() {
        Self::forecast_compute();
    }

    fn get_data() -> String;
}

struct Pre {
    name: String,
}

impl Pre {
    fn new() -> Pre {
        Pre{
            name:"预测计算模型".to_string(),
        }
    }
}

impl Forecast for Pre {
    fn materials_compute() {
        println!("当前材料,{:?},{:?},{:?}",SystemTime::now(),env::current_dir(),env::var("USER"))
    }

    fn forecast_compute() -> String {
        Self::materials_compute();
        println!("根据当前所拥有的材料判断用户的行为，提前准备好数据");
        let data = Self::get_data();
        data
    }

    fn real_compute() {
        let data = Self::forecast_compute();
        println!("预测得到的数据:{}",data);
        println!("用到数据A开始真正的计算,..")
    }

    fn get_data() -> String{
        return "存储中的数据A".to_string();
    }
}

#[test]
fn test_forecast() {
    Pre::real_compute();
}