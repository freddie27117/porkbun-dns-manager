mod dns_creator;
mod dns_setter;

pub use dns_creator::create_dns_record;
pub use dns_setter::update_dns_record as set_dns_record;
