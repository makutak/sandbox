pub struct Game {
    rolls: [u16; 21],
    current_roll: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            rolls: [0; 21],
            current_roll: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    pub fn score(&self) -> u16 {
        let mut score: u16 = 0;
        let mut frame_index: usize = 0;

        for _ in 0..10 {
            if self.rolls[frame_index] == 10 {
                score += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            }
            if self.is_spare(frame_index) {
                score += 10 + self.rolls[frame_index + 2];
                frame_index += 2;
            } else {
                score += self.rolls[frame_index] + self.rolls[frame_index + 1];
                frame_index += 2;
            }
        }

        score
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        return self.rolls[frame_index] + self.rolls[frame_index + 1] == 10;
    }
}
