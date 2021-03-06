use crate::r#match::game::MatchResult;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct LeagueTable {
    pub rows: Vec<LeagueTableRow>
}

impl LeagueTable {
    pub fn new(clubs: Vec<u32>) -> Self {
        let mut rows = Vec::with_capacity(clubs.len());
        
        for club_id in clubs {
            let table_row = LeagueTableRow {
                club_id,
                played: 0,
                win: 0,
                draft: 0,
                lost: 0,
                goal_scored: 0,
                goal_concerned: 0,
                points: 0
            };
            
            rows.push(table_row)
        }
        
        LeagueTable {
            rows
        }
    }
   
    #[inline]
    fn get_club(&mut self, club_id: u32) -> &mut LeagueTableRow {
        self.rows.iter_mut()
            .find(|c| c.club_id == club_id)
            .unwrap()
    }

    fn winner(&mut self, club_id: u32, goal_scored: u8, goal_concerned: u8) {
        let mut club = self.get_club(club_id);

        club.played += 1;
        club.win += 1;
        club.goal_scored += goal_scored;
        club.goal_concerned += goal_concerned;
        club.points += 3;
    }

    fn looser(&mut self, club_id: u32, goal_scored: u8, goal_concerned: u8) {
        let mut club = self.get_club(club_id);

        club.played += 1;
        club.lost += 1;
        club.goal_scored += goal_scored;
        club.goal_concerned += goal_concerned;
    }

    fn draft(&mut self, club_id: u32, goal_scored: u8, goal_concerned: u8) {
        let mut club = self.get_club(club_id);

        club.played += 1;
        club.draft += 1;
        club.goal_scored += goal_scored;
        club.goal_concerned += goal_concerned;
        club.points += 1;
    }

    pub fn update(&mut self, match_result: &Vec<MatchResult>) {
        for result in match_result {
            match Ord::cmp(&result.home_goals, &result.away_goals) {
                Ordering::Equal => {
                    self.draft(result.home_club_id, result.home_goals, result.away_goals);
                    self.draft(result.away_club_id, result.away_goals, result.away_goals);
                },
                Ordering::Greater => {
                    self.winner(result.home_club_id, result.home_goals, result.away_goals);
                    self.looser(result.away_club_id, result.away_goals, result.home_goals);
                },
                Ordering::Less => {
                    self.looser(result.home_club_id, result.home_goals, result.away_goals);
                    self.winner(result.away_club_id, result.away_goals, result.home_goals);
                }
            }
        }

        self.rows.sort_by(|a, b| Ord::cmp(&a.points, &b.points));
    }

    pub fn get(&self) -> &[LeagueTableRow] {
        &self.rows
    }
}

#[derive(Debug)]
pub struct LeagueTableRow {
    pub club_id: u32,
    pub played: u8,
    pub win: u8,
    pub draft: u8,
    pub lost: u8,
    pub goal_scored: u8,
    pub goal_concerned: u8,
    pub points: u8,
}

impl LeagueTableRow {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_draft() {
        let first_club_id = 1;
        let second_club_id = 2;
        
        let clubs = vec![
            first_club_id,
            second_club_id
        ];
        
        let mut table = LeagueTable::new(clubs);
        
        let match_results = vec![
            MatchResult {
                schedule_id: "123".to_string(),
                home_club_id: 1,
                away_club_id: 2,
                home_goals: 3,
                away_goals: 3,
                player_changes: Vec::new()
            },
        ];
        
        table.update(&match_results);
        
        let returned_table = table.get();
       
        let home = &returned_table[0];
        
        assert_eq!(1, home.played);
        assert_eq!(1, home.draft);
        assert_eq!(0, home.win);
        assert_eq!(0, home.lost);
        assert_eq!(3, home.goal_scored);
        assert_eq!(3, home.goal_concerned);
        assert_eq!(1, home.points);

        let away = &returned_table[0];

        assert_eq!(1, away.played);
        assert_eq!(1, away.draft);
        assert_eq!(0, away.win);
        assert_eq!(0, away.lost);
        assert_eq!(3, away.goal_scored);
        assert_eq!(3, away.goal_concerned);
        assert_eq!(1, away.points);
    }

    #[test]
    fn table_winner() {
        let first_club_id = 1;
        let second_club_id = 2;

        let clubs = vec![
            first_club_id,
            second_club_id
        ];

        let mut table = LeagueTable::new(clubs);

        let home_club_id = 1;
        let away_club_id = 2;
        
        let match_results = vec![
            MatchResult {
                schedule_id: "123".to_string(),
                home_club_id,
                away_club_id,
                home_goals: 3,
                away_goals: 0,
                player_changes: Vec::new()
            },
        ];

        table.update(&match_results);

        let returned_table = table.get();

        let home = returned_table.iter()
            .find(|c| c.club_id == home_club_id)
            .unwrap();
        
        assert_eq!(1, home.club_id);
        assert_eq!(1, home.played);
        assert_eq!(0, home.draft);
        assert_eq!(1, home.win);
        assert_eq!(0, home.lost);
        assert_eq!(3, home.goal_scored);
        assert_eq!(0, home.goal_concerned);
        assert_eq!(3, home.points);

        let away = returned_table.iter()
            .find(|c| c.club_id == away_club_id)
            .unwrap();
        
        assert_eq!(2, away.club_id);
        assert_eq!(1, away.played);
        assert_eq!(0, away.draft);
        assert_eq!(0, away.win);
        assert_eq!(1, away.lost);
        assert_eq!(0, away.goal_scored);
        assert_eq!(3, away.goal_concerned);
        assert_eq!(0, away.points);
    }

    #[test]
    fn table_looser() {
        let first_club_id = 1;
        let second_club_id = 2;

        let clubs = vec![
            first_club_id,
            second_club_id
        ];

        let mut table = LeagueTable::new(clubs);

        let home_club_id = 1;
        let away_club_id = 2;

        let match_results = vec![
            MatchResult {
                schedule_id: "123".to_string(),
                home_club_id,
                away_club_id,
                home_goals: 0,
                away_goals: 3,
                player_changes: Vec::new()
            },
        ];

        table.update(&match_results);

        let returned_table = table.get();

        let home = returned_table.iter()
            .find(|c| c.club_id == home_club_id)
            .unwrap();

        assert_eq!(1, home.club_id);
        assert_eq!(1, home.played);
        assert_eq!(0, home.draft);
        assert_eq!(0, home.win);
        assert_eq!(1, home.lost);
        assert_eq!(0, home.goal_scored);
        assert_eq!(3, home.goal_concerned);
        assert_eq!(0, home.points);

        let away = returned_table.iter()
            .find(|c| c.club_id == away_club_id)
            .unwrap();

        assert_eq!(2, away.club_id);
        assert_eq!(1, away.played);
        assert_eq!(0, away.draft);
        assert_eq!(1, away.win);
        assert_eq!(0, away.lost);
        assert_eq!(3, away.goal_scored);
        assert_eq!(0, away.goal_concerned);
        assert_eq!(3, away.points);
    }
}
