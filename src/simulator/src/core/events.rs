
#[derive(Clone)]
pub enum EventType{
     Birthday(u32),
     StaffContractExpired(u32),
     PlayerContractExpired(u32),
}