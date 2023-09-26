#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::{pallet_prelude::*, traits::DefensiveResult};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// A configurable maximum number of users for this pallet.
		type MaxVoters: Get<u32>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// A simple set of choices for a user vote.
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, Copy, Eq, PartialEq)]
	pub enum Vote {
		Abstain,
		Aye,
		Nay,
	}

	// A struct which connects a user to their vote.
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug)]
	pub struct UserVote<AccountId, Vote> {
		pub who: AccountId,
		pub vote: Vote,
	}

	// The set of users allowed to make a vote, not sorted.
	#[pallet::storage]
	#[pallet::getter(fn voters)]
	pub type Voters<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxVoters>, ValueQuery>;

	// The current set of votes by voters, not sorted.
	#[pallet::storage]
	#[pallet::getter(fn votes)]
	pub type Votes<T: Config> =
		StorageValue<_, BoundedVec<UserVote<T::AccountId, Vote>, T::MaxVoters>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Outcome { aye: bool },
		NewVote { who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyVoter,
		TooManyVoters,
		NotVoter,
		NotComplete,
		NoVoters,
	}

	impl<T: Config> Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Register a user which is allowed to be a voter. Only callable by the `Root` origin.
		#[pallet::call_index(0)]
		#[pallet::weight(u64::default())]
		pub fn register_voter(
			origin: OriginFor<T>,
			who: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let mut voters = Voters::<T>::get();
			ensure!(!voters.contains(&who), Error::<T>::AlreadyVoter);
			voters.try_push(who).map_err(|_| Error::<T>::TooManyVoters)?;
			Voters::<T>::set(voters);
			Ok(().into())
		}

		// Allow a registered voter to make or update their vote.
		#[pallet::call_index(1)]
		#[pallet::weight(u64::default())]
		pub fn make_vote(origin: OriginFor<T>, vote: Vote) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let voters = Voters::<T>::get();
			ensure!(voters.contains(&who), Error::<T>::NotVoter);
			let mut votes = Votes::<T>::get();

			let maybe_index = votes.iter().position(|v| v.who == who);

			let user_vote = UserVote { who: who.clone(), vote };

			if let Some(index) = maybe_index {
				votes[index] = user_vote;
			} else {
				votes.try_push(user_vote).defensive_map_err(|_| Error::<T>::TooManyVoters)?;
			}
			Votes::<T>::set(votes);
			Self::deposit_event(Event::<T>::NewVote { who });
			Ok(().into())
		}

		// Attempt to resolve a vote, which emits the outcome and resets the votes.
		#[pallet::call_index(2)]
		#[pallet::weight(u64::default())]
		pub fn close_vote(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Any user can attempt to close the vote.
			let _who = ensure_signed(origin)?;
			// Gets just the length of voters.
			let max_voters = Voters::<T>::decode_len().unwrap_or(0);
			ensure!(max_voters > 0, Error::<T>::NoVoters);
			let votes = Votes::<T>::get();
			let votes_len = votes.len();
			let mut ayes = 0;
			let mut nays = 0;
			let not_voted = max_voters - votes_len;

			votes.iter().for_each(|v| {
				match v.vote {
					Vote::Aye => ayes += 1,
					Vote::Nay => nays += 1,
					// Nothing to do for abstainers.
					Vote::Abstain => {},
				}
			});

			if ayes > nays + not_voted {
				Self::deposit_event(Event::<T>::Outcome { aye: true });
			} else if nays >= ayes + not_voted {
				Self::deposit_event(Event::<T>::Outcome { aye: false });
			} else {
				return Err(Error::<T>::NotComplete.into())
			}

			Votes::<T>::kill();
			Ok(().into())
		}
	}
}
