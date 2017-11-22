use std::fs::File;
use std::io::ErrorKind;

use std::io::Read;
use std::io::Write;
use std::io;

fn main() {
    open_or_create_file();
    open_or_create_file2();
    let mut f = ret_result().unwrap_or_else(|_| panic!("There was a problem with ret_result"));

    f.write("Hello everyone!".as_bytes()).unwrap();

    f = ret_result2().unwrap_or_else(|_| panic!("There was a problem with ret_result2"));
    f.write("Hello everyone2!".as_bytes()).unwrap();
}

fn open_or_create_file ()
{
    let f = match (File::open ("test/hello.txt")) {
        Ok (file) => file,
        Err (ref err) if err.kind() == ErrorKind::NotFound =>{
            match File::create ("test/hello.txt"){
                Ok (fc) => fc,
                Err (err2) => panic! ("Could not create file")
            }
        }
        Err (error) => panic! ("HUGE Problem! : {:?}", error)
    };

}

fn open_or_create_file2 ()
{
    let f = File::open("test2/hello.txt")
        .unwrap_or_else(|_| File::create("test2/hello.txt")
        .unwrap_or_else(|_| panic!("Huge Problem")));
}

fn ret_result() -> Result<File,io::Error>
{
    let mut f = File::create("test/hello.txt");
    match f{
        Ok(f) => Ok(f),
        Err(e) => Err(e)
    }
}

fn ret_result2() -> Result<File,io::Error>
{
    Ok(File::create("test2/hello.txt")?)
}

