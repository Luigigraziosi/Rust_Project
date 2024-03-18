/* fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}*/
/*fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
} */
/* */
/* */

fn main() {
    let example_string = "   Hello, world!   ";
    println!("Before trimming: '{}'", example_string);
    println!("After trimming: '{}'", trim_me(example_string));

    let initial_string = "Hello";
    println!("Before composing: '{}'", initial_string);
    println!("After composing: '{}'", compose_me(initial_string));

    let input_string = "I like cars";
    println!("Before replacing: '{}'", input_string);
    println!("After replacing: '{}'", replace_me(input_string));
}

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}




