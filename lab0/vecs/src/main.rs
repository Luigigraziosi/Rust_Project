
/*fn main(){
    let (_a, _v) = array_and_vec();
}

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let mut v = Vec::new();
    v.push(1);
    // TODO: declare your vector here with the macro for vectors

    (a, v)
} */
/* */


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element = *element*2
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element*2
    }).collect()
}

fn test_vec_loop() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
    let ans = vec_loop(v.clone());

    assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
}

fn test_vec_map() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
    let ans = vec_map(&v);

    assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
}
fn main(){
    test_vec_loop(); 
    test_vec_map()

}