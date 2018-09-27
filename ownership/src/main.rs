// ===== Example 1 =====
// fn main() {
//     let s = String::from("Hello");
//     takes_ownership(s);
//     // println!("{}", s); // ERROR!

//     let x = 5;
//     makes_copy(x);
//     println!("{}", x); // Can Run!
// }

// ===== Example 2 =====
// fn main() {
//     let s1 = String::from("hello");
//     println!("{}", s1);

//     let s2 = gives_ownership();
//     println!("{}", s2);

//     let s3 = takes_and_gives_back(s2);
//     println!("{}", s1);
//     // println!("{}", s2); // ERROR!
//     println!("{}", s3); // Can Run!
// }

// fn takes_and_gives_back(some_string: String) -> String {
//     some_string
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
    
//     some_string
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }


// ===== Example 3 =====
// fn main() {
//     let s1 = String::from("Hello");
//     let (s2, len) = cal_string_length(s1);

//     println!("The length of {} is {}", s2, len);
// }

// fn cal_string_length(some_string: String) -> (String, usize) {
//     let len = some_string.len();

//     (some_string, len)
// }

// ===== Example 4 =====
// fn main() {
//     let s = String::from("Hello");
//     let len = cal_string_length(&s);

//     println!("The length of {} is {}", s, len);
// }

// fn cal_string_length(some_string: &String) -> usize {
//     some_string.len()
// }

// ===== Example 5 =====
// fn main() {
//     let mut s = String::from("Hello");
//     change(&mut s);

//     println!("{}", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", World!");
// }

// ===== Example 6 =====
// fn main() {
//     let mut s = String::from("Hello World");

//     let word = first_word(&s);

//     println!("{}", word);

//     s.clear();
// }

// fn first_word(some_string: &String) -> usize {
//     let bytes = some_string.as_bytes();

//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     some_string.len()
// }

// ===== Example 7 =====
// fn main() {
//     let mut s = String::from("Hello World");
//     let hello = &s[0..5];

//     println!("{}", hello);

//     s.clear(); // ERROR!
// }