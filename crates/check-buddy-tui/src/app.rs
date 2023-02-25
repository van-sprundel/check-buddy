use check_buddy_core::BoardMap;

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub board_map: BoardMap,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec!["Start", "Engine analysis"],
            index: 0,
            board_map: BoardMap::starting(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
