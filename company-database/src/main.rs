/*
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;
use std::io;

fn help() {
    println!("
    | Command | Action                    | Usage               | Example        |
    ------------------------------------------------------------------------------
    | add     | Add a New Employee        | add name department | add john sales |
    | dept    | List Department Employees | dept department     | dept sales     |
    | all     | List All Employees        | all                 | all            |
    | quit    | Quit This Program         | quit                | quit           |
    | help    | Print This Menu           | help                | help           |
    ------------------------------------------------------------------------------\n");
}

fn main() {
    // our company has departments and employees
    let mut company = HashMap::new();
    
    // lets create some default wokers
    company.insert(String::from("bob"), String::from("sales"));
    company.insert(String::from("alice"), String::from("sales"));
    company.insert(String::from("jane"), String::from("sales"));
    company.insert(String::from("joe"), String::from("office"));
    company.insert(String::from("dave"), String::from("office"));
    company.insert(String::from("richard"), String::from("engineering"));
    company.insert(String::from("tom"), String::from("engineering"));
    
    help(); // display main info

    loop {
        let mut names: Vec<_> = company.iter().collect(); // create vec of company for sorting
        names.sort_by_key(|k| k.0);                       // sort by names
    
        let mut departments: Vec<_> = vec![]; // blank vec to hold each type of departments
        for dept in company.values() {
            departments.push(dept);           // save all departments to vec
        }
        departments.sort();                   // needs to be sorted to run dedup
        departments.dedup();                  // remove duplicates

        // recive and modify input for processing
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Issue Reading User Input");
            
        let mut input = input.trim().split_whitespace();
        let command = input.next().unwrap().to_lowercase();

        // process input
        match command.as_str() {
            // when we add a new user
            "add" => match input.next() {
                Some(name) => match input.next() {
                    Some(dept) => {
                        company.insert(String::from(name), String::from(dept));
                        println!("Added {} to {}", name, dept);
                    },
                    None => println!("No Department Input"),
                }
                None => println!("No Name Input"),
            },
            // when we list by dept
            "dept" => match input.next() {
                Some(dept) => {
                    println!("Employees in the {} Department\n{}", dept, "=".repeat(28 + dept.len()));
                    // go though company vector and list workers related to a certain department
                    for (name, dept_2) in &names {
                        if dept == *dept_2 {
                            println!("{}", name);
                        }
                    }
                }
                None => println!("No Department Selected"),
            },
            // when we list all workers
            "all" => {
                println!("All Employees in our Fake Company\n{}", "=".repeat(33));
                // go though department vector and list workers related to it
                for department in departments {
                    for (name, dept) in &names {
                        if department == *dept {
                            println!("{}: {}", department, name);
                        }
                    }
                }
            },
            // when to print help menu
            "help" => help(),
            // when we quit
            "quit" => break,
            // everything else is invalid
            _ => println!("Invalid Command"),
        }
    }
}