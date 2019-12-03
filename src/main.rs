mod one;
mod two;

fn main() {}

fn print_list(input: Vec<usize>) {
    for i in input.iter() {
        print!("{}", i);
    }
}
