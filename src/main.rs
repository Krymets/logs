use std::fs;
use std::io::Error;

fn string_test(
    a: String,
    b: &String,
    c: &str
) {

}

fn main() {
    string_test(String::from("hello"), &String::from("world"), "hello");

    // let text = fs::read_to_string("logs.txt");
    // println!("{:#?}", text);

    match fs::read_to_string("logs2.txt") {
        Ok(text) => println!("{:#?}", text.len()),
        Err(e) => println!("{:#?}", e),
    }


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