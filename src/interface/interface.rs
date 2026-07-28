use crate::interface::input;

pub fn show_menu() {
    loop {
        println!(
            "\
        =============== MENU ===============\n\
        -> Select option:\n\
        1 - Register Customer\n\
        2 - Edit Customer Information\n\
        3 - Show Customers\n\
        4 - Delete Customer\n\
        0 - Exit\n\
        "
        );

        let option: i32 = input::input_data_int();

        match option {
            1 => println!("Opt1"),
            2 => println!("Opt2"),
            3 => println!("Opt3"),
            4 => println!("Opt4"),
            0 => {
                println!("Bye!");
                return;
            }
            _ => println!("Invalid..."),
        }
    }
}
