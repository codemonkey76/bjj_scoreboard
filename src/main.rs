use bjj_scoreboard::BjjScoreboard;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "BJJ Scoreboard",
        options,
        Box::new(|_cc| Box::<BjjScoreboard>::default())
    )
}