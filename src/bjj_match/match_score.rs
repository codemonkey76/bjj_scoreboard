use crate::bjj_match::player_score::PlayerScore;

#[derive(Default ,Debug)]
pub struct MatchScore {
    pub competitor_one_score: PlayerScore,
    pub competitor_two_score: PlayerScore,
    pub time_remaining_milliseconds: usize,
}