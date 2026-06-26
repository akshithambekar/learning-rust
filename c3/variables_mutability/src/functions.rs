fn functions() {
    another_function();
}

fn another_function(x: i32, unit_label: char) {
    println!("Value of x is {x}{unit_label}");
}

fn expression_test() {
    let y = {
        let x = 3;
        x + 1 // this is an expression, does not include semicolon
        // by not using a semicolon, the value is returned
        // if a semicolon was used, it becomes a statement and doesn't return
    }
    println!("The value of y is {y}"); // evals to 4
}

// can return a value either by using `return` or just returning the last expression implicitly
fn five() -> i32 { // declare type after an arrow
    5 // final expression gets
}

fn test_five() {
    let x = five();
    return x;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
