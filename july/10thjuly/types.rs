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