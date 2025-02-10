use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");
    let nothing = fs::read_to_string("nothing.txt");

    // Result Enum
    match text {
        Ok(text) => {
            println!("(1) {:?}", text);
        }
        Err(error) => {
            println!("(1) {:?}", error);
        }
    }
    match nothing {
        Ok(text) => {
            println!("(2) {:?}", text);
        }
        Err(error) => {
            println!("(2) {:?}", error);
        }
    }

    let num1 = divide(1.0, 2.0);
    println!("(3) {:#?}", num1);
    let num2 = divide(1.0, 0.0);
    println!("(4) {:#?}", num2);
    let num3 = divide(1.0, 0.0);

    match num3 {
        Ok(result) => {
            println!("(5) {:#?}", result);
        }
        Err(err) => {
            println!(
                "(5)\nkind: {:#?}\nmessage: {:#?}",
                err.kind(),
                err.to_string()
            );
        }
    }

    let valid_email_1 = validate_email(String::from("test@gmail.com"));
    println!("(6) {:#?}", valid_email_1);

    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            println!("(7) {:#?}", text);
        }
        Err(err) => {
            println!("(7) {:#?}", err);
        }
    }

    string_test(String::from("test1"), &String::from("test2"), "test3");
    string_test(
        "test1".to_string(),
        &String::from("test2"),
        &String::from("test3").to_string(),
    );

    let mut error_logs = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors(text.as_str());
            println!("(9) {:#?}", &error_logs);
        } // #1 text is about to go out of scope, error_logs borrow values from text
        Err(err) => {
            println!("(9) {:#?}", err);
        }
    }
    // #1 error occurred
    // println!("(9) {:#?}", error_logs);

    let mut error_logs = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors_fixed(text.as_str());
        }
        Err(err) => {
            println!("(10) {:#?}", err);
        }
    }
    println!("(10) {:#?}", error_logs);

    let mut error_logs = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors_fixed(text.as_str());

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("(11) success"),
                Err(err) => println!("(11) error, {:#?}", err),
            };
        }
        Err(err) => {
            println!("(11) {:#?}", err);
        }
    }

    // try to occur panic
    let text = fs::read_to_string("logs.txt").expect("failed to read file");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors_2.txt", error_logs.join("\n")).expect("failed to write to file");

    match question() {
        Ok(value) => {
            println!("(13) ok:\n{:#?}", value);
        }
        Err(err) => {
            println!("(13) err:\n{:#?}", err);}
    };
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        return Err(Error::other("Can't divide by zero"));
    }

    Ok(a / b)
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        return Ok(());
    }
    Err(Error::other("email must have an @"))
}

fn string_test(a: String, b: &String, c: &str) {
    println!("(8)\na:{:#?}\nb:{:#?}\nc:{:#?}", a, b, c);
}

fn extract_errors(text: &str) -> Vec<&str> {
    let line_text = text.split("\n");
    let mut results = vec![];

    for line in line_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    results
}

fn extract_errors_fixed(text: &str) -> Vec<String> {
    let line_text = text.split("\n");
    let mut results = vec![];

    for line in line_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}

fn question() -> Result<(), Error> {
    // try operator
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("nothing/errors_3.txt", error_logs.join("\n"))?;

    Ok(())
}
