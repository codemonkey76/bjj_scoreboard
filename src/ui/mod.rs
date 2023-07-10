use eframe::egui::Color32;

pub struct ColorScheme {
    pub competitor_one_bg: Color32,
    pub competitor_one_name: Color32,
    pub competitor_one_team: Color32,
    pub competitor_one_adv_bg: Color32,
    pub competitor_one_adv: Color32,
    pub competitor_one_pen_bg: Color32,
    pub competitor_one_pen: Color32,
    pub competitor_one_points_bg: Color32,
    pub competitor_one_points: Color32,
    pub competitor_two_bg: Color32,
    pub competitor_two_name: Color32,
    pub competitor_two_team: Color32,
    pub competitor_two_adv_bg: Color32,
    pub competitor_two_adv: Color32,
    pub competitor_two_pen_bg: Color32,
    pub competitor_two_pen: Color32,
    pub competitor_two_points_bg: Color32,
    pub competitor_two_points: Color32,
    pub bottom_pane_bg: Color32,
    pub time: Color32,
    pub fight_info_heading: Color32,
    pub fight_info_sub_heading: Color32,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            competitor_one_bg: Color32::from_rgb(0, 0, 0),
            competitor_one_name: Color32::from_rgb(255, 255, 255),
            competitor_one_team: Color32::from_rgb(255, 255, 255),
            competitor_one_adv_bg: Color32::from_rgb(0, 0, 0),
            competitor_one_adv: Color32::from_rgb(255, 255, 255),
            competitor_one_pen_bg: Color32::from_rgb(0, 0, 0),
            competitor_one_pen: Color32::from_rgb(255, 255, 255),
            competitor_one_points_bg: Color32::from_rgb(227, 85, 141),
            competitor_one_points: Color32::from_rgb(255, 255, 255),
            competitor_two_bg: Color32::from_rgb(49, 42, 109),
            competitor_two_name: Color32::from_rgb(255, 255, 255),
            competitor_two_team: Color32::from_rgb(255, 255, 255),
            competitor_two_adv_bg: Color32::from_rgb(49, 42, 109),
            competitor_two_adv: Color32::from_rgb(255, 255, 255),
            competitor_two_pen_bg: Color32::from_rgb(49, 42, 109),
            competitor_two_pen: Color32::from_rgb(255, 255, 255),
            competitor_two_points_bg: Color32::from_rgb(46, 100, 209),
            competitor_two_points: Color32::from_rgb(255, 255, 255),
            bottom_pane_bg: Color32::from_rgb(0, 0, 0),
            time: Color32::from_rgb(255, 255, 180),
            fight_info_heading: Color32::from_rgb(200, 200, 140),
            fight_info_sub_heading: Color32::from_rgb(255, 255, 255),
        }
    }
}

pub struct FontSizes {
    pub competitor_name: f32,
    pub competitor_team: f32,
    pub competitor_adv_label: f32,
    pub competitor_adv: f32,
    pub competitor_pen_label: f32,
    pub competitor_pen: f32,
    pub competitor_points: f32,
    pub time: f32,
    pub fight_info_heading: f32,
    pub fight_info_sub_heading: f32,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            competitor_name: 48.0,
            competitor_team: 32.0,
            competitor_adv_label: 12.0,
            competitor_adv: 32.0,
            competitor_pen_label: 12.0,
            competitor_pen: 32.0,
            competitor_points: 120.0,
            time: 32.0,
            fight_info_heading: 32.0,
            fight_info_sub_heading: 28.0,
        }
    }
}