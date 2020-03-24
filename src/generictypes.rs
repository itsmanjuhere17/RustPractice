use crate::quickpractice;
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

    //Traits.
    let news = News::new();
    println!("{}",news.Summarize());
    let tweet = Tweet::new();
    //Default Summarize from tweet.
    println!("{}",tweet.Summarize()); //Check, I would have removed implementation in tweet trait.


    println!("############ EXITING GenericTypes function ##############");
}

fn find_largest(vect:Vec<i32>)->i32{
    let mut largest = vect[0];
    for ele in &vect{ //Borrowed as immutable reference.
        if ele > &largest{ //As ele is reference, need to compare with reference only.
            largest = *ele; //Accessing va;ue should be done via deference operator.But, cannot update it.
        }
    }
    largest
}
