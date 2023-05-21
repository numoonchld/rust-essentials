# rust-essentials

- rust is a statically-typed language
    - every variable must have a known data type at compile time
    - like C, C++, Java and Go

## importing rust files in project

- [Importing module from another file in same project](https://stackoverflow.com/a/26390046)


## create rust project

```rust
cargo new new-project-name
```

## run rust project

```rust
cd new-project-name
cargo run
```

## primitive variables

- variables is rust are immutable by default
    - use `mut` keyword to make them mutable

### naming rules

- variable names are case-sensitive

- variable names can contain:
    - letters 
    - digits
    - underscore

- variable names cannot begin with digits, only letters and underscore

- cannot use keywords reserved by the language as variable names
    - i.e. `let`, `mut` etc.

- [general naming conventions - RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md#general-naming-conventions)

### primitive scaler types 
- integers 
- floats 
- booleans 
- characters

#### integers 

- by default, integers are `i32` in rust
    - signed
        - positive and negative
    - 32 bit 
        - `2^32` possible values 

- beware of overflow and storing negation in unsupported integer data types 

#### floating points 
- represent decimal point numbers with IEEE 754
- `f32`: single precision
- `f64`: double precision 
    - default float type for inference

#### arithmetic operations

- cannot mix integers and floats for arithmetic operations 
    - necessary to cast integers as floats 
    - `as` keyword 
    - example: 
        - `a as f64`
            - where `a` is default integer type of `i32` before operation
    - when floats are cast to integers, the fractional part is cut off and information is lost
        - it is truncated, not rounded,
            - so `3.9` cast ot integer is `3` not `4`

##### casting

- use casting with caution!

- produces strange behaviors 
    - i.e.
        - `300 as u8` => `44`
            - because of overflow
        - `-300 as u32` => `4294966996`
            - because of type mismatch
    - these are considered valid behaviors and compiler does not throw error 

##### order of operations 

- **BODMAS** rule applies:
    - Brackets (Parenthesis)
    - Order (Exponent/Raised to)
    - Division
    - Multiplication
    - Addition
    - Subtraction

#### booleans

- 0: false
- 1: true

- casting boolean into integer will result in 1 or 0 

- applying boolean operations to booleans also result in boolean values 

##### short-circuiting
- &&: short circuiting AND
- ||: short circuiting OR

#### comparison operators
- the entities being compared have to be the same data type

## compound data types 

### array
- homogeneous (same) data 
- zero indexed
- ordered
### tuples: 
- heterogeneous (mixed) data
- zero indexed
- ordered

### multidimensional arrays

- the inner arrays must all be of the same length
    - if there are different number of columns in some of the inner arrays, then it is considered to be of a different type altogether 

## functions

- `fn` keyword to define functions
- use function name followed by`()` to invoke function

### parameter 

- provide input data to a function
- defined in the function signature

### statement vs. expression

- statement ends with semicolon
    - does not return a value 
- expression does not end with semi-colon
    - evaluates to a resulting value

### return values
- if the last line of a function is an expression, i.e. without a semi-colon, then the value of that expression gets passed out as the return value of that function

#### unit data type

- used when there is no meaningful value returned from a function

- represented with `()`
