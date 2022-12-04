fn main() {
    // Two numbers to will the sum
    let num1 = 120;
    let num2 = 321;

    let result = num1 + num2;

    loop {
        // Show the two numbers to display
        println!("Favor to write the sum to {num1} and {num2}");

        // Get form user the number to represent the sum
        let mut result_user = String::new();
        std::io::stdin().read_line(&mut result_user).unwrap();

        let result_user_int : i32 = result_user.trim().parse().unwrap();

        if result_user_int == result {
            break println!("You do great, the result {result} is correct");
        }

        println!("The result {result_user_int} is not correct, try again please");
    }

}
