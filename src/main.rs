use std::{io::{self, ErrorKind, Read}, collections::HashMap, fs::{File, self}, thread, time::Duration, rc::Weak, sync::{mpsc, Mutex, Arc}, fmt};
use rand::{Rng, seq::index};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;

/// documentation
fn main() {
    // guess_game();
    // variables();
    // functions();
    // flow_control();
    // strings();
    // ownership();
    // borrowing();
    // slice();
    // struct_ex();
    // enums();
    // match_ex();
    // modules();
    // collections_ex();
    // ex1();
    // error_handling();
    // generics();
    // traits();
    // lifetimes();
    // tests();
    // closures();
    // iterators();
    // smart_pointers();
    // concurrency();
    // objects();
    // pattern_matching();
    // unsafe_rust();
    // advanced_traits();
    // advanced_types();
    // advanced_functions();
    // macros();
}


// structs are similar to a tuple
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// another example:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// methods:
impl Rectangle {
    // methods always receive &self as its first parameter, 
    // which represents the instance of the struct the method is being called on
    // &self is shorthand for: self: &Self
    // remember to use &mut self if pretending to change something
    // self without & is used to take ownership of the instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // ypu can create a fn with the same name as one of its fields
    fn width(&self) -> bool {
        self.width > 0
    }
    // you can also make getters for example
    
    fn can_hold(&self, rect: &Rectangle) -> bool {
        (rect.width < self.width) && (rect.height < self.height)
    }

    // associated functions (called with, for example, Rectangle::square(5)):
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // associated functions are methods that doesnt use self
    // you usually make constructors like this, calling an associated function new, 
    // but new isnt a keyword so you could name it as you like
}

// you can have more than one impl for the same struct
impl Rectangle {}

// tuple structs are similar to tuples but they have name
// each tuple struct is its own type, so a Point cannot be a Color even if 
// their fields contain the same types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs:
struct AlwaysEqual;
// used to implement traits when you dont have any data that you want to store 


enum IpAddrKind {
    // these are called variants of an enum
    V4,
    V6,
}

enum IpAddr {
    // this works as a constructor, the name V4 becomes a function that receives a String and
    // returns an instance of the IpAddr type
    // you can put any kind of data inside an enum variant
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// the previous enum is similar to:
/*
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/
// this solution with structs is much more complicated, because for example if you want to receive 
// a message in a fn, you would need to pass every struct, instead with enums you just need to pass
// the enum

// you can also implement methods in an enum: 
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn variables() {
    // const:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // mutable variable
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    // shadowing and immutable variable
    let x = 7;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // shadowing with type change
    let wheee = "wheee";
    println!("{wheee}");
    let wheee = wheee.len();
    println!("{wheee}");

    // integer types
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64     u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    //
    // float types are f32 and f64

    // tuples (heap?) and pattern matching (destructuring)
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    // when vars are not used, _ goes
    // before the name, like _x or _one
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    // empty tuple () is called struct 
    
    // arrays (stack)
    let _a = [1, 2, 3, 4, 5];
    // left is type, right is length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; stores [3,3,3,3,3]
    let _first = a[0];
    let _second = a[1];

    // structs
    // immutable struct:
    let _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // mutable struct:
    let mut user2 = User {
        active: true,
        username: String::from("Nakiri"),
        email: String::from("kawayo@wah.com"),
        sign_in_count: 1,
    };
    // to change a single value the entire struct must be mut
    user2.email = String::from("anotheremail@example.com");

    // struct update syntax (both are the same):
    // normal update:
    // let user3 = User {
    //     active: user2.active,
    //     username: user2.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user2.sign_in_count,
    // };
    // struct update:
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    // note that since this is assigning a previous user to user3, is moving the user, 
    // so the previous one is invalidated 
    // (unless the only values changed where the ones that implement the Copy trait, 
    // in this case sign_in_count and active)
    
    // tuple structs initialization:
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit_like struct initialization:
    let _subject = AlwaysEqual;
}

// struct constructor:
fn build_user(email: String, username: String) -> User {
    // this can be returned because is an expression
    User {
        active: true,
        // you can use shorthand initialization when 
        // parameter names are the same as the fields name
        username: username, // shorthand: username, explicit: username: username
        email, // explicit: email: email, shorthand: email
        sign_in_count: 1,
    }
}


fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("type:");
        let mut guess = String::new();
        // read from the terminal
        // and store it in guess
        io::stdin().read_line(&mut guess).expect("a");
        // pattern matching
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {}", guess);
        // pattern matching
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
// you could add verifications to check if the value is negative 
// using i32 instead u32; in such case you would need to check 
// for this case everytime. The solution to this would be 
// creating a new type: 
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
    // getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
// which will guarantee that the value is always positive without having 
// a lot of checks. 


fn flow_control() {
    let number = 3;
    // if
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /* this doesnt work:
    let number = 3;

    if number {
        println!("number was three");
    }
    because number isnt bool
    */

    // else if (could be done with match)
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression, so it is 
    // assignable. this stores 5 in number
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // loops
    loop {
        // continue;
        break;
    }

    // assigning loops result to a variable
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // returns counter
            break counter;
            // break counter * 2 would return 20
        }
    };
    println!("The result is {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // breaks inner loop
                break;
            }
            if count == 2 {
                // breaks outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // conditional loops: while 
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // next code will be slower than for,
    // because compiler adds runtime
    // code to perform the conditional check, 
    // or whether the index is within the bounds 
    // of the array on every iteration through the loop.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        index += 1;
    }
    // so, same with for:
    for element in a {
        println!("the value is: {element}");
    }
    
    // using for as the previously defined while:
    // next code is equal to for number in (1..4).rev() {}
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn strings() {
    // string literal
    let _a = "asdasd";
    // String
    let _s = String::from("asdasd");
    // s is made from the string literal

    // also, strings can be mutated because they are stored in the heap,
    // but string literals cannot
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    
    // continuing with strings:
    // string is a wrapper around Vec<T>
    let mut s = String::new();
    // this creates an empty string 
    let data = "initial contents";
    // converting data string literal to string:
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // or its equivalent:
    let s = String::from("initial contents");

    // since it is UTF-8, all of the following are valid: 
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // pushing data to a string: 
    let mut s = String::from("foo");
    s.push_str("bar");
    // now s has "foobar"

    // push_str doesnt take ownership of the value: 
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    // notice that you still can use s2 after push_str because 
    // this is internally defined as:
    // pub fn push_str(&mut self, string: &str) {
    // notice the use of &str

    // push method concatenates only one char to the string: 
    let mut s = String::from("lo");
    s.push('l');

    // concatenating with + operator: 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // + operator uses the add method, that looks like this: 
    // fn add(self, s: &str) -> String {
    // thats why it receives s1 and a reference to s2, and thats also why 
    // s1 is no longer valid after the usage of + operator (but s2 is)
    // this isnt copying both values and creating a new one, it takes ownership of s1,
    // appends a copy of the contents of s2, and then returns 
    // ownership of the result
    
    // note that you cant concatenate a string to a string, 
    // only an &str, and the previous code compiles because 
    // strings can coerce to &str (deref coercion)

    // this:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = s1 + "-" + &s2 + "-" + &s3;
    // is almost the same as:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{s1}-{s2}-{s3}");
    // but format doesnt take ownership of any of its parameters 
    
    // rust doesnt support string indexing:
    // let s1 = String::from("hello");
    // let h = s1[0];
    // this doesnt compile because not all letters of all alphabets
    // use the same amount of space. For example, "Hola" has len 4, 
    // but "Здравствуйте" doesnt have len 12, its 24, because each character 
    // takes more than 1 byte of space. As this could lead to unexpected 
    // behaviour, rust doesnt let it compile

    // strings from rust perspective (for the word "नमस्ते" in hindi):
    // bytes (u8 values):
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]
    // unicode scalar value (char):
    // ['न', 'म', 'स', '्', 'त', 'े']
    // grapheme clusters:
    // ["न", "म", "स्", "ते"]
    // this means that you doesnt know what the result type of a 
    // string index operation would be , it could be in bytes, chars,
    // grapheme clusters or string slices 
    
    // what you could do is to create a slice of the "index" you want to retrieve 
    // for example: 
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // would store "Зд" in s, because it stores the first 4 bytes of the string 
    // and since each one of the letters takes 2 bytes, only that will be stored 
    // notice that if you slice a letter in half, for example: 
    // &hello[0..1]
    // the program will crush

    // you can explicitly say whether you want chars or bytes: 
    for c in "Зд".chars() {
        println!("{c}");
    }
    // this will return Зд
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // this will return 208, 151, 208, 180
    // to do the same with grapheme clusters is difficult and is not provided 
    // by the standard library 
}


