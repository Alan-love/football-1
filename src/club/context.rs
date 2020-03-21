use crate::core::SimulationContext;
use chrono::NaiveDateTime;

pub struct ClubSimulationContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,

    pub contract_improve_requests: Vec<u32>,
    pub transfer_requests: Vec<u32>,
}

impl ClubSimulationContext {
    pub fn new(context: &SimulationContext) -> Self {
        ClubSimulationContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
            contract_improve_requests: Vec::new(),
            transfer_requests: Vec::new(),
        }
    }

    pub fn request_contract_improvement(&mut self, player_id: u32) {
        self.contract_improve_requests.push(player_id);
    }

    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}
