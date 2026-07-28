use std::io;

pub fn input_data() -> String {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Input failure...");
    data.trim().to_string()
}

pub fn input_data_int() -> i32 {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Input failure...");
    data.trim().parse().expect("Conversion error...")
}