fn ownership() {
    /*
    * this doesnt work:
    * let s1 = String::from("hello");
    * let s2 = s1;
    * println!("{}, world!", s1);
    * s1 moved to s2, so its values owner is now s2
    * is like a shallow copy but invalidating s1, so its called move
    */
    // you should do it like this:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // this works because it copies not only the ptr, the len, and the capacity, 
    // but also the value in the heap, so when it drops s2 and s1 its dropping 
    // different places of memory

    // but that only happens with the values that reside on the heap.
    // if the value is stored in the stack, then, there is no problem with this:
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // this is the case for integers, booleans, chars, unsigned integers, floats, 
    // and tuples when they ONLY have the types above inside it 
    // (types that implement Copy trait)
    
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn borrowing() {
    // next code works fine because of this:
    // https://doc.rust-lang.org/book/img/trpl04-05.svg
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // you cant do:
    let s = String::from("hello");
    change1(&s);
    // because references are immutable

    //instead you should make the reference mutable:
    let mut s = String::from("hello");
    change2(&mut s);

    // you cant reference the same value twice:
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // this returns an error

    // what you can do is:
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;

    // another example:
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);
    // this doesnt work, 
    // because you cannot have a mutable reference 
    // while you have an immutable one to the same value.

    // note: a reference’s scope starts from where it is introduced 
    // and continues through the last time that reference is used
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // rust doesnt compile if there are dangling pointers:
    // let reference_to_nothing = dangle();
    // instead, you should do:
    let _reference = no_dangle();

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change1(_some_string: &String) {
    // some_string.push_str(", world");
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/ 
// dangle fix:
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}


fn functions() {
    // functions
    another_function(5);
    print_labeled_measurement(5, 'h');
    // let x = (let y = 6);
    // this is wrong, because statements
    // such as let doesnt return anything
    // what you could do is:
    let condition = true;
    let _number = if condition { 5 } else { 6 };
    // because if is an expression,
    // and expressions do return something,
    // in this case 5
    
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    // previous code works because
    // {} blocks are considered expressions.
    //
    // x + 1 DOES NOT HAVE A ; AT THE END
    // because is an expression. if 
    // I were to put ;, it will 
    // become a statement
    //
    // functions are alse expressions
    // macros are also expressions

    println!("fn five returns: {}", five());
}


fn slice() {
    // slices are references to a contiguous sequence of elements, 
    // rather than a whole collection
    // find the first word in the string would be 

    // without slices:
    let s = String::from("ateshi minato aqua");
    find_word(&s);

    //slices:
    let hello_world = String::from("hello world");
    let _hello = &hello_world[0..5];
    // this is the same as previous line
    let _slice = &hello_world[..5];
    
    let _world = &hello_world[6..11];
    // is the same as:
    let len = hello_world.len();
    let _slice = &hello_world[3..len];
    // or:
    let _slice = &hello_world[3..];

    // this works like this: https://doc.rust-lang.org/book/img/trpl04-06.svg

    // slices of the entire string:
    let _slice = &hello_world[0..len];
    let _slice = &hello_world[..];

    // now, the previous fn with slices:
    let _a = find_word2(&s);
    // s.clear(); wont work, because when clear truncates the string, it modifies it, 
    // and since it is not mut then it wont work
    println!("{}", _a);

    // STRING LITERALS ARE SLICES, SO THEY ARE OF TYPE &str 
    // they are immutable because &str is immutable

    // you can make slices of other things too, like arrays:
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

fn find_word(s: &String) -> usize { // its better to have &str as a parameter
                                    // because &str accepts both &str and &String
    // converts string to array
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // checks wether the item is equal to the byte that represents the space
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn find_word2(s: &String) -> &str { // its better to have &str as a parameter
                                    // because &str accepts both &str and &String
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // DONT PUT SEMICOLON
    // OR IT WILL BREAK
    x + 1
}


fn struct_ex() {
    let rect = Rectangle {
        width: 30,
        height: 60
    };
    let area_rect = area(&rect);
    println!("{}", area_rect);
    println!("{:?}", rect);
    // this is the same as:
    println!("{}", rect.area());
    // you can also use methods to verify things:
    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // using associated function to create a square:
    let square = Rectangle::square(5);
    println!("{:?}", square);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}


fn enums() {
    // creating an instance of an enum
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum:
    // all of the following work:
    let some_number = Some(5);
    // some without Option works because not only Option enum but 
    // all Options variants are in the prelude (prelude are the
    // standard libraries imported in every rust project without explicit import)
    let some_char: Option<char> = Some('e');
    // you can write the type or not because of type inference
    let absent_number: Option<i32> = None;
    // same as null value
    // if a variable is i32, it cant be null. For it to be null it has to be an optional,
    // and the compiler will detect if youre not evaluating if its null or not
}

fn route(_ip_kind: IpAddrKind) {}


fn match_ex() {
    // in order to use an Option<T> value, you want to have code that will handle each variant. 
    // match can do this.
    // match allows you to compare a value against a series of patterns 
    // and then execute code based on which pattern matches.
    // match ensures every single case is handled
    let coin: Coin = Coin::Penny;
    let coin2: Coin = Coin::Quarter(UsState::Texas);
    value_in_cents(coin);
    value_in_cents(coin2);

    // match with Option
    let five = Some(5);
    let six = match_plus_one(five);
    let none = match_plus_one(None);
    dbg!("six: {}", six);
    dbg!("none: {}", none);

    // catch all arms
    let dice_roll: u8 = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch all value, it goes last because 
        // arms are evaluated in order
        other => move_player(other),
        // you could use '_' instead of other in the first part of the arm
        // if you are not going to do anything with the value:
        // _ => reroll() for example, if it can only be 3 or 7, then reroll,
        // I dont care if I get a 9 because I'm looking for another number
        // you can also use the unit value if you dont want to do anything:
        // _ => ()
    };

    // if let is another way to do pattern matching
    // with match:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // with let: 
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // The code in the if let block isn’t run if the value doesn’t match the pattern.
    
    // you can also do an if let else, with the else being the same as '_' in match:
    // with match: 
    let coin3: Coin = Coin::Quarter(UsState::Alaska);
    let mut _count = 0;
    match coin3 {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => _count += 1,
    }
    // with if let else: 
    let coin4: Coin = Coin::Quarter(UsState::Louisiana);
    let mut _count = 0;
    if let Coin::Quarter(state) = coin4 {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match is similar to an if but it doesnt evaluate just boolean expressions, 
    // just like this example
    match coin {
        // each one of this "if" are called arms 
        // arms has 2 parts, a pattern with the => operator, 
        // and some code that happens when the pattern is found
        // arms are separated with ',' and they return an expression; 
        // match returns the expression of the pattern that matches
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        // the ',' is optional when using "{}"
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn match_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}


fn modules() {
    /*
    Modules Cheat Sheet

    Here we provide a quick reference on how modules, paths, the use keyword, 
    and the pub keyword work in the compiler, and how most developers organize 
    their code. We’ll be going through examples of each of these rules throughout 
    this chapter, but this is a great place to refer to as a reminder of how modules work.

        Start from the crate root: When compiling a crate, the compiler first 
    looks in the crate root file (usually src/lib.rs for a library crate or 
    src/main.rs for a binary crate) for code to compile.

        Declaring modules: In the crate root file, you can declare new modules; 
    say, you declare a “garden” module with mod garden;. The compiler will look for the
    module’s code in these places:
            Inline, within curly brackets that replace the semicolon following mod garden
            In the file src/garden.rs
            In the file src/garden/mod.rs

        Declaring submodules: In any file other than the crate root, you can 
    declare submodules. For example, you might declare mod vegetables; in src/garden.rs.
    The compiler will look for the submodule’s code within the directory named for the
    parent module in these places:
            Inline, directly following mod vegetables, within curly brackets instead of the semicolon
            In the file src/garden/vegetables.rs
            In the file src/garden/vegetables/mod.rs

        Paths to code in modules: Once a module is part of your crate, you 
    can refer to code in that module from anywhere else in that same crate, as long 
    as the privacy rules allow, using the path to the code. For example,
    an Asparagus type in the garden vegetables module would be found at
    crate::garden::vegetables::Asparagus.

        Private vs public: Code within a module is private from its parent
    modules by default. To make a module public, declare it with pub mod instead
    of mod. To make items within a public module public as well, use 
    pub before their declarations.

        The use keyword: Within a scope, the use keyword creates shortcuts
    to items to reduce repetition of long paths. In any scope that can refer
    to crate::garden::vegetables::Asparagus, you can create a shortcut with 
    use crate::garden::vegetables::Asparagus; and from then on you only need 
    to write Asparagus to make use of that type in the scope.
    */
}


fn collections_ex() {
    // I suppose this is included in the prelude
    // a vector is an arraylist
    // contiguous memory
    let v: Vec<i32> = Vec::new();

    // vec! macro is an easier way to create a vec
    let v2 = vec![1, 2, 3];

    // mutable vec
    let mut v3 = Vec::new();

    // push to mut vec
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v = vec![1, 2, 3, 4, 5];

    // accessing the vector:
    // with indexes
    let third: &i32 = &v[2];
    // in this case it will work without & because i32
    // implements Copy trait, so borrowing is not needed
    println!("The third element is {third}");
    // with get method:
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // which one to use depends on how you want the program to behave
    // first one will panic if you try to access an index without value, so if 
    // you want your program to crash when this happens, you use this method
    // the second one will not panic, it will be handled by the logic within match, 
    // so use this one when accessing a nonexistent element could happen in 
    // normal circumstances
    
    // remembering the borrow checker:
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // aside from push, pop deletes and also returns the last element.
    //
    // println!("The first element is: {first}");
    // this line would break the code, because you cant hold an immutable value 
    // of a mut vector, since it could change because reallocations of 
    // memory within the vector, the borrow checker will not let this compile.
    
    // iterating a immutable array:
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // iterating a immutable array:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * is derefence operator
        *i += 50;
    }

    // vectors can only store 1 type, but that type 
    // can be an enum, which could have different types inside:
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let a: &SpreadsheetCell = &row[1];
    // if you delete & it wont work because this enum 
    // doesnt implement Copy trait 
    println!("{:?}", a);
    // this can also be done with trait objects 

    // hash maps (this is not in the prelude, needs to be imported with "use"):
    let mut scores = HashMap::new();
    // inserting in a hash map:
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // where Blue and Yellow are the keys and 10 and 50 are the values
    // keys and values must be of the same type

    // note: the number of key and value pairs in a hash map is growable

    // getting values from the hash map: 
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get method returns an Option<&V> 
    // This program handles the Option by calling copied to get 
    // an Option<i32> rather than an Option<&i32>, 
    // then unwrap_or to set score to zero if scores 
    // doesn't have an entry for the key.

    // iterating over a hash map:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ownership in hash maps: 
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point 

    // overriding a value:
    // If we insert a key and a value into a hash map and then 
    // insert that same key with a different value, the value associated 
    // with that key will be replaced:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // inserting only if not exists (and ignoring if it exists):
    // entry that takes the key you want to check as a parameter. 
    // The return value of the entry method is an enum called 
    // Entry that represents a value that might or might not exist.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    // The or_insert method on Entry is defined to return 
    // a mutable reference to the value for the corresponding 
    // Entry key if that key exists, and if not, inserts the 
    // parameter as the new value for this key and returns a
    // mutable reference to the new value.
    // The first call to entry will insert the key for the 
    // Yellow team with the value 50 because the Yellow team doesn’t 
    // have a value already. The second call to entry will not 
    // change the hash map because the Blue team already has the value 10.

    // inserting only if not exists (and modifying if it exists)
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // split_whitespace method returns an iterator over sub-slices, 
    // separated by whitespace, of the value in text
    // The or_insert method returns a mutable reference (&mut V) 
    // to the value for the specified key. 
    // Here we store that mutable reference in the count 
    // variable, so in order to assign to that value, we must first 
    // dereference count using the asterisk (*). The mutable reference 
    // goes out of scope at the end of the for loop, so all of 
    // these changes are safe and allowed by the borrowing rules
    
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions
    // this mentions how to change hashing fn, and says that by default the hashing 
    // used in hash maps provides resistance to DoS attacks but could be slow
}


fn ex1() {
    // Given a list of integers, use a vector and return 
    // the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; 
    // a hash map will be helpful here) of the list.
    // 
    // create a vec with numbers
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9];
    // calculate median
    let median: i32 = numbers[numbers.len()/2];
    println!("{median}");
    // create empty hash map
    let mut aux: HashMap<i32, u8> = HashMap::new();
    // calculate mode
    for num in numbers {
        // entry checks if the key exists, and or_insert is a method of entry that, 
        // if it not exists, it inserts 0 in it (in this case) 
        // else it wont do anything 
        // in both cases returns a mut ref to the value that has key num (in this case)
        let count = aux.entry(num).or_insert(0);
        // so I increment that value to calculate the mode 
        *count += 1;
    }
    println!("{:?}", &aux);

    // Convert strings to pig latin. The first consonant of each word is moved 
    // to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
    // Keep in mind the details about UTF-8 encoding!
    //
    // try switdching apple with first
    let start_word: String = String::from("apple");
    // I assume that the first letter will be 1 byte
    let first_letter1: &str = &start_word[0..1];
    let length = String::len(&start_word);
    // I grab a slice of word (&str), make it mutable, and change it to string 
    // to use push methods. The reason behind using another var is to not lose 
    // the start_word string
    let mut word: String = start_word[0..length].to_string();
    match first_letter1 {
        "a" | "e" | "i" | "o" | "u" => {
            word.push_str("-hay");
        } 
        _ => {
            word = start_word[1..length].to_string();
            word.push('-');
            word.push_str(first_letter1);
            word.push_str("ay");
        }
    }
    println!("{:?}", word);

    // Using a hash map and vectors, create a text interface to allow a user 
    // to add employee names to a department in a company. For example, 
    // “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user 
    // retrieve a list of all people in a department or all people in the 
    // company by department, sorted alphabetically.
    //

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut option: String = String::new();
        println!("What do you want to do?");
        println!("1 - Add user to department");
        println!("2 - list people in a department");
        println!("3 - list people in the company by department");
        println!("4 - exit");
        io::stdin().read_line(&mut option).expect("a");
        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("type a number");
                continue;
            }
        };
        match option {
            1 => {
                let mut dep: String = String::new();
                println!("insert department: ");
                io::stdin().read_line(&mut dep).expect("a");
                let mut name: String = String::new();
                println!("insert username: ");
                io::stdin().read_line(&mut name).expect("a");
                let users = company.entry(dep).or_insert(vec![]);
                users.push(name);
                users.sort();
            },
            2 => {
                let mut dep: String = String::new();
                println!("which department would you like to see? ");
                io::stdin().read_line(&mut dep).expect("a");
                match company.get(&dep) {
                    Some(users) => println!("{:?}", users),
                    None => println!("Department not found"),
                }
            },
            3 => {
                for user in company.values() {
                    let mut sorted_users = user.clone();
                    sorted_users.sort();
                    println!("{:?}", sorted_users);
                }
            },
            4 => break,
            _ => continue,
        }
    }
}


