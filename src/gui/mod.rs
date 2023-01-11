#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe;
use eframe::egui;
use egui::{panel::Side, widgets, Id, InnerResponse, RawInput, TextBuffer};
//Create a gui struct

#[derive(Default)]
struct GuiApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    my_value: u32,
}

impl GuiApp {
    fn new() -> Self {
        //Init all the values that the gui will hold. probably the invoice data , statuses and other important data
        Self {
            allowed_to_close: false,
            show_confirmation_dialog: false,
            my_value: 0,
        }
    }
}

impl eframe::App for GuiApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([false, false]).show(ui, |ui| {
                ui.label("Project repo:");
                ui.add(widgets::Hyperlink::new("https://github.com/actuallydoc"));
            })
        });

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

pub fn entry() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Invoice GUI",
        options.clone(),
        Box::new(|_cc| Box::new(GuiApp::new())),
    );
}
