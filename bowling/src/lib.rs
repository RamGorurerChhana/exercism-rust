#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: [Frame; 10],
    curr_frame: usize,
    pins_left: u16,
    allow_fill_ball: bool,
    throw_no: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: [Frame::default(); 10],
            curr_frame: 0,
            pins_left: 10,
            allow_fill_ball: false,
            throw_no: 0,
        }
    }

    /// determine whether the game is complete
    fn is_game_complete(&self) -> bool {
        !self.allow_fill_ball && self.curr_frame > 9
    }

    /// register the throw points in the corresponding frame
    fn add_roll_point(&mut self, pins: u16) {
        if self.curr_frame < 10 {
            self.frames[self.curr_frame].add_point(self.throw_no, pins);
        }
        if self.curr_frame > 0 {
            self.frames[self.curr_frame - 1].add_spare_point(pins);
            self.frames[self.curr_frame - 1].add_strike_point(pins);
        }
        if self.curr_frame > 1 {
            self.frames[self.curr_frame - 2].add_strike_point(pins);
        }
    }

    /// update the allow_fill_ball flag
    /// when the 10th frame has strike or spare then true otherwise false
    fn allow_fill_ball(&mut self) {
        self.allow_fill_ball = self.frames[9].count_spare
            || self.frames[9].count_strike_1
            || self.frames[9].count_strike_2;
    }

    /// update the curr_frame and other game states
    fn update_curr_frame(&mut self, pins: u16) {
        // for second throw in a frame
        // reset all values
        if self.throw_no == 1 {
            self.throw_no = 0;
            self.pins_left = 10;
            self.inc_curr_frame();
            return;
        }
        // for first throw in a frame
        // when strike occurred
        if pins == 10 {
            // for curr_frame 10 even if strike happen
            // we don't go to next frame
            self.throw_no = if self.curr_frame == 10 { 1 } else { 0 };
            self.pins_left = 10;
            self.inc_curr_frame();
        } else {
            // for open frame
            self.pins_left = 10 - pins;
            self.throw_no = 1;
        }
    }

    /// increment curr_frame value
    /// if the value is already 10 then stays the same
    fn inc_curr_frame(&mut self) {
        self.curr_frame = if self.curr_frame < 10 {
            self.curr_frame + 1
        } else {
            10
        };
    }

    /// register the throw points
    /// if the game is complete already or invalid no of pin count
    /// passed then throw error
    /// otherwise register the points and update the game state
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }
        if self.pins_left < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.add_roll_point(pins);
        self.allow_fill_ball();
        self.update_curr_frame(pins);
        Ok(())
    }

    /// get score of the game
    /// if the game is not complete it will return None
    /// otherwise the total score
    pub fn score(&self) -> Option<u16> {
        if self.is_game_complete() {
            let sum = self.frames.iter().map(|v| v.get_total()).sum::<u16>();
            Some(sum)
        } else {
            None
        }
    }
}

/// Frame struct to keep track of the points scored in each frame
/// In each frame at most 3 points can be counted
/// additionally 3 flags to keep track of strike/spare points
#[derive(Debug, Default, Clone, Copy)]
struct Frame {
    points: [u16; 3],
    count_strike_1: bool,
    count_strike_2: bool,
    count_spare: bool,
}

impl Frame {
    /// add throw point to a frame
    fn add_point(&mut self, throw_no: usize, pins: u16) {
        self.points[throw_no] = pins;
        if throw_no == 0 && pins == 10 {
            self.mark_strike();
        } else if throw_no == 1 && self.points[0] + self.points[1] == 10 {
            self.mark_spare();
        }
    }

    /// add a spare point to a frame
    fn add_spare_point(&mut self, pins: u16) {
        if self.count_spare {
            self.points[2] = pins;
            self.count_spare = false;
        }
    }

    /// add a strike point to a frame
    fn add_strike_point(&mut self, pins: u16) {
        if self.count_strike_1 {
            self.points[1] = pins;
            self.count_strike_1 = false;
        } else if self.count_strike_2 {
            self.points[2] = pins;
            self.count_strike_2 = false;
        }
    }

    /// get total point in the frame
    fn get_total(&self) -> u16 {
        self.points.into_iter().sum()
    }

    /// mark the flag when strike occurred
    fn mark_strike(&mut self) {
        self.count_strike_1 = true;
        self.count_strike_2 = true;
    }

    /// mark the flag when spare occurred
    fn mark_spare(&mut self) {
        self.count_spare = true;
    }
}
