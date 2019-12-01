use std::fs;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let content_split = contents.split("\n");
    let mut x = 0_i64;
    for line in content_split {
        match line.parse::<i64>() {
            Ok(parsed) => x = x + calculate_fuel(parsed),
            Err(_err) => x = x,
        }
    }
}

fn calculate_fuel(module_mass: i64) -> i64 {
    return match (module_mass / 3) - 2 {
        x if x >= 0 => x + calculate_fuel(x),
        _ => 0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fuel_test() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(2), 0);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }
}
