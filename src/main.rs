use crate::closures::use_closures;
use crate::error_handling::error_handling;
use crate::hash_maps::working_with_hashmaps;
use crate::iterators::use_iterators;
use crate::smart_pointers::use_smart_pointers;
use crate::types_traits_lifetimes::ttl;
use crate::vectors::working_with_vectors;

mod closures;
mod error_handling;
mod hash_maps;
mod iterators;
mod smart_pointers;
mod types_traits_lifetimes;
mod vectors;

fn main() {
    // data_types();
    // // Functions
    // another_function();
    // // Control Flow
    // control_flow();
    // Ownership
    // ownership();
    // references_and_borrowing();
    // structs();
    // rectangles();
    // value_in_cent(Coin::Quarter(UsState::Alabama));
    // working_with_vectors();
    // working_with_hashmaps();
    // error_handling();
    // ttl::ttl_global();
    // use_closures::build();
    // use_iterators::build();
    use_smart_pointers::build();
}

/* >>> Enums <<< */
enum IpAddrKind {
    V4,
    V6,
}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn test_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_addr_kind: IpAddrKind) {}

enum Message {
    Quit,                       // plain enum variant
    Move { x: i32, y: i32 },    // like a struct
    Write(String),              // like a tuple struct
    ChangeColor(i32, i32, i32), // like a tuple struct
}

// Methods can also be defined on enums like they are on structs
impl Message {
    fn call(&self) { // '&self' basically means use the 'Message' enum
                     // method body would be defined here
    }
}

// The Option Enum
enum Option<T> {
    None,    // Either nothing
    Some(T), // or something
}

// The 'match' control flow
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Atlanta,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn more(coin: Coin) {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("A quarter is equal to 25 cents! And this one is from {state:?}");
            25
        }
    }
}

/* >>> Enums End <<< */

/* >>> Program to calculate rectangle <<< */
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangles() {
    // let width1 = 30;
    // let height1 = 50;
    //
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );
    //
    // let struct_rect = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // println!(
    //     "The area of the rectangle using 'struct' is {} square pixels.",
    //     struct_area(&struct_rect)
    // );
    // println!("rect1 is {struct_rect:?}");

    // From method syntax
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    print!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    // rect1.area() is the method syntax
}

