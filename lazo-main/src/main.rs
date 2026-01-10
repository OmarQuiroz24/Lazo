use eframe::egui;
use egui::{Color32, RichText, Stroke};
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_active(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Lazo",
        options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = Color32::from_rgb(5, 5, 5);
            visuals.override_text_color = Some(Color32::WHITE);
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(LazoMain::default()))
        }),
    )
}

struct LazoMain {
    project_name: String,
    project_base_path: String,
    show_modal: bool,
    pending_module: Option<String>,
}

impl Default for LazoMain {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            project_base_path: String::new(),
            show_modal: false,
            pending_module: None,
        }
    }
}

impl eframe::App for LazoMain {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Top HUD ---
        egui::TopBottomPanel::top("top_bar")
            .frame(egui::Frame::default().inner_margin(12.0).fill(Color32::from_white_alpha(5)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Q O R E X  S C I T E C H").color(Color32::from_gray(140)).strong().size(12.0));
                        ui.label(RichText::new("SYSTEM INTELLIGENCE CORE").color(Color32::from_gray(140)).size(9.0));
                    });
                    ui.add_space(30.0);
                    ui.label(RichText::new("SYSTEM READY").color(Color32::from_gray(140)).size(10.0));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("OMAR QUIROZ").strong().color(Color32::from_gray(140)).size(11.0));
                    });
                });
            });

        // --- Main Content ---
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(5, 5, 5))) 
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() * 0.1);
                    ui.label(RichText::new("L A Z O").color(Color32::from_gray(140)).size(80.0).strong());
                    ui.label(RichText::new("ENGINEERING AND SIMULATION SUITE").color(Color32::from_gray(140)).size(16.0));
                    ui.add_space(60.0);
                });

                let spacing = 40.0;
                let column_width = (ui.available_width() - (spacing * 3.0)) / 2.0;
                
                ui.horizontal(|ui| {
                    ui.add_space(spacing);
                    
                    // Node Module
                    ui.vertical(|ui| {
                        let btn = ui.add_sized([column_width, 200.0], egui::Button::new(RichText::new("> NODE").color(Color32::from_gray(140)).size(28.0))
                            .fill(Color32::from_rgb(20, 5, 5))
                            .stroke(Stroke::new(1.5, Color32::from_rgb(200, 0, 0))));
                        if btn.clicked() {
                            self.pending_module = Some("NODE".to_string());
                            self.show_modal = true;
                        }
                    });

                    ui.add_space(spacing);

                    // Model Module
                    ui.vertical(|ui| {
                        let btn = ui.add_sized([column_width, 200.0], egui::Button::new(RichText::new("> MODEL").color(Color32::from_gray(140)).size(28.0))
                            .fill(Color32::from_rgb(20, 18, 5))
                            .stroke(Stroke::new(1.5, Color32::from_rgb(200, 160, 0))));
                        if btn.clicked() {
                            self.pending_module = Some("MODEL".to_string());
                            self.show_modal = true;
                        }
                    });
                });
            });

        // --- Project Initialization Modal ---
        if self.show_modal {
            let module = self.pending_module.clone().unwrap_or_default();
            let accent = if module == "NODE" { Color32::from_rgb(200, 0, 0) } else { Color32::from_rgb(200, 160, 0) };

            egui::Window::new("PROJECT INITIALIZATION")
                .pivot(egui::Align2::CENTER_CENTER)
                .fixed_pos(ctx.content_rect().center()) 
                .frame(egui::Frame::window(&ctx.style()).fill(Color32::from_rgb(10, 10, 10)).stroke(Stroke::new(1.0, accent)))
                .show(ctx, |ui| {
                    ui.set_width(500.0);
                    ui.label("PROJECT NAME");
                    ui.text_edit_singleline(&mut self.project_name);
                    
                    ui.add_space(15.0);
                    ui.label("STORAGE LOCATION");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.project_base_path);
                        if ui.button("BROWSE").clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.project_base_path = path.display().to_string();
                            }
                        }
                    });

                    ui.add_space(20.0);
                    let can_start = !self.project_name.trim().is_empty() && !self.project_base_path.is_empty();
                    ui.horizontal(|ui| {
                        if ui.add_enabled(can_start, egui::Button::new("START")).clicked() {
                            self.launch_and_create_project();
                        }
                        if ui.button("ABORT").clicked() { self.show_modal = false; }
                    });
                });
        }
    }
}

impl LazoMain {
    fn launch_and_create_project(&mut self) {
        let name = self.project_name.trim();
        let full_path = PathBuf::from(&self.project_base_path).join(name);

        if !full_path.exists() {
            fs::create_dir_all(&full_path).ok();
            let toml = format!("project_name = \"{}\"\nversion = \"0.1.0\"", name);
            fs::write(full_path.join("project.toml"), toml).ok();
        }

        if let Some(module) = &self.pending_module {
            let bin = if cfg!(target_os = "windows") { format!("lazo-{}.exe", module.to_lowercase()) } else { format!("lazo-{}", module.to_lowercase()) };
            Command::new(format!("./target/debug/{}", bin))
                .arg("--project-path")
                .arg(&full_path)
                .spawn()
                .ok();
        }
        self.show_modal = false;
        self.project_name.clear();
    }
}