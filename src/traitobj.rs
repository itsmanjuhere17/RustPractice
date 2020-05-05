trait Draw{
    fn draw(&self);
}

pub struct Screen{
    components:Vec<Box<dyn Draw>>
}

impl Screen{
    pub fn run(&self){
        for comp in self.components.iter(){
            comp.draw();
        }
    }
}

//Button
struct Button{
    width:u16,
    height:u16,
    text:String,
}
impl Button{
    fn getButton(width:u16,height:u16,text:String)->Button{
        //Directly inserting params into struct. Field Init shorthand.
        let but = Button{
            width,
            height,
            text
        };
        but
    }
}
impl Draw for Button{
    fn draw(&self){
        println!("Inside Button draw method");
    }
}

//CheckBox
pub struct CheckBox{
    height:u16,
    width:u16,
    options:Vec<String>
}

impl CheckBox{
    fn getCheckBox(height:u16,width:u16,options:Vec<String>)->CheckBox{
        let check_box = CheckBox{
            height,
            width,
            options
        };
        check_box
    }
}

impl Draw for CheckBox{
    fn draw(&self){
        println!("Inside CheckBox draw method");
    }
}

//State Design Pattern.
trait State{
    fn review(self:Box<Self>)->Box<dyn State>;
    fn publish(self:Box<Self>)->Box<dyn State>;
    fn content<'a>(&self,post:&'a Blog)->&'a str{
        ""
    }
}

struct Draft{

}
impl State for Draft{
    fn review(self:Box<Self>)->Box<dyn State>{
        Box::new(Review{})
    }
    fn publish(self:Box<Self>)->Box<dyn State>{
        self
    }
}

struct Review{

}
impl State for Review{
    fn review(self:Box<Self>)->Box<dyn State>{
        self
    }
    fn publish(self:Box<Self>)->Box<dyn State>{
        Box::new(Publish{})
    }
}

struct Publish{

}

impl State for Publish{
    fn review(self:Box<Self>)->Box<dyn State>{
        self
    }
    fn publish(self:Box<Self>)->Box<dyn State>{
        self
    }
    fn content<'a>(&self,post:&'a Blog)->&'a str{
        &post.content
    }
}

struct Blog{
    state:Option<Box<dyn State>>,
    content:String
}

impl Blog{
    fn create()->Blog{
        let blog = Blog {
            state: Some(Box::new(Draft{})),
            content: String::new()
        };
        blog
    }
    fn content(&self)->&str{
        //let x =self;
        if let Some(state) = &self.state{
           state.content(self)
        }
        else{
            ""
        }
        //self.state.as_ref().unwrap().content(self)
    }
    fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }
    fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.review());
        }
    }
    fn publish(&mut self){
        if let Some(s)=self.state.take(){
            self.state = Some(s.publish());
        }
    }
}

pub fn trait_objects(){
    println!("################ INSIDE TRAIT OBJECTS ##########################");
    let but = Button::getButton(10,20,"Click Me".to_string());
    let ch_box = CheckBox::getCheckBox(11,21,[String::from("On"),String::from("Off")].to_vec());
    let screen = Screen{
        components:vec![Box::new(but),Box::new(ch_box)]
    };
    screen.run();
    //State Pattern.
    let mut blog = Blog::create();
    blog.add_text("I am Manjunath");
    assert_eq!("",blog.content());
    blog.request_review();
    assert_eq!("",blog.content());
    blog.publish();
    assert_eq!("I am Manjunath",blog.content());
    println!("################ EXITING TRAIT OBJECTS #########################");
}