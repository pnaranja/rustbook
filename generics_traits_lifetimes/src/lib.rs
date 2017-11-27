use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;


pub trait Summarizable{
    fn summary(&self) -> String;
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
