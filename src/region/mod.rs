pub mod country;
pub mod state;

use std::fmt::Debug;

#[derive(Debug)]
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
