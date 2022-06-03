use crate::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct Supersig<AccountId> {
	pub members: Vec<AccountId>,
}

impl<AccountId> Supersig<AccountId> {
	pub fn new(members: Vec<AccountId>) -> Option<Self> {
		if members.is_empty() {
			return None
		}
		Some(Self { members })
	}
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
pub struct PreimageCall<AccountId, Balance> {
	pub data: Vec<u8>,
	pub provider: AccountId,
	pub deposit: Balance,
}

pub type SigIndex = u128;
pub type CallIndex = u128;