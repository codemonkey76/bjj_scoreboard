use eframe::egui::{Pos2, Rect};

#[derive(Debug)]
pub struct MatchGrid {
    pub full: Rect,
    pub competitor_one: CompetitorGrid,
    pub competitor_two: CompetitorGrid,
    pub time: TimeGrid
}

#[derive(Debug)]
pub struct TimeGrid {
    pub full: Rect,
    pub time: Rect,
    pub fight_info_heading: Rect,
    pub fight_info_sub_heading: Rect,
    pub logo: Rect
}
#[derive(Debug)]
pub struct CompetitorGrid {
    pub full: Rect,
    pub main: Rect,
    pub left: Rect,
    pub right: Rect,
    pub comp: Rect,
    pub flag: Rect,
    pub name: Rect,
    pub team: Rect,
    pub points: Rect,
    pub advantages: Rect,
    pub penalties: Rect
}

pub enum SplitDirection {
    Horizontal,
    Vertical
}
pub enum SplitMode {
    Exact,
    Proportional
}
trait Split where Self: Sized {
    fn split(&self, direction: SplitDirection, mode: SplitMode, at: f32) -> (Self, Self);
    fn split_v(&self, mode: SplitMode, at: f32) -> (Self, Self);
    fn split_h(&self, mode:SplitMode, at:f32) -> (Self, Self);
}

impl Split for Rect {
    fn split(&self, direction: SplitDirection, mode: SplitMode, at: f32) -> (Self, Self) {
        match direction {
            SplitDirection::Vertical => {
                let new_y = match mode {
                    SplitMode::Exact => self.min.y + at,
                    SplitMode::Proportional => self.min.y + (self.max.y - self.min.y) * at
                };
                (
                    Rect::from_min_max(self.min, Pos2::new(self.max.x, new_y)),
                    Rect::from_min_max(Pos2::new(self.min.x, new_y), self.max)
                )
            }
            SplitDirection::Horizontal => {
                let new_x = match mode {
                    SplitMode::Exact => self.min.x + at,
                    SplitMode::Proportional => self.min.x + (self.max.x - self.min.x) * at
                };

                (
                    Rect::from_min_max(self.min, Pos2::new(new_x, self.max.y)),
                    Rect::from_min_max(Pos2::new(new_x, self.min.y), self.max)
                )
            }
        }

    }
    fn split_v(&self, mode: SplitMode, at: f32) -> (Self, Self) {
        self.split(SplitDirection::Vertical, mode, at)
    }
    fn split_h(&self, mode:SplitMode, at:f32) -> (Self, Self) {
        self.split(SplitDirection::Horizontal, mode, at)
    }
}

pub fn calc_grids(rect: Rect) -> MatchGrid {
    let (top,bottom) = rect.split_v(SplitMode::Proportional, 0.75);
    let (top, middle) = top.split_v(SplitMode::Proportional, 0.5);

    let competitor_one = calc_competitor_grid(top);
    let competitor_two = calc_competitor_grid(middle);

    let time = calc_time_grid(bottom);

    MatchGrid {
        full: rect,
        competitor_one,
        competitor_two,
        time
    }
}

fn calc_time_grid(rect: Rect) -> TimeGrid {
    let (left, logo) = rect.split_h(SplitMode::Proportional, 5.0 / 6.0);
    let (time, fight_info) = left.split_h(SplitMode::Proportional, 1.0 / 3.0);
    let (fight_info_heading, fight_info_sub_heading) = fight_info.split_v(SplitMode::Proportional, 0.5);

    TimeGrid {
        full: rect,
        time,
        fight_info_heading,
        fight_info_sub_heading,
        logo
    }
}

fn calc_competitor_grid(rect: Rect) -> CompetitorGrid {
    let (main, points) = rect.split_h(SplitMode::Proportional, 5.0 / 6.0);
    let (left, right) = main.split_h(SplitMode::Proportional, 10.0 / 11.0);
    let (comp, team) = left.split_v(SplitMode::Proportional, 2.0 / 3.0);
    let (flag, name) = comp.split_h(SplitMode::Proportional, 1.0 / 8.0);
    let (advantages, penalties) = right.split_v(SplitMode::Proportional, 0.5);

    CompetitorGrid {
        full: rect,
        main,
        left,right,
        comp,
        flag,
        name,
        team,
        points,
        advantages,
        penalties
    }
}