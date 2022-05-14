use piston_window::{Context, G2d};

use crate::ball::Ball;
use crate::paddle::Paddle;

pub struct Game {
    ball: Ball,
    player_paddle: Paddle,
    player_score: u16,
    opponent_paddle: Paddle,
    opponent_score: u16,
    canvas_dim: [f64; 2],
}

impl Game {
    pub fn new(canvas_dim: [f64; 2]) -> Game {
        Game {
            ball: Ball::new(canvas_dim[0] / 2., canvas_dim[1] / 2., canvas_dim),
            player_paddle: Paddle::new(0., 200., true),
            player_score: 0,
            opponent_paddle: Paddle::new(canvas_dim[0] - 10., 200., false),
            opponent_score: 0,
            canvas_dim: canvas_dim,
        }
    }

    pub fn update_game(&mut self, con: &Context, g: &mut G2d, key_pressed: char, held: bool) {
        self.player_paddle.update_player(key_pressed, held);
        self.player_paddle.display(&con, g);
        self.opponent_paddle.update_opponent(self.ball.pos_y);
        self.opponent_paddle.display(&con, g);
        self.hits_ball();
        self.ball.display(&con, g);
    }

    fn hits_ball(&mut self) {
        if self.ball.pos_x <= 10. {
            if (self.ball.pos_y >= self.player_paddle.pos_y)
                & (self.ball.pos_y <= self.player_paddle.pos_y + 100.)
            {
                self.ball.vel_x *= -1.;
            }
        } else if self.ball.pos_x <= 0. {
            self.opponent_score += 1;
        }
        if self.ball.pos_x >= self.canvas_dim[0] - 10. {
            if (self.ball.pos_y >= self.opponent_paddle.pos_y)
                & (self.ball.pos_y <= self.opponent_paddle.pos_y + 100.)
            {
                self.ball.vel_x *= -1.;
            }
        } else if self.ball.pos_x >= self.canvas_dim[0] {
            self.player_score += 1;
        }
    }
}

