use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;


pub trait Summarizable{
    fn summary(&self) -> String;

    // Create default implementation
    fn summary2(&self) -> String{
        String::from("Read more...")
    }
}

pub struct NewsArticle{
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summarizable for NewsArticle{
    fn summary(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

}

pub struct Tweet{
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summarizable for Tweet{
    fn summary(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}


// Implement a local trait on a public type
impl <T> Summarizable for Vec<T>{
    fn summary(&self) -> String{
        "This is a Vector summary".to_string()
    }
}

// Implement a public trait to a local type
impl Display for Tweet{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{} - {} - {} - {}", self.username, self.content, self.reply, self.retweet)
    }
}


// Using default implementation of Summarizable2
pub trait Summarizable2{
    fn summary(&self) -> String{
        format!("Read more... {}", self.summary_content())
    }

    fn summary_content(&self) -> String;
}

pub struct NewsArticle2{
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summarizable2 for NewsArticle2{
    fn summary_content(&self) -> String{ self.content.clone() }
}


// Trait Bounds
pub trait Summarizable3{
    fn summary_generic(&self) -> String{
        format!("Read more... {}", self.summary_content_generic())
    }

    fn summary_content_generic(&self) -> String;
}

// Implement a trait for a generic bounded by a trait (aka "bounded implementations")
pub struct Tweet2{
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summarizable for Tweet2{
    fn summary(&self) -> String{
        format!("Tweet2: {}, by {}", self.content, self.username)
    }

}

impl <T: Summarizable> Summarizable3 for T{
    fn summary_content_generic(&self) -> String{
        format!("generic stuff!!!!")
    }


}




//Lifetimes
//Lifetime annotations help relate the lifetimes of multiple references to each other

//Both x and y must both live as long as the returned value's lifetime
//The returned reference is guaranteed to be valid as long as the shorter of the lifetimes
//of x and y
pub fn longest<'a> (x: &'a str, y: &'a str ) -> &'a str{
    if x.len () > y.len (){x} else {y}
}

//Can't return a reference which was created locally in the function
//Compiler complains that ret does not live long enough
//pub fn bad_return<'a> (x: &'a str, y: &str ) -> &'a str{
//    if x.len () > y.len (){x}
//    else {
//        let ret = String::from("The y value");
//        &ret
//    }
//}

//Struct which holds a ref
//Need to declare lifetime in the beginning, like for a function
pub struct HoldRef<'a>{
    pub myref : &'a str,
}

//No need to explicitly declare lifetimes because of 3rd lifetime elision rule
impl<'a> HoldRef<'a>{
    pub fn announce_and_ret(&self, announcement: &str) -> &str{
        println!("Attention! {}", announcement);
        self.myref
    }

}

// Add Generic type of announcement
// Compiler errors led me to add T to implement trait -  std::fmt::Debug
// and remove bound - ?Sized
pub fn longest_with_announcement<'a,T> (x: &'a str, y: &'a str, announcement: &'a T) -> &'a str
    where T:Display, T:std::fmt::Debug, T: ?Sized
{
    println!("Attention! {:?}", announcement);
    if x.len () > y.len (){x} else {y}
}
