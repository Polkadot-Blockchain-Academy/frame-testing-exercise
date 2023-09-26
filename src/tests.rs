use crate::{
	mock::*,
	pallet::{self as pallet_template, *},
};
use frame_support::{pallet_prelude::*, testing_prelude::*};

mod register_voter {
	use super::*;

	#[test]
	fn works_only_if_root() {}

	#[test]
	fn cannot_register_too_many() {}
}

mod make_vote {
	use super::*;

	#[test]
	fn only_voters_can_vote() {}

	#[test]
	fn can_update_vote() {}
}

mod close_vote {
	use super::*;

	#[test]
	fn cannot_close_if_no_voter() {}

	#[test]
	fn cannot_close_if_not_enough_voted() {}

	#[test]
	fn can_close() {}

	#[test]
	fn close_even_will_fail() {}

	#[test]
	fn close_with_abstinence() {}
}