fn error_handling() {
    // there are two types of errors in rust 
    // recoverable (with Result type) and unrecoverable (with panic!)
    //
    // if a program fails it will panic, and there is also the panic! macro 
    // for example: 
    // fn main() {
    //     panic!("crash and burn");
    // }
    // would result in: 
    // $ cargo run
    //    Compiling panic v0.1.0 (file:///projects/panic)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.25s
    //      Running `target/debug/panic`
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    
    // result type: 
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // for example, this method returns a Result: 
    let greeting_file_result = File::open("hello.txt");
    // handling Result type:
    let _greeting_file: File = match greeting_file_result {
        // returns file, now it is stored in greeting_file
        Ok(file) => file,
        // else panics, enters here if there’s no file named hello.txt in our current directory
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // same with unwrap method: 
    let _greeting_file: File = File::open("hello.txt").unwrap();
    // this will store hello.txt file in greeting_file, 
    // and if the file doesnt exists it will panic 
    // is less verbose than the previous match but less recommended, 
    // unless you use unwrap_or or unwrap_or_else or unwrap_or_default
    // you could also use expect method, that lets you write 
    // the panic message:
    let _greeting_file: File = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    // this will store the file in greeting_file, but if it panics 
    // it will print the text written on it. 
    // it is preferred to use expect instead of unwrap

    // all functions that might fail should return Result type

    // even better Result handling:
    // file_handling();
    // another way to write the same would be: 
    // file_handling2();

    // propagating errors:
    // let res: Result<String, io::Error> = read_username_from_file();
    // now if read username was successfull the username is in res 
    // else res will be an io::Error 
    // now we decide what to do with the error

    // use unwrap or expect when running tests so that it panics when 
    // the test fails 
    // It would also be appropriate to call unwrap or expect when you have some other 
    // logic that ensures the Result will have an Ok value, but the logic isn’t 
    // something the compiler understands.
    // If someone calls your code and passes in values that don’t make sense, 
    // it’s best to return an error, propagate it, and panic if necessary 
    // when failure is expected, return a Result 
    // When your code performs an operation that could put a user at risk 
    // if it’s called using invalid values, your code should verify the values
    // are valid first and panic if the values aren’t valid
}

fn file_handling() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        // File::open returns the struct io::Error inside Err 
        // this struct has a method kind to get the enum io::ErrorKind, 
        // which has variants representing the different kinds of errors
        // that might result from an io operation
        Err(error) => match error.kind() {
            // io::ErrorKind variant when file is not found
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// same as file_handling() but with closures: 
fn file_handling2() {
    let _greeting_file: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// this returns the error to its caller
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // saves the file content inside username
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// another way to do exactly the same with ? operator: 
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// ? operator does almost the same as previous match: 
// if the value is OK it will be returned from the expression 
// if the value is Err it will be returned from the whole function and 
// propagate the error to its caller
// another useful thing about ? is that it calls "from" function from the 
// "From" trait with the error value, which converts the error received into 
// the error that appears in the return statement, so its useful when a fn could 
// return a bunch of different errors, because they will all be converted into the 
// one that is returned 
// 
// you could even do: 
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
// this is the same as the prevoius one 

// ? operator also works with Optional 
// if the value is None, None will be returned early from the fn 
// if the value is Some, it will store Some in the variable and 
// continue with the fn. For example:
/* this has nothing to do with error handling */
fn last_char_of_first_line(text: &str) -> Option<char> {
    // text is a string slice, lines returns an iterator over the 
    // lines of the string. next gets the first value of the iterator, 
    // and will return None from the fn or Some. 
    // if its Some, you can use chars iterator over the chars of the string. 
    // last returns the last item of the iterator
    text.lines().next()?.chars().last()
}
/* this has nothing to do with error handling */
// note: ? operator wont convert a Result into an Option nor viceversa 
// use ok method in Result or ok_or method in Option

// lastly, you could do read_username_from_file it like this: 
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
// this is using the standard library

// main can also return Result type: 
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//
//     Ok(())
// }
// Box<dyn Error> is a trait object that means any kind of error 
// and main will return 0 if ? returns Some and another number if 
// main returns an error
// dyn means dynamic


struct Point2<T> {
    x: T,
    y: T,
}
// when the compiler is going to process this struct, it will 
// create the same struct but with the concrete type that is being 
// used: 
/*
// generic Option<T> will be replaced by:
enum Option_i32 {
    Some(i32),
    None,
}
// and:
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    // because those where the types that are being used here
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
*/
// impl and methods can also be generics
// note that impl has <T> also, that is what tells the compiler 
// that the <T> in point isnt a concrete type 
impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// you could also restrict some methods, so they only can be 
// used if the generic has a certain type: 
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// now Point<f32> will have a distance_from_origin method; 
// other instances of Point<T> where T is not of type f32 
// will not have this method defined

// seems that T and U could be the same type
struct Point3<T, U> {
    x: T,
    y: U,
}

// generics arent slower than concrete types!!!!
// (this is because of monomorphization)
fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point2 { x: 5, y: 10 };
    let float = Point2 { x: 1.0, y: 4.0 };
    // this wont work:
    // let wont_work = Point { x: 5, y: 4.0 };
    // because x and y have to be both T (the same type)
    // you would have to have multiple generics, like this: 
    let both_integer = Point3 { x: 5, y: 10 };
    let both_float = Point3 { x: 1.0, y: 4.0 };
    let integer_and_float = Point3 { x: 5, y: 4.0 };
    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// since not all types T could be are comparable by > operator, you have to restrict it 
// with the trait std::cmp::PartialOrd 
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


use aggregator::{Summary, Tweet, NewsArticle};
fn traits() {
    // traits are similar to interfaces

    // overwritten behaviour:
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // default behaviour:
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    notify(&tweet);
}

// this parameter accepts any type that implements the specified trait
// calling with a type that doesnt implement it wont compile
pub fn notify(item: &impl Summary) {
    // here we can call any Summary method
    println!("Breaking news! {}", item.summarize());
}
// this way of writing it is a simplified version of: 
pub fn notifyG<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// this is called trait bound, and works better in more complex scenarios, 
// for example:
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {}
// this works better if you allow item1 and item2 to be 
// different types, because this doesnt constrain the parameters
pub fn notify3<T: Summary>(item1: &T, item2: &T) {}
// if you want to force item1 and item2 to be the same 
// type, you should do it like this

// if you want a parameter to implement more than one trait: 
pub fn notify4(item: &(impl Summary + Display)) {}
// or:
pub fn notify5<T: Summary + Display>(item: &T) {}

// if there are a lot of trait bounds:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () {}
// you can write it like this:
fn some_function2<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// you can also return a type that implements a trait: 
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
// this returns a tweet, but could return a NewsArticle and it would work, 
// but only one at a time, in this case tweet. For example, this wont work: 
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// this implementation will only be available to the types that 
// implements Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// you can also implement a trait for the types 
// that implement another trait (blanket implementations)
// for example, the standard library implements the ToString trait 
// on any type that implements the Display trait:
/*
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        todo!()
    }
}
// this gives an error because there is no local trait in it
*/


fn lifetimes() {
    // lifetime is the scope for which a variable is valid
    // references have lifetimes
    // in life1() r's lifetime is longer than x's, and 
    // because of that r results as a dangling pointer 
    // the borrow checker is the rust tool that checks lifetimes 
    // in this case r ('a) has a bigger lifetime than x ('b) and is 
    // referencing it, so it wont compile
    // life2() will compile 
    
    // longest() will not compile, because the fn doesnt know if the 
    // reference in the result type is a reference to x or to y 
    // for it to compile you must tell it the lifetime of x and y 
    
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
    // One lifetime annotation by itself doesn’t have much meaning

    // longest2() is valid because there are lifetime annotations
    let a = longest2("wah", "kira kira koseki bijou");
    println!("{}", a);

    // Note that the longest function doesn’t need to know exactly 
    // how long x and y will live, only that some scope can be substituted for 
    // 'a that will satisfy this signature.

    // annotations go in the function signature, not in the function body.
    // The lifetime annotations become part of the contract of the function

    // the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y

    let string1 = String::from("kira kira koseki bijou");
    {
        let string2 = String::from("wah");
        let result = longest2(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }
    // this will work

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    // this wont work because result has a reference to string2, 
    // but string2 gets dropped before using result, because the 
    // compiler cannot tell that string1 is larger than string2

    // if a function that takes x and y as paramenters only returns x, 
    // then it is not necessary to explicitly define y's lifetime:
    let a = longest3("x", "y");
    println!("{}", a);

    // lifetime paramenter for the return type needs to be the same as 
    // one of the parameters, because if it returns a reference to 
    // something created inside it, it will get dropped when going back 
    // to its caller, creating a dangling pointer:
    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
    // result goes out of scope when the fn returns 

    // you can create a struct with a reference inside, in which case 
    // you will need to add a lifetime to the reference:
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // as long as first_sentence isnt dropped, it will compile. Else, you have to 
    // be sure that i gets dropped before dropping first_sentence or the code will break 

    // some lifetimes can be predictable by the compiler, so in those cases lifetimes 
    // arent necessary (lifetime elision). For example in method find_word2() 
    // originally, find_word2 declaration would have been: 
    // fn find_word2<'a>(s: &'a str) -> &'a str {} 
    // but now it is not needed in this case

    // lifetimes on function or method parameters are called input lifetimes 
    // lifetimes on return values are called output lifetimes
    // The compiler uses three rules to determine :
    // first: 
    // the compiler assigns a lifetime parameter to each parameter that’s a reference
    // a function with one parameter gets one lifetime parameter:
    // fn foo<'a>(x: &'a i32);
    // a function with two parameters gets two separate lifetime parameters:
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
    // second: 
    // if there is exactly one input lifetime parameter,
    // that lifetime is assigned to all output lifetime parameters:
    // fn foo<'a>(x: &'a i32) -> &'a i32.
    // third: 
    // if there are multiple input lifetime parameters, but one of 
    // them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters
    // examples:
    //
    // first example: find_word2() 
    // fn find_word2(s: &str) -> &str {
    // Then the compiler applies the first rule, which specifies that each 
    // parameter gets its own lifetime.
    // fn find_word2<'a>(s: &'a str) -> &str {
    // The second rule applies because there is exactly one input lifetime. 
    // The second rule specifies that the lifetime of the one input parameter 
    // gets assigned to the output lifetime
    // fn find_word2<'a>(s: &'a str) -> &'a str {
    // now it can compile 
    // second example: longest()
    // fn longest(x: &str, y: &str) -> &str {
    // applying the first rule: each parameter gets its own lifetime.
    // This time we have two parameters instead of one, so we have two lifetimes:
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    // but since there are more than one input lifetime the second rule doesnt apply 
    // and since it is not a method it does not have self, so the third rule doesnt apply 
    // Since the result type still doesnt know its lifetime, the compiler throws an error 
    
    // there is another type of lifetime, that stays for the whole program: 'static 
    let _s: &'static str = "I have a static lifetime.";
    // all string literals are 'static 
    // this can be because is stored inside the program's binary
}

/*
fn life1() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
*/

fn life2() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
// We want the signature to express the following constraint: 
// the returned reference will be valid as long as both the parameters are valid.
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// this lifetime annotation in tehe impl is required every time 
// you use a reference inside it 
impl<'a> ImportantExcerpt<'a> {
    // here we are not required to put lifetimes because of the first rule
    fn level(&self) -> i32 {
        3
    }
    
    // in this case the third rule applies: this function returns &self's lifetime 
    // Rust applies also the first lifetime elision rule and gives both &self and 
    // announcement their own lifetimes 
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// everything: generic, trait bounds and lifetimes 
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn tests() {
    // Each test is run in a new thread, and when the main thread sees
    // that a test thread has died, the test is marked as failed
    // this means tests shouldnt depend on each other nor should have 
    // any shared state nor shared environment 
    // to not run tests in parallel (I think they mean concurrently) run:
    // cargo test -- --test-threads=1 

    // every values used in an assert must implement the PartialEq 
    // and the Debug traits; all primitive types and most of the 
    // std library implements those, but structs and enums defined 
    // by the developer must implement those too by adding:
    // #[derive(PartialEq, Debug)]

    // Result types in tests can be good because it lets you use 
    // the ? operator, which will return an Err if any operation 
    // within them fails

    // you cant use #[should_panic] in a test that returns Result 
    // To assert that an operation returns an Err variant, 
    // don’t use the question mark operator on 
    // the Result<T, E> value. Instead, use assert!(value.is_err())

    // std output is captured by Rust's test library, so it isnt printed 
    // to show printed values, run: 
    // cargo test -- --show-output
    
    // to run all ignored tests, run: 
    // cargo test -- --ignored

    // to run a specific test, run cargo test and its name 
    // a substring also works, for example: 
    // cargo test gre 
    // will run greeting_contains_name and 
    // greater_than_100

    // Rusts recommends doing unit and integration tests
    // unit tests go on the same file as the code they are testing 
    // so integration tests doesnt need #[cfg(test)], since they should 
    // be in another directory

    // to test a private fn you only need to bring it to scope 

    // integration tests go into tests folder
    // integration tests are in aggregator
    
    // to run only one integration test file, run: 
    // cargo test --test integration_test

    // cargo detects the folder named tests so when you run cargo test 
    // its files get tested. this is why the setup is inside a folder 
    // named common; if not the setup would end up being tested too
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// #[cfg(test)] tells the compiler to run the tests only 
// when running cargo tests, and not on cargo build
#[cfg(test)]
mod tests {
    // since some structs are not public like Rectangle, 
    // we need to bring them to scope 
    // the glob (glob is *) is used to bring everything 
    // in the outer module (super)
    use super::*;

    // #[test] annotation tells you that this is a test
    #[test]
    fn it_works() {
        let result = 2 + 2;
        // assert_eq is the same as passing annotation 
        // expression (with ==) to an assert 
        // this works different than expect and actual, 
        // since here they are called left or right and 
        // it doesnt matter which one is which
        assert_eq!(result, 4);
        // for instance this code is the same as: 
        // assert_eq!(4, result);
        // there is also assert_ne (with !=)
    }
    #[test]
    // tests can also return Result type, in this case the test 
    // will pass if the fn returns Ok and fails if it returns Err 
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    // #[ignore] ignores the test
    #[ignore]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // assert macro calls panic macro
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // custom error message:
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    /*
    #[test]
    // #[should_panic] means that the test fails if it doesnt panic
    // if the code panics but the message of the panic is not the same 
    // as the one in expected this test will fail, even if it panics 
    // (EXPECTED CAN BE A SUBSTRING OF THE ERROR MESSAGE)
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    */
}


fn closures() {
    // basically anonymous functions 

    // giveaway uses a closure: 
    let store = Inventory { shirts: vec![Shirt::Red, Shirt::Blue, Shirt::Red] };
    let user_preference: Option<Shirt> = Some(Shirt::Blue);
    let giveaway1 = store.giveaway(user_preference);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference, giveaway1
    );
    let user_preference: Option<Shirt> = None;
    let giveaway1 = store.giveaway(user_preference);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference, giveaway1
    );

    // closures doesnt need to have an explicit return type nor parameter type, 
    // because it wont be part of a contract, meaning that 
    // it wont be in the interface the user will be seeing, 
    // it will be used internally
    // although the compiler can infer those types, you can make them explicit:  
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // other syntax for closures:
    // this one is not a closure, is a function
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // these are closures:
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    // spaces arent necessary

    // add_one_v3 and add_one_v4 needs to be used 
    // for the compiler to infer its type:
    let moai = add_one_v3(2) + add_one_v4(3);
    // without this line the code wont compile, 
    // similar to let v = Vec::new(), needing 
    // either type annotations or values to grab the 
    // type of

    // this code wont compile:
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    // because of s, the compiler thinks example_closure returns 
    // a string, so it gives x the type string, and since 5 is 
    // not a string, it gives an error

    // closures can infer if the parameters are borrowed or not, 
    // depending what the closure does in its body
    // borrowing immutably:
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // borrowing mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    
    // if needed you can add the keyword move to the 
    // closure to force it to take ownership of its 
    // parameters
    // this is used when creating threads, more 
    // precisely to pass data from a thread to another 
    // (for example if the main thread has a parameter 
    // move will make the new thread the owner of that parameter)
    //
    // this takes ownership:
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // in this case you need to move the list, because if the 
    // thread that originated the list finishes before the new thread, 
    // the reference would become invalid if it still has ownership, 
    // so moving it into the closure is needed

    // depending on how the closure handles values from the environment, 
    // the trait it will implement will be different:
    // FnOnce: takes ownership of the environment. Can be called only once
    // FnMut: borrows mutably
    // Fn: borrows immutably or doesnt borrow at all
    // the compiler will infer which trait to implement depending on 
    // how the closure uses the environment

    // for example, unwrap_or_else takes a closure that implements FnOnce
    /*
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
    */
    // and calls it if the option is None

    // it isnt necessary to call a closure in those functions, 
    // you could do for example: 
    // unwrap_or_else(Vec::new) 
    // to get a new, empty vector if the value is None 

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
        // sort_operations.push(value);
        // r.width
    // });
    // this wont work because sort_by_key takes ownership of 
    // value, and since sort_by_key is called multiple times, 
    // the second time it will try to use value, it will be 
    // already dropped, so it wont work 
    // to fix this, the closure cant move the value out of 
    // the environment: 
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug, Copy, Clone)]
enum Shirt {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<Shirt>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<Shirt>) -> Shirt {
        // this is a closure without parameters 
        // (parameters go inside ||)
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> Shirt {
        let mut red = 0;
        let mut blue = 0;
        for shirt in &self.shirts {
            match shirt {
                Shirt::Red => red += 1,
                Shirt::Blue => blue += 1,
            }
        }
        if red >= blue {
            Shirt::Red
        }
        else {
            Shirt::Blue
        }
    }
}


fn iterators() {
    // iterator pattern allows you to perform some operation 
    // on a sequence of items in turn 
    // iterators are lazy, they dont do anything until you 
    // call methods that consume the iterator to use it up, 
    // for example: 
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // this doesnt do anything 
    // now v1_iter is an iterator that goes through the 
    // elements of v1 
    // now you can use it like this: 
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iterators implement a trait called Iterator, which 
    // looks like this: 
    /*
    pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
    }
    */
    // the Item associated type is the one that is returned in 
    // the next method, and the next method is the only 
    // method that needs to be implemented 
    // More precisely, the next method returns an Option<T>, 
    // containing one item of the iterator at a time, and 
    // when there are no more items, it returns None
    // test iterator_demonstration() shows this behaviour

    // note that you dont have to make the iterator mutable 
    // when you use a for loop, because the for loop takes 
    // ownership of the iterator and makes it mutable internally 

    // the values that the next method returns are immutable references 
    // to the values in the original collection, but if you 
    // want the iterator to take ownership of the values and 
    // return owned values, you can use the into_iter method instead 
    // of iter 
    // you can also use iter_mut to iterate over mutable references 

    // methods that call next are called consuming adaptors, 
    // because they use up the iterator by consuming it, for 
    // example in the test iterator_sum()

    // apart from the consuming adaptors, there are other methods that 
    // dont consume the iterator called iterator adaptors
    // instead of consuming the iterator, they produce a new iterator 
    // that changes some aspect of the original iterator, for example 
    // map() takes a closure and calls it on each element, producing 
    // a new iterator that contains the results of the closure calls 

    let v1: Vec<i32> = vec![1, 2, 3];
    println!("before map: {:?}", v1);

    // collect() method consumes the iterator and collects the 
    // values inside it into a collection
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("after map{:?}", v2);

    // example of a closure that takes ownership of its environment: 
    let v1: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        }
    ];
    let in_my_size = shoes_in_size(v1, 10);
    println!("shoes in my size: {:?}", in_my_size);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    
    // note that v1_iter is a mutable reference, because 
    // iterators keep track of where they are in the sequence,
    // so calling next changes the internal state of the iterator
    let mut v1_iter = v1.iter();
    // this code "consumes", or uses up, the iterator 

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // after this call you cant use v1_iter anymore, 
    // since it has been consumed by sum(), and even 
    // if it wasnt consumed, the code would break 
    // because sum() takes ownership of the iterator

    assert_eq!(total, 6);
}


use std::rc::Rc;
use std::cell::RefCell;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}
#[derive(Debug)]
enum ListRcRefCell {
    ConsRcRefCell(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    NilRcRefCell,
}
use crate::List::{Cons, Nil};
use crate::ListRc::{ConsRc, NilRc};
use crate::ListRcRefCell::{ConsRcRefCell, NilRcRefCell};
fn smart_pointers() {
    // smart pointers are pointers that have additional metadata,
    // for example String and Vec<T> are smart pointers 
    
    // unlike references that only borrows data, smart pointers 
    // own the data they point to

    // smart pointers are usually implemented as structs that 
    // implement the Deref and Drop traits. The Deref trait 
    // allows an instance of the smart pointer struct to behave 
    // like a reference, and the Drop trait is like a destructor 

    // most commonly used smart pointers:
    // Box<T> for allocating values on the heap
    // Rc<T> a reference counting type that enables multiple ownership
    // Ref<T> and RefMut<T> accessed through RefCell<T>, a type that 
    // enforces the borrowing rules at runtime instead of compile time 

    // Box<T> is a pointer in the stack that points to data in the heap 
    // Box<T> is useful when you have a type whose size cant be known, 
    // when you have data that you want to transfer ownership but 
    // dont want to copy it,
    // or when you want to own a value and you care only that it is a 
    // type that implements a particular trait 
    //
    // the second case is convenient when transferring ownership of a 
    // large amount of data, because the compiler doesnt have to 
    // copy the data on the stack, only the pointer on the stack, and 
    // the data in a box in the heap 
    // third case is a trait object, and first case is called a 
    // recursive type 

    // to store something on the heap you can wrap it in a box:
    let b = Box::new(5);
    println!("b = {}", b);

    // a value of recursive type can have another value of the same 
    // type as part of itself. Since this nesting of values could 
    // continue infinitely, Rust doesnt know how much space a 
    // recursive type needs. However, boxes have a known size, 
    // so by inserting a box in a recursive type definition, 
    // you can have recursive types
    // recursive type example:
    // cons list: (1, (2, (3, None))) 
    // each element in the list has a value and a pointer to the 
    // next element (lisp's definiton of a linked list)
    // in rust, this would be: 
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // instead of enum List
    // since box is a pointer, it doesnt matter how big the value in the heap is, 
    // rust only has to calculate the size of the pointer on the stack


    //deref: 
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // * is deref operator, it follows the reference to the value 
    // it points to, so *y is the value stored in x
    assert_eq!(5, *y);
    // next code is almost the same but with box<T>: 
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // box copies the value in x, so if you change x the box will 
    // remain the same, unlike the first code, in which y is a 
    // reference to x, so if you change x, y will change too

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // in this code, rust substitutes *y with *(y.deref())


    // deref coercion:
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // this is possible because rust converts a reference to a type 
    // that implements the Deref trait into a reference to another type
    // if it werent for deref coercion, we would have to write: 
    // hello(&(*m)[..]);
    // where *m dereferences the MyBox<String> into a String, 
    // and then & and [..] take a string slice of the String that 
    // is equal to the whole string 

    // deref coercion rules: 
    // from &T to &U when T: Deref<Target=U>
    // from &mut T to &mut U when T: DerefMut<Target=U>
    // from &mut T to &U when T: Deref<Target=U>
    // this means that if you have a ref to an immutable value, 
    // you can coerce it to a ref to an immutable value, 
    // if you have a ref to a mutable value, you can coerce it to 
    // a ref to a mutable value, and if you have a ref to a mutable 
    // value, you can coerce it to a ref to an immutable value 
    // (because you can have only one mutable ref to a value in a, 
    // but you cant guarantee the same for immutable values, and you 
    // cant coerce only one ref to an immutable value to a mutable value
    // because it could change the value of another immutable ref, 
    // basically, only the first is true because ownership rules)


    // drop trait:
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // drop is called automatically when a value goes out of scope, 
    // so rust doesnt let you call it explicitly, because it would 
    // result in a double free error (your call and the automatic call)
    // even so, you can disable automatic drop, which can be useful for example 
    // when using a mutex, you may want to force drop method that releases 
    // the lock so that another thread can acquire the lock
    // to do this, you can use std::mem::drop (it is in the prelude):
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");


    // there are some times where you want to have multiple owners of the same data, 
    // for example in a graph with a node that has multiple edges 
    // pointing to it, and for example you cant make the cleanup of the node until 
    // it doesnt have any edges pointing at it and so has no owners
    // for that you can use Rc<T> (reference counting), which keeps track of the 
    // number of references to a value, and determines whether or not a value
    // is still in use (this solution only works in single-threaded code)
    // for example, this wont work:
    /*
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
    */ 
    // because a is moved into b, it cant be used to create c 
    // to solve this, you can use Rc<T> (remember to add use std::rc::Rc;)):
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    let b = ConsRc(3, Rc::clone(&a));
    let c = ConsRc(4, Rc::clone(&a));
    // difference between a.clone and Rc::clone(&a) is that a.clone 
    // makes a deep copy of the data, but Rc::clone only increments 
    // the reference count, so it is more efficient

    // almost the same code as before but printing the number of references to a:
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // Rc::strong_count returns the number of references list a has
    // (there is also a weak_count method)


    // RefCell<T> uses interior mutability, which means that you can 
    // mutate the value inside the RefCell<T> even when the RefCell<T>
    // is immutable, using unsafe code inside the RefCell<T> data structure 

    // in Box<T> and references the borrowing rules are enforced at compile time, 
    // but in RefCell<T> they are enforced at runtime, so if you break them the 
    // program will panic and exit 

    // only works in single-threaded code 

    // recap:
    /*
    * Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    * Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only 
      immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows 
      checked at runtime. 
    * Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value 
    * inside the RefCell<T> even when the RefCell<T> is immutable.
    */
    
    // RefCell<T> has methods borrow and borrow_mut, which returns a smart pointer  Ref<T> and 
    // a smart pointer RefMut<T> 
    // Note that you still need to respect the borrowing rules when using RefCell<T>, because having 
    // two mutable references will not result in a compile error, but in a panic at runtime

    
    // you can combine Rc<T> and RefCell<T> to have multiple owners of mutable data
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsRcRefCell(Rc::clone(&value), Rc::new(NilRcRefCell)));

