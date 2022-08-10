pub struct Position {
    pub idx: i32,
    pub line: i32,
    pub col: i32,

    pub file_name: String,
    pub file_contents: String,
}

// custom print function for this struct
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Position: \n\n{}, \n{}, \n{}, \n{}, \n{}",
            self.idx, self.line, self.col, self.file_name, self.file_contents
        )
    }
}

pub trait AdvancePosition {
    fn advance_position(&mut self, current_char: char);
}

impl AdvancePosition for Position {
    fn advance_position(&mut self, current_char: char) {
        self.idx += 1;
        self.col += 1;

        if (current_char == '\n') {
            self.line += 1;
            self.col = 0;
        }
    }
}
