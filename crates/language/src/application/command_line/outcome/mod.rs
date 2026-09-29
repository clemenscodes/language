use ddd::{Layered, PresentationLayer};
use std::process::ExitCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Outcome {
    Done,
    Error,
}

impl Layered for Outcome {
    type Layer = PresentationLayer;
}

impl From<Outcome> for ExitCode {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Done => Self::SUCCESS,
            Outcome::Error => Self::FAILURE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_done_outcome_leaves_the_status_of_a_success() {
        let outcome = Outcome::Done;

        let status = ExitCode::from(outcome);

        assert_eq!(format!("{status:?}"), format!("{:?}", ExitCode::SUCCESS));
    }

    #[test]
    fn an_error_leaves_the_status_of_a_failure() {
        let outcome = Outcome::Error;

        let status = ExitCode::from(outcome);

        assert_eq!(format!("{status:?}"), format!("{:?}", ExitCode::FAILURE));
    }
}
