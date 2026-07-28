mod interface;
mod models;

use interface::interface as view;
use models::customer::Customer;

fn main() {
    view::show_menu();
}
