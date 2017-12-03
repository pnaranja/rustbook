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