    let b = ConsRcRefCell(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ConsRcRefCell(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Mutex<T> is the thread-safe version of RefCell<T> 


    // memory leaks: 
    // Rust allows memory leaks by using Rc<T> and RefCell<T>:
    // it’s possible to create references where items refer to each other in a cycle.
    // This creates memory leaks because the reference count of each item in the 
    // cycle will never reach 0, and the values will never be dropped.

    // to avoid this, you can use Weak<T>, which is a weak reference, which doesnt 
    // have ownership of the value, and their count doesnt need to reach 0 for the 
    // value to be dropped. What this means is that the value could be dropped while 
    // there are references to it, so before using a weak reference you need to check 
    // if the value still exists, using the method upgrade, which returns an Option<Rc<T>>
    // with Some if the value still exists, and None if it doesnt
    // to create a weak reference, you can use the method Rc::downgrade, passing a reference 
    // to the Rc<T> you want to create a weak reference to 
    // Weak references still counts the number of weak references to the value; calling 
    // Rc::downgrade increases the weak reference count

    // tree data structure:
    let leaf = Rc::new(Node1 {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node1 {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    // now leaf has two owners: branch and leaf 
    // now you can get from branch to leaf with branch.childen, but not from leaf to branch
    // if you want leaf to be able to see its father, one option that could be used is Rc<T>, 
    // but this would create a memory leak, because when out of scope leaf wont be dropped because 
    // branch is pointing to it, and branch wont be dropped because leaf is pointing to it. 
    // In this case, a parent should own its childen, and if a parent is dropped, its children 
    // should be dropped too. However, a child should not own its parent: if you drop a child, 
    // the parent should still exist. For this you can use Weak<T> instead of Rc<T> 

    let leaf = Rc::new(Node {
        value: 3,
        // parent doesnt exists yet, so we use Weak::new()
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        // branch doesnt have parents
        parent: RefCell::new(Weak::new()),
        // branch has leaf as its child
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // now we assign a weak reference from leaf to its parent (branch, that now exists)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // now leaf has a weak reference to branch, and branch has a strong reference to leaf
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// this wont compile because the compiler doesnt know how much 
// space the type will need:
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// this is to see how to implement deref in a type:
// tuple struct (named tuple)
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    // associated type
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // returns a reference to the value we want to access with the deref operator
        // (.0 is the first element of the tuple)
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}
// Drop trait is included in the prelude, so you dont have to import it
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // cleanup code
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node1 {
    value: i32,
    children: RefCell<Vec<Rc<Node1>>>,
}
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn concurrency() {
    // threads in rust are 1:1 threads (1 os thread per rust thread)
    // to create a new thread, you use the function thread::spawn:
    println!("first example:");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // thread::sleep stops the execution of the thread for the specified time 

    // when the main thread ends, all the threads it spawned end too, even if they havent finished 
    // their execution. To wait for a thread to finish, you can use the method join, which returns
    // a Result<T, E> with Ok if the thread finished successfully, and Err if it panicked 
    // (if the thread panics, the panic will be propagated to the main thread)

    // thread::spawn returns a JoinHandle<T>, which is an owned value that, when we call join on it, 
    // will wait for its thread to finish. 
    
    println!("second example:");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join on the handle blocks the thread currently running until 
    // the thread represented by the handle terminates
    handle.join().unwrap();


    println!("third example:");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main thread waits...
    handle.join().unwrap();

    // main thread enters to for loop after spawned thread has finished
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // if you need to use a variable from the main thread in the spawned thread,
    // you can use the move keyword to move the ownership of the variable to the thread: 

    println!("fourth example:");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // if you dont use the move keyword, the variable might be dropped before the thread ends, 
    // which would make the reference invalid, thus resulting in a compilation error 

    println!("fifth example:");
    // a channel is a way of communicating messages between threads. A channel has two parts: 
    // a transmitter and a receiver (tx and rx). The tx part is used to send messages, and the rx 
    // part is used to receive messages. To create a channel, you use the function mpsc::channel,
    // which returns a tuple with the transmitter and the receiver:
    let (tx, rx) = mpsc::channel();
    // mpsc stands for multiple producer, single consumer (multiple threads can send messages, 
    // but only one can receive them)
    
    // move tx to the spawned thread
    thread::spawn(move || {
        let val = String::from("hi");
        // tx sends the value to the rx in the main thread
        tx.send(val).unwrap();
    });

    // recv (short for receive) blocks the main thread until a value is sent through 
    // the channel, and returns a Result type
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // apart from recv, there is also try_recv, which doesnt block the thread, and returns 
    // a Result type with Ok if a value was received, and Err if not. This is useful if you 
    // want to do something else while waiting for a value to be received, so you 
    // could write a loop that calls try_recv every so often, handles a message
    // if one is available, and otherwise does other work for a little while until 
    // checking again.

    // after you send a value through a channel, you cannot use it anymore, for example:
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     val is sent to the main thread
    //     tx.send(val).unwrap();
    //     now you cant use val in this thread anymore, because it was moved to the main thread
    //     println!("val is {}", val);
    //     this print will result in an error
    // });
    // outside of the thread, the rx takes ownership of val

    // you can send multiple values through a channel: 
    println!("sixth example:");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx is being used as an iterator
    for received in rx {
        println!("Got: {}", received);
    }

    // you can also clone the transmitter to send multiple values from multiple threads:
    println!("seventh example:");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    
    // shared-state concurrency
    // is like multiple ownership, in contrast with message passing, which is like single ownership 
    
    // this shared-state can be achieved with mutex 

    let m: Mutex<i32> = Mutex::new(5);
    {
        // access the data inside a mutex with lock
        // num is a smart pointer of type MutexGuard, wrapped in a LockResult, which implements 
        // Deref to point to our inner data, and also implements Drop to release the lock 
        // automatically when a MutexGuard goes out of scope
        // in other words, the lock method call returns a smart pointer that wraps the data and
        // stores it in num
        let mut num = m.lock().unwrap();
        *num = 6;
    } // lock release
    // now m is 6
    println!("m = {:?}", m);

    /*
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            // counter moved inside the thread
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // since counter is not in the main thread anymore, this will break
    println!("Result: {}", *counter.lock().unwrap());
    */

    // to solve this, we can try to use Rc<T> to wrap the Mutex<T> in order to have multiple owners
    /*
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    */
    // this still wont work, because Rc<T> is not safe to use in concurrent situations 
    // (since it doesnt implement Send trait, it is not thread safe)

    // to solve this problem, we can use Arc<T> (atomic reference counting), which is the 
    // atomic version of Rc<T>, which is safe to use in concurrent situations.
    // this comes with a performance penalty, so thats why Rc<T> is still used in single-threaded 
    // situations

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    // now this works as intended

    // notice that counter is immutable, but we obtained a mutable reference to the value in it,
    // this is because the Mutex<T> type provides interior mutability, as the RefCell<T> type does
    // mutex and arc is like refcell and rc

    // remember that mutex could cause deadlocks 
    

    // to create a concurrency feature, you can use Sync and Send traits 
    // Send trait indicates that the ownership of the type implementing Send can be transferred 
    // from one thread to another 
    // all primitive types (except raw pointers) are Send, and types composed entirely of Send 
    // types are automatically Send as well 
    // the same happens with sync, primitives are Sync and types composed entirely of Sync types 
    // are automatically Sync as well 
    // a type is Sync if it is safe to be referenced from multiple threads, in other words, 
    // &T is Sync if T is Sync (the reference of a Sync can be sent to another thread safely 
    // with Send)
    
    // since types that are made up of Send and Sync traits are automatically Send and Sync, 
    // implementing Send and Sync manually is unsafe, and thus involves using unsafe rust 
}


fn objects() {
    // you can make objects with struct and methods with impl like AveragedCollection
    // you can make encapsulation with pub (everything else is private)
    // for example AveragedCollection, the only way to access or modify its data is 
    // by calling one of its methods (add, remove, or average)
    
    // rust doesnt have inheritance 
    // what you can do is implement a trait for a struct, and then use that trait as a type 
    // for the struct, like the Summary trait and the NewsArticle and Tweet structs 
    // you can also have default implementations, and override them if you want for example 
    // with the summarize method 
    // to use polymorphism in rust, you can use generics to abstract over different possible types 
    // and trait bounds to impose constraints; that is called bounded parametric polymorphism 

    // instead of inheritance, rust has trait objects
    // a trait object points to both an instance of a type implementing our specified trait as well 
    // as a table used to look up trait methods on that type at runtime 
    // we can create a trait object by specifying some sort of pointer, such as a & reference or 
    // a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait 
    // an example would be screen struct defined below 

    // one difference between generics with trait bounds and trait objects is that 
    // a generic type parameter can only be substituted with one concrete type at a time, 
    // whereas trait objects allow for multiple concrete types to fill in for the trait object 
    // at runtime

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // as opposed to generics, that are monomorphized (static dispatch) at compile time,
    // Rust uses dynamic dispatch when working with trait objects. This means that Rust 
    // uses the pointers inside the trait object to know which method to call at runtime,
    // which results in a performance penalty, because of the lookup that it has to do, 
    // and also because the compiler cant optimize the code as much as it could with static dispatch
    // this is why you should use generics and trait bounds when possible

    // state pattern example using trait objects:
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // the same example can be made using enums to represent states instead of trait objects
    // this approach isnt very rusty (doesnt take advantage of rusts strengths); a 
    // better approach would be: 
    let mut post = Post2::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // components is a vector of boxes that hold any type that implements the Draw trait 
    // this is a trait object
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// state pattern example using trait objects: 
pub struct Post {
    // state is an option because when the state changes, we want to move the state out of the 
    // struct, but a struct doesnt let you have an unpopulated field, so we use an option even 
    // if it has None as its value
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // this wont panic because the state will always contain a some value, even if the 
        // compiler isnt aware of it, so its safe to use unwrap here
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        // take method is what takes ownership of the state value, leaving None in its place 
        // this approach is better than just changing the state value in place, because it 
        // ensures that the Post cant use the old state value after we've transformed it into 
        // a new state (because we've moved the old state value into the new state value)
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// rusty approach
pub struct Post2 {
    content: String,
}
pub struct DraftPost {
    content: String,
}
impl Post2 {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}


fn pattern_matching() {
    // where can patterns be used? 
    //
    // match arms: 
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }
    // for example: 
    let x = Some(5);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    //
    // if let expressions:
    // its a shorter way to write a match that only matches one case 
    // and it can have an else case which is the same as the _ case in a match 
    // for example:
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    // 
    // if let can also introduce shadowed variables 
    // the line if let Ok(age) = age introduces a new shadowed age variable that contains the 
    // value inside the Ok variant. This means we need to place the if age > 30 condition 
    // within that block: we can’t combine these two conditions into if let Ok(age) = age && age > 30.
    // The shadowed age we want to compare to 30 isn’t valid until the new scope starts with the curly bracket.
    //
    // lastly, if let isnt exhaustive, so it could lead to bugs if you forget to handle a case 
    //
    // while let loops:
    // while let allows a while loop to run for as long as a pattern continues to match
    // for example: 
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // loop continues until stack.pop() returns None, which happens when the stack is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 
    // for loops: 
    // in a for loop, the value that directly follows the keyword for is a pattern, not a variable name 
    // for example: 
    let v = vec!['a', 'b', 'c'];
    // (index, value) is a pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    // 
    // let statements: 
    // let statements are patterns too! 
    // let PATTERN = EXPRESSION;
    // for example: 
    let x = 5;
    // to see it more clearly, you could use a let to destructure a tuple like this: 
    let (x, y, z) = (1, 2, 3);
    // Rust compares the value (1, 2, 3) to the pattern (x, y, z) and sees that the value matches the pattern,
    // so Rust binds 1 to x, 2 to y, and 3 to z
    // this for example: 
    // let (x, y) = (1, 2, 3); 
    // wouldnt work because the pattern and the value dont have the same shape 
    // 
    // function parameters: 
    // function parameters are also patterns 
    // for example: 
    fn foo(x: i32) {
        // code goes here
    }
    // the parameter x is a pattern that matches any value of type i32 
    // you could even destructure a tuple in a function parameter like this: 
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
    // The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.
    // the same can be done with closures

    
    // patterns come in two forms: refutable and irrefutable 
    // patterns that will match for any possible value passed are irrefutable 
    // for example:
    let x = 5;
    // x matches any value, so this is irrefutable
    // patterns that can fail to match for some possible value are refutable 
    // for example: 
    let a_value: Option<i32> = None;
    // this wont match
    if let Some(x) = a_value {
        println!("{}", x);
    }
    // because some_option_value could be None, which wouldnt match Some(x)
    let a_value: Option<i32> = Some(5);
    // this will match
    if let Some(x) = a_value {
        println!("{}", x);
    }
    // functions, let statements, and for loops can only accept irrefutable patterns 
    // if let and while let accept refutable and irrefutable patterns, but 
    // the compiler warns against irrefutable patterns because by definition they're 
    // intended to handle possible failure (the functionality of a conditional is in its
    // ability to perform differently depending on success or failure. it doesnt make 
    // sense to use an irrefutable pattern that will always succeed as a conditional)
    // for example, this doesnt compile: 
    // let Some(x) = some_option_value;
    // because if some_option_value is None, the Some(x) pattern will fail to match, which 
    // means that the pattern is refutable, but let only accepts irrefutable patterns 
    // what you CAN do is using if let: 
    if let Some(x) = a_value {
        println!("{}", x);
    }
    // you can also use while let with an irrefutable pattern:
    if let x = 5 {
        println!("{}", x);
    };
    // this will trigger a warning, but it will compile 
    // (it doesnt make sense though)
    // i could be useful for example in a match statement, because you would use 
    // a refutable pattern in every match arm, except for the last one, which would 
    // be an irrefutable pattern that catches any remaining values 


    // pattern syntax: 
    // matching literals: 
    // you can match literals directly 
    // for example: 
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // this is the catchall arm 
        _ => println!("anything"),
    }
    // This syntax is useful when you want your code to take an action if it 
    // gets a particular concrete value.
    //
    // matching named variables:
    // named variables are irrefutable patterns that match any value. 
    // They have a complication when used in match expressions because they 
    // shadow outer variables
    // for example:
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // this y is a new variable, not the outer y, 
        // so y now has x's value (Some(5))
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
    // to use the outer y, we would need to use a match guard 

    // multiple patterns: 
    // you can match multiple patterns using the | syntax (or operator)
    // for example:
    let x = 1;
    match x {
        // this matches 1 or 2
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        // this is the catchall arm 
        _ => println!("anything"),
    }

    // matching ranges of values with ..=:
    // the ..= operator allows you to match on an inclusive range of values 
    // for example:
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    // If x is 1, 2, 3, 4, or 5, the first arm will match 
    // another way to write the same would be with 1 | 2 | 3 | 4 | 5, 
    // ranges can only be used with numeric values or char values, because 
    // the compiler checks that the range isnt empty at compile time 
    // and the only types Rust can tell if a range is empty or not are those two 
    // for example: 
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    // destructuring structs: 
    // for example: 
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // now a is 0 and b is 7
    assert_eq!(0, a);
    assert_eq!(7, b);

    // this is the same as: 
    let Point { x, y } = p;
    // you can only do this if var names (x and y) are the same as the 
    // field names in the struct
    assert_eq!(0, x);
    assert_eq!(7, y);

    // let Point { x: x, y: y } = p; 
    // creates two new variables x and y, and assigns the value of p.x to x and 
    // the value of p.y to y (create x and y that match the pattern of p)
    
    // you can also destructure structs with literal values:
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    // the first arm only creates an x variable, and its positioned on the x axis 
    // the second arm only creates a y variable, and its positioned on the y axis 
    // the third arm creates both x and y variables, and its positioned on neither axis 

    
    // destructuring enums: 
    // for example:
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
    // for the enum variants that doesnt hold any data (for example Quit), you 
    // cant destructure them, you can only match the on the literal Message::Quit value,
    // and no variables are created in that arm
    // for struct-like enum variants (for example Move), you can destructure 
    // the data inside the enum variant, and create variables for each piece of data, in 
    // this case x and y 
    // for tuple-like enum variants (for example Write), that holds only one value, 
    // and Message::ChangeColor that holds three values, the pattern is similar to 
    // the pattern specified when matching tuples (the number of variables in the pattern
    // must match the number of values in the variant)


    // destructuring nested structs and enums:
    // for example: 
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
    // even if two enums are involved, you can still destruct them in one match expression 


    // destructuring structs and tuples:
    // complex destructuring example:
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    // this creates four variables: feet, inches, x, and y 

    // ignoring values in a pattern: 
    // _ will match any value but not bind to it 
    // can be used inside a match as a catchall, but also in function parameters for 
    // example:
    fn foo2(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    // (since it is a pattern, it can be used anywhere a pattern can be used)
    foo2(4, 5);
    // this will ignore the value 4 and only use 5 
    // it can be used when implementing a function defined in a trait, but you dont 
    // need one of the parameters declared in its signature 

    // ignoring parts of a value with a nested _: 
    // you can use _ inside of another pattern to ignore just part of a value 
    // for example: 
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    // here it ignores the value inside the Some variant, but it still matches on the 
    // Some variant, so the match arm will execute 
    // you can also use _ in multiple places, to ignore particular values: 
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // ignoring an unused variable by starting its name with an underscore: 
    // for example: 
    let x = 5;
    let _y = 10;
    // this will not throw a warning about unused variable _y 
    // note that this is different from the _ in a pattern, because _ at the start 
    // of a variable name will bind the value to the variable, whereas _ wont 
    // for example, this will throw an error:
    /*
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
    */
    // this will throw an error because s will be moved to _s, so you cant use s 
    // again 
    // that is different than this: 
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
    // this will not throw an error because s is not moved into _ 


    // ignoring remaining parts of a value with ..: 
    // you can use .. to ignore the remaining parts of a value that you dont need, 
    // is like using _ for each value you dont need, but shorter: 
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }
    // this is easier than writing Point2 { x, y: _, z: _ }
    // another example:
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}")
        }
    }
    // .. must be unambigous, so you cant do this: 
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //    (.., second, ..) => {
    //        println!("Some numbers: {second}")
    //    }
    // }


    // extra conditionals with match guards:
    // a match guard is an additional if condition specified after the pattern in a match arm 
    // for example: 
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {x}"),
        Some(x) => println!("{x}"),
        None => (),
    }
    // the first arm will only match if the value inside the Some variant is less than 5
    // match guards are useful for expressing more complex ideas than a pattern alone allows 
    // 
    // there is no way to express a condition within a pattern, so match guards are the only 
    // way to do that, but when match guards are involved, match is no longer exhaustive

    // match guards can be used to solve the problem of shadowing variables inside match arms:
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
    // The pattern in the second match arm doesn’t introduce a new variable y that 
    // would shadow the outer y, meaning we can use the outer y in the match guard. 
    // instead of doing Some(y), which would have shadowed the outer y, we specify 
    // Some(n), which creates a new variable n that doesnt shadow the outer y 

    // if you use | operator in a match arm, and you also put a match guard on that arm, 
    // the match guard will apply to all patterns in that arm 
    // for example:
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    // in this case, if y applies to 4, 5 and 6 
    // the precedence works like this: (4 | 5 | 6) if y => ... 

    // @ bindings: 
    // the at operator (@) lets us create a variable that holds a value at the same time 
    // we’re testing that value to see whether it matches a pattern 
    // for example: 
    enum Message3 {
        Hello { id: i32 },
    }
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {id_variable}")
        }
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => {
            println!("Found some other id: {id}")
        }
    }
    // this will print "Found an id in range: 5"
    // the id_variable @ part of the pattern means "bind the value that matched the 
    // range 3..=7 to the variable id_variable, so we can use it in the code associated 
    // with this arm of the match 
    // the syntax id: id_variable @ 3..=7 is how you can use @ in a pattern to 
    // bind to a variable the value that matched the pattern 
    // this is useful when matching against a complex value and we want to avoid 
    // repeating that value in the code we execute for this arm 
}


fn unsafe_rust() {
    // unsafe rust is rust with superpowers 
    // when the compiler cant tell wether your code is safe or not, it will reject it 
    // even if its valid; unsafe rust is like telling the compiler: 
    // "Trust me, I know what I’m doing". 
    // also, given that hardware is inherently unsafe, you wouldnt be able to perform 
    // some tasks without unsafe rust, such as interacting with the OS or even 
    // writing a new OS 
    // it allows you to do 5 things that you cant do in safe rust: 
    // 1. dereference a raw pointer 
    // 2. call an unsafe function or method 
    // 3. access or modify a mutable static variable 
    // 4. implement an unsafe trait 
    // 5. access fields of unions
    // unsafe rust still has the borrow checker and other safety checks, the only 
    // thing it does is giving you access to the things mentioned above, which 
    // then arent checked by the compiler 
    
    
    // in unsafe rust you can create raw pointers 
    // they can be mutable or immutable and are written as *const T and *mut T, 
    // where T is the type the pointer points to 
    // the asterisk isnt the dereference operator, is part of the type name. 
    // immutable in the context of raw pointers means that the pointer cant be 
    // directly assigned to after being dereferenced 
    // raw pointers: 
    // 1. are allowed to ignore the borrowing rules by having both immutable and 
    //   mutable pointers or multiple mutable pointers to the same location
    // 2. arent guaranteed to point to valid memory
    // 3. are allowed to be null
    // 4. dont implement any automatic cleanup
    // an example of raw pointers:
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // as you can see, you can create raw pointers in safe rust, what you cant 
    // do is dereference them outside an unsafe block 
    // in this example the pointers are valid, but you cant assume they are 
    // indeed valid, take for example this:
    let address = 0x012345usize;
    let r = address as *const i32;
    // this is a valid raw pointer, but it points to a random location in memory 
    // and we cant know if that location is valid or not 

    // to dereference a raaw pointer: 
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // creating a pointer does no harm, its only when we try to access the value 
    // that it points to that we might end up with an invalid value 
    
    // the use of raw pointers is needed for example when calling C code from rust 


    // unsafe functions: 
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
    // you can only call an unsafe function from within an unsafe block
    // if not the compiler will give you an error 
    // bodies of unsafe functions are effectively unsafe blocks, so you dont 
    // need to add another unsafe block inside the function 


    // creating a safe abstraction over unsafe code: 
    // for example: 
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // the split_at_mut method is safe, but internally it uses unsafe code 
    // it takes one slice and makes it two by splitting the slice at the index 
    // given as an argument
    // if you try to write this with safe rust, you will end up with something 
    // like this: 
    /*
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        assert!(mid <= len);
        (&mut values[..mid], &mut values[mid..])
    }
    */
    // this is fundamentally ok, because when you split a slice in two, you 
    // are taking two different parts of the slice that doesnt overlap, so 
    // the two slices are valid, but the compiler cant know that, it can 
    // only know that youre borrowing from the same slice twice, which is 
    // not allowed 
    // to solve this, you can use unsafe rust to implement the function: 
    // split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) 
    //
    // note that the function isnt unsafe, and you can call it from safe rust, 
    // even if internally it uses unsafe rust 
    
    // but for example: 
    let address = 0x01234usize;
    let r = address as *mut i32;
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // attempting to use values will result in undefined behavior
    // the compiler cant know if the pointer is valid or not 
    

    // extern functions: 
    // extern funcions are functions that call external code outside the rust 
    // codebase, for example C standard library functions 
    // external functions are always unsafe 
    // for example:
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    
    // you can also call rust functions from other languages, for example C 
    // for example: 
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // mangling is a process that the compiler does in which it changes the 
    // name of a function to a different name that contains more information 
    // about it 
    // in this case you need to disable it for it to be able to be called from 
    // another language 

    // accessing or modifying a mutable static variable: 
    // a static variable is a variable with the static lifetime, which means 
    // that the variable can live for the entire duration of the program 
    // basically, a global variable 
    // it is declared with the static keyword, like HELLO_WORLD variable, declared 
    // outside this function 
    // it can be used like this: 
    println!("name is: {}", HELLO_WORLD);
    // static variables can be problematic because of ownership rules 
    // (for example, if you have two threads that are accessing the same 
    // mutable static variable, it can cause a data race)

    // static variables are similar to constants, but there are a few differences: 
    // 1. static variables have a fixed address in memory, constants dont 
    // 2. static variables can be mutable, constants cant 
    // if you want to mutate a static variable, you need to use unsafe rust 
    // for example: 
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    

    // unsafe traits: 
    // a trait is unsafe when at least one of its methods has some invariant 
    // that the compiler cant verify 
    // for example: trait Foo 
    //  If we implement a type that contains a type that is not Send or Sync,
    //  such as raw pointers, and we want to mark that type as Send or Sync,
    //  we must use unsafe. Rust can’t verify that our type upholds the 
    //  guarantees that it can be safely sent across threads or accessed 
    //  from multiple threads


    // accessing fields of unions: 
    // unions are similar to structs, but only one of the fields can be used 
    // at a time. They are used when interfacing with unions in C code
}

// you can declare static variables outside a function
static HELLO_WORLD: &str = "Hello, world!";

use std::slice;
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            // The function slice::from_raw_parts_mut is unsafe because it 
            // takes a raw pointer and must trust that this pointer is valid
            slice::from_raw_parts_mut(ptr, mid),
            // The add method on raw pointers is also unsafe, because it must 
            // trust that the offset location is also a valid pointer
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// note the mut keyword to make it mutable
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}


