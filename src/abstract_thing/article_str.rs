
pub trait JiXuWen<'a> {
    fn jixuwen(& mut self,content: &String) -> String ;
}

pub trait ShuoMingWen<'a> {
    fn shuomingwen(& mut self,content: &String) -> String;
}

pub trait YiLunWen<'a> {
    fn yilunwen(& mut self,content: &String) -> String ;
}

pub trait SanWen<'a> {
    fn sanWen(& mut self,content: &String) -> String ;
}

struct Article<'a> {
    write_time: &'a  str,
    write_place: &'a  str,
    write_author: &'a  str,
    content:String,
}

impl<'a> Article<'a> {
    fn new(write_time: &'a  str,write_place: &'a  str,write_author: &'a  str,content: String) -> Self {
        Self {
            write_place,
            write_time,
            write_author,
            content,
        }
    }
}

impl<'a> JiXuWen<'a> for Article<'a>  {
    fn jixuwen(& mut self,content: &String) -> String {
        self.content = content.to_string();
        content.to_string()
    }
}

impl<'a> YiLunWen<'a> for Article<'a>  {
    fn yilunwen(& mut self,content: &String) -> String {
        self.content = content.to_string();
        content.to_string()
    }
}


#[test]
fn test_article() {
    let j_content = String::from("aa");
    let y_content = String::from("bb");
    let mut jixunwen = Article::new("2023-01-01","hongshan","zzr","a".to_string());
    let mut yilunwen = Article::new("2022-01-01","shuiku","zld","b".to_string());


    println!("{}",jixunwen.jixuwen(&j_content));
    println!("{}",yilunwen.yilunwen(&y_content));
}