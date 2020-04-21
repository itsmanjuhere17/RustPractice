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

pub fn trait_objects(){
    println!("################ INSIDE TRAIT OBJECTS ##########################");
    let but = Button::getButton(10,20,"Click Me".to_string());
    let ch_box = CheckBox::getCheckBox(11,21,[String::from("On"),String::from("Off")].to_vec());
    let screen = Screen{
        components:vec![Box::new(but),Box::new(ch_box)]
    };
    screen.run();
    println!("################ EXITING TRAIT OBJECTS #########################");
}