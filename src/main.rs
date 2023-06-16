#!allow[(dead_code)]

fn plus(number1: i32, number2: i32) -> i32 {
    let result = number1 + number2;
    println!("{} + {} is {}", number1, number2, result);
    result
}

fn divide(number_one: f32, number_two: f32) -> f32 {
    let result = number_one / number_two;
    println!("{} divide for {} is {}", number_one, number_two, result);
    result
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is = {}", number_one, number_two, result);
}

fn number() -> i32 {
    5
}

fn main() { 

    let mut num1: u8 = 10; 

    println!("{}", num1);

    // change the value of the num1
    num1 = 100;

    // cast unsize as a char
    println!("{}", num1 as char);

    let x = 108;

    // cast the unsize as a char
    println!("{}", x as u8 as char);

    let t: char = 'A';

    // cast char as a unsize
    println!("{}", t as u8);
    let test_char: char = 'L';
    println!("{}", test_char as u8);

    let count_char = "This ís about somewhere";
    println!("{}, {}", count_char.len(), count_char.chars().count());
    
    println!("{}", std::mem::size_of::<char>());

    let number1: i32 = 1_00000_0000;
    let number2 = 90_121______21___3___i32;
    let float_number: f64 = 65.4;
    let float_number2 = 9.;
    let float_number3: f32 = 6.32;
    println!("{}, {}, {}, {}, {}", number1, number2, float_number, float_number2, float_number3);

    println!("{}", 100);
    println!("{}, {}", 3, 5);

    println!("Function call - {}", number());

    multiply(10, 10);

    let some_number = 21;
    let some_other_number = 3;
    multiply(some_number, some_other_number);

    //take float variables to function and divide them
    let some_number_float: f32 = 35.;
    let some_other_number_float: f32 = 7.;

    let result_divide = divide(some_number_float, some_other_number_float);
    divide(10.0, 3.);
    println!("{} divide for {} is {}", some_number_float, some_other_number_float, result_divide);

    // given two number to function and calling it
    let number1 = 12;
    let number2 = 43;
    plus(number1, number2);

    // code block that return a value
    let my_number = {
        let second_number = 10;
        second_number + 9
    };

    println!("{}", my_number);

}
