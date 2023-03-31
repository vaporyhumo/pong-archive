use crate::{GAME_WIDTH, BLOCK_SIZE, GAME_HEIGHT, BALL_SPEED};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
  NorthWest,
  NorthEast,
  SouthWest,
  SouthEast
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct Ball {
  x: usize,
  y: usize,
  dir: Direction
}

impl Ball {
  pub fn new(x: usize, y: usize, dir: Direction) -> Ball {
    Ball { x: x, y: y, dir: dir }
  }

  pub fn move_once(&self) -> Ball {
    let (x, y) = match self.dir {
      Direction::NorthWest => (self.x - BALL_SPEED, self.y - BALL_SPEED),
      Direction::NorthEast => (self.x + BALL_SPEED, self.y - BALL_SPEED),
      Direction::SouthWest => (self.x - BALL_SPEED, self.y + BALL_SPEED),
      Direction::SouthEast => (self.x + BALL_SPEED, self.y + BALL_SPEED)
    };
    Ball { x: x, y: y, dir: self.dir }
  }

  pub fn invert_x(&self) -> Ball {
    let dir = match self.dir {
      Direction::NorthWest => Direction::NorthEast,
      Direction::NorthEast => Direction::NorthWest,
      Direction::SouthWest => Direction::SouthEast,
      Direction::SouthEast => Direction::SouthWest
    };
    Ball { x: self.x, y: self.y, dir: dir }
  }

  pub fn invert_y(&self) -> Ball {
    let dir = match self.dir {
      Direction::NorthWest => Direction::SouthWest,
      Direction::NorthEast => Direction::SouthEast,
      Direction::SouthWest => Direction::NorthWest,
      Direction::SouthEast => Direction::NorthEast
    };
    Ball { x: self.x, y: self.y, dir: dir }
  }

  pub fn face_south(&self) -> Ball {
    let dir: Direction = match self.dir {
      Direction::NorthWest => Direction::SouthWest,
      Direction::NorthEast => Direction::SouthEast,
      Direction::SouthWest => Direction::SouthWest,
      Direction::SouthEast => Direction::SouthEast,
    };
    Ball { x: self.x, y: self.y, dir: dir }
  }

  pub fn face_north(&self) -> Ball {
    let dir: Direction = match self.dir {
      Direction::NorthWest => Direction::NorthWest,
      Direction::NorthEast => Direction::NorthEast,
      Direction::SouthWest => Direction::NorthWest,
      Direction::SouthEast => Direction::NorthEast,
    };
    Ball { x: self.x, y: self.y, dir: dir }
  }

  pub fn face_east(&self) -> Ball {
    let dir: Direction = match self.dir {
      Direction::NorthWest => Direction::NorthEast,
      Direction::NorthEast => Direction::NorthEast,
      Direction::SouthWest => Direction::SouthEast,
      Direction::SouthEast => Direction::SouthEast,
    };
    Ball { x: self.x, y: self.y, dir: dir }
  }

  pub fn face_west(&self) -> Ball {
    let dir: Direction = match self.dir {
      Direction::NorthWest => Direction::NorthWest,
      Direction::NorthEast => Direction::NorthWest,
      Direction::SouthWest => Direction::SouthWest,
      Direction::SouthEast => Direction::SouthWest,
    };
    Ball { x: self.x, y: self.y, dir: dir }
  }

  pub fn touching_top(&self) -> bool {
    self.y == 0
  }

  pub fn touching_bottom(&self) -> bool {
    self.y == GAME_HEIGHT - BLOCK_SIZE
  }

  pub fn touching_left(&self) -> bool {
    self.x == 0
  }

  pub fn touching_right(&self) -> bool {
    self.x == GAME_WIDTH - BLOCK_SIZE
  }
}
