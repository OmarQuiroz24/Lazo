use eframe::egui;
use egui::{Color32, RichText, Stroke, StrokeKind};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // 1. CLI ARGUMENT PARSING
    // Captures the --project-path argument passed by the Lazo Main manager
    let args: Vec<String> = env::args().collect();
    let project_path = if let Some(pos) = args.iter().position(|x| x == "--project-path") {
        args.get(pos + 1).cloned().unwrap_or_else(|| ".".to_string())
    } else {
        ".".to_string()
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("LAZO NODE - EXECUTION RUNTIME"),
        ..Default::default()
    };

    eframe::run_native(
        "LAZO NODE",
        options,
        Box::new(move |cc| {
            // Apply QOREX SCITECH industrial dark theme
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = Color32::from_rgb(5, 5, 5); // Background #050505
            visuals.override_text_color = Some(Color32::WHITE);
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(LazoNode::new(project_path)))
        }),
    )
}

struct LazoNode {
    project_path: String,
    project_name: String,
    is_running: bool,
}

impl LazoNode {
    fn new(path: String) -> Self {
        let mut node = Self {
            project_path: path.clone(),
            project_name: "Unnamed Project".to_string(),
            is_running: false,
        };
        node.sync_with_project_file();
        node
    }

    /// Reads the shared project.toml to sync with the Suite's context
    fn sync_with_project_file(&mut self) {
        let toml_file = PathBuf::from(&self.project_path).join("project.toml");
        if let Ok(content) = fs::read_to_string(&toml_file) {
            // Basic parsing of the project name
            if let Some(line) = content.lines().next() {
                self.project_name = line.replace("project_name = ", "").replace("\"", "");
            }
        }
    }
}

impl eframe::App for LazoNode {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- RUNTIME TOP BAR ---
        egui::TopBottomPanel::top("node_top_bar")
            .frame(egui::Frame::default().inner_margin(10.0).fill(Color32::from_white_alpha(5)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("NODE RUNTIME").strong().color(Color32::from_rgb(200, 0, 0)));
                    ui.separator();
                    ui.label(format!("LINKED PROJECT: {}", self.project_name));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let status_color = if self.is_running { Color32::GREEN } else { Color32::from_gray(100) };
                        ui.label(RichText::new(if self.is_running { "LIVE" } else { "STANDBY" }).color(status_color).strong());
                    });
                });
            });

        // --- EXECUTION MONITOR (CENTRAL) ---
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(5, 5, 5)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(RichText::new(format!("HELLO, WORLD FROM {}", self.project_name)).size(32.0).strong());
                    ui.add_space(10.0);
                    ui.label(RichText::new("HARDWARE ABSTRACTION LAYER & SIGNAL MONITOR").color(Color32::from_gray(120)));
                    
                    ui.add_space(50.0);

                    // Runtime Control Button
                    let btn_text = if self.is_running { "STOP EXECUTION" } else { "START RUNTIME" };
                    let btn_color = if self.is_running { Color32::from_rgb(60, 0, 0) } else { Color32::from_rgb(0, 40, 0) };
                    
                    if ui.add_sized([250.0, 60.0], egui::Button::new(RichText::new(btn_text).size(18.0)).fill(btn_color)).clicked() {
                        self.is_running = !self.is_running;
                    }

                    ui.add_space(40.0);

                    // Visual feedback area (Terminal/Oscilloscope placeholder)
                    let rect = ui.available_rect_before_wrap().shrink(30.0);
                    ui.painter().rect_stroke(
                        rect, 
                        2.0, 
                        egui::Stroke::new(1.0, Color32::from_white_alpha(15)),
                        egui::StrokeKind::Middle
                    );
                    ui.painter().text(
                        rect.center(), 
                        egui::Align2::CENTER_CENTER, 
                        "SYSTEM LOGS & SIGNAL TELEMETRY", 
                        egui::FontId::monospace(14.0), 
                        Color32::from_gray(60)
                    );
                });
            });

        // --- SYSTEM METRICS FOOTER ---
        egui::TopBottomPanel::bottom("node_footer")
            .frame(egui::Frame::default().inner_margin(6.0).fill(Color32::from_white_alpha(3)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("KERNEL: V0.1-STABLE").color(Color32::from_gray(70)).size(10.0));
                    ui.add_space(20.0);
                    ui.label(RichText::new("Uptime: 00:00:00").color(Color32::from_gray(70)).size(10.0));
                });
            });
    }
}