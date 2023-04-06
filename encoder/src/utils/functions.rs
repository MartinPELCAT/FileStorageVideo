pub fn get_hex_string(num: i32) -> String {
    let mut hex_string = format!("{}", num); // Convert to hexadecimal string
    while hex_string.len() < 6 {
        hex_string.push('f'); // Append "f" characters until the length is 6
    }
    hex_string
}

pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), std::num::ParseIntError> {
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok((r, g, b))
}

pub fn encode_str_to_hex(str: &str) -> String {
    return hex::encode(str);
}
