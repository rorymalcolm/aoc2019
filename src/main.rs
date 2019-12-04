mod one;
mod two;
mod four;


fn main() {
    let mut acceptable_password = 0;
    for x in 136818_u64..685979_u64 {
        match four::valid_password(x.to_string()) {
            true => acceptable_password = acceptable_password + 1,
            false => (),
        }
    }
    println!("{}", acceptable_password);
}

fn print_list(input: Vec<usize>) {
    for i in input.iter() {
        print!("{}", i);
    }
}
