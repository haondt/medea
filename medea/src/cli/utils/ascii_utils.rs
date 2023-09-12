pub fn decode(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8).collect()
}
pub fn encode(input: &[u8]) -> String {
    input.iter().map(|&b| String::from(b as char)).collect()
}