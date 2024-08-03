use {
	crate::{error::VestingError, state::Frequency},
	solana_program::{
		clock::UnixTimestamp,
		instruction::{AccountMeta, Instruction},
		program_error::ProgramError,
		program_option::COption,
		pubkey::Pubkey,
	},
};

const PUBKEY_BYTES: usize = 32;
const BYTES_64: usize = 8;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum VestingInstruction<'a> {
	/// Initialises a vesting schedule
	///
	/// Accounts expected:
	/// 0. `[w]` Vesting schedule account
	/// 1. `[w, s]` Payer
	/// 2. `[]` System program
	///
	/// Optional accounts:
	/// 3. `[w]` Vault account (Vesting schedule ATA)
	/// 4. `[]` Token program
	/// 5. `[]` Associated token program
	InitVestingSchedule {
		authority: Pubkey,
		mint: Pubkey,
		schedule: Frequency,
		start: UnixTimestamp,
		duration: i64,
		vault: COption<Pubkey>,
	},

	/// Creates a vesting account
	///
	/// Accounts expected:
	///
	/// 0. `[]` Vesting schedule account
	/// 1. `[w, s]` Authority
	/// 2. `[]` Mint
	/// 3. `[w]` Vesting account
	/// 4. `[w]` Vesting account ATA
	/// 3. `[]` System program
	/// 4. `[]` Token program
	/// 5. `[]` Associated token program
	CreateAccount { owner: Pubkey, amount: u64 },

	/// Amend amount
	///
	/// Accounts expected:
	///
	/// 0. `[]` Vesting schedule account
	/// 1. `[w, s]` Authority
	/// 2. `[w]` Vesting account ATA
	/// 2. `[]` Token program
	AmendAmount { amount: u64 },

	/// Amend the vesting schedule
	///
	/// Accounts expected:
	///
	/// 0. `[]` Vesting schedule account
	/// 1. `[w, s]` Authority
	AmendSchedule {
		start: Option<UnixTimestamp>,
		schedule: Option<Frequency>,
		duration: Option<i64>,
	},

	/// Claim vested tokens
	///
	/// Accounts expected:
	///
	/// 0. `[]` Vesting schedule
	/// 1. `[]` Mint
	/// 2. `[w]` Vesting account
	/// 3. `[w]` Vesting account ATA
	/// 4. `[w,s]` Recipient wallet
	/// 5. `[w]` Reciepients ATA
	/// 6. `[]` System program
	/// 7. `[]` Token program
	/// 8. `[]` Associated token program
	Claim,

	/// Closes a vesting account and its ATA
	///
	/// Accounts expected:
	///
	/// 0. `[]` Vesting schedule
	/// 1. `[w, s]` Authority
	/// 2. `[]` Mint
	/// 3. `[w]` Vesting account
	/// 4. `[w]` Vesting account ATA
	/// 5. `[]` Recipient wallet
	/// 6. `[w]` Recipient's ATA
	/// 7. `[]` System program
	/// 8. `[]` Token program
	/// 9. `[]` Associated token program
	CloseAccount,

	/// Closes a vesting schedule
	///
	/// Accounts expected:
	/// 0. `[w]` Vesting schedule
	/// 1. `[w, s]` Authority
	/// 2. `[]` System program
	CloseVestingSchedule,
}

impl<'a> VestingInstruction<'a> {
	/// Unpacks a byte buffer into a [VestingInstruction](enum.VestingInstruction.html).
	fn unpack(input: &'a [u8]) -> Result<Self, ProgramError> {
		use VestingError::InvalidInstruction;
		let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
		Ok(match tag {
			0 => {}
			1 => {}
			2 => {}
			3 => {}
			4 => {}
			5 => {}
			6 => {}
		})
	}
	/// Packs a [VestingInstruction](enum.VestingInstruction.html) into a byte buffer
	fn pack(&self) -> Vec<u8> {
		let mut buf = Vec::with_capacity(size_of::<Self>());
		match self {
            &Self::InitVestingSchedule
        }
	}

	pub(crate) fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
		let pk = input
			.get(..PUBKEY_BYTES)
			.and_then(|x| Pubkey::try_from(x).ok())
			.ok_or(VestingError::InvalidInstruction)?;
		Ok((pk, &input[PUBKEY_BYTES..]))
	}

	pub(crate) fn unpack_pubkey_option(
		input: &[u8],
	) -> Result<(COption<Pubkey>, &[u8]), ProgramError> {
		match input.split_first() {
			Option::Some((&0, rest)) => Ok((COption::None, rest)),
			Option::Some((&1, rest)) => {
				let (pk, rest) = Self::unpack_pubkey(rest)?;
				Ok((COption::Some(pk), rest))
			}
			_ => Err(VestingError::InvalidInstruction.into()),
		}
	}

	pub(crate) fn pack_pubkey_option(value: &COption<Pubkey>, buf: &mut Vec<u8>) {
		match *value {
			COption::Some(ref key) => {
				buf.push(1);
				buf.extend_from_slice(&key.to_bytes());
			}
			COption::None => buf.push(0),
		}
	}

	pub(crate) fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
		let value = input
			.get(..BYTES_64)
			.and_then(|slice| slice.try_into().ok())
			.map(u64::from_le_bytes)
			.ok_or(VestingError::InvalidInstruction)?;
		Ok((value, &input[BYTES_64..]))
	}

	pub(crate) fn unpack_i64(input: &[u8]) -> Result<(i64, &[u8]), ProgramError> {
		let value = input
			.get(..BYTES_64)
			.and_then(|slice| slice.try_into().ok())
			.map(i64::from_le_bytes)
			.ok_or(VestingError::InvalidInstruction)?;
		Ok((value, &input[BYTES_64..]))
	}
}
