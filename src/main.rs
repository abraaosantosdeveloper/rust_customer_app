mod interface;
mod models;
mod services;

use interface::interface as view;
use models::customer::Customer;

fn main() {
    let mut customers: Vec<Customer> = Vec::new();
    view::show_menu(&mut customers);
}
