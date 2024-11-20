pub trait KeyGen {
    const SEC_PARAM: usize;

    /// evaluation key
    type EK;

    /// verification key
    type VK;

    /// function input
    type Input;

    /// function output
    type Output;

    /// generate evaluation/verification keypair
    fn new(f: &dyn Fn(Self::Input) -> Self::Output) -> (Self::EK, Self::VK);
}
