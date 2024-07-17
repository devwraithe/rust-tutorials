pub mod ttl {
    fn largest_old(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // fn largest<T>(list: &[T]) -> &T {
    //     let mut largest = &list[0];
    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }
    //
    // fn ttl_plain() {
    //     let number_list = vec![34, 50, 25, 100, 65];
    //     let mut largest = &number_list[0]; // Reference to the first number in the  list
    //     for number in &number_list {
    //         // Iterate through all the numbers in the list
    //         if number > largest {
    //             // If current number is greater than number in largest...
    //             largest = number; // Replace the reference
    //         }
    //     }
    //     println!("The largest number is {largest}");
    // }
    //
    // fn ttl_dynamic() {
    //     let number_list = vec![34, 50, 25, 100, 65];
    //     let result = largest(&number_list);
    //     println!("The largest number is {result}");
    //
    //     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //     let result = largest(&number_list);
    //     println!("The largest number is {result}");
    // }
    //
    // pub fn ttl_global() {
    //     ttl_plain();
    //     ttl_dynamic();
    // }
    //
    // // In struct definitions
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // fn struct_main() {
    //     let integer = Point { x: 5, y: 10 };
    //     let float = Point { x: 1.0, y: 4.0 };
    //     // let wont_work = Point { x: 5.0, y: 4 }; // Won't work cos must be same type
    // }
    //
    // struct NewPoint<T, U> {
    //     x: T,
    //     y: U,
    // }
    //
    // fn struct_main_two() {
    //     let both_integer = NewPoint { x: 5, y: 10 };
    //     let both_float = NewPoint { x: 1.0, y: 4.0 };
    //     let integer_and_float = NewPoint { x: 5, y: 4.0 }; // will work cos of multiple generic type params on NewPoint
    // }
    //
    // // In Enum Definitions
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    //
    // enum Result<T, E> {
    //     // Multiple generics
    //     Ok(T),
    //     Err(E),
    // }
    //
    // // In Method Definitions
    // impl<T> Point<T> {
    //     fn x(&self) -> &T {
    //         &self.x
    //     }
    // }
    //
    // fn method_main() {
    //     let p = Point { x: 5, y: 10 };
    //
    //     println!("p.x = {}", p.x());
    // }
    //
    // /** Traits **/
    // // Defining a trait
    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }
    //
    // // Implementing a trait on a type
    // pub struct NewsArticle {
    //     pub headline: String,
    //     pub location: String,
    //     pub author: String,
    //     pub content: String,
    // }
    //
    // impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }
    //
    // pub struct Tweet {
    //     pub username: String,
    //     pub content: String,
    //     pub reply: bool,
    //     pub retweet: bool,
    // }
    //
    // impl Summary for Tweet {
    //     fn summarize(&self) -> String {
    //         format!("{}: {}", self.username, self.content)
    //     }
    // }
    //
    // // Trait as Parameters
    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    //
    // // pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    // pub fn notify_two<T: Summary>(item: &T) {
    //     // Trait bound
    //     println!("Breaking news! {}", item.summarize());
    // }
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {
    // pub fn notify(item: &(impl Summary + Display)) { // Multiple trait bounds with +
    // pub fn notify<T: Summary + Display>(item: &T) { // Multiple trait bounds with +

    // >>> Clearer trait bounds with 'where' clauses
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug
    // {

    // /** Returning Types that implement Traits **/
    // fn returns_summarizable() -> impl Summary {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }

    // You can only use impl Trait if you’re returning a single type.
    // fn returns_summarizable_multi(switch: bool) -> impl Summary { // This code won't work cos of condition
    //     if switch {
    //         NewsArticle {
    //             headline: String::from(
    //                 "Penguins win the Stanley Cup Championship!",
    //             ),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from(
    //                 "of course, as you probably already know, people",
    //             ),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }

    // /** Validating References with Lifetimes **/
    // Lifetimes ensure that references are valid as long as we need them to be.
    // Lifetime is the scope for which a reference is valid.
}
