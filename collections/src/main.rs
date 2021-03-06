use std::collections::HashMap;
use std::io;
use std::process;
use std::i32;

fn main() {
    // VECTORS
    // Vectors are homogeneous
    let _v : Vec<i32> = Vec::new();

    // Use vec! macro to create vector with initial values
    let v2 = vec![1,2,3];

    // How to push values in a Vector
    let mut v3 : Vec<i32> = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);

    // Reading elems of a vector
    let _third_1: &i32 = &v2[2];
    let third_2: Option<&i32> = v2.get(2);

    let x = v2[2];
    println!("x: {}", x);
    println!("third_* is: {}", third_2.unwrap_or(&0));

    /// Remember, can't mutate when there is a mutable and immutable reference
    /// Pushing new value might force the compiler to reallocate new memory and
    /// copy old elements to the new space.  First element ref would be dangling.
//    let immutable_ref = &v3;
//    v3.push(0);


    // If you want different types you can use a vector of enums
    enum SpreadSheetCell{
        Int(i32), Float(f64), Text(String)
    }

    let _row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue"))
    ];

    // STRINGS
    // Only 1 type of String in the Rust core - str (string slice)
    // Remember, str are a reference to some UTF-8 encoded string data stored elsewhere

    // String is in the Rust Std Library
    // It is a growable, mutable, owned UTF-8 string type
    let mut _s : String  = String::new();
    let _s : String = "A new String".to_string();
    let _s : String = String::from("Another String");

    let mut s : String  = String::new();
    s.push_str("Try this");
    let mut s2 = " Ok!".to_string();
    s.push_str(&s2);
    s2 = s2.to_string();
    s2.push_str(" Again");

    println!("s: {}",s);
    println!("s2: {}",s2);
    println!("s1+s2={}",s + &s2);

    // '+' looks like an inline function of add
    // Definition: add(self, s: &str) -> String
    // s is now moved to the '+' function. Main function lost ownership of s
