use std::{collections::HashMap, io, str::FromStr};

#[derive(PartialEq)]
pub enum Operations {
    Set,
    Get,
    Delete,
}

impl FromStr for Operations {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" | "Set" | "SET" => Ok(Operations::Set),
            "get" | "Get" | "GET" => Ok(Operations::Get),
            "delete" | "Delete" | "DELETE" => Ok(Operations::Delete),
            _ => Err("Invalide ops"),
        }
    }
}

fn main() {
    let mut global_state: HashMap<String, String> = HashMap::new();
    println!("input *exit* to stop the program");
    loop {
        println!("Enter the input: \n");
        let mut input_buf = String::new();
        if let Err(err) = io::stdin().read_line(&mut input_buf) {
            println!("Failed to catch the input due to {}", err)
        }
        let operation_vec = input_buf.split_whitespace().collect::<Vec<&str>>();
        if input_buf.trim() == "exit" {
            println!("Shoutting down the process");
            break;
        }
        if operation_vec.len() > 3 || operation_vec.len() <= 1 {
            println!("invalide!!...");
            continue;
        }
        println!("operation_vec {:?}", operation_vec);
        if let Ok(ops) = operation_vec[0].parse::<Operations>() {
            match ops {
                Operations::Set => {
                    if operation_vec.len() == 3 {
                        let key = operation_vec[1].trim().to_string();
                        let value = operation_vec[2].trim().to_string();

                        let _ = global_state.insert(key, value);
                    } else {
                        println!("Invalide SET!!..");
                    }
                }
                Operations::Get => {
                    if operation_vec.len() == 2 {
                        if let Some(result) = global_state.get(operation_vec[1].trim()) {
                            println!("Value is {}", result);
                        } else {
                            println!("Nothing is here!!..")
                        }
                    }
                }
                Operations::Delete => {
                    if operation_vec.len() == 2 {
                        match global_state.remove_entry(operation_vec[1].trim()) {
                            Some((k, v)) => println!("delete key: {k} with it's value: {v}"),
                            None => println!("Nothing to dele"),
                        }
                    }
                }
            }
        } else {
            println!("failed to understand it ")
        }
        println!("Alive ")
    }
}
