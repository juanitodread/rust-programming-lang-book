const PI: f64 = 3.141516;

fn main() {
    println!("Hello, world!");
    println!("This is a constant: {}", PI);

    let area = circle_area(3.0);
    println!("This is the area: {}", area);

    let heart_eyed_cat = '😻';
    println!("Emoji: {}", heart_eyed_cat);

    let boolean = true;
    println!("This is a boolean: {}", boolean);

    let tuple = (1, "dos");
    println!("This is a tuple: ({}, {})", tuple.0, tuple.1);

    let array: [f64; 3] = [1.0, 2.0, 3.0];
    println!("This is an array [{},{},{}]", array[0], array[1], array[2]);

    if_expression(3);
    if_expression(5);
    if_expression(10);

    iterate_collection(&array);
}

fn circle_area(rad: f64) -> f64 {
    return PI * rad * rad;
}

fn if_expression(input: i32) {
    if input < 5 {
        println!("Input is < 5: input = {}", input);
    } else if input == 5 {
        println!("Input is 5: input = {}", input);
    } else {
        println!("Input is >= 5: input = {}", input);
    }
}

fn iterate_collection(array: &[f64]) {
    for element in array {
        println!("Element: {}", element)
    }
}
