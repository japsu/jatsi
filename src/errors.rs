use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum InvalidAction {
  NotYourTurn,
  WrongState,
  OutOfBounds,
  AlreadyOccupied,
  NotSelectable,
}

impl Display for InvalidAction {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match &self {
      Self::NotYourTurn => write!(f, "not your turn"),
      Self::WrongState => write!(f, "cannot perform this action in this state"),
      Self::OutOfBounds => write!(f, "out of bounds (this should'nt happen :)"),
      Self::AlreadyOccupied => write!(f, "the selected scoring row is already occupied"),
      Self::NotSelectable => write!(f, "the bonus row is not selectable"),
    }
  }
}

impl Error for InvalidAction {}
