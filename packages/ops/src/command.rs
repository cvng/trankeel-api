use crate::error::Result;
use crate::event::Event;

/// Command.
pub trait Command {
    type Input;

    fn run(self, input: Self::Input) -> Result<Vec<Event>>;
}
