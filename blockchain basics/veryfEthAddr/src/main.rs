use regex::Regex;

fn is_valid_ethereum_address(address: &str) -> bool {
    let re = regex::Regex::new(r"^(0x)?[0-9a-fA-F]{40}$").unwrap();
    re.is_match(address)
}

fn main() {
    let address = "0x1234567890123456789012345678901234567890";
    if is_valid_ethereum_address(address) {
        println!("Valid Ethereum Address: {}", address);
    } else {
        println!("Invalid Ethereum Address: {}", address);
    }
}
