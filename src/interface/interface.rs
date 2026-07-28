use crate::interface::basic_operations::clearTerminal;
use crate::interface::input;
use crate::models::customer::Customer;
use crate::services::customer_service::{register_customer, return_customers};

pub fn show_menu(customers: &mut Vec<Customer>) {
    loop {
        clearTerminal();
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

        clearTerminal();

        match option {
            1 => register_customer(customers),
            2 => println!("Opt2"),
            3 => return_customers(customers),
            4 => println!("Opt4"),
            0 => {
                println!("Bye!");
                return;
            }
            _ => println!("Invalid..."),
        }
    }
}
