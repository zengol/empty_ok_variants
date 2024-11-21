use std::fs;
use std::io::Error;

fn main() {
    // let text = fs::read_to_string("logs.txt");

    // println!("{:#?}", text);

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    match validate_email(String::from("rodautilamail.com")) {
        Ok(..) => println!("email is valid"),
        Err(reason_this_failed_validation) => {
            println!("{}", reason_this_failed_validation)
        }
    } 
}
/* "contains()" Returns true if the given pattern matches a sub-slice of this string slice.

Returns false if it does not.

The [pattern] can be a &str, [char], a slice of [char]s, or a function or closure that determines if a character matches */
fn validate_email(email: String) -> Result<(),Error> {
    if email.contains("@") {
        //Success
        Ok(())
    } else {

        Err(Error::other("emails must have an @"))
    }
}



fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}
