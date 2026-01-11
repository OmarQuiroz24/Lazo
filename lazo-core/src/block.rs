use crate::terminal::{Terminal, TerminalKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub id: String,
    pub kind: BlockKind,
    pub function: String,
    pub size: egui::Vec2,
    pub pos: egui::Pos2,
    pub input: Vec<Terminal>,
    pub output: Vec<Terminal>,
    pub is_flipped: bool
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockKind {
    Model(ModelKind),
    Node(NodeKind)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelKind {
    Process,
    Summing,
    Input,
    Output
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Rust,
    Python,
    Input,
    Output
}

impl Block {
    pub fn new(id: &str, kind: BlockKind, function: &str, size: egui::Vec2, pos: egui::Pos2, is_flipped: bool) -> Self {
        let mut block = Self {
            id: id.to_string(),
            kind,
            function: function.to_string(),
            size,
            pos,
            input: Vec::new(),
            output: Vec::new(),
            is_flipped
        };

        block.config();

        block
    }

    pub fn config(&mut self) {
        self.input.clear();
        self.output.clear();

        match &self.kind {
            BlockKind::Model(model_kind) => {
                match model_kind {
                    ModelKind::Process => {
                        self.size = egui::Vec2 { x: 100.0, y: 80.0 };
                        self.input.push(Terminal::new(&format!("{}_in_0", self.id), TerminalKind::Input, 1.0, egui::vec2(10.0, 10.0)));
                        self.output.push(Terminal::new(&format!("{}_out_0", self.id), TerminalKind::Output, 1.0, egui::vec2(10.0, 10.0)));
                    }
                    ModelKind::Summing => {
                        self.size = egui::Vec2 { x: 80.0, y: 80.0 };
                        self.input.push(Terminal::new(&format!("{}_in_0", self.id), TerminalKind::Input, 1.0, egui::vec2(10.0, 10.0)));
                        self.input.push(Terminal::new(&format!("{}_in_1", self.id), TerminalKind::Input, 1.0, egui::vec2(10.0, 10.0)));
                        self.output.push(Terminal::new(&format!("{}_out_0", self.id), TerminalKind::Output, 1.0, egui::vec2(10.0, 10.0)));
                    }
                    ModelKind::Input => {
                        self.size = egui::Vec2 { x: 100.0, y: 80.0 };
                        self.output.push(Terminal::new(&format!("{}_out_0", self.id), TerminalKind::Output, 1.0, egui::vec2(10.0, 10.0)));
                    }
                    ModelKind::Output => {
                        self.size = egui::Vec2 { x: 100.0, y: 80.0 };
                        self.input.push(Terminal::new(&format!("{}_in_0", self.id), TerminalKind::Input, 1.0, egui::vec2(10.0, 10.0)));
                    }

                }
            }
            // Pending For Upcoming Commits
            BlockKind::Node(node_kind) => {
                match node_kind {
                    NodeKind::Rust => {

                    }
                    NodeKind::Python => {

                    }
                    NodeKind::Input => {

                    }
                    NodeKind::Output => {

                    }
                }
            }
        }
    }
}