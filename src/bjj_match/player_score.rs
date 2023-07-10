#[derive(Debug, PartialEq)]
pub enum ScoreField {
    Points,
    Advantages,
    Penalties
}

#[derive(Default, Debug)]
pub struct PlayerScore {
    pub points: usize,
    pub advantages: usize,
    pub penalties: usize
}

impl PlayerScore {
    pub fn subtract(&mut self, field: ScoreField) {
        match field {
            ScoreField::Points => {
                if self.points > 0 {
                    self.points -= 1;
                }
            }
            ScoreField::Advantages => {
                if self.advantages > 0 {
                    self.advantages -= 1;
                }
            }
            ScoreField::Penalties => {
                if self.penalties > 0 {
                    self.penalties -= 1;
                }
            }
        }
    }
}