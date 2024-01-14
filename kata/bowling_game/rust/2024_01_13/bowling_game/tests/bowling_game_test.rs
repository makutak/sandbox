use bowling_game::Game;

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_many(game: &mut Game, n: u16, pins: u16) {
        for _ in 0..n {
            game.roll(pins)
        }
    }

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 0);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 1);
        assert_eq!(game.score(), 20);
    }
}
