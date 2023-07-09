pub mod grid;
use std::collections::HashMap;
use std::ops::Add;
use anyhow::Result;
use bjj_scoreboard::{BJJMatch, Competitor, CompetitorNumber, Country, MatchInformation};
use eframe::egui;
use eframe::egui::{Align2, Color32, Key, Pos2, Rounding, TextureHandle, TextureOptions};
use eframe::emath::Rect;
use egui_extras::image::FitTo;
use crate::AppState::NewMatchDialog;
use strum::IntoEnumIterator;
use crate::grid::calc_grids;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "BJJ Scoreboard",
        options,
        Box::new(|_cc| Box::<BjjScoreboard>::default())
    )
}

enum AppState {
    NewMatchDialog,
    Normal
}

struct BjjScoreboard {
    bjj_match: BJJMatch,
    app_state: AppState,
    match_dialog_open: bool,
    first_run: bool,
    color_scheme: ColorScheme,
    font_sizes: FontSizes,
    flags: HashMap<String, TextureHandle>
}

struct FontSizes {
    competitor_name: f32,
    competitor_team: f32,
    competitor_adv_label: f32,
    competitor_adv: f32,
    competitor_pen_label: f32,
    competitor_pen: f32,
    competitor_points: f32,
    time: f32,
    fight_info_heading: f32,
    fight_info_sub_heading: f32,
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
            competitor_points: 130.0,
            time: 32.0,
            fight_info_heading: 32.0,
            fight_info_sub_heading: 28.0,
        }
    }
}

struct ColorScheme {
    competitor_one_bg: Color32,
    competitor_one_name: Color32,
    competitor_one_team: Color32,
    competitor_one_adv_bg: Color32,
    competitor_one_adv: Color32,
    competitor_one_pen_bg: Color32,
    competitor_one_pen: Color32,
    competitor_one_points_bg: Color32,
    competitor_one_points: Color32,
    competitor_two_bg: Color32,
    competitor_two_name: Color32,
    competitor_two_team: Color32,
    competitor_two_adv_bg: Color32,
    competitor_two_adv: Color32,
    competitor_two_pen_bg: Color32,
    competitor_two_pen: Color32,
    competitor_two_points_bg: Color32,
    competitor_two_points: Color32,
    bottom_pane_bg: Color32,
    time: Color32,
    fight_info_heading: Color32,
    fight_info_sub_heading: Color32,
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
            bottom_pane_bg: Color32::from_rgb(0, 160, 0),
            time: Color32::from_rgb(255, 255, 180),
            fight_info_heading: Color32::from_rgb(200, 200, 140),
            fight_info_sub_heading: Color32::from_rgb(255, 255, 255),
        }
    }
}

impl Default for BjjScoreboard {
    fn default() -> Self {
        Self {
            bjj_match: Default::default(),
            app_state: NewMatchDialog,
            match_dialog_open: true,
            first_run: true,
            color_scheme: Default::default(),
            font_sizes: Default::default(),
            flags: HashMap::new()
        }
    }
}

impl eframe::App for BjjScoreboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.first_run {
            self.setup(ctx);
            self.first_run = false;
        }
        match self.app_state {
            AppState::NewMatchDialog => {
                self.draw_new_match_modal(ctx)
            },
            AppState::Normal => {
                self.draw_active_match_screen(ctx);
                ctx.request_repaint();
            }
        }
    }
}

