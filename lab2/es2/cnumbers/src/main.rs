use std::ops::Add;
use std::ops::AddAssign;
use std::default::Default;


#[derive(Copy, Clone)]
struct ComplexNumber {
    r :f64,
    i :f64,
}

impl ComplexNumber {
    fn new(r :f64, i :f64) -> ComplexNumber {
        ComplexNumber{r,i}
    }
    fn real(&self) -> f64{
        self.r
    }
    fn imag(&self) -> f64{
        self.i
    }
    fn from_real(r :f64, ) -> ComplexNumber {
        let i:f64 = 0.0;
        ComplexNumber{r,i}
    }
    fn to_tuple(&self) -> (f64, f64) {
        (self.r, self.i)
    }    
}
impl Add for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, rhs: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}
impl Add<f64> for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, rhs: f64) -> ComplexNumber {
        ComplexNumber {
            r: self.r + rhs,
            i: self.i,
        }
    }
}
impl Add<&ComplexNumber> for ComplexNumber{
    type Output = ComplexNumber;
    fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}
impl Add for &ComplexNumber{
    type Output = ComplexNumber;
    fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, rhs: ComplexNumber){
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl Default for ComplexNumber{
    fn default() -> ComplexNumber{
        ComplexNumber{
            r : 0.0,
            i : 0.0,
        }
    }
}
impl Into<f64> for ComplexNumber{
    fn into(self) -> f64{
        if self.i == 0.0 {
            self.r
        } else {
            0.0
        }
    }
}






























fn main() {
    println!("Hello world!")
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
pub fn test_create() {
    let a = ComplexNumber::new(1.0, 2.0);
    assert_eq!(a.real(), 1.0);
    assert_eq!(a.imag(), 2.0);
}

#[test]
pub fn test_create_from_real() {
    let a = ComplexNumber::from_real(10.0);
    assert_eq!(a.real(), 10.0);
    assert_eq!(a.imag(), 0.0);
}

#[test]
pub fn test_add() {
    // implement Add trait
    // rember to set: type Output = Self;
    // see: https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#add--addassign

    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = a + b;

    assert_eq!(c.to_tuple(), (2.0, 4.0));
}
#[test]
pub fn test_add_with_real() {
    // set RHS (rihgt hand side) type for Add!!! It's default value is Self, but it can be changed to anything  
    let a = ComplexNumber::new(1.0, 2.0);
    let b = a + 10.0;

    assert_eq!(b.to_tuple(), (11.0, 2.0))
}

#[test]
pub fn test_inc_add() {
    let mut a = ComplexNumber::new(1.0, 2.0);
    a +=  ComplexNumber::new(2.0, 4.0); 

    assert_eq!(a.to_tuple(), (3.0, 6.0))
}

#[test]
pub fn test_add_with_reference() {
    // references for Rust are new types: you must define the trait for them as RHS
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = a + &b;

    assert_eq!(c.to_tuple(), (2.0, 4.0))
}
#[test]
pub fn test_add_reference_with_reference() {
    // write yourself the test and adjust traits
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(1.0, 2.0);

    let c = &a + &b;
    assert_eq!(c.to_tuple(), (2.0, 4.0))
}

#[test]
pub fn test_enable_copy(){
    // why this code won't compile? Read carefully the error message
    // what do we nee to do to make it work?
    let a = ComplexNumber::new(1.0, 2.0);

    let b = a + a;

    assert_eq!(b.to_tuple(), (2.0, 4.0));
}

#[test]
pub fn test_default_values() {
    // If we want to create an array of complex numbers we need to initialize values with something
    // Arrays can't be not initialized in Rust
    let array: [ComplexNumber; 10] = Default::default();

    for el in array.as_slice() {
        assert_eq!(el.to_tuple(), (0.0, 0.0));
    }
}


#[test]
pub fn test_convert_into_real() {
    let a = ComplexNumber::from_real(1.0);
    let b: f64 = a.into();
    
    assert_eq!(b, 1.0);

}

#[test]
pub fn test_panic_when_impossible_to_convert_to_real() {
    // we can convert into a real only if imag is 0
    let a = ComplexNumber::new(1.0, 2.0);

    let result = std::panic::catch_unwind(|| {
        let _: f64 = a.into();
    });

    assert!(result.is_err());
}

// #[test]
// pub fn test_try_into_f64() {
//     // write trait and a test for the Trait TryInto for converting into f64
//     // the test must check both success and error conditions

//     // Warning: when implementing this trait you will get a compilation error.
//     // Why? Because the std lib has a default implementation for TryInto for all types that implement Into. 
//     // (You can try to write it yourself, it's trivial)
//     // How do we solve this? We delete the Into implementation and the above tests using Into
//     // The purpose is that if the conversion may fail, then you are encouraged to write only TryInto, and we are not allowed to use Into. 
//     // Instead if we have Into the implementation of TryInto is trivial   

//     assert!(true);
// }

// #[test]
// pub fn test_try_form_f64() {
//     // write a trait allowing let complex = f64.into()
//     // and write test
// }


// #[test]
// pub fn test_comparison() {
//     let c = ComplexNumber::new(3.0, 6.0);
//     let mut v = vec![ComplexNumber::new(1.0, 2.0), ComplexNumber::new(2.0, 4.0), c];

//     v.retain(|el| *el == c);

//     assert_eq!(v.len(), 1);
// }


// #[test]
// pub fn test_sorting() {
//     // for sorting we can use the modulus of a complex number 
//     //https://www.cuemath.com/algebra/modulus-of-complex-number/
//     // if |a| > |b| than a > b

//     // Be careful: f64 does not implement Ord since NaN != NaN and you can't 
//     // use cmp from f64 to implement Ord for ComplexNumber
//     // However f64 has total_cmp which produces total ordering
//     // https://doc.rust-lang.org/beta/std/primitive.f64.html#method.total_cmp

//     let a = ComplexNumber::new(1.0, 2.0);
//     let b = ComplexNumber::new(2.0, 4.0);
//     let c = ComplexNumber::new(3.0, 6.0);
//     let mut v = vec![c, b, a];

//     v.sort();

//     assert_eq!(v[0], a);
//     assert_eq!(v[1], b);
//     assert_eq!(v[2], c);
// }

// #[test]
// pub fn test_as_ref() {
//     // implement AsRef<f64> for ComplexNumber
//     // allow a mutable ref to real part as &f64

//     let a = ComplexNumber::new(1.0, 2.0);
//     let r = a.as_ref();

//     assert_eq!(*r, 1.0);
// }

// #[test]
// pub fn test_as_mut() {
//     // implement AsMut<f64> for ComplexNumber
//     // allow a mutable ref to real part as &mut f64

//     let mut a = ComplexNumber::new(1.0, 2.0);
//     let r = a.as_mut();

//     *r = 10.0;

//     assert_eq!(a.real(), 10.0);
// }

// #[test]
// pub fn test_hash_with_hash_map() {
//     // in order to use comeplex numbers in a hash map we need to implement Hash
//     // https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#hash
//     // we can use the to_bits method from f64 to get a u64 representation of the float
//     let a = ComplexNumber::new(1.0, 2.0);
//     let b = ComplexNumber::new(2.0, 4.0);
//     let c: ComplexNumber = 3.0.into();

//     let mut map = std::collections::HashMap::new();
    
//     // first insert must return None: not present
//     match map.insert(a, b) {
//         None => assert!(true),
//         Some(_) => assert!(false)
//     };

//     // trty ro replace value with c
//     match map.insert(a, c) {
//         None => assert!(false),
//         Some(x) => assert_eq!(x.to_tuple(), (2.0, 4.0)) // should return the old value, b
//     };
        

// }


// #[test]
// pub fn test_deque() {
//     // implement VecDeque for ComplexNumber
//     // 1. create a VecDeque with capacity 10
//     // 2. push 10 values in the deque
//     // 4. find the index of a value with binary_search: it works only if the deque is sorted!!!
//     // 5. check the result: it should be meaningless
//     // 3. sort the deque and check afain the result of binary_search, now it should be meaningful

// }























}