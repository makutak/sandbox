use bowling_game::Game;

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_many(game: &mut Game, n: u16, pins: u16) {
        for _ in 0..n {
            game.roll(pins)
        }
    }

    fn roll_standard_spare(game: &mut Game) {
        game.roll(5);
        game.roll(5);
    }

    fn roll_different_spare(game: &mut Game) {
        game.roll(3);
        game.roll(7);
    }

    fn roll_strike(game: &mut Game) {
        game.roll(10);
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

    #[test]
    fn test_standard_spare() {
        let mut game = Game::new();
        roll_standard_spare(&mut game);
        game.roll(3);
        roll_many(&mut game, 17, 0);
        assert_eq!(game.score(), 16);
    }

    #[test]
    fn test_different_spare() {
        let mut game = Game::new();
        roll_different_spare(&mut game);
        game.roll(3);
        roll_many(&mut game, 17, 0);
        assert_eq!(game.score(), 16);
    }

    #[test]
    fn test_one_strike() {
        let mut game = Game::new();
        roll_strike(&mut game);
        game.roll(3);
        game.roll(4);
        roll_many(&mut game, 16, 0);
        assert_eq!(game.score(), 24);
    }
}
