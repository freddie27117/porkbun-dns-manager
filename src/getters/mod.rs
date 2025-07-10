pub mod dns_entry;
pub mod public_ip;

pub use dns_entry::dns_entry as get_dns_entry;
pub use public_ip::current_ip as get_public_ip;

