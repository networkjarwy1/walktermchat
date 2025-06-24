pub mod login;
pub mod views;

use eframe::egui;
use views::View;

use crate::app::views::chat::Chats;

pub struct MyApp {
    pub logged_in: bool,
    pub username_input: String,
    pub password_input: String,
    pub login_error: Option<String>,
    pub current_view: View,
    pub chats: Chats,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            logged_in: false,
            username_input: String::new(),
            password_input: String::new(),
            login_error: None,
            current_view: View::Home,
            chats: Chats {
                chats_num: 0,
                chats: Vec::new(),
                selected_chat: None,
                new_chat_input: String::new(),
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.logged_in {
            login::show_login(self, ctx);
        } else {
            views::show_views(self, ctx);
        }
    }
}
