//! Bit Manipulation Instruction (BMI) Set 1.0.
//!
//! For a quick overview see
//! [wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI1_.28Bit_Manipulation_Instruction_Set_1.29).
//! The reference is [Intel 64 and IA-32 Architectures Software Developer's
//! Manual Volume 2: Instruction Set Reference,
//! A-Z](http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf).
//!
//! It consists of the following instructions:
//!
//! - `ANDN`
//! - `BEXTR`
//! - `BLSI`
//! - `BLSMSK`
//! - `BLSR`
//! - `LZCNT` (note: it is _officially_ part of BMI1, but Intel CPUs
//!   advertise it as ABM).
//!

mod bextr;
mod bextri;

pub use bmi::bextr::*;
pub use bmi::bextri::*;
pub use alg::bmi::andn::*;
pub use alg::bmi::blsi::*;
pub use alg::bmi::blsmsk::*;
pub use alg::bmi::blsr::*;
pub use alg::bmi::tzcnt::*;