fn advanced_traits() {
    // specifying placeholder types in trait definitions with associated types: 
    // for example:
    pub trait IteratorEx {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    // the Item type is a placeholder type, and the next method is using it 
    // in its signature 
    // this is useful because it allows you to define a trait that uses some 
    // types without needing to know exactly what those types are until the 
    // trait is implemented 
    // this is similar to generics
    // for example: 
    pub trait IteratorExG<T> {
        fn next(&mut self) -> Option<T>;
    }
    // the difference is that with generics, you can implement a trait for a 
    // type multiple times, each time with a different concrete type for the 
    // generic type parameters 
    // with associated types, each type can be implemented only once for each 
    // trait 
    // this means that associated types are a bit less flexible than generics, 
    // for example, with generics you could have Vec<i32> and Vec<u8>, but 
    // with associated types you would have something like: 
    struct Rock;
    trait Moai {
        type Rocky;
        fn moaify(&mut self) -> i32;
    }
    impl Moai for Rock {
        type Rocky = i32;
        fn moaify(&mut self) -> i32 {
            5
        }
    }
    // and now you can only call moaify on rock with i32, not with other types. 
    // its like part of the contract, when implementing the trait, you will 
    // imlement it for one type (to clarify, you can have multiple associated
    // types in a trait, but you can only implement the trait once for each)


    // default generic type parameters and operator overloading:
    // you can specify a default concrete type for a generic type parameter. 
    // this eliminates the need for implementors of the trait to specify a 
    // concrete type if the default type works 
    // this is useful for example with operator overloading, in which you 
    // customize the behavior of an operator such as + in particular situations 
    // you cant create a new operator but you can overload the existing ones 
    // in the std::ops, for example: 
    use std::ops::Add;
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct PointOpOv {
        x: i32,
        y: i32,
    }
    impl Add for PointOpOv {
        type Output = PointOpOv;
        fn add(self, other: PointOpOv) -> PointOpOv {
            PointOpOv {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        PointOpOv { x: 1, y: 0 } + PointOpOv { x: 2, y: 3 },
        PointOpOv { x: 3, y: 3 }
    );
    // now if you have two instances of PointOpOv, you can add them together 
    // with the + operator to create a new instance of PointOpOv 
    // 
    // the default generic type for this code is inside Add trait: 
    // trait Add<RHS=Self> {
    //    type Output;
    //    fn add(self, rhs: RHS) -> Self::Output;
    // }
    // RHS=Self is the syntax for default type paramenter 
    // if you dont specify a concrete type for RHS when implementing Add, 
    // it will default to Self, which will be the type we are implementing 
    // Add on
    // an example that doesnt use default type parameter would be: 
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }


    // fully qualified syntax for disambiguation:
    // if you have a trait with a method name that is the same as another 
    // trait, you can specify which one you want to use 
    // for example:
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    // if you call the fly method, it will call the one in Human by default 
    // because it is more specific than the ones in the traits 
    let person = Human;
    // this will print *waving arms furiously*
    person.fly(); // calls Human::fly() by default

    // to call the fly method from the traits, you can use: 
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person); // same as person.fly()
    
    // this works because fly accepts a self parameter, but associated 
    // functions that are not methods dont have a self parameter 
    // in this case, you need to use the fully qualified syntax to specify 
    // which one you want to call 
    // for example:
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    // this will print spot, and if you want to call the baby_name from the 
    // Animal trait like this:
    // println!("A baby dog is called a {}", Animal::baby_name()); 
    // it will give an error because it doesnt accept a self parameter 
    // and there could be another implementation of the trait, so it doesnt 
    // know which one to call 
    // to fix this, you can use the fully qualified syntax: 
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // this will print puppy, because it is calling the implementation of 
    // Animal for Dog, and not the one for Dog itself
    // the syntax for fully qualified syntax is <Type as Trait>::function(receiver_if_method, next_arg, ...); 


    // supertraits:
    // when a trait requires another trait to be implemented, the required 
    // trait is called a supertrait of the trait with the requirement 
    // for example: 
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    // in this case, OutlinePrint requires Display to be implemented 
    // this means that any type that implements OutlinePrint must also 
    // implement Display 
    // now if you try to implement OutlinePrint on Point, it will give an 
    // error because Point doesnt implement Display 
    // to fix this, you can implement Display on Point: 
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    // now you can implement OutlinePrint on Point:
    impl OutlinePrint for Point {}


    // newtype pattern to implement external traits on external types: 
    // the orphan rule states that you're only allowed to implement a trait 
    // of either the trait or the type is local to your crate 
    // you can get around this restriction by using the newtype pattern, which
    // involves creating a new type in a tuple struct 
    // the tuple struct will have one field and be a thin wrapper around the 
    // type you want to implement a trait for 
    // then the wrapper type is local to your crate, and you can implement 
    // the trait on the wrapper (there is no runtime penalty for using this)
    // for example, if you want to implement Display on Vec<T>, you normally 
    // cant because Display and Vec<T> are defined outside of your crate 
    // but you can create a wrapper type around Vec<T> and implement Display 
    // on the wrapper type 
    // for example:
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // one problem with this is that Wrapper is a new type, so it doesnt 
    // have the methods of Vec<T> available 
    // to fix this, you can implement Deref on Wrapper to return a reference 
    // to the inner type 
    // another option would be implement all Vec methods on Wrapper directly
    
    // another thing you could do with the newtype pattern is to hide the 
    // details of the implementation of a type, for example: 
    struct Person(HashMap<i32, String>);
    // Person struct would be part of the public API, so that the code using 
    // the struct wouldnt need to know that the data is stored in a hash map 
    // this way, you can change the internal implementation without changing 
    // the public API 
    // this is a lightweight way to achieve encapsulation 
}


fn advanced_types() {
    // type alias: 
    // type keyword lets you give an existing type another name 
    // for example: 
    type Kilometers = i32;
    // now Kilometers is a synonym for i32 
    // for example, if a fn asks for an i32, you can pass a Kilometers instead
    // type aliases are useful when you want to reduce repetition, 
    // for example if you have a long type like this: 
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // you can use a type alias to reduce the repetition:
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| println!("hi"))
    }
    fn takes_short_type(f: Thunk) {
        // --snip--
    }
    fn returns_short_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }
    // type aliases are also commonly used with Result<T, E>
    // for example:
    // std::io module has a type named io::Result<T> 
    // which is a type alias for the Result<T, io::Error> type:
    // type Result<T> = std::result::Result<T, std::io::Error>;
    // this means that you can use Result<T> in your code instead of 
    // writing out Result<T, std::io::Error> every time
    // for example: 
    // pub trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //     fn flush(&mut self) -> Result<(), Error>;
    //
    //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    // }
    // would result in: 
    // pub trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //     fn flush(&mut self) -> Result<(), Error>;
    //
    //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    // }


    // the never type: 
    // there is a type called empty type in type theory because it has no 
    // values. This type is called in rust the never type (!), and is used to 
    // indicate that a function will never return
    // for example:
    /*
    fn bar() -> ! {
        // --snip--
    }
    */
    // this is called a diverging function because it never returns 
    // (it never returns because you cant create a variable of type !)
    //
    // another important thing about the never type is taht it can be coerced 
    // into any other type 
    // for example, a match expression must return the same in every arm, 
    // so if one arm has a value of type u32 and another arm has a value of 
    // type !, the type of the match expression is u32 
    // for example, in a loop:
    loop {
        // match expression: 
        let guess: u32 = match 3 {
            3 => 3,
            _ => continue,
        };
        break
    }
    // this works even if continue isnt of type u32, because continue has a 
    // ! value, which can be coerced into u32, so the match assumes it is an u32 
    // and when it tries to assign continue to guess, it will never actually 
    // be assigned, because continue is !, so guess wont have a value 

    // panic! macro also uses the never type
    // another expression of type ! is loop


    // dynamically sized types and the Sized trait: 
    // dynamically sized types (DSTs) are types whose size cant be known at 
    // compile time, for example str (not &str)
    // this means you cant create an str variable like this: 
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // because rust doesnt know how much memory to allocate for s.
    // this is because every instance of a type should use the same amount of 
    // space, but for example s1 takes 12 bytes and s2 takes 15 bytes 
    // meaning it is not possible to create the variable 
    // this is why we use &str, because the size of the address of the str plus its 
    // length is known at compile time, more precisely, it is twice the length of 
    // a usize
    // this means, that if you want to have a dynamically sized type, you must 
    // put it behind a pointer of some kind 
    // for example, you can use Box<str> or Rc<str>
    // another example of a DST (dynamically sized types) is a trait, every trait 
    // is a DST, and this is why when you want to use a trait object, you must 
    // put it behind a pointer, such as:
    // &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).


    // the Sized trait:
    // Sized trait determines wether or not a type's size is known at compile time 
    // this trait is automatically implemented for everything whose size is known 
    // at compile time 
    // Also, rust adds a bound on Sized to every generic function 
    // for example:
    fn generic<T>(t: T) {
        // --snip--
    }
    // is actually:
    fn generic_expressed<T: Sized>(t: T) {
        // --snip--
    }
    // this means that by default, generic functions will only work on types 
    // whose size is known at compile time 
    // but if you want to also work with dynamically sized types, you can use 
    fn generic_relax<T: ?Sized>(t: &T) {
        // --snip--
    }
    // and it means that T may or may not be Sized
    // note that T is now &T, because if T is not Sized, it must be behind a 
    // pointer, so you take a reference to it (or a Box or Rc)
}


