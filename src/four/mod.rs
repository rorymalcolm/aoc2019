pub fn valid_password(input: String) -> bool {
    match input.len() {
        6 => return process_password(input),
        _ => return false,
    }
}

fn process_password(input: String) -> bool {
    let mut double_digit = false;
    for index in 0..input.len()-1 {
        if input.as_bytes()[index] as char == input.as_bytes()[index+1] as char {
            double_digit = true;
        }
        if input.as_bytes()[index] as char > input.as_bytes()[index+1] as char {
            return false;
        }
    }
    return double_digit;
}
