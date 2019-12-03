pub fn process_input_list(input: &mut Vec<usize>) -> Vec<usize> {
    let mut index = 0;
    while input[index] != 99 {
        let x = process_op_code(input[index], input[index + 1], input[index + 2], &input);
        let mod_index = input[index + 3];
        input[mod_index] = x;
        index += 4;
    }
    return input.to_owned();
}

fn process_op_code(code: usize, x: usize, y: usize, parsed: &Vec<usize>) -> usize {
    match code {
        1 => parsed[x] + parsed[y],
        2 => parsed[x] * parsed[y],
        _ => panic!("invalid"),
    }
}

pub fn search_combo(input_code: &Vec<usize>, computed_value: usize) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut current = input_code.clone();
            current[1] = noun;
            current[2] = verb;
            if process_input_list(&mut current)[0] == computed_value {
                return 100 * noun + verb;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_op_code_test() {
        assert_eq!(
            vec![2, 0, 0, 0, 99],
            process_input_list(&mut vec![1, 0, 0, 0, 99])
        );
        assert_eq!(
            vec![2, 3, 0, 6, 99],
            process_input_list(&mut vec![2, 3, 0, 3, 99])
        );
        assert_eq!(
            vec![2, 4, 4, 5, 99, 9801],
            process_input_list(&mut vec![2, 4, 4, 5, 99, 0])
        );
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            process_input_list(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
        assert_eq!(
            2890696,
            process_input_list(&mut vec![
                1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 9, 19, 23, 2, 13,
                23, 27, 1, 6, 27, 31, 2, 6, 31, 35, 2, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1,
                9, 47, 51, 1, 51, 13, 55, 1, 55, 13, 59, 2, 59, 13, 63, 1, 63, 6, 67, 2, 6, 67, 71,
                1, 5, 71, 75, 2, 6, 75, 79, 1, 5, 79, 83, 2, 83, 6, 87, 1, 5, 87, 91, 1, 6, 91, 95,
                2, 95, 6, 99, 1, 5, 99, 103, 1, 6, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99, 2,
                14, 0, 0
            ])[0]
        );
    }
}
