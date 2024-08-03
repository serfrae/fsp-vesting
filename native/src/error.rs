use num_derive::FromPrimitive;
use num_traits::FromPrimitive as FromPrimitiveTrait;
use solana_program::{
	decode_error::DecodeError,
	msg,
	program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum VestingError {
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<VestingError> for ProgramError {
	fn from(e: VestingError) -> Self {
		ProgramError::Custom(e as u32)
	}
}

impl<T> DecodeError<T> for VestingError {
	fn type_of() -> &'static str {
		"Vesting error"
	}
}

impl PrintProgramError for VestingError {
	fn print<E>(&self)
	where
		E: 'static + std::error::Error + DecodeError<E> + FromPrimitiveTrait + PrintProgramError,
	{
		msg!(&self.to_string())
	}
}
