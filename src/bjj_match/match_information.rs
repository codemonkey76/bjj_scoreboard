use crate::bjj_match::competitor::Competitor;

#[derive(Debug)]
pub struct MatchInformation {
    pub competitor_one: Competitor,
    pub competitor_two: Competitor,
    pub match_time_minutes: usize,
    pub mat_number: usize,
    pub fight_number: usize
}

impl Default for MatchInformation {
    fn default() -> Self {
        Self {
            competitor_one: Competitor {
                last_name: "One".to_owned(),
                ..Default::default()
            },
            competitor_two: Competitor {
                last_name: "Two".to_owned(),
                ..Default::default()
            },
            match_time_minutes: 5,
            mat_number: 1,
            fight_number: 1
        }
    }
}
