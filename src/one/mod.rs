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