//    s.push_str("Uh oh");

    let s = String::from("Another!");
    // For >2 strings to concatenate, use format.  Returns a String
    println!("Using format!: {}", format!("Attaching strings: {} - {}", s, s2));

    //Strings are actually of type Vec<u8> (unsigned 8 bit or unsigned 1 byte))
    println!("Length of Hello is {}", "Hello".to_string().len()); // 5 bytes - each letter is a byte
    println!("Length of дра is {}", "дра".to_string().len()); // 6 bytes! - UTF-8 requires 2 bytes each

    // Therefore an index to a string's bytes will not always correlate to a valid Unicode scalar value
    // b should get 104 (byte value), but that's not really expected!
    // To avoid confusion, the Compiler will not allow this
    //let b = &"hello"[0];


    // Use chars method to access elements in a String
    [1,2,3,4].iter().for_each(|x| println!("{}",x));
    "sdf".chars().for_each(|x| println!("{}",x));
    let some_char : String = "Здравствуйте".chars().filter(|x| x.eq(&'й')).collect();
    println!("some char of Здравствуйте: {}", some_char);


    // Use String splice to get the first character (first byte in this case)
    let sdf = &"sdf"[0..1];
    println!("First letter of sdf is {}", sdf); // First letter of sdf is s

    // HASHMAPS
    // Homogeneous values (keys must be same type, values must be same type)
    // Stored on the Heap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Also can be created by collecting a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let _scores2 : HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();

    // Ownership
    // Remember, String ownership is MOVED -> in this case, to the HashMap
    let field_name = String::from("The field name");
    let field_value = String::from("The field value");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Cannot use field_name anymore
    //field_name.len();

    // But if you pass in references, then you can use the field_name
    let field_name2 = String::from("The field name");
    let field_value2 = String::from("The field value");
    let mut map2 = HashMap::new();
    map2.insert(&field_name2, &field_value2);
    field_name2.len();
    // but the field_name2 and field_value2 must be valid for as long as the hash map is valid -> Lifetimes

    // Access Hash Map values using get
    // Notice get returns an Option
    let _the_field_value : Option<&String> = map.get(&String::from("The field name"));

    // Iterate a HashMap
    // Notice the differences of the format of the output
    println!("\nScores: ");
    scores.iter().for_each(|x| println!("{:?} ", x));

    for (k,v) in &scores{
        println!("{}, {}", k,v);
    }
    println!("{:?}", scores);


    // Only insert if key has no value
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Green")).or_insert(500);
    println!("\nNew Scores: {:?}", scores); // {"Green": 500, "Blue": 10, "Yellow": 50}


    // Update value based on old value
    println!("\nUpdate values based on new values");
    let text = "hello world wonderful world oh so wonderful";
    let mut map_text = HashMap::new();

    // The count receives a mutable reference to the value of the key (which is an i32 in this case)
    // Increase value of count dereference (whatever count is referring to)
    for word in text.split_whitespace(){
       let count : &mut i32 = map_text.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map_text);

    let mut map_text2 :HashMap<&str,i32> = HashMap::new();
    text.split_whitespace().for_each(|x|{
        let count : &mut i32 = map_text2.entry(x).or_insert(0); // Get the ref to the count, or insert 0
        *count += 1; // since count is a reference, need to increment what the ref is pointing to
    });
    println!("{:?}", map_text2);

    // Summary Problems
    // Get Mean, Median and Mode of a list of integers
    fn mean(v : Vec<i32>) -> i32{
        v.iter().fold(0, |acc, x| acc+x) / v.len() as i32
    }
    println!("Mean of [1,2,3,4,5] is {}", mean(vec![1, 2, 3, 4, 5]));

    fn median(v: &Vec<i32>) -> i32{
        let v_middle = v.len() / 2;
        let mut v_mut = v.to_vec(); // copy the vector to a mutable vector
        v_mut.sort_by(|a,b|a.cmp(b));
        let v2 = v_mut.get(v_middle).unwrap_or(&0); //unwraps return a reference
        return *v2; //dereference to get the value
    }
    let vec2 = &mut vec![5,5,1,4,3];
    println!("Median of vec2 [5,5,1,4,3] is {}", median(vec2));
    println!("vec2 has not changed! : {:?}", vec2);

    /// Provide a hashmap of {word : wordcount}
    fn num_hashmap_count(v: &Vec<i32>) -> HashMap<&i32,i32>
    {
        let mut h : HashMap<&i32,i32> = HashMap::new();
        for num in v{
            let count = h.entry(num).or_insert(0);
            *count += 1;
        }
        return h;
    }

    fn mode(v: &Vec<i32>) -> i32{
        let h = num_hashmap_count(&v).clone();
        h.iter().for_each(|x| println!("{:?}",x));
        let init_key = *h.keys().min().unwrap(); //unwrap returns a reference
        let highest_key = h.keys().fold(init_key,|a,b| if h.get(a).unwrap() > h.get(b).unwrap(){return a}else{return b});
        *highest_key //dereference to get the value
    }
    println!("Mode of vec2 [5,5,1,4,3] is {}", mode(vec2));


    /// Convert strings to Pig Latin
    /// Examples: first -> irst-fay ; apple -> apple-hay
    fn pig_latin_word(s: &str) -> String{
        if s.starts_with("a") || s.starts_with("e") || s.starts_with("i") || s.starts_with("o") || s.starts_with("u")
        {
            s.to_string()+"-hay"
        }
        else{
            let fst = s[1..s.len()].to_string();
            let snd = s[0..1].to_string()+"ay";
            format!("{}-{}",fst,snd)
        }

    }
    println!("{}", pig_latin_word("abbot"));
    println!("{}", pig_latin_word("empty"));
    println!("{}", pig_latin_word("ice"));
    println!("{}", pig_latin_word("order"));
    println!("{}", pig_latin_word("unless"));
    println!("{}", pig_latin_word("paul"));
    println!("{}", pig_latin_word("mae"));

    /// Split string by whitespace and apply pig latin.  Add a space after each word
    fn pig_latin_string(s:&str)->String{
        s.to_string().split_whitespace().map(|x| pig_latin_word(&x)+" ").collect()
    }
    let str="Hello Paul how are you";
    println!("String \"{}\" converted to pig latin: {}",str,pig_latin_string(str));



    println!("Press any key to start Name-Department program");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    name_to_dept();
    

    /// Program for adding/showing employee names and departments
    fn name_to_dept()
    {
        let mut name_dept : HashMap<String,String> = HashMap::new();

        let clear_screen = || process::Command::new("clear").status().unwrap();
        
        let print_menu = ||
            {   
                clear_screen();
                println!("Main Menu:\n");
                println!("1. Enter Name and Department");
                println!("2. Show list of people of a department");
                println!("3. Show list of all people in the company by department (sorted alphabetically)");
                println!("4. Quit program");
                eprint!("Enter number choice: ");
            };

        /// Reads a line as a String and returns the value
        fn readline() -> String{
            let mut ret = String::new();
            io::stdin().read_line(&mut ret).expect("Failed to read line");
            ret
        }

        /// User Prompt to go back to Main Menu
        fn back_to_main_menu(){
            println!("\nPress any key to go back to the Main Menu");
            io::stdin().read_line(&mut String::new()).expect("Failed to read line");
        }

        /// Add name and department to HashMap
        fn add_name_dept(name_dept : &mut HashMap<String,String>){
            eprint! ("Enter Name: ");
            let name = readline();
            eprint! ("Enter Department: ");
            let dept = readline();
            name_dept.insert(name,dept);
            back_to_main_menu();
        }

        /// Return all people in a particular department
        fn get_people_in_dept(dept :&String, name_dept : &HashMap<String,String>) -> Vec<String>{
            let new_hash : HashMap<String,String>
                = name_dept.iter()
                    .filter(|&(_a,b)| dept.clone().eq(b))
                    .map(|(a,b)|(a.clone(),b.clone()))
                    .collect();

            new_hash.keys().map(|v|v.clone()).collect::<Vec<String>>()
        }


        /// Show all people in a particular department
        fn show_people_in_dept(name_dept : &HashMap<String,String>){
            eprint! ("Enter Department you want to see: ");
            let dept = readline();

            let names = get_people_in_dept(&dept, name_dept);

            if names.is_empty()
            {
                println!("Did not find any names in department {}", dept);
            }
            else
            {
                names.iter().for_each(|x| eprint!("{}",x));
            }

            back_to_main_menu();
        }

        /// Show all people by dept (in alphabetical order)
        fn show_all_people_by_dept(name_dept : &HashMap<String,String>)
        {
            println!("Showing list of all people in the company by department");

            let mut departments = name_dept.values().collect::<Vec<&String>>();
            departments.sort(); departments.dedup();

            departments.iter().for_each(|dept|
            {
                println!("Department: {}", dept);
                let mut people = get_people_in_dept(dept, name_dept);
                people.sort();
                people.iter().for_each(|person| println!("Name: {}",person))
            });

            back_to_main_menu();
        }



        // Main Loop
        loop {
            print_menu();
            let choice = readline().chars().find(|a| a.is_digit(10)).and_then(|a| a.to_digit(10)).unwrap_or(0);
            println!("You chose: {}\n", choice);

            match choice{
                1 => add_name_dept(&mut name_dept),
                2 => show_people_in_dept(&name_dept),
                3 => show_all_people_by_dept(&name_dept),
                4 => break,
                _ => println! ("Unknown menu choice")
            };
        }
    }
}

