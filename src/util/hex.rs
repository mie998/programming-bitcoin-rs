#[allow(dead_code)]
pub fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}
