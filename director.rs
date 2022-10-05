use std::env;
use std::fs;
use std::collections::HashMap;

fn main(){
    // The language's variables
    let mut ints=HashMap::new();
    let mut strs=HashMap::new();
    let mut bools=HashMap::new();
    let mut chars=HashMap::new();

    let args: Vec<String> = env::args().collect();
    file_path = &args[1];
    println!("File Path: {}", file_path);
}
