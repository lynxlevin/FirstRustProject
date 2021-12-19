// finished in 64 minutes, written while reading instructions
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
        let mut frame_index = 0;
        for _frame in 0..10 {
            if self.is_strike(frame_index) {
                // strike
                score += 10 + self.strike_bonus(frame_index);
                frame_index += 1;
            } else if self.is_spare(frame_index) {
                score += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            } else {
                score += self.sum_of_pins_in_frame(frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn sum_of_pins_in_frame(&self, frame_index: usize) -> u32 {
        self.rolls[frame_index] + self.rolls[frame_index + 1]
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index + 1] == 10
    }

    fn strike_bonus(&self, frame_index: usize) -> u32 {
        self.rolls[frame_index + 1] + self.rolls[frame_index + 2]
    }

    fn spare_bonus(&self, frame_index: usize) -> u32 {
        self.rolls[frame_index + 2]
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

        fn assert_score(&self, score: u32) {
            assert_eq!(score, self.score());
        }

        fn roll_spare(&mut self) {
            self.roll(5);
            self.roll(5);
        }

        fn roll_strike(&mut self) {
            self.roll(10);
        }
    }

    #[test]
    fn test_gutter_game() {
        let mut g = Game::new();
        g.roll_many(20, 0);
        g.assert_score(0);
    }

    #[test]
    fn test_all_ones() {
        let mut g = Game::new();
        g.roll_many(20, 1);
        g.assert_score(20);
    }

    #[test]
    fn test_one_spare() {
        let mut g = Game::new();
        g.roll_spare();
        g.roll(3);
        g.roll_many(17, 0);
        g.assert_score(16);
    }

    #[test]
    fn test_one_strike() {
        let mut g = Game::new();
        g.roll_strike();
        g.roll(3);
        g.roll(4);
        g.roll_many(16, 0);
        g.assert_score(24);
    }

    #[test]
    fn test_perfect_game() {
        let mut g = Game::new();
        g.roll_many(12, 10);
        g.assert_score(300);
    }
}
