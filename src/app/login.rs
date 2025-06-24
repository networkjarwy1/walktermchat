use crate::app::MyApp;
use eframe::egui::{self, Color32, Direction, Layout, Vec2};

pub fn show_login(app: &mut MyApp, ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(30, 30, 30))
                .stroke(egui::Stroke::new(1.0, Color32::DARK_GRAY))
                .corner_radius(12.0)
                .inner_margin(egui::Margin::symmetric(24, 24))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("🔐 Welcome Back")
                                .size(24.0)
                                .color(Color32::LIGHT_BLUE),
                        );

                        ui.add_space(16.0);

                        ui.label(egui::RichText::new("👤 Username").color(Color32::GRAY));
                        ui.add(
                            egui::TextEdit::singleline(&mut app.username_input)
                                .hint_text("Enter your username")
                                .desired_width(200.0),
                        );

                        ui.add_space(12.0);

                        ui.label(egui::RichText::new("🔒 Password").color(Color32::GRAY));
                        ui.add(
                            egui::TextEdit::singleline(&mut app.password_input)
                                .password(true)
                                .hint_text("Enter your password")
                                .desired_width(200.0),
                        );

                        ui.add_space(16.0);

                        if let Some(err) = &app.login_error {
                            ui.colored_label(Color32::RED, format!("❌ {}", err));
                            ui.add_space(8.0);
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("➡️ Log In")
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(Color32::from_rgb(70, 90, 255))
                                .rounding(8.0)
                                .min_size(egui::vec2(120.0, 32.0)),
                            )
                            .clicked()
                        {
                            if app.username_input == "admin" && app.password_input == "password" {
                                app.logged_in = true;
                                app.login_error = None;
                            } else {
                                app.login_error = Some("Invalid username or password".to_string());
                            }
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("📝 Register")
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(Color32::from_rgb(0, 150, 70))
                                .rounding(8.0)
                                .min_size(egui::vec2(120.0, 32.0)),
                            )
                            .clicked()
                        {
                            app.username_input.clear();
                            app.password_input.clear();
                            app.login_error = None;
                            println!("Register button clicked");
                        }

                        ui.add_space(12.0);

                        ui.hyperlink_to("Need help?", "https://example.com/help");
                    });
                });
        });
    });
}
