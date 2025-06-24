mod app;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "WalkTermChat",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::default()))),
    )
}
