pub trait Proof {
    /// evaluation key
    type EK;

    /// function input
    type Input;

    /// function output
    type Output;

    type Proof;

    /// deterministic worker algorithm
    fn new(key: Self::EK, input: Self::Input) -> (Self::Output, Self::Proof);
}
