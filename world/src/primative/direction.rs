use crate::primative::Error;
use std::str::FromStr;
use tracing::error;
use crate::geom_traits::Reverse;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub enum RotationDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

impl FromStr for RotationDirection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "CW" | "cw" | "clockwise" => Ok(RotationDirection::Clockwise),
            "CCW" | "ccw" | "counterclockwise" => Ok(RotationDirection::CounterClockwise),
            _ => {
                error!("RotationDirection can't be created from: {}", s);
                Err(Error::StrToRotationDirectionError)
            }
        }
    }
}

impl Reverse for RotationDirection {
    fn reverse(&self) -> Self {
        match self {
            RotationDirection::Clockwise => {RotationDirection::CounterClockwise}
            RotationDirection::CounterClockwise => {RotationDirection::Clockwise}
        }
    }

    fn reverse_mut(&mut self) {
        *self = match self {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_clockwise() {
        let direction = RotationDirection::Clockwise;
        assert_eq!(direction.reverse(), RotationDirection::CounterClockwise);
    }


    #[test]
    fn test_reverse_counterclockwise() {
        let direction = RotationDirection::CounterClockwise;
        assert_eq!(direction.reverse(), RotationDirection::Clockwise);
    }

    #[test]
    fn test_reverse_mut() {
        let mut direction = RotationDirection::Clockwise;
        direction.reverse_mut();
        assert_eq!(direction, RotationDirection::CounterClockwise);
    }
}

