use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct DifferentKeyComp;

impl fmt::Display for DifferentKeyComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Comparing two records with different keys")
    }
}

impl Error for DifferentKeyComp {}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyAnswerStackCheck;

impl fmt::Display for EmptyAnswerStackCheck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Checking answer before getting any tasks")
    }
}

impl Error for EmptyAnswerStackCheck {}
