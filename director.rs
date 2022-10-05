use std::env;
use std::fs;
use std::str::Split;
use std::collections::HashMap;
use std::process::exit;

fn main(){
    
    let mut var_types: HashMap<String, String> = HashMap::new();
    let mut int_vars: HashMap<String, i32> = HashMap::new();
    
    let var_types_three_char: [&str; 2] = ["int", "str"];
    let var_types_four_char: [&str; 2] = ["bool", "char"];

    let args: Vec<String> = env::args().collect();
    let mut tempstr: String = "".to_owned();
    let mut tempstr2: String = "".to_owned();
    if args.len() > 1{
        let file_path = &args[1];
        let fileread = fs::read_to_string(file_path).expect("File able to be read");
        let mut splitfile: Vec<&str> = fileread.lines().collect();
        for i in 0..splitfile.len(){
            for j in 0..splitfile[i].len(){
                tempstr = format!("{}{}", tempstr, splitfile[i].chars().nth(j).unwrap().to_string());
            }
            if tempstr.len() >= 3{
                if var_types_three_char.contains(&&tempstr[0..3]){
                    let mut c = 4;
                    tempstr2 = "".to_owned();
                    while tempstr.chars().nth(c).unwrap().to_string() != " "{
                        tempstr2 = format!("{}{}", tempstr2, tempstr.chars().nth(c).unwrap().to_string());
                        c += 1;
                    }
                    println!("new {} found: {}",&tempstr[0..3] , tempstr2);
                }
            }
            if tempstr.len() >= 4{
                if var_types_four_char.contains(&&tempstr[0..4]){}
            }
            tempstr = "".to_owned();
        }
    }
    else{
        println!("Please specify a file");
        exit(1);
    }
}
