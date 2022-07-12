pub fn print_array_8(array: &[u8], width: usize) {
    for i in 0..array.len() {
        if i % width == 0 {
            println!();
        }
        print!("{:02X} ", array[i]);
    }
}

pub fn print_array_16(array: &[u16], width: usize) {
    for i in 0..array.len() {
        if i % width == 0 {
            println!();
        }
        print!("{:04X} ", array[i]);
    }
}
