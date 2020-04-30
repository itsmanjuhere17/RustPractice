use std::ops::Add; //Add trait to perform + operation.

static STATIC_HELLO_WORLD:&str = "Hello World"; //static lifetime.
static mut COUNTER:i32 = 0;

trait advanced{
    type Item;
    fn dummy(&self)->Self::Item;
}

struct adv{

}
//NOTE: You can only implement one concrete trait with associated type when defined inside trait.
impl advanced for adv{
    type Item = u32;
    fn dummy(&self)->Self::Item{
        0 as u32
    }
}

//Demonstration of Add trait.
#[derive(Debug)]
struct Point{
    x:u32,
    y:u32
}

impl Add for Point{
    type Output = Point;
    fn add(self,other:Point)->Point{
        let res = Point{
            x: self.x+other.x,
            y: self.y+other.y
        };
        res
    }
}
//Function pointers.
fn add_one(param:i32)->i32{
    param+1
}

fn add_twice(f:fn(i32)->i32,arg:i32)->i32{
    f(arg)+f(arg)
}

pub fn advanced_features(){
    println!("############################ INSIDE ADVANCED FEATURES #################################");

    //Unsafe Rust
       //Unsafe Super Powers:
           //Dereference a raw pointer
           //Call an unsafe function or method
           //Access or modify a mutable static variable
           //Implement an unsafe trait
           //Access fields of union S

    //a) De-referencing a raw pointer.
         //Raw pointer.
            //Defy borrowing rules of rust. i.e can have mutable and immutable pointer or both mutable pointers point to same location.
            //Can not point to anything.
            //allowed to be null.
            //doesn't implement any auto clean-up
    let mut x=5;
    let refe = &x as *const i32; //This is immutable raw-pointer.
    let refe1 = &mut x as *mut i32;//This is mutable pointer.
    //Dereferencing.
    unsafe{
        println!("De-referencing raw pointers:{} {}",*refe,*refe1);
    }

    //b) Calling an unsafe method or function.
    unsafe {
        dangerous();
    }
    //Calling external method of other programming language.
    unsafe {
        println!("Calling abs method of C: {}",abs(-8));
    }

    //c) Accessing or modifying mutable static variable.
    println!("Accessing immutable static variable:{}",STATIC_HELLO_WORLD);
    unsafe {
        COUNTER+= 1;
        println!("Counter value is:{}",COUNTER);
    }
    //d) Unsafe traits.
    unsafe trait dummy{

    }
    struct strDummy{

    }
    unsafe impl dummy for strDummy{

    }
    /* unsafe {
        let mut x=10;
        x=x+100;
        println!("X value in unsafe bloc is:{}",x);
    }
    */

    //Advanced Traits
    //a) Associated types.
    let adv = adv{};
    println!("Using associated type in trait:{}",adv.dummy());
    let p1 = Point{x:10,y:5};
    let p2 = Point{x:12,y:6};
    let p3 = p1+p2;
    println!("Result struct is:{:#?}",p3);

    //New types. Alias types.
    type signed_int = i32;
    let x=5;
    let y:signed_int = x;
    println!("{} {}",x,y);

    type thunk = Box<dyn Fn() + Send + 'static>;
    let f:thunk = Box::new(||{println!("Thunk")});

    //function pointers. Denoted by fn, Don't confuse with Fn which is defined for closures.
    let res = add_twice(add_one,5);
    println!("Res of fun pointer is:{}",res);

    //Returning closure.
    let re_clos = return_closure();
    println!("returning from closure which is returned from function: {}",re_clos(10));
    println!("############################ EXITING ADVANCED FEATURES #################################");
} //End of function.

//return closure.
fn return_closure()->Box<dyn Fn(i32)->i32>{ //Here, we need to use Box type is due t the fact that closure size can't be evaluated at compile time.
    Box::new(|x| x+1)
}
//function signature prefixed with unsafe are called unsafe functions or methods.
unsafe fn dangerous(){
    //Some unsafe code here.
}

//extern function of C language.
extern "C"{
    fn abs(input:i32)->i32;
}

//This is the syntax to be defined to call rust function from other language.
#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Rust function calling from C language");
}