pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button{
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self) {
        println!("Button: {:?}", self);
    }
}

#[derive(Debug)]
pub struct TextView{
    pub width: i32,
    pub height: i32,
    pub content: String,
}


impl Draw for TextView{
    fn draw(&self) {
        println!("Text View: {:?}", self);
    }
}
