pub mod match_information;
pub mod competitor;
pub mod match_score;
pub mod match_time;
pub mod player_score;

use crate::bjj_match::competitor::{Competitor, CompetitorNumber};
use crate::bjj_match::match_information::MatchInformation;
use crate::bjj_match::match_score::MatchScore;
use crate::bjj_match::match_time::MatchTime;
use crate::bjj_match::player_score::ScoreField;

#[derive(Debug, PartialEq)]
pub enum MatchState {
    NotStarted,
    InProgress,
    Finished
}

#[derive(Default, Debug)]
pub struct BJJMatch {
    pub info: MatchInformation,
    pub score: MatchScore,
    pub time: MatchTime
}

impl BJJMatch {
    pub fn new(competitor_one: Competitor, competitor_two: Competitor, match_time_minutes: usize, mat_number: usize, fight_number: usize) -> BJJMatch {
        BJJMatch{
            info: MatchInformation {
                competitor_one,
                competitor_two,
                match_time_minutes,
                mat_number,
                fight_number
            },
            score: MatchScore::default(),
            time: MatchTime {
                duration_millis: match_time_minutes * 60 * 1000,
                ..Default::default()
            }
        }
    }

    pub fn get_match_state(&self) -> MatchState {
        match self.time.last_started {
            None => MatchState::NotStarted,
            _ => match self.time.get_remaining_time_milliseconds() {
                0 => MatchState::Finished,
                _ => MatchState::InProgress,
            },
        }
    }

    pub fn add_points(&mut self, points: usize, competitor: CompetitorNumber) {
        match competitor {
            CompetitorNumber::One => self.score.competitor_one_score.points += points,
            CompetitorNumber::Two => self.score.competitor_two_score.points += points
        };
    }

    pub fn add_advantage(&mut self, competitor: CompetitorNumber) {
        match competitor {
            CompetitorNumber::One => self.score.competitor_one_score.advantages += 1,
            CompetitorNumber::Two => self.score.competitor_two_score.advantages += 1
        };
    }

    pub fn add_penalty(&mut self, competitor: CompetitorNumber) {
        match competitor {
            CompetitorNumber::One => self.score.competitor_one_score.penalties += 1,
            CompetitorNumber::Two => self.score.competitor_two_score.penalties += 1
        };
    }

    pub fn subtract_point(&mut self, competitor: CompetitorNumber) {
        match competitor {
            CompetitorNumber::One => self.score.competitor_one_score.subtract(ScoreField::Points),
            CompetitorNumber::Two => self.score.competitor_two_score.subtract(ScoreField::Points)
        };
    }

    pub fn subtract_advantage(&mut self, competitor: CompetitorNumber) {
        match competitor {
            CompetitorNumber::One => self.score.competitor_one_score.subtract(ScoreField::Advantages),
            CompetitorNumber::Two => self.score.competitor_two_score.subtract(ScoreField::Advantages)
        };
    }

    pub fn subtract_penalty(&mut self, competitor: CompetitorNumber) {
        match competitor {
            CompetitorNumber::One => self.score.competitor_one_score.subtract(ScoreField::Penalties),
            CompetitorNumber::Two => self.score.competitor_two_score.subtract(ScoreField::Penalties)
        };
    }

    pub fn start(&mut self) {
        self.time.duration_millis = self.info.match_time_minutes * 60 * 1000;
        self.time.start();
    }

    pub fn toggle_start_stop(&mut self) {
        self.time.toggle_start_stop();
    }
}