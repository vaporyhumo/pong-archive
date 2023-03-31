use crate::ball::Ball;

mod ball;

const BLOCK_SIZE: usize = 20;
const BALL_SPEED: usize = 20;
const GAME_WIDTH: usize = 800;
const GAME_HEIGHT: usize = 600;

fn main() {
  println!("Hello, world!");
  let mut ball: Ball = Ball::new(0, 0, ball::Direction::NorthEast);
  loop {
    println!("Ball: {:#?}", ball);

    if ball.touching_top() {
      ball = ball.face_south();
    }
    if ball.touching_bottom() {
      ball = ball.face_north();
    }
    if ball.touching_left() {
      ball = ball.face_east();
    }
    if ball.touching_right() {
      ball = ball.face_west();
    }

    ball = ball.move_once();
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
}