impl BjjScoreboard {
    fn setup(&mut self, ctx: &egui::Context) {
        println!("Starting setup");
        self.load_fonts(ctx);
        self.load_flags(ctx);
    }
    fn load_flags(&mut self, ctx: &egui::Context) {
        for country in Country::iter() {
            let data = country.data();

            let image = egui_extras::image::load_svg_bytes_with_size(
                data.flag,
                FitTo::Height(360)
            );

            match image {
                Ok(color_image) => {
                    let texture = ctx.load_texture(
                        data.code.as_str(),
                        color_image,
                        TextureOptions::default()
                    );
                    self.flags.insert(data.code, texture);
                }
                Err(e) => {
                    println!("Error loading SVG: {}", e);
                }
            }
        }
    }
    fn load_fonts(&self, ctx: &egui::Context) {
        println!("Loading fonts");
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "main_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/BebasNeue-Regular.ttf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "main_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("main_font".to_owned());

        ctx.set_fonts(fonts);
    }
    fn draw_competitor_dialog(heading: &str, competitor: &mut Competitor, ui: &mut egui::Ui) {
        ui.heading(heading);
        ui.end_row();

        let first = ui.label("First Name");
        ui.text_edit_singleline(&mut competitor.first_name).labelled_by(first.id);
        ui.end_row();

        let last = ui.label("Last Name");
        ui.text_edit_singleline(&mut competitor.last_name).labelled_by(last.id);
        ui.end_row();

        let team = ui.label("Team");
        ui.text_edit_singleline(&mut competitor.team_name).labelled_by(team.id);
        ui.end_row();

        let country = ui.label("Country");
        egui::ComboBox::from_id_source(country.id)
            .selected_text(format!("{:?}", competitor.country))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                for country in Country::iter() {
                    let name = country.data().name;
                    ui.selectable_value(&mut competitor.country, country, name);
                }
            });
        ui.end_row();
    }

    fn draw_active_match_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
            self.handle_input(ctx);
        });
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(Key::Q)) {
            self.bjj_match.add_points(2, CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::W)) {
            self.bjj_match.add_points(3, CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::E)) {
            self.bjj_match.add_points(4, CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.bjj_match.add_advantage( CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::T)) {
            self.bjj_match.add_penalty( CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::Y)) {
            self.bjj_match.subtract_point( CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::U)) {
            self.bjj_match.subtract_advantage( CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::I)) {
            self.bjj_match.subtract_penalty( CompetitorNumber::One);
        }
        if ctx.input(|i| i.key_pressed(Key::A)) {
            self.bjj_match.add_points(2, CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::S)) {
            self.bjj_match.add_points(3, CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::D)) {
            self.bjj_match.add_points(4, CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::F)) {
            self.bjj_match.add_advantage( CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::G)) {
            self.bjj_match.add_penalty( CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::H)) {
            self.bjj_match.subtract_point( CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::J)) {
            self.bjj_match.subtract_advantage( CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::K)) {
            self.bjj_match.subtract_penalty( CompetitorNumber::Two);
        }
        if ctx.input(|i| i.key_pressed(Key::Space)) {
            self.bjj_match.toggle_start_stop();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let scale_factor = ui.clip_rect().width() / 600.0;
        let match_grid = calc_grids(ui.clip_rect());

        ui.painter().rect_filled(match_grid.full, Rounding::none(), Color32::BLACK);

        ui.painter().rect_filled(match_grid.competitor_one.left, Rounding::none(), self.color_scheme.competitor_one_bg);
        ui.painter().rect_filled(match_grid.competitor_one.advantages, Rounding::none(), self.color_scheme.competitor_one_adv_bg);
        ui.painter().rect_filled(match_grid.competitor_one.penalties, Rounding::none(), self.color_scheme.competitor_one_pen_bg);
        ui.painter().rect_filled(match_grid.competitor_one.points, Rounding::none(), self.color_scheme.competitor_one_points_bg);
        ui.painter().rect_filled(match_grid.competitor_two.left, Rounding::none(), self.color_scheme.competitor_two_bg);
        ui.painter().rect_filled(match_grid.competitor_two.advantages, Rounding::none(), self.color_scheme.competitor_two_adv_bg);
        ui.painter().rect_filled(match_grid.competitor_two.penalties, Rounding::none(), self.color_scheme.competitor_two_pen_bg);
        ui.painter().rect_filled(match_grid.competitor_two.points, Rounding::none(), self.color_scheme.competitor_two_points_bg);
        ui.painter().rect_filled(match_grid.time.full, Rounding::none(), self.color_scheme.bottom_pane_bg);

        ui.painter().text(
            match_grid.competitor_one.name.left_center(),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_one.get_display_name(),
            egui::FontId { size: self.font_sizes.competitor_name * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_name);

        ui.painter().text(
            match_grid.competitor_one.team.left_center(),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_one.team_name.as_str(),
            egui::FontId { size: self.font_sizes.competitor_team * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_team);

        ui.painter().text(
            match_grid.competitor_one.advantages.center_top(),
            Align2::CENTER_TOP,
            "Adv.",
            egui::FontId { size: self.font_sizes.competitor_adv_label * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_adv);

        ui.painter().text(
            match_grid.competitor_one.advantages.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.score.competitor_one_score.advantages.to_string(),
            egui::FontId { size: self.font_sizes.competitor_adv * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_adv);

        ui.painter().text(
            match_grid.competitor_one.penalties.center_top(),
            Align2::CENTER_TOP,
            "Pen.",
            egui::FontId { size: self.font_sizes.competitor_pen_label * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_pen);

        ui.painter().text(
            match_grid.competitor_one.penalties.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.score.competitor_one_score.penalties.to_string(),
            egui::FontId { size: self.font_sizes.competitor_pen * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_pen);

        ui.painter().text(
            match_grid.competitor_one.points.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.score.competitor_one_score.points.to_string(),
            egui::FontId { size: self.font_sizes.competitor_points * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_points);

        ui.painter().text(
            match_grid.competitor_two.name.left_center(),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_two.get_display_name(),
            egui::FontId { size: self.font_sizes.competitor_name * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_name);

        ui.painter().text(
            match_grid.competitor_two.team.left_center(),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_two.team_name.as_str(),
            egui::FontId { size: self.font_sizes.competitor_team * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_team);

        ui.painter().text(
            match_grid.competitor_two.advantages.center_top(),
            Align2::CENTER_TOP,
            "Adv.",
            egui::FontId { size: self.font_sizes.competitor_adv_label * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_adv);

        ui.painter().text(
            match_grid.competitor_two.advantages.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.score.competitor_two_score.advantages.to_string(),
            egui::FontId { size: self.font_sizes.competitor_adv * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_adv);

        ui.painter().text(
            match_grid.competitor_two.penalties.center_top(),
            Align2::CENTER_TOP,
            "Pen.",
            egui::FontId { size: self.font_sizes.competitor_pen_label * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_pen);

        ui.painter().text(
            match_grid.competitor_two.penalties.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.score.competitor_two_score.penalties.to_string(),
            egui::FontId { size: self.font_sizes.competitor_pen * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_pen);

        ui.painter().text(
            match_grid.competitor_two.points.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.score.competitor_two_score.points.to_string(),
            egui::FontId { size: self.font_sizes.competitor_points * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_points);

        let country_data = self.bjj_match.info.competitor_one.country.data();
        ui.painter().image(
            self.flags.get(country_data.code.as_str()).unwrap().id(),
            match_grid.competitor_one.flag,
            Rect::from_min_max(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 1.0, y: 1.0}),
            Color32::WHITE
        );

        let country_data = self.bjj_match.info.competitor_two.country.data();
        ui.painter().image(
            self.flags.get(country_data.code.as_str()).unwrap().id(),
            match_grid.competitor_two.flag,
            Rect::from_min_max(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 1.0, y: 1.0}),
            Color32::WHITE
        );

        ui.painter().text(
            match_grid.time.time.center(),
            Align2::CENTER_CENTER,
            format_millis(self.bjj_match.time.get_remaining_time_milliseconds()),
            egui::FontId { size: self.font_sizes.time * scale_factor, ..Default::default()},
            self.color_scheme.time);

        ui.painter().text(
            match_grid.time.fight_info_heading.left_center(),
            Align2::LEFT_CENTER,
            "Fight Info",
            egui::FontId { size: self.font_sizes.fight_info_heading * scale_factor, ..Default::default()},
            self.color_scheme.fight_info_heading);

        ui.painter().text(
            match_grid.time.fight_info_sub_heading.left_center(),
            Align2::LEFT_CENTER,
            "Fight Sub Heading",
            egui::FontId { size: self.font_sizes.fight_info_sub_heading * scale_factor, ..Default::default()},
            self.color_scheme.fight_info_sub_heading);
    }


    fn draw_match_info_dialog(heading: &str, info: &mut MatchInformation, ui: &mut egui::Ui) {
        ui.heading(heading);
        ui.end_row();

        let match_time = ui.label("Match Duration (mins)");
        ui.add(egui::DragValue::new(&mut info.match_time_minutes).speed(0.1).clamp_range(1..=30)).labelled_by(match_time.id);
        ui.end_row();

        let mat_num = ui.label("Mat Number");
        ui.add(egui::DragValue::new(&mut info.mat_number).speed(0.1).clamp_range(1..=20)).labelled_by(mat_num.id);
        ui.end_row();

        let fight_num = ui.label("Fight Number");
        ui.add(egui::DragValue::new(&mut info.fight_number).speed(0.1).clamp_range(1..=30)).labelled_by(fight_num.id);
        ui.end_row();
    }

    fn draw_new_match_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("Match Settings")
            .open(&mut self.match_dialog_open)
            .show(ctx,|ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            BjjScoreboard::draw_competitor_dialog("Competitor One", &mut self.bjj_match.info.competitor_one, ui);
                            ui.separator();
                            ui.end_row();
                            BjjScoreboard::draw_competitor_dialog("Competitor Two", &mut self.bjj_match.info.competitor_two, ui);
                            ui.separator();
                            ui.end_row();
                            BjjScoreboard::draw_match_info_dialog("Match Information", &mut self.bjj_match.info, ui);
                            ui.separator();
                            ui.end_row();
                            if ui.add(egui::Button::new("Start Match")).clicked() {
                                self.app_state = AppState::Normal;
                                self.bjj_match.start();
                            }
                        });
                }
            );
    }


}



pub fn format_millis(millis: usize) -> String {
    let hours = millis / 3_600_000;
    let minutes = (millis % 3_600_000) / 60_000;
    let seconds = (millis % 60_000) / 1_000;
    let milliseconds = millis % 1_000;

    if hours > 0 {
        format!("{:01}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
    } else {
        format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
    }
}