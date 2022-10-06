use std::env;
use std::fs;
use std::collections::HashMap;
use std::process::exit;

fn main(){

    // Variables stored in HashMaps, cause easy
    let mut indiv_var_types: HashMap<String, String> = HashMap::new();
    let mut int_vars: HashMap<String, i64> = HashMap::new();
    let mut str_vars: HashMap<String, String> = HashMap::new();
    let mut bool_vars: HashMap<String, bool> = HashMap::new();
    let mut char_vars: HashMap<String, char> = HashMap::new();

    // Store currently accepted variable types and functions
    let var_types: [&str; 4] = ["int", "str", "bool", "char"];
    let func_types: [&str; 1] = ["println"];

    // read in args
    let args: Vec<String> = env::args().collect();
    let mut tempstr: String = "".to_owned();
    let mut tempstr2: String = "".to_owned();
    // check to make sure args were given
    if args.len() > 1{
        // read in file
        let file_path = &args[1];
        let fileread = fs::read_to_string(file_path).expect("File not able to be read");
        let mut splitfile: Vec<&str> = fileread.lines().collect();
        // run file line by line
        for i in 0..splitfile.len(){
            // run through each char in the file, assign to str
            for j in 0..splitfile[i].len(){
                tempstr = format!("{}{}", tempstr, splitfile[i].chars().nth(j).unwrap().to_string());
            }
            // create count var
            let mut c = 0;
            tempstr2 = "".to_owned();
            // read in first given thing
            while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != " " && tempstr.chars().nth(c).unwrap().to_string() != "("{
                tempstr2 = format!("{}{}", tempstr2, tempstr.chars().nth(c).unwrap().to_string());
                c += 1;
            }
            // handle given variables
            if var_types.contains(&tempstr2.as_str()){
                // grab wpecific components of variables
                let mut var_name = "".to_owned();
                let mut var_content = "".to_owned();
                c += 1;
                while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != " " && tempstr.chars().nth(c).unwrap().to_string() != "="{
                    var_name = format!("{}{}", var_name, tempstr.chars().nth(c).unwrap().to_string());
                    c += 1;
                }
                while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() == " " || tempstr.chars().nth(c).unwrap().to_string() == "="{
                    c += 1;
                }
                while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != " " && tempstr.chars().nth(c).unwrap().to_string() != "="{
                    var_content = format!("{}{}", var_content, tempstr.chars().nth(c).unwrap().to_string());
                    c += 1;
                }
                // now add to HashMaps
                indiv_var_types.insert(var_name.clone(), tempstr2.clone());
                if tempstr2.as_str() == "int"{
                    int_vars.insert(var_name.clone(), var_content.parse::<i64>().unwrap().clone());
                }
                else if tempstr2.as_str() == "str"{
                    str_vars.insert(var_name.clone(), var_content.replace("\"", "").clone());
                }
                else if tempstr2.as_str() == "char"{
                    char_vars.insert(var_name.clone(), var_content.replace("\'", "").chars().nth(0).unwrap().clone());
                }
                else if tempstr2.as_str() == "bool"{
                    match var_content.as_str(){
                        "true"=>bool_vars.insert(var_name.clone(), true.clone()),
                        "false"=>bool_vars.insert(var_name.clone(), false.clone()),
                        _=>exit(1),
                    };
                }
                else{
                    exit(1);
                }
            }
            else if func_types.contains(&tempstr2.as_str()){
                let mut in_parentheses = "".to_owned();
                let mut parentheses_var_name = "".to_owned();
                if tempstr2.to_string() == "println"{
                    c += 1;
                    while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != "(" && tempstr.chars().nth(c).unwrap().to_string() != ")"{
                        if tempstr.chars().nth(c).unwrap().to_string() == "\""{
                            c += 1;
                            while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != "\""{
                                in_parentheses = format!("{}{}", in_parentheses, tempstr.chars().nth(c).unwrap().to_string());
                                c += 1;
                            }
                        }
                        while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != "\"" && tempstr.chars().nth(c).unwrap().to_string() != ")" && tempstr.chars().nth(c).unwrap().to_string() != " "{
                            parentheses_var_name = format!("{}{}", parentheses_var_name, tempstr.chars().nth(c).unwrap().to_string());
                            c += 1;
                        }
                        if indiv_var_types.contains_key(&parentheses_var_name){
                                if indiv_var_types.get(&parentheses_var_name).expect("Idk").to_string() == "int".to_string(){
                                    in_parentheses = format!("{}{}", in_parentheses, int_vars.get(&parentheses_var_name).expect("Idk").to_string());
                                }
                                if indiv_var_types.get(&parentheses_var_name).expect("Idk").to_string() == "str".to_string(){
                                    in_parentheses = format!("{}{}", in_parentheses, str_vars.get(&parentheses_var_name).expect("Idk"));
                                }
                                if indiv_var_types.get(&parentheses_var_name).expect("Idk").to_string() == "char".to_string(){
                                    in_parentheses = format!("{}{}", in_parentheses, char_vars.get(&parentheses_var_name).expect("Idk"));
                                }
                                if indiv_var_types.get(&parentheses_var_name).expect("Idk").to_string() == "bool".to_string(){
                                    in_parentheses = format!("{}{}", in_parentheses, bool_vars.get(&parentheses_var_name).expect("Idk").to_string());
                                }
                        }
                        parentheses_var_name = "".to_owned();
                        c += 1;
                    }
                    println!("{}", in_parentheses);
                    in_parentheses = "".to_owned();
                }
            }
            tempstr = "".to_owned();
            tempstr2 = "".to_owned();
        }
    }
    else{
        println!("Please specify a file");
        exit(1);
    }
}