fn advanced_functions() {
    // function pointers: 
    // you can pass a function as an argument to another function 
    // functions coerce to fn type, which is a pointer to a function 
    // called function pointer 
    // for example:
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    
    // unlike closures, fn is a type rather than a trait, so we specify fn as 
    // the parameter type directly rather than declaring a generic type parameter 
    // with one of the Fn traits as a trait bound 
    // function pointers implement all three of the closure traits (Fn, FnMut, FnOnce) 
    // so you can always pass a function pointer as an argument for a function 
    // that expects a closure 

    // As an example of where you could use either a closure defined inline or a named function, 
    // you could use map to see how it works: 
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // or:
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // the first one uses a closure, the second one uses a function pointer 
    // note that you have to use the fully qualified syntax here because there 
    // are multiple functions available with the name to_string 

    // another interesting thing is that enum variants also becomes initializer functions 
    // so you can use them as function pointers that implement 
    // the closure traits 
    // for example: 
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // this is equivalent to: 
    let list_of_statuses: Vec<Status> = (0u32..20).map(|i| Status::Value(i)).collect();


    // returning closures:
    // closures are represented by traits, which means you cant return closures 
    // directly, because you cant return traits directly 
    // for example, this wont work:
    // fn returns_closure() -> Fn(i32) -> i32 {}
    // this is because the size of the return value isnt known at compile time 
    // so you must use a trait object:
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    // this works because Box is a pointer, so its size is known at compile time 
}


