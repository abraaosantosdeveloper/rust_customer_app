use crate::interface::basic_operations::{clearTerminal, wait};
use crate::interface::input::{input_data, input_data_int};
use crate::models::customer::Customer;

//========================================//
//                                        //
//          CREATE OPERATION              //
//                                        //
//========================================//
pub fn register_customer(customers: &mut Vec<Customer>) {
    clearTerminal();

    // Instance of "Customer"
    let mut customer: Customer = Customer::default();

    // Filling data fields
    customer.id = customers.len() + 1;
    insert_customer_data(&mut customer);

    // Append customer to customers vector
    customers.push(customer);
    return;
}

//========================================//
//                                        //
//            LIST OPERATION              //
//                                        //
//========================================//
pub fn return_customers(customers: &mut Vec<Customer>) {
    clearTerminal();
    if is_registry_empty(customers) {
        return;
    };
    // List users
    println!("{}", "-".to_string().repeat(40));
    for customer in customers {
        show_customer(customer);
    }
    println!("Press Enter to proceed...");
    input_data();
}

//========================================//
//                                        //
//          UPDATE OPERATION              //
//                                        //
//========================================//
pub fn update_customer(customers: &mut Vec<Customer>) {
    clearTerminal();
    if is_registry_empty(customers) {
        return;
    }

    let id = get_id();
    if let Some((index, customer)) = get_customer_by_id(customers, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Updating customer with id: {}", id);
        show_customer(customer);
        insert_customer_data(&mut customers[index]);
        clearTerminal();
        println!("Customer updated successfully!");
    } else {
        clearTerminal();
        println!("Customer not found...");
    };
    wait(1);
}

//========================================//
//                                        //
//          DELETE OPERATION              //
//                                        //
//========================================//
pub fn delete_customer() {}

//========================================//
//                                        //
//          AUXILIARY FUNCTIONS           //
//                                        //
//========================================//
fn show_customer(customer: &Customer) {
    println!("ID: {}", customer.id);
    println!("Name: {}", customer.name);
    println!("Email: {}", customer.email);
    println!("CPF: {}", customer.cpf);
    println!("Address: {}", customer.address);
    println!("{}", "-".to_string().repeat(40));
}

// Empty registry verification
fn is_registry_empty(customers: &mut Vec<Customer>) -> bool {
    // Check if there is any user
    if customers.len() == 0 {
        println!("There are no customers registered...");
        wait(2);
        return true;
    };
    return false;
}

// Get customer ID
fn get_id() -> usize {
    clearTerminal();
    println!("Insert customer ID: ");
    input_data_int()
}

fn get_customer_by_id(customers: &Vec<Customer>, id: usize) -> Option<(usize, &Customer)> {
    customers
        .iter()
        .enumerate()
        .find(|(_, customer)| customer.id == id)
}

// insert customer data
fn insert_customer_data(customer: &mut Customer) {
    println!("Type customer name: ");
    customer.name = input_data();
    println!("Type customer Email: ");
    customer.email = input_data();
    println!("Type customer CPF: ");
    customer.cpf = input_data();
    println!("Type customer address: ");
    customer.address = input_data();
}
