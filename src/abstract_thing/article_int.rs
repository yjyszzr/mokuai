
pub trait JiXuWen {
    fn jixuwen(& mut self,content: i32) -> i32 ;
}

pub trait ShuoMingWen {
    fn shuomingwen(& mut self,content: i32) -> i32 ;
}

pub trait YiLunWen {
    fn yilunwen(& mut self,content: i32) -> i32 ;
}

pub trait SanWen {
    fn sanWen(& mut self,content: i32) -> i32 ;
}

struct Article {
    write_time:i32,
    write_place:i32,
    write_author:i32,
    content:i32,
}

impl Article {
    fn new(write_time:i32,write_place:i32,write_author:i32,content:i32) -> Self {
        Self {
            write_place,
            write_time,
            write_author,
            content,
        }
    }
}

impl JiXuWen for Article  {
    fn jixuwen(& mut self,content:i32) -> i32 {
        self.content = content;
        content
    }
}

impl YiLunWen for Article {
    fn yilunwen(&mut self,content:i32) -> i32 {
        self.content = content;
        content
    }
}


#[test]
fn test_article() {
    let mut jixunwen = Article::new(1,2,3,4);
    let mut yilunwen = Article::new(5,6,7,8);

    println!("{}",jixunwen.jixuwen(12));
    println!("{}",yilunwen.yilunwen(21));
}