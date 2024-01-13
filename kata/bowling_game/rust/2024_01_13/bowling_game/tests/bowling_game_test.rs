use bowling_game::Game;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_gutter_game() {
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(0);
        }
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(1);
        }
        assert_eq!(game.score(), 20);
    }
}
