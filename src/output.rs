use serde::Serialize;

use crate::error::AppError;

pub fn print_json<T: Serialize>(data: &T) {
    let json = serde_json::to_string_pretty(data).expect("failed to serialize output");
    println!("{json}");
}

pub fn print_error(err: &AppError) {
    eprintln!("{}", err.to_json());
}
