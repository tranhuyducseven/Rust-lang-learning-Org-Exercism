# This is my note for learning Rust lang from org
## Chapter 1
### 1.3 Cargo
- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## Chapter 2 Guessing game

## Chapter 3 Common Programming Concepts
### 3.1 Variables and Mutability
***Shadowing***

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be `immutable` after those transformations have been completed.

The other difference between `mut` and `shadowing` is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number

### 3.2 Data types

**_Scalar types_**

Rust has four primary scalar types: **integers**, **floating-point numbers**, **Booleans**, and **characters**
- Integer types

  | Length | Signed | Unsigned |
  | ----------- | ----------- | ----------- |
  | 8-bit | i8 | u8 |
  | 16-bit | i16 | u16 |
  | 32-bit | i32 | u32 |
  | 64-bit | i64 | u64 |
  | 128-bit | i128 | u128 |
  | arch | isize | usize |
  
- Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

**Compound Types**

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

- Tuples: A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

- Array: 

### 3.3 Function

View at https://learning-rust.github.io/

### 3.4 Operator
- Type Casting Operator:

  `let a = 15;`
  
  `let b = (a as f64) / 2.0; //7.5`
  
- String concatenation:
  ```rust
      let (s1, s2) = ("some", "thing"); // both &str
      // All bellow codes return `String`; something

      let s = String::from(s1) + s2; // String + &str

      let mut s = String::from(s1); // String
      s.push_str(s2); // + &str

      let s = format!("{}{}", s1, s2); // &str/String + &str/String

      let s = [s1, s2].concat(); // &str or String array
  ```

### 3.5 Control flow
- match
  - ex1:
  ```rust
        let tshirt_width = 20;
        let tshirt_size = match tshirt_width {
          16 => "S", // check 16
          17 | 18 => "M", // check 17 and 18
          19 ..= 21 => "L", // check from 19 to 21 (19,20,21)
          22 => "XL",
          _ => "Not Available",
        };
        println!("{}", tshirt_size); // L

  ```
  - ex2:
  ```rust
        let marks_paper_a: u8 = 25;
        let marks_paper_b: u8 = 30;
        let output = match (marks_paper_a, marks_paper_b) {
          (50, 50) => "Full marks for both papers",
          (50, _) => "Full marks for paper A",
          (_, 50) => "Full marks for paper B",
          (x, y) if x > 25 && y > 25 => "Good",
          (_, _) => "Work hard"
        };

        println!("{}", output); // Work hard

  ```
- loop
  - **loop**
  - **while**
  - **for**
  > can set label for loop
  
  

## Chapter 4 Beyond the Basic
### 4.1 Vector
- Create empty vector
  ```rust
        let mut a = Vec::new(); //1.With new() keyword
        let mut b = vec![]; //2.Using the vec! macro        
  ```
- With data types
  ```rust
        let mut a2: Vec<i32> = Vec::new();
        let mut b2: Vec<i32> = vec![];
        let mut b3 = vec![1i32, 2, 3];//Suffixing 1st value with data type

        let mut b4 = vec![1, 2, 3];
        let mut b5: Vec<i32> = vec![1, 2, 3];
        let mut b6  = vec![1i32, 2, 3];
        let mut b7 = vec![0; 10]; //Ten zeroes
  ```
- Access and change data
  ```rust
        //Accessing and changing existing data
        let mut c = vec![5, 4, 3, 2, 1];
        c[0] = 1;
        c[1] = 2;
        //c[6] = 2; Cannot assign values this way, index out of bounds
        println!("{:?}", c); //[1, 2, 3, 2, 1]

        //push and pop
        let mut d: Vec<i32> = Vec::new();
        d.push(1); //[1] : Add an element to the end
        d.push(2); //[1, 2]
        d.pop(); //[1] : : Remove an element from the end


        // 🔎 Capacity and reallocation
        let mut e: Vec<i32> = Vec::with_capacity(10);
        println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

        // These are all done without reallocating...
        for i in 0..10 {
            e.push(i);
        }
        // ...but this may make the vector reallocate
        e.push(11);
  ```
  
⭐️ Mainly a vector represent 3 things,

    - A pointer to the data
    - No of elements currently have(length)
    - Capacity (Amount of space allocated for any future elements).
If the length of a vector exceeds its capacity, its capacity will be increased automatically. But its elements will be reallocated(which can be slow). So always use Vec::with_capacity whenever it’s possible.

💯 Vectors can be used with iterators in three ways,
  ```rust
        let mut v = vec![1, 2, 3, 4, 5];
        for i in &v {
            println!("A reference to {}", i);
        }
        for i in &mut v {
            println!("A mutable reference to {}", i);
        }
        for i in v {
            println!("Take ownership of the vector and its element {}", i);
        }
  ```

### 4.2 Struct
There are 3 variants of structs: **C-struct, Tuple, Unit sctruct**

- C-like structs
  - One or more comma-separated name:value pairs  
  - Brace-enclosed list  
  - Similar to classes (without its methods) in OOP languages  
  - Because fields have names, we can access them through dot notation
 
