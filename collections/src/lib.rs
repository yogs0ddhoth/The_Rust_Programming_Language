use std::collections::HashMap;
use std::io;

pub mod hash_maps;
pub mod strings;
pub mod vectors;

pub fn median_n_mode(list: &[i32]) -> (i32, i32) {
    let mut v: Vec<i32> = Vec::new();
    let mut m = HashMap::new();

    for i in list {
        v.push(*i);
        let count = m.entry(i).or_insert(0);
        *count += 1;
    }
    v.sort();

    let mut mode = 0;
    for v in m.values() {
        if *v > mode {
            mode = *v;
        }
    }

    (v[v.len() / 2], mode)
}

pub fn translate_pig_latin(text: &str) -> String {
    let mut translation = String::from("");

    for word in text.split_whitespace() {
        translation += &pig_latin(word);
    }
    return translation;
}
fn pig_latin(word: &str) -> String {
    let mut suffix = String::new();
    let mut index: usize = 0;
    for (i, c) in word.char_indices() {
        if i == 0 {
            match c {
                'a'|'e'|'i'|'o'|'u' => {
                    suffix.push('h');
                    break;
                }
                _ => (),
            }
        }
        match c {
            'a'|'e'|'i'|'o'|'u' => {
                index = i;
                break;
            }
            _ => {
                suffix.push(c);
            }
        }
    }
    format!("{}-{}ay ", &word[index..], suffix)
}

pub fn employee_interface() {
    println!("Hiya Chuck, enter an employee.");

    let mut employees = HashMap::new();
    loop {
        println!("commands:\nadd employee\nget all\nget department");
        
        let input = read_input();
        match input.trim() {
            "add employee" => {
                println!("Enter employee name:");
                let name = read_input();
    
                println!("Enter employee department:");
                let department = read_input();
    
                println!("employee: {}, department: {}", name, department );

                let department_vec = employees.entry(department).or_insert(vec![]);
                if department_vec.contains(&name) {
                    println!("Name already exists!");
                    continue;
                }
                department_vec.push(name);

                println!("{:#?}", employees);
            }
            "get all" => println!("{:#?}", employees),
            "get department" => {
                println!("Enter department:");
                let department = read_input();
                
                let entry = employees.get(&department).unwrap();
                println!("{}: {:#?}", department, entry);

            }
            "exit" => {
                println!("Be seeing you, Chuck.");
                break;
            }
            _ => {
                println!("Invalid command!");
            }
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Enter a command.");
    input.trim().to_string()
}