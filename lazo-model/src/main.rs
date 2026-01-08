use eframe::egui;
use egui::{Color32, RichText, Stroke, StrokeKind};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // 1. CLI ARGUMENT PARSING
    let args: Vec<String> = env::args().collect();
    
    // Captures the --project-path argument passed by the Lazo Main manager
    let project_path = if let Some(pos) = args.iter().position(|x| x == "--project-path") {
        args.get(pos + 1).cloned().unwrap_or_else(|| ".".to_string())
    } else {
        ".".to_string()
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("LAZO MODEL - SYSTEM DESIGNER"),
        ..Default::default()
    };

    eframe::run_native(
        "LAZO MODEL",
        options,
        Box::new(move |cc| {
            // Apply QOREX SCITECH industrial gold theme
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = Color32::from_rgb(5, 5, 5); // Background #050505
            visuals.override_text_color = Some(Color32::WHITE);
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(LazoModel::new(project_path)))
        }),
    )
}

struct LazoModel {
    project_path: String,
    project_name: String,
    error_msg: Option<String>,
}

impl LazoModel {
    fn new(path: String) -> Self {
        let mut model = Self {
            project_path: path.clone(),
            project_name: "Unnamed Project".to_string(),
            error_msg: None,
        };
        model.sync_with_project_file();
        model
    }

    /// Synchronizes the editor with the shared project.toml file
    fn sync_with_project_file(&mut self) {
        let toml_file = PathBuf::from(&self.project_path).join("project.toml");
        
        match fs::read_to_string(&toml_file) {
            Ok(content) => {
                // Simple parsing to extract project_name
                if let Some(line) = content.lines().find(|l| l.starts_with("project_name")) {
                    self.project_name = line.replace("project_name = ", "").replace("\"", "");
                }
            }
            Err(_) => {
                self.error_msg = Some("CRITICAL: project.toml not found in the specified path.".to_string());
            }
        }
    }
}

impl eframe::App for LazoModel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- DESIGNER TOP BAR ---
        egui::TopBottomPanel::top("model_top_bar")
            .frame(egui::Frame::default().inner_margin(10.0).fill(Color32::from_white_alpha(5)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("SYSTEM DESIGNER").strong().color(Color32::from_rgb(200, 160, 0)));
                    ui.separator();
                    ui.label(format!("PROJECT: {}", self.project_name));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(&self.project_path).color(Color32::from_gray(100)).size(10.0));
                    });
                });
            });

        // --- DESIGN CANVAS (CENTRAL AREA) ---
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(5, 5, 5)))
            .show(ctx, |ui| {
                if let Some(error) = &self.error_msg {
                    ui.centered_and_justified(|ui| {
                        ui.label(RichText::new(error).color(Color32::RED).size(18.0).strong());
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label(RichText::new(format!("HELLO, WORLD FROM {}", self.project_name)).size(32.0).strong());
                        ui.label(RichText::new("MATHEMATICAL MODELING & LOGIC DESIGN").color(Color32::from_gray(110)));
                        
                        ui.add_space(40.0);
                        
                        // Workspace boundary visualization
                        let rect = ui.available_rect_before_wrap().shrink(20.0);
                        ui.painter().rect_stroke(
                            rect, 
                            2.0, 
                            egui::Stroke::new(1.0, Color32::from_white_alpha(20)),
                            egui::StrokeKind::Middle
                        );
                        
                        ui.painter().text(
                            rect.center(), 
                            egui::Align2::CENTER_CENTER, 
                            "VISUAL GRAPH EDITOR READY", 
                            egui::FontId::monospace(14.0), 
                            Color32::from_gray(60)
                        );
                    });
                }
            });

        // --- STATUS FOOTER ---
        egui::TopBottomPanel::bottom("model_footer")
            .frame(egui::Frame::default().inner_margin(6.0).fill(Color32::from_white_alpha(3)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("MODE: SCHEMATIC").color(Color32::from_gray(70)).size(10.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("X: 0.0 Y: 0.0").color(Color32::from_gray(70)).size(10.0));
                    });
                });
            });
    }
}