pub mod chat;
pub mod home;
pub mod posts;
pub mod search;

use crate::app::MyApp;
use eframe::egui;

#[derive(PartialEq)]
pub enum View {
    Home,
    Chat,
    Search,
    Posts,
}

pub fn show_views(app: &mut MyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("🏠 Home").clicked() {
                app.current_view = View::Home;
            }
            if ui.button("💬 Chat").clicked() {
                app.current_view = View::Chat;
            }
            if ui.button("🔍 Search").clicked() {
                app.current_view = View::Search;
            }
            if ui.button("📜 Posts").clicked() {
                app.current_view = View::Posts;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🚪 Logout").clicked() {
                    app.logged_in = false;
                    app.username_input.clear();
                    app.password_input.clear();
                }
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| match app.current_view {
        View::Home => home::show(ui),
        View::Chat => app.chats.show(ui),
        View::Search => search::show(ui),
        View::Posts => posts::show(ui),
    });
}
