use check_buddy_core::BoardMap;

pub struct App<'a> {
    pub board_map: BoardMap,
    pub tab_titles: Vec<&'a str>,
    pub index: usize,
    pub input: String,
    pub move_history: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            board_map: BoardMap::starting(),
            tab_titles: vec!["Start", "Engine analysis"],
            index: 0,
            input: String::new(),
            move_history: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tab_titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tab_titles.len() - 1;
        }
    }
}
