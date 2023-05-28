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

## program flow control

- `if` always expects a boolean
    - it does not work with "truthy/falsy" type checks

- `loop` can pass a value when it exits out of the loop

## memory management

- scope contributes to when a variable is in memory and when it goes away from memory

### shadowing 
- variables are immutable,
- but new variables with same name can be initialized 
    - new variable will mask the old variable with the same name as long as both are in the same scope
    - it does not replace the exist variable, it only masks it in the order and scope
- this is *not the same* as changing the value of a mutable variable, this is simply shadowing/masking 
- cna lead to weird bugs in program

### stack and heap memory
- in rust, program memory is divided into stack and heap

#### stack memory
- values stored in sequential order
- added and removed in LIFO (last in, first out)
- like a stack of boxes piling up 
- stack automatically grows and shrinks as the program executes

- push and pop data quickly
- access data quickly
- small, known fixed data size only allowed
    - various sizes allowed, but should be fixed through run time execution and known at compile time

##### fixed size 

- integer 
- floating point
- boolean
- char
- array 
- tuple

#### heap memory

- storing boxes of data in a giant warehouse
- allocates memory for data in heap
    - location of memory is given with pointer 
    - pointers have fixed size, so can be stored on the stack

- since we have to follow a pointer, accessing heap data is slower than the stack
- adding is slower also, becuse we have to search for a suitable section of memory to store data

- we can dynamically resize data unlike on the stack
- heap memory space is larger compared to the stack 

### string data type

- string: sequence of characters 
    - since the string is stored on the heap instead of the stack, it can be dynamically changed

**string literal**
- hard-coded into executables
- immutable
- value must be known at compile time
- cannot be used for user input during runtime
**string type**
- allocated on the head
- mutable
- dynamic space allocation during runtime


#### string type

- pointer stored on the stack, (along with length and capacity)
    - actual string stored on the heap
    - sometimes the capacity can be more than the length, when the string has been allocated more space on the heap then required, leaving it room to grow into the allocated capacity 
    - capacity should always be greater than or equal to the length

##### dynamic resizig of strings

- mutable strings can grow in size
    - `push_str` grows the string on the heap to include the appended string
- if the string had to be moved to accommodate the addition, the pointer of the string will be updated accordingly, so it is safe to assume that the pointer will change each time the capacity of the string changes 

- since strings can be dynamically resized, they can be manipulated in a [ton of ways](https://doc.rust-lang.org/std/string/struct.String.html)
    

### memory clean-up and ownership

- it is necessary to clean up allocated memory block that are no longer needed

>>> _memory leak_: memory is failed to be released after use 
>>> _invalid memory access_: after a piece of data is freed from the memory access to it is attempted

#### traditional approach
- make the programmer responsible for memory management such as in C and C++ (`malloc()` and `free()`)
    - programmer has a lot of control but leads to a lot of bugs and memory leaks and invalid memory access


#### garbage collection
- automatically cleans up memory by running a subroutine to detect unused memory for clean up, such as Java, C#, Python, Ruby, Go etc. 
    - this approach is easy to use, but automatic memory clean-up can be inefficient and waste a lot of memory compared to traditional approach
    - the garbage collector can run at inconvenient times, causing program to slow down or pause at critical moments

#### ownership

- rust's somewhat unique approach to memory management
    - *resources can only have one owner at a time* 
- variables are responsible for freeing their own resources

>>> every value is owned by one and only one variable called its owner
>>> when owning variable goes out of scope, the value is dropped and memory is freed

- ownership can be transferred from one owner to another, 
    - but at a given point in time, a value can have only one owner

- memory safe for invalid access
- efficient since no garbage collector overhead

- requires understanding of ownership, makes language difficult to learn 
    - different paradigm from other languages 

- different types of data work differently 

##### `move`

- moving a value invalidates the first variable
    - new variable points to the location of the resource in heap memory 
    - old variable is removed from stack 
- similar to shallow copy

##### `clone`

- cloning creates two instances of the resource in heap 
    - both old and the new variables point to their own resource in heal memory
- the clones are independent and what we do to one will not affect the other 

##### `copy`

- for stack resources, the variable directly is copied instead of ownership transfer 

##### `=` operator 
- for stack resources, `=` does a clone by default
- for heap resources, `=` does a move by default
    - cloning must be done explicitly

##### functions and ownership

- when values are passed into a function, 
    - if the value is an integer, i.e. a stack resource, a copy of the value is passed 
    - if the value is a string, i.e. a heap based resource, then the ownership is transfer, not a clone
        - no implicit copy of the heap data is made
        - use the `clone()` method on the heap resource in the argument of the function to pass in a deep copy of the value into the function  
        - if changes are made to the clone inside the function, those changes will not be reflected back in the original resource outside the function  

- after the function is finished,
    - the value passed into will be dropped from memory as the resource will go out of scope 
    - the ownership of the resource can be transferred back with an return expression