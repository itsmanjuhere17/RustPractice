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

//Declaring struct with lifetime annotation.
#[derive(Debug)]
struct LifeTimeStruct<'a>{
    part:&'a str
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

    //Traits Bounds.
    //Generic function that handles all types.
    let mut vectInt = vec![11,121,214,546,101];
    let mut largest = find_genLargest(vectInt);
    println!("Largest ele from vector is:{}",largest);
    let mut vectChar = vec!['a','m','n','j','u'];
    let mut largest = find_genLargest(vectChar);
    println!("Largest ele from vector is:{}",largest);
    let mut dynVect = Vec::new();
    dynVect.push(1001);
    dynVect.push(1010);
    dynVect.push(1100);
    dynVect.push(2100);
    let mut largeest = find_genLargest(dynVect);
    println!("Largest ele from vector is:{}",largeest);

    let vecString = vec![String::from("Manjuuuuuuuuuuuu"),String::from("Nath")];
    let largest_str = find_genLargest(vecString);
    println!("Largest ele from vector is:{}",largest_str);
    //Lifetimes.
    let mut intX = 10;
    let mut intY = 20;
    let mut largestInt = findLargestInt(&mut intX,&mut intY);
    println!("Largest ele from vector is:{}",largestInt);
    *largestInt = 50;
    println!("Reference is changed as:{}",intY);

    let str1 = String::from("Manju is Manju and always Manju");
    {
        let longest;
        let str2 = String::from("nath");
        longest = find_longest(str1.as_str(),str2.as_str());
        println!("Longest string is:{}",longest);
    }
    //println!("Longest string is:{}",longest); //It throws error. It means, the generic lifetime is in general followed to least scope. In this case, it is str2 scope.
    //Rust, by default thinks that the returned lifetime is always equal to lifetime of lesser lifetime. To fix, move longest inside.

    let strSeq = "Manjunath is living in Finland. He works in Intopalo Digital";
    let firstSent = strSeq.split('.').next().expect("Cannot split with . ");
    let struc = LifeTimeStruct{
        part:firstSent
    };
    println!("Displaying struc {:#?}",struc);
    //Lifettime Rules. or Lifetime Elision rule.
    //1. If there are more than one parameter, each refernce has it's own lifetime annotation.
    //2. If there is one input parameter, the same input lifetime notation is assigned to output reference.
    //3. If one of the param is either self or mut self, the lifetime of self is assigned to all output lifetimes.(Output is return value)

    println!("############ EXITING GenericTypes function ##############");
}

fn find_longest<'a>(first:&'a str,second:&'a str)->&'a str{
    if first.len() > second.len(){
        first
    }
    else {
        second
    }
}

//Lifetimes function signature.
fn findLargestInt<'a>(item1:&'a mut i32,item2:&'a mut i32)->&'a mut i32{
    if item1 > item2{
        item1
    }
    else{
        item2
    }

}
//Trait Bounds.
/* fn traitBound<T,Summary>(item:T){
    println!("Trait Bound {}",item.Summarize());
}
*/

//Trait Bounds.
fn find_genLargest<T>(list:Vec<T>)->T
where T:PartialOrd + Clone //Types that implement Copy trait will automatically implement Clone.
{
    let mut largest = list[0].clone();
    for ele in list{ //Borrowed as immutable reference.
        if ele > largest{
            largest = ele; //Accessing value should be done via deference operator in case if borrowed as immutable reference.But, cannot update it.
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
