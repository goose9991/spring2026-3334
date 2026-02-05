
const FREEZING: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING 
}

fn temperature_converter() {
    let mut temp: f64 = 32.0;

    println!("{}° Fahrenheit -> {:.2}° Celsius", temp, fahrenheit_to_celsius(temp));

    for _ in 0..5 {
        temp += 1.0;
        println!("{}° Fahrenheit -> {:.2}° Celsius", temp, fahrenheit_to_celsius(temp));
    }

}


//--------------------------
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn number_analyzer() {
    //create array
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    
    //fizzbuzz
    for &num in nums.iter() {

        // FizzBuzz 
        match (num % 3, num % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{} is {}", num, if is_even(num) { "even" } else { "odd" }),
        }
    }

    //sum all numbers in array
    let mut x = 0;
    let mut sum = 0;
    while x < nums.len() {
        sum += nums[x];
        x += 1;
    }
    println!("total sum of array values: {}", sum);

    //find greatest number in array
    let mut greatest = nums[0];

    for i in 1..nums.len() {
        if nums[i] > greatest {
            greatest = nums[i];
        }
    }
    println!("greatest value in array: {}", greatest);
}

//----------------------
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn guessing_game() {
    let secret_num = 39;
    let guesses_list = [10, 25, 50, 40, 39];
    let mut guesses = 0;

    for guess in guesses_list {
        guesses += 1;

        let result = check_guess(guess, secret_num);

        if result == 0 {
            println!("Correct! The guess was {}", guess);
            break;
        } else if result == 1 {
            println!("{} is too high", guess);
        } else {
            println!("{} is too low", guess);
        }
    }

    println!("It took {} guesses", guesses);
    
}

fn main() {

    //Assignment 1
    temperature_converter();

    //Assignment 2
    number_analyzer();

    //Assignment 3
    guessing_game();

}