fn macros() {
    // there are two kinds of macros in rust: declarative macros with macro_rules! 
    // and three kinds of procedural macros: custom derive, attribute-like, and function-like 

    // macros are a way of writing code that writes other code, which is known as 
    // metaprogramming 
    // one advantage of using macros instead of functions is that macros can 
    // take a variable number of parameters 
    // for example, println! can take: 
    println!("hello, world"); 
    println!("The answer is {}", 42); 
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // another thing is that macros are expanded before the compiler interprets 
    // the meaning of the code, which means that macros can do things that functions 
    // cant, such as implementing a trait on a given type 
    // a function cant do that because is called at runtime, but traits needs to be
    // implemented at compile time
    // however, macros are more complex than functions 
    // another difference is that you need to declare macros before you use them, 
    // but with functions you can declare them anywhere and use them anywhere 


    // declarative macros with macro_rules! for general metaprogramming: 
    // these macros are created using the macro_rules! macro 
    // they work like a match, where the value is the literal rust source code 
    // passed to the macro, the patterns are compared with the structure of
    // that source code, and the code associated with each pattern, 
    // when matched, replaces the code passed to the macro 
    // for example: 
    let v: Vec<u32> = vec![1, 2, 3];
    // this macro creates a vector with 3 integers 
    // We could also use the vec! macro to make a vector of two integers or 
    // a vector of five string slices.
    // a simplified version of the vec! macro would look like this: 
    #[macro_export]
    macro_rules! vecMac {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    // the actual definition of vec!  includes code to preallocate the correct 
    // amount of memory, which is an optimization not needed for this example 
    // the #[macro_export] annotation indicates that this macro should be made 
    // available whenever the crate in which the macro is defined is brought 
    // into scope 
    // first thing we have in the body of the macro definition is similar to 
    // a match expression; there is one arm with the pattern 
    // $( $x:expr ),* => {...} 
    // if this pattern matches, the code inside the curly brackets will be 
    // emitted 
    // this is a simple macro with only one arm, but it could have more. 
    // if the pattern doesnt match, the macro will result in an error 

    // explaining of ($($x:expr),*):
    // first, we have a set of parentheses around the whole pattern 
    // $ is used to declare a variable that will contain the rust code 
    // matching the pattern 
    // the inner parentheses are used to capture values that match the pattern 
    // within the parentheses for use in the replacement code 
    // within $() is $x:expr, which matches any rust expression and gives the 
    // expression the name $x 
    // the comma following $x:expr indicates that a literal comma separator 
    // character could optionally appear after the code that matches the 
    // code in $()
    // the * indicates that the pattern matches zero or more of whatever precedes 
    // the * 

    // When we call this macro with vec![1, 2, 3];, the $x pattern matches three
    // times with the three expressions 1, 2, and 3.
    
    // inside the arm, we have a let statement that creates a new, empty vector 
    // to hold the values that are passed into the macro 
    // the next line starts with $(), which indicates that the next lines 
    // are repeated for each $x that matched previously 
    // the line temp_vec.push($x); is the code that is emitted for each 
    // time the pattern matches, and the $x is replaced with each 
    // expression matched 
    
    // this macro generates a code like this:
    // {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);
    //     temp_vec
    // }
    

    // procedural macros for generating code from attributes: 
    // procedural macros accept some code as an input, operate on that code, 
    // and produce some code as an output rather than matching against patterns 
    // and replacing the code with other code as declarative macros do 
    // procedural macros come in three flavors, and they all work in a similar way 
    // the three kinds are custom derive macros, attribute-like macros, and 
    // function-like macros 

    // when creating a procedural macro, the definitions must reside in their 
    // own crate with a special crate type (this might be not needed in the future) 
    
    // an example of a procedural macro could be: 
    // use proc_macro;
    // #[some_attribute]
    //     pub fn some_name(input: TokenStream) -> TokenStream {
    // }
    // with some_attribute being a placeholder for the different kinds of 
    // procedural macros 
    // the input and output of the function are both TokenStreams; 
    // the input is the raw source code that the user passes in, and the output 
    // is the source code that the macro should emit 


    // custom derive macros: 
    // details inside crate hello_macro 
    // and crate hello_macro_derive
    // execution is in pancakes/src/main.rs
    // they only work on structs and enums


    // attribute-like macros: 
    // these macros are similar to custom derive macros, but instead of generating 
    // code for the derive attribute, they allow you to create new attributes 
    // for example: 
    // #[route(GET, "/")]
    // fn index() {}
    
    // the signature of the macro definition looks like this: 
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
    // the two arguments it takes are the GET, "/" and the function definition, 
    // in this case fn index() {}


    // function-like macros:
    // function-like macros define macros that look like function calls
    // they are more flexible than functions, because they can take an unknown
    // number of arguments
    // they are similar to macro_rules! macros, but instead of matching against 
    // patterns, they are expanded into tokens 
    // an example of a function-like macro would be: 
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
    // the definition of the macro would look like this:
    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {}
    // this macro would parse the input as an SQL statement and check that it's 
    // syntactically valid at compile time, which is much more complex 
    // processing than declarative macros can do
}


