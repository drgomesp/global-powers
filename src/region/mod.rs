pub mod country;
pub mod state;

use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}
