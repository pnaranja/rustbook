pub struct Counter{
    count : i32
}

impl Counter{
    /// Create a new Counter
    pub fn new() -> Counter{
        Counter{count: 0}
    }
}

impl Iterator for Counter{
    type Item = i32;

    fn next (&mut self) -> Option<Self::Item> {
        self.count +=1;
        if self.count <10 {Some (self.count)}
        else {None}
    }
}


#[test]
fn try_counter_iterator (){
    let mut counter = Counter::new();

    assert_eq! (counter.next(), Some(1));
    assert_eq! (counter.next(), Some(2));
}

#[test]
fn use_other_iterator_methods (){
    let mut counter = Counter::new();

    let sum : i32 = counter.zip(Counter::new().skip(1))
                .map(|(a, b)| a*b) 
                .inspect(|x| println! ("{}",x))
                .sum();

    assert_eq!(240,sum);
}
