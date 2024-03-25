

fn main() {
    let n = 10; // Define the value of N

    // Calculate the sum of the first N natural numbers
    let sum_of_naturals = (1..=n).sum::<u64>();

    // Calculate the square of the sum
    let square_of_sum = sum_of_naturals * sum_of_naturals;

    // Calculate the sum of the squares of the first N natural numbers
    let sum_of_squares = (1..=n).map(|x| x * x).sum::<u64>();

    // Calculate the difference between the square of the sum and the sum of the squares
    let difference = square_of_sum - sum_of_squares;

    // Print the result
    println!("The difference is: {}", difference);
}
