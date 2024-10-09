use rust_oop_polymorphism_trait_object_gui::{Draw, Screen, Button, TextView};

#[derive(Debug)]
struct ScrollView{
    length: i32
}

impl Draw for ScrollView {
    fn draw(&self) {
        println!("ScrollView: {:?}", self);
    }
}

fn main() {
    Screen{
        components: vec![
            Box::new(Button{
                width: 10,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(TextView{
                width: 25,
                height: 25,
                content: String::from("Hi"),
            }),
            Box::new(ScrollView{
                length: 50,
            }),
        ]
    }.run();
}
