#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    line: u32,
    column: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenPosition {
    start: Position,
    end: Position,
}

impl Position {
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> u32 {
        self.column
    }
}

impl TokenPosition {
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    pub fn start(&self) -> Position {
        self.start
    }
    
    pub fn end(&self) -> Position {
        self.end
    }
}