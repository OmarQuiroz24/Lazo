#[derive(Debug, Clone, PartialEq)]
pub struct Terminal {
    pub id: String,
    pub kind: TerminalKind,
    pub sign: f32,
    pub size: egui::Vec2
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminalKind {
    Input,
    Output
}

impl Terminal {
    pub fn new(id: &str, kind: TerminalKind, sign: f32, size: egui::Vec2) -> Self {
        Self {
            id: id.to_string(),
            kind,
            sign,
            size
        }
    }
}