use std::fs;
use std::io::Error;


// fn string_test(
//     a: String,
//     b: &String,
//     c: &str) {}

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut result = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line.to_string());
        }
    }

    result
}

fn main() -> Result<(), Error> {

    let text = fs::read_to_string("logs.txt")?;
    // println!("{}", text.len());
    let error_logs = extract_errors(text.as_str());
    fs::write("logs2.txt", error_logs.join("\n"))?;
    Ok(())

    // string_test(String::from("hello"), &String::from("world"), "hello");

    // let text = fs::read_to_string("logs.txt");
    // println!("{:#?}", text);

    // let mut error_logs = vec![];
    //
    //  let text = fs::read_to_string("logs.txt")
    //      .expect("Something went wrong reading the file");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("logs2.txt", error_logs.join("\n"))
    //     .expect("Something went wrong writing the file");

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());
    //         // println!("{:#?}", error_logs);
    //         match fs::write("logs2.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Log file written to"),
    //             Err(error) => println!("{}", error),
    //         }
    //     }
    //     Err(e) => println!("{:#?}", e),
    // }
    // println!("{:#?}", error_logs);


    // match devide(5.0, 0.0) {
    //     Ok(result_of_division) => {
    //         println!("Result of division: {}", result_of_division);
    //     }
    //     Err(error) => {
    //         println!("Error: {}", error);
    //     }
    // }
    //
    // match validate_email(String::from("edsfjhsdfj@gds.com")) {
    //     Ok(..) => println!("Success"),
    //     Err(error) => println!("Error: {}", error),
    // }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Email must contains @"))
    }
}

fn devide(a:f64, b:f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("b is zero"))
    } else {
        Ok(a / b)
    }
}