use crate::abstract_thing::emotion;

trait Emotion {
     fn show(&self);
}

struct Happiness {
    name: String,
}

impl Emotion for Happiness {
     fn show(&self) {
        println!("happy {}!",self.name)
    }
}

struct Angry {
    name: String,
}

impl Emotion for Angry {
     fn show(&self) {
        println!("angry {}!",self.name)
    }
}

 fn person_emotion(choice:i8) -> Box<dyn Emotion>  {
    match choice {
        1 => Box::new(Happiness {name:"panda".to_string()}),
        2 => Box::new(Angry {name:"elphant".to_string()}),
        _ => Box::new(Happiness { name: "".to_string() }),
    }
}

#[test]
fn test_emotion() {
    let chice = 1;
    let emotion = person_emotion(chice);
    emotion.show();
}
