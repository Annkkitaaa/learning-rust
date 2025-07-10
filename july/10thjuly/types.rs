let wallet_address 
let mut balance
let mut wallet_name
// variable in rust is immutable by default


// Using primitive-types crate for large integers
use primitive_types::U256;

// Transaction structure
let transaction_id: U256 = U256::from(12345678901234567890u128);  // Immutable
let mut amount_satoshis: u64 = 100000000;  // 1 BTC = 100M satoshis, mutable
let mut confirmation_count: u8 = 0;        // Mutable
let mut is_verified: bool = false;         // Mutable  
let transaction_type: char = 'S';          // Immutable

fn btc_to_satoshis(btc: f64) -> u64 {
    let satoshis = btc * 100_000_000.0;  // Note the .0 for f64 multiplication
    satoshis as u64  // Type conversion with 'as'
}
enum Result<T, E> {
    Ok(T),    // Success case - contains the value
    Err(E),   // Error case - contains the error
}

fn validate_transaction_amount(satoshis: u64) -> Result<(), &'static str> {
    let max_satoshis: u64 = 21_000_000 * 100_000_000;  // 21M BTC in satoshis
    
    if satoshis == 0 {
        return Err("Transaction amount cannot be zero");
    }
    
    if satoshis > max_satoshis {
        return Err("Transaction amount exceeds maximum Bitcoin supply");
    }
    
    Ok(())  // Success case
}