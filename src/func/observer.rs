use std::collections::HashMap;
use crate::func::observer::Event::{Button1, Load, Save, Setting1};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Event {
    Save,
    Load,
    Button1,
    Button2,
    Setting1,
    Setting2,
}

pub type Subscriber = fn(file_path: String);

#[derive(Default)]
struct Publisher {
    //观察者模式的数据结构就是这个map，类似于事件总线
    events:HashMap<Event,Vec<Subscriber>>
}

//操作这个map
impl Publisher {

    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.get_mut(&event_type).unwrap().retain(|&x| x != listener);
    }

    pub fn notify(&mut self, event_type: Event, file_path: String) {
        let listeners = self.events.get_mut(&event_type).unwrap();
        for listener in listeners  {
            listener(file_path.clone());
        }
    }

}

#[derive(Default)]
struct Editor {
    publisher:Publisher,
    file_path: String,
}

impl Editor {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn load(&mut self,event_path: String) {
        self.file_path = event_path.clone();
        self.publisher.notify(Load,event_path);
    }

    pub fn save(&mut self) {
        self.publisher.notify(Save,self.file_path.clone());
    }

    pub fn press_btn1(&mut self) {
        self.publisher.notify(Button1,self.file_path.clone());
    }

    pub fn press_set1(&mut self,event_path: String){
       // print!("event_path: {}",event_path.clone());
        self.publisher.notify(Button1,self.file_path.clone())
    }
}

fn save_listener(file_path: String) {
    let email = "admin@example.com".to_string();
    println!("Email to {}: Save file {}", email, file_path);
}

fn btn1(file_path: String) {
    println!("按钮1的效果")
}

fn setting1(file_path: String) {
    println!("设置1的效果")
}

#[test]
fn test_observer() {
    let mut editor = Editor::default();
    // editor.events().subscribe(Event::Load,|file_path| {
    //     let log = "/path/to/log/file.txt".to_string();
    //     println!("Save log to {}: Load file {}", log, file_path);
    // });
    //
    // editor.events().subscribe(Event::Save,save_listener);

    // editor.load("test1.txt".into());
    // editor.load("test2.txt".into());
    // editor.save();
    // editor.events().unsubscribe(Event::Save, save_listener);
    // editor.save();

    editor.events().subscribe(Button1,btn1);

    editor.press_set1("a".into());

    // editor.events().subscribe(Setting1,setting1);
    // editor.press_btn1();



}
