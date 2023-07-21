fn array_challenge(arr: &mut [i32]) -> i32 {
    let sum: i32 = arr.iter().sum();

    let (mut fibonacci1, mut fibonacci2) = (0, 1);
    let mut fibonacci_sum = 0;

    while fibonacci_sum < sum {
        fibonacci_sum = fibonacci1 + fibonacci2;
        fibonacci1 = fibonacci2;
        fibonacci2 = fibonacci_sum;
    }

    arr[0] = fibonacci_sum - sum;
    arr[0]
}

fn main() {
    // Test the function with the example input
    let mut arr = [15, 1, 3];
    let output = array_challenge(&mut arr);
    println!("{}", output); // Output: 2
}
