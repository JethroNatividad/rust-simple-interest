// A program that calculates interest.

// Inputs: principal_amount, interest_rate_percentage, years_invested
// Process: investment_amount = principal_amount * (1 + (interest_rate_percentage * years_invested))
// Output: After {years_invested} years at {interest_rate_percentage}%, the investment will be worth ${investment_amount}.

fn calculate_interest(principal_amount, interest_rate_percentage, years_invested) -> f64 {
    principal_amount * (1 + (interest_rate_percentage * years_invested))
}




fn main() {
    println!("Hello, world!");
}
