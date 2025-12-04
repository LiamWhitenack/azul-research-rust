#[derive(Copy, Clone, Debug)]
pub struct PatternLine {
    pub count: i8,
    pub color: i8,
    pub potential_colors: [bool; 6],
}

impl PatternLine {
    pub fn new() -> Self {
        Self {
            count: 0,
            color: -1,
            potential_colors: [true; 6],
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct PatternLines([PatternLine; 6]);

impl PatternLines {
    pub fn new() -> Self {
        Self([PatternLine::new(); 6])
    }

    pub fn line(&self, index: usize) -> &PatternLine {
        &self.0[index]
    }


    pub fn set_line(&mut self, index: usize, line: PatternLine) {
        self.0[index] = line;
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, PatternLine> {
        self.0.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, PatternLine> {
        self.0.iter()
    }
}


#[derive(Clone, Debug)]
pub struct GameProgression {
    pub score: i8,
    pub patterns: Vec<PatternLines>,
    pub wall: [[bool; 6]; 6],
}

impl GameProgression {
    pub fn new(score: i8, patterns: Vec<PatternLines>, wall: [[bool; 6]; 6]) -> Self {
        Self {
            score,
            patterns,
            wall,
        }
    }
}

