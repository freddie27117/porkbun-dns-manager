mod address_getter;
mod address_setter;
pub mod deblogger;
pub mod structs;
pub mod utils;

use deblogger::deblogger;

fn main() {
    let current_ip = address_getter::current_ip();
    let dns_entry = address_getter::current_dns_entry();

    if current_ip != dns_entry {
        deblogger(format!(
            "Current IP address: {} does not match the cached DNS address: {}",
            current_ip, dns_entry
        ));
        deblogger("Updating...");

        // update logic

        deblogger("Done!")
    }

    // println!("{}", address_getter::current_dns_entry());
    // println!("{}", address_getter::current_ip());
}
