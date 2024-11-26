use crate::traits::proof::Proof;

pub trait Verify {
    /// verification key
    type VK;

    /// function input
    type Input;

    /// function output
    type Output;

    /// proof
    type Proof: Proof;

    /// deterministic verification algorithm
    fn verify(vk: Self::VK, input: Self::Input, output: Self::Output, proof: Self::Proof) -> bool;
}
