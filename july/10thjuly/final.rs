
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



fn process_transaction(btc: f64) -> Result<u64, &'static str> {
    // Step 1: Convert BTC to satoshis
    let satoshis = match btc_to_satoshis(btc) {
        Ok(value) => value,
        Err(_) => return Err("Transaction amount could not be converted"),
    };
    
    // Step 2: Validate the satoshi amount
    match validate_transaction_amount(satoshis) {
        Ok(()) => Ok(satoshis),  // Return the satoshis if valid
        Err(_) => Err("Transaction amount is invalid"),
    }
}