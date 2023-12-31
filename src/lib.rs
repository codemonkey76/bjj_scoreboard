pub mod flags;
pub mod ui;
pub mod audio;
pub mod grid;
pub mod bjj_match;

use std::collections::BTreeMap;
use std::ops::Add;
use eframe::egui::{self, Align2, Color32, Key, Pos2, Rect, Rounding, Vec2};

use crate::flags::{Flag, Country};
use crate::audio::Audio;
use crate::bjj_match::BJJMatch;
use crate::bjj_match::competitor::CompetitorNumber;
use crate::bjj_match::match_information::MatchInformation;
use crate::grid::{calc_grids, RectReduce};
use crate::ui::{ColorScheme, FontSizes};

pub enum AppState {
    NewMatchDialog,
    InProgress,
    Ready,
}

pub struct BjjScoreboard {
    bjj_match: BJJMatch,
    fullscreen: bool,
    app_state: AppState,
    match_dialog_open: bool,
    first_run: bool,
    color_scheme: ColorScheme,
    font_sizes: FontSizes,
    flags: BTreeMap<Country, Flag>,
    audio: Audio,
}

impl Default for BjjScoreboard {
    fn default() -> Self {
        Self {
            bjj_match: Default::default(),
            fullscreen: false,
            app_state: AppState::NewMatchDialog,
            match_dialog_open: true,
            first_run: true,
            color_scheme: Default::default(),
            font_sizes: Default::default(),
            flags: BTreeMap::new(),
            audio: Default::default()
        }
    }
}

impl eframe::App for BjjScoreboard {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.first_run {
            self.setup(ctx);
            self.first_run = false;
        }

        match self.app_state {
            AppState::NewMatchDialog => {
                self.draw_new_match_modal(ctx)
            },
            AppState::InProgress => {
                if self.bjj_match.time.get_remaining_time_milliseconds() == 0 {
                    self.audio.play_air_horn();
                    self.app_state = AppState::Ready;
                    self.draw_new_match_modal(ctx);
                    ctx.request_repaint();
                    return;
                }
                self.draw_active_match_screen(ctx);
                self.handle_input(ctx, frame);
                ctx.request_repaint();
            },
            AppState::Ready => {
                self.draw_active_match_screen(ctx);
                self.handle_input(ctx, frame);
                ctx.request_repaint();
            }
        }
    }
}

impl BjjScoreboard {
    fn setup(&mut self, ctx: &egui::Context) {
        self.load_fonts(ctx);
        self.flags = Flag::load_textures(ctx);
        self.audio.init();
    }

    fn load_fonts(&self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        let mut font_data = egui::FontData::from_static(include_bytes!("../assets/fonts/BebasNeue-Regular.ttf"));
        font_data.tweak.y_offset_factor = 0.08;

        fonts.font_data.insert(
            "main_font".to_owned(),
            font_data,
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

    fn draw_active_match_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }

    fn handle_input(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::F11)) {
            self.fullscreen = !self.fullscreen;
            frame.set_fullscreen(self.fullscreen);
        }
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
            match self.app_state {
                AppState::NewMatchDialog => {},
                AppState::Ready => {
                    self.audio.play_air_horn();
                    self.bjj_match.start();
                    self.app_state = AppState::InProgress
                },
                AppState::InProgress => {
                    self.bjj_match.toggle_start_stop();
                }
            }

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
            match_grid.competitor_one.name.left_center().add(Vec2 { x: 10.0 * scale_factor, y: 0.0}),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_one.get_display_name(),
            egui::FontId { size: self.font_sizes.competitor_name * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_name);

        ui.painter().text(
            match_grid.competitor_one.team.left_center().add( Vec2 { x: 10.0 * scale_factor, y: 0.0 }),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_one.team_name.as_str(),
            egui::FontId { size: self.font_sizes.competitor_team * scale_factor, ..Default::default()},
            self.color_scheme.competitor_one_team);

        ui.painter().text(
            match_grid.competitor_one.advantages.center_top().add(Vec2 { x: 0.0, y: 2.0 * scale_factor }),
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
            match_grid.competitor_one.penalties.center_top().add(Vec2 { x: 0.0, y: 2.0 * scale_factor }),
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
            match_grid.competitor_two.name.left_center().add(Vec2 { x: 10.0 * scale_factor, y: 0.0}),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_two.get_display_name(),
            egui::FontId { size: self.font_sizes.competitor_name * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_name);

        ui.painter().text(
            match_grid.competitor_two.team.left_center().add( Vec2 { x: 10.0 * scale_factor, y: 0.0 }),
            Align2::LEFT_CENTER,
            self.bjj_match.info.competitor_two.team_name.as_str(),
            egui::FontId { size: self.font_sizes.competitor_team * scale_factor, ..Default::default()},
            self.color_scheme.competitor_two_team);

        ui.painter().text(
            match_grid.competitor_two.advantages.center_top().add(Vec2 { x: 0.0, y: 2.0 * scale_factor }),
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
            match_grid.competitor_two.penalties.center_top().add(Vec2 { x: 0.0, y: 2.0 * scale_factor }),
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

        if let Some(flag) = self.flags.get(&self.bjj_match.info.competitor_one.country) {
            if let Some(handle) = &flag.handle {
                ui.painter().image(
                    handle.id(),
                    match_grid.competitor_one.flag.shrink_to_aspect_ratio(2.0).shrink(5.0 * scale_factor),
                    Rect::from_min_max(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 1.0, y: 1.0 }),
                    Color32::WHITE
                );
            }
        }


        if let Some(flag) = self.flags.get(&self.bjj_match.info.competitor_two.country) {
            if let Some(handle) = &flag.handle {
                ui.painter().image(
                    handle.id(),
                    match_grid.competitor_two.flag.shrink_to_aspect_ratio(2.0).shrink(5.0 * scale_factor),
                    Rect::from_min_max(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 1.0, y: 1.0 }),
                    Color32::WHITE
                );
            }
        }

        ui.painter().text(
            match_grid.time.time.center(),
            Align2::CENTER_CENTER,
            self.bjj_match.time.get_remaining_time_string(),
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
                        let competitor = &mut self.bjj_match.info.competitor_one;
                        ui.heading("Competitor One");
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
                                for value in self.flags.values() {
                                    ui.selectable_value(&mut competitor.country, value.country, value.name.as_str());
                                }
                            });
                        ui.end_row();
                        ui.separator();

                        let competitor = &mut self.bjj_match.info.competitor_two;
                        ui.heading("Competitor Two");
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
                                for value in self.flags.values() {
                                    ui.selectable_value(&mut competitor.country, value.country, value.name.as_str());
                                }
                            });
                        ui.end_row();

                        ui.separator();
                        ui.end_row();
                        BjjScoreboard::draw_match_info_dialog("Match Information", &mut self.bjj_match.info, ui);
                        ui.separator();
                        ui.end_row();
                        if ui.add(egui::Button::new("Start Match")).clicked() {
                            self.app_state = AppState::Ready;
                        }
                    });
            }
            );
    }

}










