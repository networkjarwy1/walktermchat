use crate::app::MyApp;
use eframe::egui;

pub fn show_login(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("🔐 Login");

        ui.label("Username:");
        ui.text_edit_singleline(&mut app.username_input);

        ui.label("Password:");
        ui.add(egui::TextEdit::singleline(&mut app.password_input).password(true));

        if let Some(err) = &app.login_error {
            ui.colored_label(egui::Color32::RED, err);
        }

        if ui.button("Login").clicked() {
            if app.username_input == "admin" && app.password_input == "password" {
                app.logged_in = true;
                app.login_error = None;
            } else {
                app.login_error = Some("Invalid username or password".to_string());
            }
        }
    });
}
