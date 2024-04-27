use clap::Parser;
use std::fs::OpenOptions;
//use std::fs::File;
use std::io::prelude::*;


#[derive(Parser, Debug)]
struct Args {
// input string
modality: String,
file_path: String,
l: String,
}


fn main() {
    let args = Args::parse();
    let lunghezza : Vec<i32> = args.l
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap()) // Converti ogni sottostringa in un numero
        .collect();

    if args.modality == "new".to_string(){
        newboard(&args.file_path, lunghezza);
    }
    if args.modality == "add".to_string(){
        posiziona();
    }
}


fn newboard(fp : &str, l : Vec<i32>) {
    let mut file = match OpenOptions::new()
    .create(true).write(true)
    .open(fp){
        Ok(file) => file,
        Err(e) =>{
             panic!("impossibile aprire {}, occurs {}", fp, e);
        }
    };
    let mut s : String = format!("{} {} {} {}\n", l[0], l[1], l[2], l[3]);
    file.write_all(s.as_bytes()).unwrap();
    println!("{}", s);
    s = "                    \n".to_string();
    for _i in 0..20{
        file.write_all(s.as_bytes()).unwrap();
    }
    println!("File creato correttamente!");
}

fn posiziona(){





    
}