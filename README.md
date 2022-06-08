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





