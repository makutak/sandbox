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
            if self.is_strike(frame_index) {
                score += self.strike_bonus(frame_index);
                frame_index += 1;
            }
            if self.is_spare(frame_index) {
                score += self.spare_bonus(frame_index);
                frame_index += 2;
            } else {
                score += self.sum_of_balls_in_frame(frame_index);
                frame_index += 2;
            }
        }

        score
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        return self.rolls[frame_index] == 10;
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        return self.rolls[frame_index] + self.rolls[frame_index + 1] == 10;
    }

    fn sum_of_balls_in_frame(&self, frame_index: usize) -> u16 {
        return self.rolls[frame_index] + self.rolls[frame_index + 1];
    }

    fn spare_bonus(&self, frame_index: usize) -> u16 {
        return 10 + self.rolls[frame_index + 2];
    }

    fn strike_bonus(&self, frame_index: usize) -> u16 {
        return 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
    }
}
