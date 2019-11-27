use calc::calculator_functions::basic_functions;
use calc::calculator_functions::power_functions;

fn main() {
    println!("Addition: {}", basic_functions::addition(2,4));
    println!("subtraction: {}", basic_functions::subtraction(4,2));
    println!("Multiplications: {}", basic_functions::mutliply(2,4));
    println!("Division: {}", basic_functions::divide(4,4));
    println!("Square: {}", power_functions::square(4));
    println!("Cube: {}", power_functions::cube(4));
    println!("power: {}", power_functions::power(9,3));
}