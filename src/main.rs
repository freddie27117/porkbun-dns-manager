pub mod deblogger;
mod getters;
use deblogger::deblogger;
use getters::{get_dns_entry, get_public_ip};
mod installer;
mod setters;
pub mod structs;
pub mod utils;
use std::env::args;
mod address_setter;

#[tokio::main]
async fn main() {
    let args: Vec<_> = args().collect();

    if args.len() == 1 {
        let current_ip = get_public_ip().await;
        let dns_entry = get_dns_entry().await;

        if current_ip != dns_entry {
            deblogger(format!(
                "Current IP address: {} does not match the cached DNS address: {}",
                current_ip, dns_entry
            ));
            deblogger("Updating...");

            address_setter::dns_record(current_ip);

            deblogger("Done!")
        } else {
            deblogger("Your current IP already matches the cached record")
        }
    } else if args.len() > 1 && args[1] == "--install" {
        installer::install();
    } else {
        println!("Invalid argument entered...")
    }
}
