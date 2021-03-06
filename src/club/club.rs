use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{ClubMood, TacticsSelector, TransferItem, ClubResult, MatchHistory, TrainingSchedule, Training, ClubFinances, ClubSponsorshipContract, StaffCollection, PlayerCollection, Player, PlayerSelector, ClubReputation};
use crate::simulator::context::GlobalContext;
use crate::shared::Location;

#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,
    
    pub location: Location,
    
    pub mood: ClubMood,
    pub board: ClubBoard,
    
    pub finance: ClubFinances,
    
    pub reputation: ClubReputation,
    
    pub tactics: Option<Tactics>,

    pub players: PlayerCollection,
    pub staffs: StaffCollection,

    pub training_schedule: TrainingSchedule,

    pub transfer_list: Vec<TransferItem>,

    pub match_history: Vec<MatchHistory>,
}

impl Club {
    pub fn new(id: u32, 
               name: String,
               location: Location,
               finance: ClubFinances, 
               reputation: ClubReputation,
               training_schedule: TrainingSchedule, 
               players: PlayerCollection, 
               staffs: StaffCollection) -> Self {
        Club {
            id,
            name,
            location,
            finance,
            reputation,
            mood: ClubMood::default(),
            board: ClubBoard::new(),
            players,
            staffs,
            tactics: None,
            training_schedule,
            transfer_list: Vec::new(),
            match_history: Vec::new(),
        }
    }
    
    pub fn players(&self) -> Vec<&Player> {
        self.players.players()
    }

    pub fn get_match_squad(&self) -> Squad {
        let main_coach = self.staffs.main_coach();

        Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, main_coach),
            players: PlayerSelector::select(self, main_coach),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> ClubResult {
        let result = ClubResult::new(
            self.board.simulate(ctx.with_board()),
            self.players.simulate(ctx.with_player(None)),
            self.staffs.simulate(ctx.with_staff()),
            self.finance.simulate(ctx.with_finance())
        );

        if ctx.simulation.is_week_beginning() {
            let weekly_salary = self.players.get_week_salary();
            self.finance.push_salary(weekly_salary);
        }
        
        if self.training_schedule.is_time(ctx.simulation.date) {
            Training::train_players(&mut self.players.players, self.staffs.coaches());
        }

        result
    }
}
