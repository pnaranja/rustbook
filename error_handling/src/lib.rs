pub mod new_guessing_game{

    extern crate rand;
    use std::io;
    use std::cmp::Ordering;
    use self::rand::Rng;

    pub fn game() {
        let secret_num = rand::thread_rng().gen_range (1,101);
        println!("Let's play: GUESS THE NUMBER!");

        loop{
            // Need to re-initialize the guess as a String for read_line
            let mut guess = String::new();
            println!("What's your guess?: ");

            io::stdin ().read_line (&mut guess).expect ("Failed to read line");

            println! ("You guessed: {}", guess);

            // Handle invalid guess (non-number
            let guess: u32 = match guess.trim ().parse ()
                {
                    Ok (num) => num,
                    Err (_) => {println! ("Enter a number"); continue},
                };

            match guess.cmp (&secret_num)
                {
                    Ordering::Less => println! ("Too small"),
                    Ordering::Greater => println! ("Too big"),
                    Ordering::Equal => {println! ("You Win!"); break}
                }
        }

    }
}
