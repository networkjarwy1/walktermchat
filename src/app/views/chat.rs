use eframe::egui::{self, Ui};

pub struct Chats {
    pub chats_num: i16,
    pub chats: Vec<String>, // usernames/IPs/etc.
    pub selected_chat: Option<usize>,
    pub new_chat_input: String,
}

impl Chats {
    pub fn show(&mut self, ui: &mut Ui) {
        egui::SidePanel::left("chat_list_panel")
            .resizable(false)
            .default_width(200.0)
            .show_inside(ui, |ui| {
                ui.heading("Chats");

                for (i, chat_id) in self.chats.iter().enumerate() {
                    let selected = self.selected_chat == Some(i);
                    if ui.selectable_label(selected, chat_id).clicked() {
                        self.selected_chat = Some(i);
                    }
                }

                ui.separator();
                ui.label("➕ Start new chat:");

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.new_chat_input);
                    if ui.button("Add").clicked() {
                        if !self.new_chat_input.trim().is_empty() {
                            self.chats.push(self.new_chat_input.trim().to_string());
                            self.new_chat_input.clear();
                        }
                    }
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(index) = self.selected_chat {
                ui.heading(format!("Chat with {}", self.chats[index]));
                ui.label("📨 Chat messages would go here...");
                // Future: Show messages, input field, etc.
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a chat to start messaging.");
                });
            }
        });
    }
}
