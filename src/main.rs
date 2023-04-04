// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

fn main() {
    // guess();
    // variables();
    // modifying();
    // shadowing();
    // dataTypes();
    // println!("{}", functions(6))
    // control_flow();
    // println!("{}", fibonacci(6));
    // ownership_fn();
    slice_type();
}

fn slice_type() {
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }
    let s1 = String::from("hello world");
    println!("first: {}", first_word(&s1));
    fn second_word(s: &String) -> (&str, &str) {
        let hello = &s[..6];
        let world = &s[6..];
        (hello, world)
    }
    let (x, y) = second_word(&s1);
    println!("first: {}, second: {}", x, y);
    fn other_slice() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..3];
        println!("res: {}", slice == &[2, 3]);
    }
    other_slice();
}

// fn ownership_fn() {
//     let i = 1;
//     let j = i;
//     println!("i: {i}, j: {j}");
//     let x = String::from("hello");
//     let y = &x;
//     println!("x: {}, y: {}", x, y);
//     let s0 = String::from("world");
//     fn no_dangle_borrow(s: &String) -> &String {
//         &s
//     }
//     let s1 = no_dangle_borrow(&s0);
//     println!("s1: {s1}");
//     let mut s2 = String::from("hi");
//     let bs1 = &s2;
//     let bs2 = &s2;
//     println!("bs1: {bs1}, bs2: {bs2}");
//     let mbs = &mut s2;
//     println!("mbs: {}", mbs)
// }

// fn control_flow() {
//     loop_fn();
//     while_fn();
//     for_fn();
//     fn loop_fn() {
//         let mut num = 0;
//         'outer_loop: loop {
//             println!("num: {}", num);
//             'inner_loop: loop {
//                 if num < 5 {
//                     println!("true");
//                     break 'inner_loop;
//                 } else {
//                     println!("false");
//                     break 'outer_loop;
//                 }
//             }
//             num += 1;
//         }
//     }
//     fn while_fn() {
//         let mut num = 3;
//         while num != 0 {
//             println!("{num}");
//             num -= 1;
//         }
//         println!("LIFTOFF")
//     }
//     fn for_fn() {
//         let numbers = [10, 20, 30];
//         for num in numbers {
//             println!("num: {num}");
//         }
//     }
// }

// fn functions(int_param: i32) -> i32 {
//     let y = {
//         let x = int_param;
//         // expressions do not include ending semicolons
//         x + 1
//     };
//     fn five() -> i32 {
//         5
//     }
//     fn plus_five(x: i32) -> i32 {
//         x + five()
//     }
//     plus_five(y)
// }

// fn dataTypes() {
//     let int32: i32 = 32;
//     let int64: i64 = 64;
//     let float32: f32 = 32.0;
//     let float64: f64 = 64.0;
//     let boolean_type: bool = true;
//     let character_type: char = 'z';
//     let tup: (i32, f64, u8) = (32, 64.0, b'G');
//     // tuple can be destructure
//     let (x, y, z) = tup;
//     let numbers: [i32; 3] = [1, 2, 3];
//     let threes = [3; 3]; // equal to `let threes = [3, 3, 3]`
// }

// fn shadowing() {
//     let x = 6;
//     let x = x + 1;
//     println!("The value of x is: {x}")
// }

// fn modifying() {
//     let x = "   ";
//     let x = x.len();
//     println!("The length of x is: {x}")
// }

// fn variables() {
//     let x = 6;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");
// }

// fn guess() {
//     println!("Guess number!");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {secret_number}");
//     loop {
//         println!("Please input your guess.");
//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read the line.");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("You guessed: {guess}");
//         match guess.cmp(&secret_number) {
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Less => println!("Too small!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
