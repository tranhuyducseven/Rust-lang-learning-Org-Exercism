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









