// finished in 22 minutes, written without reading instructions
// added scores in 12 minutes

struct Game {
    rolls: [u32; 21],
    current_roll: usize,
    scores: [u32; 10],
}

impl Game {
    fn new() -> Game {
        Game {
            rolls: [0; 21],
            current_roll: 0,
            scores: [0; 10],
        }
    }

    fn roll(&mut self, pins: u32) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
        self.update_scores();
    }

    fn score(&mut self) -> u32 {
        self.scores[9]
    }

    fn update_scores(&mut self) {
        let mut score = 0;
        let mut roll_index = 0;
        for frame in 0..10 {
            if self.is_strike(roll_index) {
                score += self.score_for_strike_frame(roll_index);
                roll_index += 1;
            } else if self.is_spare(roll_index) {
                score += self.score_for_spare_frame(roll_index);
                roll_index += 2;
            } else {
                score += self.score_for_normal_frame(roll_index);
                roll_index += 2;
            }
            self.scores[frame] = score;
        }
    }

    fn is_spare(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] + self.rolls[roll_index + 1] == 10
    }

    fn is_strike(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] == 10
    }

    fn score_for_normal_frame(&self, roll_index: usize) -> u32 {
        self.rolls[roll_index] + self.rolls[roll_index + 1]
    }

    fn score_for_spare_frame(&self, roll_index: usize) -> u32 {
        10 + self.rolls[roll_index + 2]
    }

    fn score_for_strike_frame(&self, roll_index: usize) -> u32 {
        10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Game {
        fn roll_many(&mut self, n: u32, pins: u32) {
            for _ in 0..n {
                self.roll(pins)
            }
        }

        fn roll_spare(&mut self) {
            self.roll(3);
            self.roll(7);
        }

        fn roll_strike(&mut self) {
            self.roll(10);
        }
    }

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        game.roll_many(20, 0);
        assert_eq!(0, game.score());
        assert_eq!([0; 10], game.scores); // game.score() needs to be called to get correct scores
    }

    #[test]
    fn test_oll_ones() {
        let mut game = Game::new();
        game.roll_many(20, 1);
        assert_eq!(20, game.score());
        assert_eq!([2, 4, 6, 8, 10, 12, 14, 16, 18, 20], game.scores);
    }

    #[test]
    fn test_one_spare() {
        let mut game = Game::new();
        game.roll_spare();
        game.roll(5);
        game.roll_many(17, 0);
        assert_eq!(20, game.score());
        assert_eq!([15, 20, 20, 20, 20, 20, 20, 20, 20, 20], game.scores);
    }

    #[test]
    fn test_one_strike() {
        let mut game = Game::new();
        game.roll_strike();
        game.roll(2);
        game.roll(4);
        game.roll_many(17, 0);
        assert_eq!(22, game.score());
        assert_eq!([16, 22, 22, 22, 22, 22, 22, 22, 22, 22], game.scores);
    }

    #[test]
    fn test_perfect_game() {
        let mut game = Game::new();
        game.roll_many(12, 10);
        assert_eq!(300, game.score());
        assert_eq!([30, 60, 90, 120, 150, 180, 210, 240, 270, 300], game.scores);
    }
}
