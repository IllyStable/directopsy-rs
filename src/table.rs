pub struct Table {
    cols: Vec<Column>
}

enum Position {
    Left,
    Central,
    Right,
}

pub struct Column {
    title: String,
    data: Vec<String>,
    position: Position,
}

/*impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)

        let mut row = 0;
    }
}*/