fn area(/*width: u32, height: u32*/ dimensions: (u32, u32)) -> u32 {
    // width * height

    // refactor with tuples
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
/* >>> Program to calculate rectangle <<< */

/* >>> Method syntax <<< */
impl Rectangle {
    // &self is an alias for the impl block type, in this case 'Recctangle'
    fn area(&self) -> u32 {
        // '&self' and 'self: &Self' are the same
        self.width * self.height
    }
}

/* >>> Structs Starts <<< */
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn structs() {
    let mut user1 = User {
        active: true,
        username: String::from("rust_rover"),
        email: String::from("rustrover@gmail.com"),
        sign_in_count: 1,
    };
    println!("user 1 email before: {}", user1.email);
    user1.email = String::from("rustrover1@gmail.com");
    println!("user 1 email after: {}", user1.email);
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("rustrover2@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // achieve with less code
    let user2 = User {
        email: String::from("rustrover2@gmail.com"),
        ..user1 // this must come last to specify the remaining values, also moves the data as user1 cannot be used anymore after creating user2
    };
    println!("user 2 email: {}", user2.email);

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // same as 'username: username'
        email,    // same as 'email: email'
        sign_in_count: 1,
    }
}
/* >>> Structs Ends <<< */

fn another_function() {
    println!("Another function.");

    print_labeled_measurement(5, 'h');
    state_exp();
    let five = five(); // Now this is a statement [I think?]
    println!("Function five: {five}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions - Statements vs Expressions
fn state_exp() {
    let x = 10; // A statement [ends with semicolon]
    let y = {
        let z = x + 20;
        z * 30
    }; // Still a statement
    println!("Y is {y}");
}

fn data_types() {
    // Integars
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Floats
    let a = 1.0;
    let b: f32 = 2.0;

    // Compound types - Tuples
    let tup: (i32, u32) = (-10, 10);
    let (a, b) = tup;
    println!("This is a tuple accessed with destructuring: {a}");
    let unsigned_ten = tup.1;
    println!("This is another tuple accessed with period: {unsigned_ten}");

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // Contains 5 elements which will each be set to 3

    let first = a[0];
    let second = a[1];
}

fn five() -> i32 {
    -5
} // Basically an expression cos it returns something

// Control Flow
fn control_flow() {
    // 'if', 'else if' and 'else' expressions
    let number = 7;
    if number < 5 {
        println!("This condition is met! True!");
    } else if number != 7 {
        println!("This condition is seven");
    } else {
        println!("This condition is not met! False!")
    }

    // Using 'if' in a let statement
    let con = true;
    let numero = if con { 5 } else { 6 };
    println!("The value of number is: {numero}");

    // Repetition with loops
    // Repeating with 'loop'
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels for disambiguation in multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break; // breaks this loop
            }
            if count == 2 {
                break 'counting_up; // breaks the parent loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional loops with while
    let mut whileNum = 3;
    while whileNum != 0 {
        println!("{whileNum}!");
        whileNum -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("Element value: {element}")
    }

    // Conditional loop with for
    for for_num in (1..4).rev() {
        println!("{for_num}");
    }
    println!("LIFTOFF!!!");
}

// Ownership
fn ownership() {
    let s = "hello"; // this variable is only valid within this function
    let v = vec![1, 2, 3];
    // let v2 =v;
    println!("v[0] is : {}", v[0]);

    // The string type
    let new_string = String::from("Hello "); // string created from 'string literal', it can be mutated
    println!("Before mutation: {}", new_string);
    let mut new_string = String::from("Hello Mutated "); // mutated string
    new_string.push_str("Rust"); // append a literal to the mutation
    println!("After mutation: {}", new_string);

    /* Memory and Allocation */
    // Variables and Data Interacting with Move
    let s1 = String::from("hello"); // this is using 5 bytes (length), it receives 5 bytes (capacity) from the allocator for usage
    let s2 = s1; // s1 was moved into s2 and has become invalidated, so it won't work from here on out
    println!("{}", s2.capacity());

    // Variables and Data Interacting with Clone
    let s3 = String::from("hello");
    let s4 = s3.clone(); // despite being moved, the 'clone' method created a duplicate so both s3 and s4 can be used
    println!("s3 = {s3}, s4 = {s4}");

    // Stack-only Data: Copy
    let x = 5;
    let y = x; // There is no need for clone here because types such as integars have sizes known at compile time, also it's stored on the stack as opposed to strings being stored on the heap
    println!("x = {x}, y = {y}");

    // Ownership and functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    // Returns values and scope
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into // scope
    a_string // a_string is returned and moves out to the calling function
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
//     (s, length)
// }

/* >>> References and Borrowing Starts <<< */
fn references_and_borrowing() {
    // The ampersands are references
    let mut s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
    change(&mut s); // mutable reference, the value being borrowed is 'mut s'
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("r2: {}", r2);
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");

    // The Slice Type
    let word = first_word(&s); // word will get the value 5
                               // s.clear(); // this empties the String, making it equal to ""
    println!("word: {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally
    // invalid!
}

fn calculate_length(s: &String) -> usize {
    // s is a ref to a String
    s.len() // s is in scope here
} // Here s goes out of scope but because it does not have ownership of what it refers to, it is not dropped.

fn change(some_string: &mut String) {
    // accept a mutable reference
    // this function will mutate the value borrowed from s1 in 'reference_and_borrowing()'
    some_string.push_str(", rust");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
/* >>> References and Borrowing Ends <<< */
