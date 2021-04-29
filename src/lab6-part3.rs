use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("**************************************************************************");
    println!("* Type \"Add {{employee}} to {{department}}\" to insert                        *");
    println!("* Type \"{{department}}\" to retrieve a list of all people in the department *");
    println!("* Type \"Company\" to retrieve a list of all people in the company         *");
    println!("* Type \"Exit\" to exit this program                                       *");
    println!("**************************************************************************");

    loop {
        let mut query = String::new();
        let mut flag = false;

        println!("Type the query:");
        match io::stdin().read_line(&mut query) {
            Ok(_) => {
                if &query == "Exit\n" {
                    break;
                } else if &query == "Company\n" {
                    view_company(&company);
                    continue;
                } else {
                    for (dept, _) in &company {
                        if &query == dept {
                            view_department(&company, &query);
                            flag = true;
                            break;
                        }
                    }
                    if flag == false {
                        add_employee(&mut company, &query);
                    }
                    continue;
                }
            }
            Err(_) => {
                println!("Cannot read the line. Try again.");
                continue;
            }
        }
    }

    println!("Terminated...");
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, query: &String) {
    let mut space = 0;
    let mut temp = 0;

    let mut employee = "";

    for (i, &item) in query.as_bytes().iter().enumerate() {
        if item == b' ' {
            space += 1;
            if space % 2 == 1 {
                temp = i + 1;
            } else if space == 2 {
                employee = &query[temp..i + 1];
            }
        }
    }
    let department = &query[temp..];

    if space == 0 {
        println!("Try again...");
        return;
    }
    insert_employee(company, employee, department);
}

fn insert_employee(company: &mut HashMap<String, Vec<String>>, employee: &str, department: &str) {
    let key = String::from(department);
    let value = String::from(employee);

    let data = company.entry(key).or_insert({
        let temp = Vec::new();
        temp
    });

    data.push(value);
    data.sort();
}

fn view_department(company: &HashMap<String, Vec<String>>, department: &String) {
    let employees = company.get(department);

    match employees {
        Some(list) => {
            for item in list {
                println!("{}", item);
            }
        }
        None => (),
    }
}

fn view_company(company: &HashMap<String, Vec<String>>) {
    let mut total: Vec<String> = Vec::new();

    for (_dept, list) in company {
        let mut temp = list.clone();
        total.append(&mut temp);
    }
    total.sort();

    for item in total {
        println!("{}", item);
    }
}
