// finished in 47 minutes, written without instructions, with test cases
// checked for solution in bowling_game once

struct Game {
    rolls: [u32; 21],
    current_roll: usize,
}
impl Game {
    fn new() -> Game {
        Game {
            rolls: [0; 21],
            current_roll: 0,
        }
    }

    fn roll(&mut self, pins: u32) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        let mut roll_index = 0;

        for _frame in 0..10 {
            if self.is_strike(roll_index) {
                score += 10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2];
                roll_index += 1;
            } else if self.is_spare(roll_index) {
                // spare
                score += 10 + self.rolls[roll_index + 2];
                roll_index += 2;
            } else {
                score += self.rolls[roll_index] + self.rolls[roll_index + 1];
                roll_index += 2;
            }
        }
        score
    }

    fn is_spare(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] + self.rolls[roll_index + 1] == 10
    }

    fn is_strike(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] == 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Game {
        fn roll_many(&mut self, n: u32, pins: u32) {
            for _ in 0..n {
                self.roll(pins);
            }
        }
    }

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        game.roll_many(20, 0);
        assert_eq!(0, game.score());
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        game.roll_many(20, 1);
        assert_eq!(20, game.score());
    }

    #[test]
    fn test_one_spare() {
        let mut game = Game::new();
        game.roll_many(2, 5); // spare
        game.roll(2);
        game.roll_many(17, 0);
        assert_eq!(14, game.score());
    }

    #[test]
    fn test_one_strike() {
        let mut game = Game::new();
        game.roll(10); // strike
        game.roll(3);
        game.roll(4);
        game.roll_many(17, 0);
        assert_eq!(24, game.score());
    }

    #[test]
    fn test_perfect_game() {
        let mut game = Game::new();
        game.roll_many(12, 10);
        assert_eq!(300, game.score());
    }
}

// test_gutter_game
// test_all_ones
// test_one_spare
// test_one_strike
// test_perfect_game
