/// Default state of all code is PRIVATE
///
/// If an item is public, it can be accessed through any of its parent modules.
/// If an item is private,
///  it can be accessed only by its immediate parent module and any of the parent’s child modules.

extern crate modules;

fn main(){
    modules::client::connect();
}