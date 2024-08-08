/// Deploy a smart contract
pub fn create() -> Result<String, String> {
    Ok("Contract deployed".to_string())
}

/// Retrieve a smartcontract's IDL
// TODO: Return type has to be a struct that contains both ABI and IDL
// __NOTE__: All solana programs won't have an IDL. This needs to be handled with care
pub fn get(_id: String) -> Result<String, String> {
    Ok("Contract retrieved".to_string())
}

/// Upgrade a smart contract
// TODO: The new binary should point to a URI path. String is not the right type
// But for now, we can use it as a placeholder
pub fn upgrade(_id: String, _new_binary: String) -> Result<String, String> {
    Ok("Contract Upgraded".to_string())
}

/// Close a smart contract
pub fn close(_id: String) -> Result<String, String> {
    Ok("Contract closed".to_string())
}

/// Freeze a smart contract from further upgrades
pub fn freeze(_id: String) -> Result<String, String> {
    Ok("Contract frozen".to_string())
}

/// Submit a transaction to be executed by the smart contract
/// TODO: The transaction should be a struct that contains the transaction details
pub fn execute(_txn: String) -> Result<String, String> {
    Ok("Transaction submitted".to_string())
}

/// Dry run a transaction to be executed by the smart contract
pub fn dry_run(_txn: String) -> Result<String, String> {
    Ok("Transaction dry run".to_string())
}

/// Estimate the cost of a transaction to be executed by the smart contract
pub fn estimate_cost(_txn: String) -> Result<String, String> {
    Ok("Cost estimated".to_string())
}
