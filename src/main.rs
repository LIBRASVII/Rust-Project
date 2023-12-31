#!allow[(dead_code)]

// declare a string function
fn another_string_function() {
    let another_string_variable = "Blue?";
    println!("another_string_function - {}", another_string_variable);
    println!("{another_string_variable}")
}

// string function
fn string_function() -> String {
    let string_var = "Hello there".to_string();
    string_var
}

// sum tow numbers
fn plus(number1: i32, number2: i32) -> i32 {
    let result = number1 + number2;
    println!("{} + {} is {}", number1, number2, result);
    result
}

// functio that divide two numbers
fn divide(number_one: f32, number_two: f32) -> f32 {
    let result = number_one / number_two;
    println!("{} divide for {} is {}", number_one, number_two, result);
    result
}

// multiply two numbers
fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is = {}", number_one, number_two, result);
}

// return a interger
// in this case - 5
fn number() -> i32 {
    5
}

// multiply one number for two
fn times_two(number: i32) -> i32 {
    number * 2
}

fn main() { 

    let final_number = {
        let y = 3;
        let x = 6; // 6
        let x = times_two(x); // 12
        let x = x + y; // 15
        x // this return x: final_number
    };
    println!("The final_number is - {}", final_number);

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
    
    // prints the size  of a char type
    println!("Char size - {}", std::mem::size_of::<char>());

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
        second_number + 9 // No semicolon, so the code block returns 10 + 9
                        // It works just like a functiom 
    };

    println!("{}", my_number);

    let my_another_number = {
        let another_second_number = 16; // declare variable
        another_second_number + 21; // add 21 to variable
                                    // but we didn't return it!
                                    // the variable dies here 
    };
    println!("{:?}", my_another_number);

    let doesnt_print = ();
    println!("{:#?}", doesnt_print);

    // to see the small and big number of variable type
    println!("Smallest i8 is {} | Biggest i8 is {}", i8::MIN, i8::MAX);
    println!("Smallest f32 is {} | Biggest f32 is {}", f32::MIN, f32::MAX);

    // mutable variable
    // able to change the variable value
    let mut mut_variable = 10;
    println!("Before change - {}", mut_variable);
    mut_variable = 12;
    println!("After change - {}", mut_variable);

    // shadow variables
    // the primary number is killed
    let shadow_variable = 10;
    println!("{}", shadow_variable);
    let shadow_variable = 4.5;
    println!("{}", shadow_variable);

    // not change the values of the variable
    // even with the same name
    // because the block is diferent
    let shadow_variable2 = 20; // f32
    println!("Shadow_variable2 - {}", shadow_variable2); // prints 20
    {
        let shadow_variable2 = 5.5; // f64
        println!("Shadow_variable2 on diferent block - {}", shadow_variable2) // prints 5.5
    }
    println!("Shadow_variable2 original - {}", shadow_variable2); // prints 20

    // prints the string declared on the function
    println!("string_function - {}", string_function()); 

    // prints the string functiion
    another_string_function();
}
