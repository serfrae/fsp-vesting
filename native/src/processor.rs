use solana_program::{account_info::AccountInfo, pubkey::Pubkey, entrypoint::ProgramResult};

pub struct Processor;

impl Processor {
	pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
		unimplemented!();
	}
}