- Tuple structs
  - One or more comma-separated values  
  - A parenthesized list like tuples  
  - Looks like a named tuples
  
- Unit structs
  - A struct with no members at all  
  - It defines a new type but it resembles an empty tuple, ()  
  - Rarely in use, useful with generics


⭐️C-like structs
  ```rust
  // Struct Declaration
        struct Color {
            red: u8,
            green: u8,
            blue: u8
        }

        fn main() {
          // Creating an instance
          let black = Color {red: 0, green: 0, blue: 0};

          // Accessing its fields using dot notation
          println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue); //Black = rgb(0, 0, 0)

          // Structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
          let mut link_color = Color {red: 0,green: 0,blue: 255};
          link_color.blue = 238;
          println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue); //Link Color = rgb(0, 0, 238)

          // Copy elements from another instance
          let blue = Color {blue: 255, .. link_color};
          println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue); //Blue = rgb(0, 0, 255)

          // Destructure the instance using a `let` binding, this will not destruct blue instance
          let Color {red: r, green: g, blue: b} = blue;
          println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)

          // Creating an instance via functions & accessing its fields
          let midnightblue = get_midnightblue_color();
          println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)

          // Destructure the instance using a `let` binding
          let Color {red: r, green: g, blue: b} = get_midnightblue_color();
          println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)
        }

        fn get_midnightblue_color() -> Color {
            Color {red: 25, green: 25, blue: 112}
        }  
  ```
⭐️ Tuple struct:  When a tuple struct has only one element, we call it newtype pattern. Because it helps to create a new type.

  ```rust
        struct Color(u8, u8, u8);
        struct Kilometers(i32);

        fn main() {
          // Creating an instance
          let black = Color(0, 0, 0);

          // Destructure the instance using a `let` binding, this will not destruct black instance
          let Color(r, g, b) = black;
          println!("Black = rgb({}, {}, {})", r, g, b); //black = rgb(0, 0, 0);

          // Newtype pattern
          let distance = Kilometers(20);
          // Destructure the instance using a `let` binding
          let Kilometers(distance_in_km) = distance;
          println!("The distance: {} km", distance_in_km); //The distance: 20 km
        }
  ```
  
⭐️ Unit struct: rarely use.

### 4.3 Enum
  
  ```rust
  
        enum FlashMessage {
          Success, // A unit variant
          Warning{ category: i32, message: String }, // A struct variant
          Error(String) // A tuple variant
        }

        fn main() {
          let mut form_status = FlashMessage::Success;
          print_flash_message(form_status);

          form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
          print_flash_message(form_status);

          form_status = FlashMessage::Error(String::from("Connection Error"));
          print_flash_message(form_status);
        }

        fn print_flash_message(m : FlashMessage) {
          // Pattern matching with enum
          match m {
            FlashMessage::Success =>
              println!("Form Submitted correctly"),
            FlashMessage::Warning {category, message} => // Destructure, should use same field names
              println!("Warning : {} - {}", category, message),
            FlashMessage::Error(msg) =>
              println!("Error : {}", msg)
          }
        }
  ```
  
### 4.4 View at https://learning-rust.github.io/docs/b5.impls_and_traits.html

## Chapter 5 The Tough Part
### 5.1 Ownership
1. Copy Type
  - Bound resources are made a copy and assign or pass it to the function.
  - The ownership state of the original bindings is set to “copied” state.
  - Mostly Primitive types
2. Move type
  - Bound resources are moved to the new variable binding and we can not access the original variable binding anymore.
  - The ownership state of the original bindings is set to “moved” state.
  - Non-primitive types

### 5.2 Borrowing

***Shared & Mutable borrowings***
⭐️ There are two types of Borrowing,

1. Shared Borrowing (&T)
  - A piece of data can be borrowed by a single or multiple users, but data should not be altered. 
2. Mutable Borrowing (&mut T)
  - A piece of data can be borrowed and altered by a single user, but the data should not be accessible for any other users at that time.

***Rules for borrowings***
There are very important rules regarding borrowing
- One piece of data can be borrowed either as a shared borrow or as a mutable borrow at a given time. But not both at the same time.
- Borrowing applies for both copy types and move types.
- The concept of Liveness ↴

  ```rust
        fn main() {
        let mut a = vec![1, 2, 3];
        let b = &mut a;  //  &mut borrow of `a` starts here
                         //  ⁝
        // some code     //  ⁝
        // some code     //  ⁝
      }                  //  &mut borrow of `a` ends here


        fn main() {
          let mut a = vec![1, 2, 3];
          let b = &mut a;  //  &mut borrow of `a` starts here
          // some code

          println!("{:?}", a); // trying to access `a` as a shared borrow, so giving an error
        }                  //  &mut borrow of `a` ends here


        fn main() {
          let mut a = vec![1, 2, 3];
          {
            let b = &mut a;  //  &mut borrow of `a` starts here
            // any other code
          }                  //  &mut borrow of `a` ends here

          println!("{:?}", a); // allow borrowing `a` as a shared borrow
        }
  ```





