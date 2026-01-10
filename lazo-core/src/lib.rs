#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Clone)]
pub enum BlockKind {
    Model { function: String },
    Node { function: std::path::PathBuf }
}

#[derive(Debug)]
pub struct Block {
    pub id: String,
    pub pos: Vec2,
    pub kind: BlockKind,
    pub input: Vec<String>,
    pub output: Vec<String>
}

