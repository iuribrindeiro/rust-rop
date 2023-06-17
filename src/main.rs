use rop::{MapR, TapRError};
use value_objects::{StrError, StrMax115};

mod rop;
mod value_objects;

fn main() {
    let str_result = StrMax115::new(("Iuri Brindeiro").to_string())
        .tap_error(|x| x.iter().for_each(print_error))
        .map_r(|r| r.value().to_string());

    match str_result {
        Ok((v, _)) => println!("Success value is {}", v),
        Err(_) => println!("Fix the errors above"),
    }
}

fn print_error(err: &StrError) {
    match err {
        StrError::StrMax(max) => println!("Value can't have more than {} characters", max),
        StrError::NotEmpty => println!("Value can't be left blank"),
    }
}
