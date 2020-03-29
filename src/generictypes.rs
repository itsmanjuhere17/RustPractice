use crate::quickpractice;
//use std::cmp::PartialOrd;
pub trait Summary{
    fn Summarize(&self)->String{
        format!("Default Summarize")
    }
}
pub struct News{
    headline:String,
    author:String,
    country:String,
    isBreakingNews:bool
}

impl News{
    pub fn new()->News{
        let news = News{
        headline:String::from("Covid-19 is killing people"),
        author:String::from("Manjunath"),
        country:String::from("Finland"),
        isBreakingNews:true
    };
    news
    }
}

impl Summary for News{
    fn Summarize(&self)->String{
        format!("News Summary is:{} {} {} {}",self.headline,self.author,self.country,self.isBreakingNews)
    }
}

pub struct Tweet{
    tweet:String,
    account:String,
    reply:bool
}

impl Tweet{
    pub fn new()->Tweet{
        let tweet = Tweet{
            tweet:String::from("Stay at home and stay safe!!"),
            account:String::from("itsmanjuhere"),
            reply:false
        };
        tweet
    }
}

impl Summary for Tweet{
    /* fn Summarize(&self)->String{
        format!("Tweet Summary is: {} {} {}",self.tweet,self.account,self.reply)
    }
    */
}

pub struct Point<T,U>{
    x:T,
    y:U
}

impl<T,U> Point<T,U>{
    fn mixUp<V,W>(self,point:Point<V,W>)->Point<T,W>{
        Point{
            x:self.x,
            y:point.y
        }
    }
}

//Rooot function
pub fn generictypes()
{
    println!("############ In GenericTypes function ##############");
    //Generic function that removes code duplication of same types.
    let mut vect1 = vec![2,1,4,5,10];
    let mut largest = find_largest(vect1);
    println!("Largest ele from vector is:{}",largest);
    let mut vect1 = vec![22,11,40,51,100];
    let mut largest = find_largest(vect1);
    println!("Largest ele from vector is:{}",largest);

    let p1 = Point{
        x:10.0,
        y:20.1
    };
    let p2 = Point{
        x:"Manju",
        y:'j'
    };
    let p3=p1.mixUp(p2);//See fun definition, how we can mixup different generic types.
    println!("Mix up Points are:{} {}",p3.x,p3.y);

    //Traits.
    let news = News::new();
    println!("{}",news.Summarize());
    let tweet = Tweet::new();
    //Default Summarize from tweet.
    println!("{}",tweet.Summarize()); //Check, I would have removed implementation in tweet trait.

    //Traits as function parameters.


    println!("############ EXITING GenericTypes function ##############");
}

//Trait Bounds.
/* fn traitBound<T,Summary>(item:T){
    println!("Trait Bound {}",item.Summarize());
}
*/

//Trait Bounds.
fn find_genLargest<T>(list:Vec<T>)->T
where T:PartialOrd + Copy + Clone //Types that implement Copy trait will automatically implement Clone.
{
    let mut largest = list[0];
    for ele in list{ //Borrowed as immutable reference.
        if ele > largest{
            largest = ele; //Accessing value should be done via deference operator.But, cannot update it.
        }
    }
    largest
}


fn find_largest(vect:Vec<i32>)->i32{
    let mut largest = vect[0];
    for ele in &vect{ //Borrowed as immutable reference.
        if *ele > largest{
            largest = *ele; //Accessing va;ue should be done via deference operator.But, cannot update it.
        }
    }
    largest
}
