use std::time::SystemTime;

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

fn main() {
    let simple_error = Error::Simple(SystemTime::now());
    let complex_error = Error::Complex(SystemTime::now(), "Errore".to_string());

    print_error(simple_error);
    print_error(complex_error);

    println!("Hello, world!");
}

fn print_error(e: Error) {
    match e {
        Error::Simple(time) => println!("Simple error occurred at {:?}", time),
        Error::Complex(time, message) => println!("Complex error occurred at {:?} with message: {}", time, message),
    }
}




