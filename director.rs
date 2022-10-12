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
    let func_types: [&str; 3] = ["println", "print", "release"];
    let math_ops: [&str; 5] = ["+", "-", "*", "/", "%"];
    let statement_types: [&str; 3] = ["if", "for", "while"];

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
            let mut tempstr: String = "".to_owned();
            let mut tempstr2: String = "".to_owned();
            let mut comment_found = false;
            for j in 0..splitfile[i].len(){
                if j < splitfile[i].len() - 1{
                    if splitfile[i].chars().nth(j).unwrap() == '/'{
                        if splitfile[i].chars().nth(j + 1).unwrap() == '/'{
                            comment_found = true;
                        }
                    }
                }
                if !comment_found{
                    tempstr = format!("{}{}", tempstr, splitfile[i].chars().nth(j).unwrap().to_string());
                }
            }
            if comment_found && tempstr.chars().nth(tempstr.len() - 1).unwrap() == ' '{
                tempstr = format!("{}", tempstr.split_at(tempstr.len() - 1).0);
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
                c += 1;
                while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != "(" && tempstr.chars().nth(c).unwrap().to_string() != ")"{
                    if tempstr.chars().nth(c).unwrap().to_string() == "\""{
                        c += 1;
                        while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != "\""{
                            if tempstr.chars().nth(c).unwrap().to_string() == "\\"{
                                c += 1;
                                match tempstr.chars().nth(c).unwrap().to_string().as_str(){
                                    "n"=>in_parentheses = format!("{}{}", in_parentheses, "\n"),
                                    "t"=>in_parentheses = format!("{}{}", in_parentheses, "\t"),
                                    "r"=>in_parentheses = format!("{}{}", in_parentheses, "\r"),
                                    _=>in_parentheses = format!("{}{}", in_parentheses, tempstr.chars().nth(c).unwrap().to_string()),
                                }
                                c += 1;
                            }
                            else{
                                in_parentheses = format!("{}{}", in_parentheses, tempstr.chars().nth(c).unwrap().to_string());
                                c += 1;
                            }
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
                let mut raw_in_parentheses = "".to_owned();
                let mut r = tempstr2.len();
                while r < tempstr.len() && tempstr.chars().nth(r).unwrap().to_string() != "(" && tempstr.chars().nth(r).unwrap().to_string() != ")"{
                    raw_in_parentheses = format!("{}{}", raw_in_parentheses, tempstr.chars().nth(r).unwrap().to_string());
                    r += 1
                }
                if tempstr2.to_string() == "println"{
                    println!("{}", in_parentheses);
                }
                if tempstr2.to_string() == "print"{
                    print!("{}", in_parentheses);
                }
                if tempstr2.to_string() == "release"{
                    if &*indiv_var_types.get(&tempstr2.clone()).expect("Idk").to_string().as_str() == "int"{
                        int_vars.remove(&tempstr2.clone());
                    }
                    if &*indiv_var_types.get(&tempstr2.clone()).expect("Idk").to_string().as_str() == "str"{
                        str_vars.remove(&tempstr2.clone());
                    }
                    if &*indiv_var_types.get(&tempstr2.clone()).expect("Idk").to_string().as_str() == "bool"{
                        bool_vars.remove(&tempstr2.clone());
                    }
                    if &*indiv_var_types.get(&tempstr2.clone()).expect("Idk").to_string().as_str() == "char"{
                        char_vars.remove(&tempstr2.clone());
                    }
                    indiv_var_types.remove(&tempstr2);
                }
                in_parentheses = "".to_owned();
            }
            else if indiv_var_types.contains_key(&tempstr2){
                if tempstr2.len() + 2 < tempstr.len(){
                    if indiv_var_types.get(&tempstr2).expect("Idk").to_string() == "int".to_string(){
                        if tempstr.chars().nth(tempstr2.len()).unwrap().to_string() == " " && math_ops.contains(&tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string().as_str()) && tempstr.chars().nth(tempstr2.len() + 2).unwrap().to_string() == "="{
                            let mut c = tempstr2.len() + 4;
                            let mut temp_int_str = "".to_owned();
                            let mut temp_int_int = 0;
                            while c < tempstr.len(){
                                temp_int_str = format!("{}{}", temp_int_str, tempstr.chars().nth(c).unwrap().to_string());
                                c += 1;
                            }
                            if int_vars.contains_key(&temp_int_str){
                                temp_int_int = *int_vars.get(&temp_int_str).expect("Idk");
                            }
                            else{
                                temp_int_int = temp_int_str.parse::<i64>().unwrap();
                            }
                            if tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "+"{
                                int_vars.insert(tempstr2.clone(), (*int_vars.get(&tempstr2).expect("Idk") + temp_int_int));
                            }
                            if tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "-"{
                                int_vars.insert(tempstr2.clone(), (*int_vars.get(&tempstr2).expect("Idk") - temp_int_int));
                            }
                            if tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "*"{
                                int_vars.insert(tempstr2.clone(), (*int_vars.get(&tempstr2).expect("Idk") * temp_int_int));
                            }
                            if tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "/"{
                                int_vars.insert(tempstr2.clone(), (*int_vars.get(&tempstr2).expect("Idk") / temp_int_int));
                            }
                            if tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "%"{
                                int_vars.insert(tempstr2.clone(), (*int_vars.get(&tempstr2).expect("Idk") % temp_int_int));
                            }
                        }
                    }
                    else if indiv_var_types.get(&tempstr2).expect("Idk").to_string() == "str".to_string(){
                        let mut in_parentheses = "".to_owned();
                        let mut parentheses_var_name = "".to_owned();
                        let mut c = tempstr2.len() + 2;
                        while c < tempstr.len(){
                            if tempstr.chars().nth(c).unwrap().to_string() == "\""{
                                c += 1;
                                while c < tempstr.len() && tempstr.chars().nth(c).unwrap().to_string() != "\""{
                                    if tempstr.chars().nth(c).unwrap().to_string() == "\\"{
                                        c += 1;
                                        match tempstr.chars().nth(c).unwrap().to_string().as_str(){
                                            "n"=>in_parentheses = format!("{}{}", in_parentheses, "\n"),
                                            "t"=>in_parentheses = format!("{}{}", in_parentheses, "\t"),
                                            "r"=>in_parentheses = format!("{}{}", in_parentheses, "\r"),
                                            _=>in_parentheses = format!("{}{}", in_parentheses, tempstr.chars().nth(c).unwrap().to_string()),
                                        }
                                        c += 1;
                                    }
                                    else{
                                        in_parentheses = format!("{}{}", in_parentheses, tempstr.chars().nth(c).unwrap().to_string());
                                        c += 1;
                                    }
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
                        if tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "=" && tempstr.chars().nth(tempstr2.len()).unwrap().to_string() == " "{
                            str_vars.insert(tempstr2.clone(), in_parentheses.clone());
                        }
                        if tempstr.chars().nth(tempstr2.len()).unwrap().to_string() == " " && tempstr.chars().nth(tempstr2.len() + 1).unwrap().to_string() == "+" && tempstr.chars().nth(tempstr2.len() + 2).unwrap().to_string() == "="{
                            str_vars.insert(tempstr2.clone(), format!("{}{}", *str_vars.get(&tempstr2).expect("Idk"), in_parentheses));
                        }
                    }
                    else if indiv_var_types.get(&tempstr2).expect("Idk").to_string() == "bool".to_string(){
                        match &tempstr[tempstr2.len()..]{
                            " = true"=>bool_vars.insert(tempstr2.clone(), true.clone()),
                            " = false"=>bool_vars.insert(tempstr2.clone(), false.clone()),
                            _=>exit(1),
                        };
                    }
                }
            }
        }
        tempstr = "".to_owned();
        tempstr2 = "".to_owned();
    }
    else{
        println!("Please specify a file");
        exit(1);
    }
}
