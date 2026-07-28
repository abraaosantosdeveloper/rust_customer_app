use crate::interface::basic_operations::{clearTerminal, wait};
use crate::interface::input::{input_data, input_data_int};
use crate::models::customer::Customer;

pub fn register_customer(customers: &mut Vec<Customer>) {
    clearTerminal();

    // Instance of "Customer"
    let mut customer: Customer = Customer::default();

    // Filling data fields
    customer.id = customers.len() + 1;
    println!("Type customer name: ");
    customer.name = input_data();
    println!("Type customer Email: ");
    customer.email = input_data();
    println!("Type customer CPF: ");
    customer.cpf = input_data();
    println!("Type customer address: ");
    customer.address = input_data();

    // Append customer to customers vector
    customers.push(customer);
    return;
}

pub fn return_customers(customers: &mut Vec<Customer>) {
    clearTerminal();

    // Check if there is any user
    if customers.len() == 0 {
        println!("There are no customers registered...");
        wait(2);
        return;
    }

    // List users
    println!("{}", "-".to_string().repeat(40));
    for customer in customers {
        show_customer(customer);
    }
    println!("Press Enter to proceed...");
    input_data();
}

fn show_customer(customer: &Customer) {
    println!("ID: {}", customer.id);
    println!("Name: {}", customer.name);
    println!("Email: {}", customer.email);
    println!("CPF: {}", customer.cpf);
    println!("Address: {}", customer.address);
    println!("{}", "-".to_string().repeat(40));
}

pub fn update_customer(customers: &mut Vec<Customer>) {
    clearTerminal();
}

pub fn delete_customer() {}
