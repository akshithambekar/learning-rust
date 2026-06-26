fn control_flow() {
    let number = 3;
    if number < 5 { // the expression must be a boolean
        println!("true");
    } else {
        println!("false");
    } // can also do else-if
    // can't check `if number` as if checking if the number is initialized

    // use if in a let statement
    let condition = true;
    let num = if condition { 5 } else { 6 };
    // types cannot be mismatched
}

fn return_value_from_loop() {
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is {res}"); // evals to 20
}

fn loop_within_loop() {
    let mut count = 0;
    'counting_up: loop { // ' is a label to say which loop to break/continue at
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops() {
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        number -= 1;
    }
    println!("finished");
}

fn loop_in_collection() {
    let a = [1, 2, 3, 4, 5];
    let mut i = 0;
    while i < 5 {
        println!("value is {}", a[i]);
    }
    for element in a { // for-each loop
        println!("value is {element}");
    }
    for num in (1..4).rev() { // range is 1-4 inclusive, reversed
        println!("{number}"); // 4 3 2 1
    }
}
