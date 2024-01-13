pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn roll(&mut self, _pins: u16) {}

    pub fn score(&self) -> u16 {
        0
    }
}
