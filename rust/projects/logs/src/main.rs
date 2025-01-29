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
