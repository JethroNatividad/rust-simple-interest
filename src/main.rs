use std::io;
use std::io::Write;

// A program that calculates interest.

// Inputs: principal_amount, interest_rate_percentage, years_invested
// Process: investment_amount = principal_amount * (1 + (interest_rate_percentage * years_invested))
// Output: After {years_invested} years at {interest_rate_percentage}%, the investment will be worth ${investment_amount}.

fn round_decimal(number: f64) -> f64 {
    (number * 1000.0).round() / 1000.0
}

fn calculate_simple_interest(principal_amount: f64, interest_rate_percentage: f64, years_invested: f64) -> f64 {
    let interest_rate: f64 = interest_rate_percentage.max(0.0) / 100.0;
    let years: f64 = years_invested.max(0.0);
    let simple_interest = principal_amount * (1.0 + (interest_rate * years));
    round_decimal(simple_interest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_simple_interest(){
        assert_eq!(calculate_simple_interest(1500.0, 4.3, 4.0), 1758.0);
        assert_eq!(calculate_simple_interest(2000.0, 0.0, 3.0), 2000.0);
        assert_eq!(calculate_simple_interest(0.0, 4.5, 5.0), 0.0);
        assert_eq!(calculate_simple_interest(5000.0, 7.25, 6.0), 7175.0);
        assert_eq!(calculate_simple_interest(100.0, 1.0, 0.5), 100.5);
        assert_eq!(calculate_simple_interest(3000.0, -2.0, 4.0), 3000.0);
        assert_eq!(calculate_simple_interest(2500.0, 3.75, -2.0), 2500.0);
    }
}

fn main() {
    print!("Enter the principal amount: ");
    let mut principal_amount = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut principal_amount).expect("Failed to read input");
    let principal_amount: f64 = principal_amount.trim().parse().expect("Please enter a valid number");

    print!("Enter the rate of interest %: ");
    let mut interest_rate = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut interest_rate).expect("Failed to read input");
    let interest_rate: f64 = interest_rate.trim().parse().expect("Please enter a valid number");

    print!("Enter the number of years: ");
    let mut years = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let years: f64 = years.trim().parse().expect("Please enter a valid number");

    let simple_interest: f64 = calculate_simple_interest(principal_amount, interest_rate, years);
    println!("After {} {} at {}%, the investment will be worth ${}.", years, if years > 1.0 { "years" } else { "year" }, interest_rate, simple_interest);
    
}