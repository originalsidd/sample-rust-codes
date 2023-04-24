fn main() {
    // Implicit type assignment
    let x = 4;

    // cannot change the type of the above variable throughout the program

    // Explicit type assignment
    // let x: u32 = 4;

    // Printing with formatted string
    println!("x is {}", x);

    // cant do x = 5, x is immutable
    // however we can declare a variable mutable like this
    let mut y = 2;
    println!("y is {}", y);
    y = 5;
    println!("y is {}", y);

    // redeclaring variables (recreating them) (kind of overriding prev. var) is actually fine in rust
    let x = 10;
    println!("x is {}", x);
    let x = x + 1;
    println!("x is {}", x);

    // Name shadowing: Use same variable name but from a different scope
    {
        // can access exterior scope y
        // println!("y is {}", y);

        let x = 100;
        println!("x inner is {}", x);

        let y = 10;
        println!("y inner is {}", y);

        // following changes outer scope y value since its mutable
        // y = 10;
    }
    println!("y is {}", y);

    // can redefine type while redeclaring var
    let x = "Hello World";
    println!("{}", x);

    // ownership transfering
    let x = String::from("Hello World");
    let y = x;

    // error
    // println!("{}", x);

    // ok, since ownership of String::from var (object) from x is transferred to y
    println!("{}", y);

    // calue and datatype will never change in a constant declared var
    const TEDDY_COUNT: u32 = 25;
    println!("{}", TEDDY_COUNT);

    // types of datatypes:
    // 1. primitive - basic/fundamental
    //      i. scalar - finite/single value like integer, floating, boolean, char
    //      ii. compound - multiple values like array, tuple

    /* Scalar */

    // Integers

    // signed integer with 32 bits range (default)
    let x: i32 = 2;

    // similarly
    let x: i8 = 2;
    let x: i16 = 2;
    let x: i64 = -2;
    let x: i128 = 2;
    println!("{}", x);

    // unsigned integer
    let x: u8 = 2;
    let x: u16 = 2;
    let x: u32 = 2;
    let x: u64 = 2;
    let x: u128 = 2;
    println!("{}", x);

    // Floating-Points, 2 types

    // single precision (default)
    let x: f32 = 2.0;

    // double precision 
    let x: f64 = 2.0;
    println!("{}", x);

    // Boolean
    let x: bool = false;
    let x: bool = true;
    println!("{}", x);

    // Character (single quotations)
    let x: char = 'a';
    println!("{}", x);

    /* Compound */

    // Tuple
    // Fixed length sequence of elements which is immutable (can use mut to make it mutable)
    let tup: (i32, bool, char) = (1, true, 's');
    let tup2: (i8, bool, char) = (1, true, 's'); // different type tuple

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let mut tup: (i32, bool, char) = (1, true, 's');

    tup.0 = 10;
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    tup = (-10, false, 'A');
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    // cant add elements to tuples since type is fixed, cant change

    // Array (homogenous elements)
    // array type defined by type of elements and numer of elements in the array
    // thus unlike other lang, cant add more elements to this array
    // need to make another array with more number of elements
    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[4]);
    
    // error
    // arr[4] = 7;
    // println!("{}", arr[4]);
    
    let mut arr = [1, 2, 3, 4, 5];
    println!("{}", arr[4]);
    arr[4] = 7;
    println!("{}", arr[4]);

    // type define
    // [type of elements; number of elements]
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    // implicit datatype define
    let x: u8 = 4;
    let y = x; // here datatype of y is not default i32 but u8 inferred from x
    
    // error
    // let y: i32 = x;

    let x: f64 = 10.0;
    // have to add .0 in 10.0 to make it floating point else error

    let x = 10.0f32;
    let x = 120_000;
    let x = 120_000i64;
    let x = 120_000 as i64;
    let y = 10i32;

    // as works as explicit type conversion while the other methods are implicit

    let z = x / (y as i64);
    let z = x / y as i64;

    println!("{}", z);

    // overflow example
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;
    let z = x as i32 / y;
    println!("{}", z);
    
    // fix
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;
    let z = x / y as i64;
    println!("{}", z);

}
