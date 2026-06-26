fn data_types() {
    let mut x = 5; // mut = mutable
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // consts cannot depend on an expression that is computed at runtime
    // must only depend on literal values or other consts?
    const THREE_HOURS_TO_SECONDS: u32 = 60 * 60 * 3;

    // shadowing - declare new variable with same name as prev variable
    // second variable is what the compiler sees when the variable is used, or when scope ends
    let y = 5;
    let y = y + 1; // new variable, value = 6
    {
        let y = y * 2; // scoped variable, value = 12
        println!("Value of y in inner scope is {y}");
    }
    println!("value of y is {y}");
    // shadowing allows us to change variable values without making them mutable
    // requires using the `let` keyword, otherwise will throw error
    // but this also makes a new variable = more memory used?
    // also allows you to change the type of the variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("length of spaces is {spaces}");

    let guess: u32 = "42".parse().expect("Not a number");
    // when using .parse(), Rust expects the type to be defined

    // isize (or usize) = architecture-dependent integer size
    // 64 bit = i64,  32 bit = i32
    // Rust defaults to i32
    // integer literals: 98_222 = 98222, visual separation to increase readability

    let z = 4.0; // f64
    let zx: f32 = 2.0; // f32

    // boolean = bool
    let t: bool = true;
    // character = char. in Rust, 4 bytes in size, allows Unicode values
    let ch: char = 'Z';

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    // can also unpack tuples
    let pac = (500, 6.4, 1);
    let (a, b, c) = pac;
    println!("The value of a is {a}");

    // arrays allocate data on the stack
    // less flexible than vector type (vector has dynamic size)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // data type: [type of value; length]
    let arr2 = [3; 5];
    // [initial value; length], access elements = arr[0]
}
