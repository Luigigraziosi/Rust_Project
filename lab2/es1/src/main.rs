const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuwxyyzzz";
//WARNING : The slice SUBS_I is without the 'ù' to do a fourth test...

trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}


// impl MySlug for String {
//     fn is_slug(&self) -> bool {
//         for c in self.chars() {
//             if !c.is_alphanumeric() && c != '-' {
//                 return false;
//             }
//         }
//         true
//     }
//     fn to_slug(&self) -> String{
//         slugify(self)
//     }
// }
// impl MySlug for &str{
//     fn is_slug(&self) -> bool {
//         for c in self.chars() {
//             if !c.is_alphanumeric() && c != '-' {
//                 return false;
//             }
//         }
//         true
//     }
//     fn to_slug(&self) -> String{
//         slugify(self)
//     }
// }

fn is_slug(s: &str) -> bool {
    slugify(s) == s 
}
impl<T> MySlug for T
where T : AsRef<str> {
    fn is_slug(&self) -> bool {
       is_slug(self.as_ref())
    }
    fn to_slug(&self) -> String{
        slugify(self.as_ref())
    }
}

fn main() {
    // let s :String = "Ciao,com.kh.afji,a.òfaèàaf.,,.fa.f,a.fapfaokfao,.a.,..-".to_string();
    // let s :String = "Questo non sarà uno Slag!".to_string();

    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice

    // println!("{}",s1);
    // println!("slug: {}", slugify(&s1));
}

fn slugify(s: &str) -> String {
    let mut news : String = "".to_string();
    let mut newc = 'a';
    for mut char in s.chars(){
        char = char.to_lowercase().next().unwrap();
        //println!("{}", char.to_lowercase().next().unwrap());
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
    else if c.is_alphanumeric(){
        return c;
    }
    else{
        return '-';
    }
}