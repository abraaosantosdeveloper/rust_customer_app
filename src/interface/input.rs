use std::io;

pub fn input_data() -> String {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Input failure...");
    data.trim().to_string()
}

pub fn input_data_int() -> usize {
    loop {
        let mut data: String = String::new();
        io::stdin().read_line(&mut data).expect("Input failure...");

        match data.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Please type a valid number.");
            }
        }
    }
}
