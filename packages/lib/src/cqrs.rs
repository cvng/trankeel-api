pub(crate) trait Command {
    type Input;
    type State;
    type Payload;

    fn run(input: Self::Input, state: Self::State) -> crate::error::Result<Self::Payload>;
}
