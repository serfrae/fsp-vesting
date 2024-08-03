use crate::{error::VestingError, processor::Processor};
use solana_program::{
	account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
	pubkey::Pubkey,
};

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	data: &[u8],
) -> ProgramResult {
	if let Err(e) = Processor::process(program_id, accounts, data) {
		e.print::<VestingError>();
		return Err(e);
	}
	Ok(())
}
