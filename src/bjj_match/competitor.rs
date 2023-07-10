use crate::flags::Country;

#[derive(Debug, PartialEq)]
pub enum CompetitorNumber {
    One,
    Two
}

#[derive(Debug)]
pub struct Competitor {
    pub first_name: String,
    pub last_name: String,
    pub team_name: String,
    pub country: Country
}

impl Default for Competitor {
    fn default() -> Self {
        Self {
            first_name: "Competitor".to_owned(),
            last_name: "Name".to_owned(),
            team_name: "BJJ Team".to_owned(),
            country: Country::Australia
        }
    }
}

impl Competitor {
    pub fn new(first_name: &str, last_name: &str, team_name: &str, country: Country) -> Competitor {
        Competitor {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            team_name: team_name.to_owned(),
            country
        }
    }
    pub fn get_display_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}