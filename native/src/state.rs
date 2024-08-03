use {
	arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs},
	num_enum::{TryFromPrimitive, IntoPrimitive},
	solana_program::{
		clock::UnixTimestamp,
		program_error::ProgramError,
		program_option::COption,
		program_pack::{IsInitialized, Pack, Sealed},
		pubkey::Pubkey,
	},
};

// Some projects may have a `token generation event`, logic for this is not handled explicitly in
// this program - you may instead choose one of two approaches if your project has a TGE.
// 1. Create a pre-sale with no purchase price for whitelisted accounts utilising the whitelist
//    program [`https://github.com/serfrae/fsp-whitelist`], it is however, not advisable to utilise
//    this appraoch as the logic for the pre-sale assumes that there is only one pre-sale for a
//    particular token and utilises the mint to derive program addresses.
// 2. Create a vesting schedule with this program that has no duration and a frequency of
//    `Once` therefore emitting all the tokens in the vesting account upon commencement.
//
// This program also expects you to pre-load the token account of the vesting account to enable
// parallel execution and will calculate the amount claimable at a certain timestamp as the total
// amount a user should be able to claim - the amount already claimed (balance) - total amount a
// user should be able to claim - tokens emitted since the start of vesting.
// The formula used is:
// amount / (duration / frequency) = emissions_per_period
// current_timestamp - starting_timestamp / frequency = number_of_elapsed_periods
// emissions_per_period * number_of_elapsed_periods = emitted_tokens
// claimed_tokens = total_amount - current_balance
// claimable_amount = emitted_tokens - claimed_tokens
// or
// c = ((tc - ts) / f) * (a / (d/f)) - (a - b)
//
// Since there may exist multiple vesting schedules for a single token, there isn't a
// solution utilising account data that will allow for a deterministic address as nearly all
// these fields may be amended. Instead we supply a discriminant in the form of a string identifier
// to be hashed and provided as a seed for the generation of program addresses, the string is
// hashed and the first 8 bytes of the hash is used as the identifier
#[repr(u8)]
#[derive(Clone, Copy, Default, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum Frequency {
	Once,
	#[default]
	Slot,
	Second,
	Minute,
	Hour,
	Day,
	Week,
	Month,
	Quarter,
	Year,
}

/// Veesting schedule data
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VestingSchedule {
	/// Is `true` if this structure has been initialised
	pub is_initialized: bool, // 1
	/// Authority used to amend vesting details and close vesting accounts.
	pub authority: Pubkey, // 33
	/// The mint of vesting token
	pub mint: Pubkey, // 65
	/// Frequency of token emissions
	pub frequency: Frequency, // 66
	/// i64 unixtimestamp when vesting commences
	pub start: UnixTimestamp, // 74
	/// Duration of the total vesting length in seconds
	pub duration: i64, // 82
	/// Optional vault used if tokens are not pre-loaded into vesting accounts
	pub vault: COption<Pubkey>, // 118
}

impl Sealed for VestingSchedule {}
impl IsInitialized for VestingSchedule {
	fn is_initialized(&self) -> bool {
		self.is_initialized
	}
}
impl Pack for VestingSchedule {
	const LEN: usize = 118;
	fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
		let src = array_ref![src, 0, 118];
		let (is_initialized, authority, mint, frequency, start, duration, vault) =
			array_refs![src, 1, 32, 32, 1, 8, 8, 36];
		let is_initialized = match is_initialized {
			[0] => false,
			[1] => true,
			_ => return Err(ProgramError::InvalidAccountData),
		};
		let authority = Pubkey::new_from_array(*authority);
		let mint = Pubkey::new_from_array(*mint);
		let frequency = Frequency::try_from_primitive(frequency[0]).or(Err(ProgramError::InvalidAccountData))?;
		let start = i64::from_le_bytes(*start);
		let duration = i64::from_le_bytes(*duration);
		let vault = unpack_coption_key(vault)?;
		Ok(VestingSchedule {
			is_initialized,
			authority,
			mint,
			frequency,
			start,
			duration,
			vault,
		})
	}

	fn pack_into_slice(&self, dst: &mut [u8]) {
		let dst = array_mut_ref![dst, 0, 118];
		let (
			is_initialized_dst,
			authority_dst,
			mint_dst,
			frequency_dst,
			start_dst,
			duration_dst,
			vault_dst,
		) = mut_array_refs![dst, 1, 32, 32, 1, 8, 8, 36];
		let &VestingSchedule {
			is_initialized,
			ref authority,
			ref mint,
			frequency,
			start,
			duration,
			ref vault,
		} = self;
		is_initialized_dst[0] = is_initialized as u8;
		authority_dst.copy_from_slice(authority.as_ref());
		mint_dst.copy_from_slice(mint.as_ref());
		frequency_dst[0] = frequency as u8;
		*start_dst = start.to_le_bytes();
		*duration_dst = duration.to_le_bytes();
		pack_coption_key(vault, vault_dst);
	}
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Account {
	pub is_initialized: bool,     // 1
	pub vesting_schedule: Pubkey, // 33
	pub owner: Pubkey,            // 65
	pub mint: Pubkey,             // 97
	pub amount: u64,              // 105
	pub claimed: u64,             // 113
}
impl Sealed for Account {}
impl IsInitialized for Account {
	fn is_initialized(&self) -> bool {
		self.is_initialized
	}
}
impl Pack for Account {
	const LEN: usize = 113;

	fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
		let src = array_ref![src, 0, 113];
		let (is_initialized, vesting_schedule, owner, mint, amount, claimed) =
			array_refs![src, 1, 32, 32, 32, 8, 8];
		let is_initialized = match is_initialized {
			[0] => false,
			[1] => true,
			_ => return Err(ProgramError::InvalidAccountData),
		};
		let vesting_schedule = Pubkey::new_from_array(*vesting_schedule);
		let owner = Pubkey::new_from_array(*owner);
		let mint = Pubkey::new_from_array(*mint);
		let amount = u64::from_le_bytes(*amount);
		let claimed = u64::from_le_bytes(*claimed);
		Ok(Self {
			is_initialized,
			vesting_schedule,
			owner,
			mint,
			amount,
			claimed,
		})
	}

	fn pack_into_slice(&self, dst: &mut [u8]) {
		let dst = array_mut_ref![dst, 0, 113];
		let (
			is_initialized_dst,
			vesting_schedule_dst,
			owner_dst,
			mint_dst,
			amount_dst,
			claimed_dst,
		) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8];
		let &Account {
			is_initialized,
			ref vesting_schedule,
			ref owner,
			ref mint,
			amount,
			claimed,
		} = self;
		is_initialized_dst[0] = is_initialized as u8;
		vesting_schedule_dst.copy_from_slice(vesting_schedule.as_ref());
		owner_dst.copy_from_slice(owner.as_ref());
		mint_dst.copy_from_slice(mint.as_ref());
		*amount_dst = amount.to_le_bytes();
		*claimed_dst = claimed.to_le_bytes();
	}
}

pub(crate) fn pack_coption_key(src: &COption<Pubkey>, dst: &mut [u8; 36]) {
	let (tag, body) = mut_array_refs![dst, 4, 32];
	match src {
		COption::Some(key) => {
			*tag = [1, 0, 0, 0];
			body.copy_from_slice(key.as_ref());
		}
		COption::None => {
			*tag = [0; 4];
		}
	}
}

pub(crate) fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>, ProgramError> {
	let (tag, body) = array_refs![src, 4, 32];
	match *tag {
		[0, 0, 0, 0] => Ok(COption::None),
		[1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
		_ => Err(ProgramError::InvalidAccountData),
	}
}
