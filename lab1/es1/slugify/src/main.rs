use clap::Parser;

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuwxyyzzz";
//WARNING : The slice SUBS_I is without the 'ù' to do a fourth test...


#[derive(Parser, Debug)]
struct Args {
// input string
slug_in: String,
}

fn main() {
    let args = Args::parse();
    let s :String = args.slug_in.to_string();
    
    println!("{}",s);
    println!("slug: {}", slugify(&s));

}

fn slugify(s: &str) -> String {
    let mut news : String = "".to_string();
    let mut newc = 'a';
    for char in s.chars(){
        char.to_lowercase().next().unwrap();
        if !char.is_alphanumeric(){
            if newc != '-' {
                newc = '-';
                news.push(newc);
            }
        }
        else{
            newc = conv(char);
            news.push(newc);
        }
    }
    if let Some('-') = news.chars().last() {
        if news.len() > 1{
            news.pop();
        }
    }
    news
}

fn conv(c: char) -> char {
    let mut i :i32 = 0;
    c.to_lowercase().next().unwrap();
    if SUBS_I.contains(c) {
        for newc in SUBS_I.chars() {
            if c == newc  {
                break;
            }
            i += 1;
        }
        if let Some(rep_c) = SUBS_O.chars().nth(i as usize) {
            return rep_c;
        }
        return c;
    }
    else if SUBS_O.contains(c){
        return c;
    }
    else{
        return '-';
    }
}

#[cfg(test)]
mod tests {
use super::*;


#[test]
fn my_first_test() {
// valore = preparazione test
let valore = "à";
let valore_atteso = "a";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_second_test() {
// valore = preparazione test
let valore = "c";
let valore_atteso = "c";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_third_test() {
// valore = preparazione test
let valore = "°";
let valore_atteso = "-";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_fourth_test() {
// valore = preparazione test
let valore = "ù";
let valore_atteso = "-";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_fifth_test() {
// valore = preparazione test
let valore = "ciao come";
let valore_atteso = "ciao-come";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_six_test() {
// valore = preparazione test
let valore = "ciàè";
let valore_atteso = "ciae";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_seven_test() {
// valore = preparazione test
let valore = " ";
let valore_atteso = "-";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_eight_test() {
// valore = preparazione test
let valore = "    ";
let valore_atteso = "-";
assert_eq!(slugify(valore), valore_atteso)}
#[test]
fn my_nine_test() {
// valore = preparazione test
let valore = "ci{a}o}";
let valore_atteso = "ci-a-o";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_ten_test() {
// valore = preparazione test
let valore = "{{{{{";
let valore_atteso = "-";
assert_eq!(slugify(valore), valore_atteso)
}
#[test]
fn my_eleven_test() {
// valore = preparazione test
let valore = " a ";
let valore_atteso = "-a";
assert_eq!(slugify(valore), valore_atteso)
}
}
