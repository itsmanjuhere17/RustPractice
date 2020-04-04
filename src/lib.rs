pub mod front_house {
    pub mod hosting {
        fn add_waitlist() {}

        pub fn do_seating() {
            println!("In do_seating fun");
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
    #[derive(Debug)]
    pub struct Breakfast{ //By default, if struct is defined public all data fields are not public.
        pub meal:String, //You need to specifically declare it as public.
        fruit:String
    }
    impl Breakfast{
        pub fn summer(item:&str)->Breakfast{
            Breakfast{
                meal:String::from(item),
                fruit:String::from("Mango")
            }
        }
    }
    pub enum Appetizer{ //By default,enums are public.
        Soup,
        Salad
    }
}

use front_house::hosting;
use front_house::Breakfast; //Using relative path.
//or
use crate::front_house::Breakfast as strBreakfast; //Specifying relative path

//Using use keyword to define scopes.
fn eat_at_restaurant()
{
    //Absolute path.
    crate::front_house::hosting::do_seating(); //Note: if hosting and do_seating are private, you can't refer to it and rust throws error.
    //Relative path.
    front_house::hosting::do_seating();//Note: Same as above, if privately defined error is thrown.

    let mut Bfast = front_house::Breakfast::summer("Idly");
    //or
    let mut Bfast1 = Breakfast::summer("dosa");
    Bfast.meal = String::from("Dosa");
    println!("Breakfast ordered is:{:#?}",Bfast);

}
