use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const PADDLE_COLOUR: Color = [1., 1., 1., 1.];
const PADDLE_WIDTH: f64 = 10.;
const PADDLE_HEIGHT: f64 = 100.;

pub struct Paddle {
    pos_x: f64,
    pub pos_y: f64,
    is_user: bool,
}

impl Paddle {
    pub fn new(pos_x: f64, pos_y: f64, is_user: bool) -> Paddle {
        Paddle {
            pos_x: pos_x,
            pos_y: pos_y,
            is_user: is_user,
        }
    }

    pub fn display(&self, con: &Context, g: &mut G2d) {
        rectangle(
            PADDLE_COLOUR,
            [self.pos_x, self.pos_y, PADDLE_WIDTH, PADDLE_HEIGHT],
            con.transform,
            g,
        );
    }

    pub fn update_player(&mut self, key: char, pressed: bool) {
        if self.is_user {
            if (key == 'u') & (pressed) {
                self.pos_y -= 5.;
            } else if (key == 'd') & (pressed) {
                self.pos_y += 5.;
            }
        }
    }

    pub fn update_opponent(&mut self, ball_pos_y: f64) {
        if self.is_user == false {
            if self.pos_y + (3. * PADDLE_HEIGHT / 4.) < ball_pos_y {
                self.pos_y += 3.;
            } else if self.pos_y + PADDLE_HEIGHT / 4. > ball_pos_y {
                self.pos_y -= 3.;
            }
        }
    }
}

