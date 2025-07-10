



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