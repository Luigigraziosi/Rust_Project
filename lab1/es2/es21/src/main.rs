use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let file_path = "test.txt";

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path) { // Corretto il doppio parentesi chiusa prima del match
            Ok(file) => file,
            Err(e) => {
                panic!("Errore apertura file {}: {}", file_path, e);
            }
    };

    let mut contenuto_file = String::new();
    file.read_to_string(&mut contenuto_file).unwrap();

    println!("{}", contenuto_file);

    // Scrive il contenuto nel file con unwrap()
    for _ in 0..10 { // Utilizzo di _ poiché non usiamo la variabile
        file.write(contenuto_file.as_bytes()).unwrap(); // Utilizzo di write_all() per scrivere l'intero contenuto
    }
}
