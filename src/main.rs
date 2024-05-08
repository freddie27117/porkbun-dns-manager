mod address_getter;
mod address_setter;
pub mod deblogger;
mod installer;
pub mod structs;
pub mod utils;

use deblogger::deblogger;
use std::env::args;

// make config file for less repeating code?

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() > 1 && args[1] == "--install" {
        installer::install();
    } else {
        let current_ip = address_getter::current_ip();
        let dns_entry = address_getter::current_dns_entry();

        if current_ip != dns_entry {
            deblogger(format!(
                "Current IP address: {} does not match the cached DNS address: {}",
                current_ip, dns_entry
            ));
            deblogger("Updating...");

            address_setter::dns_record(current_ip);

            deblogger("Done!")
        } else {
            deblogger("Your current IP already matches what is in the DNS record")
        }
    }
}
