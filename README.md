# FRAME Testing Exercise

This is a clone of the `substrate-node-template` that includes a `template` pallet with some arbitrary logic, specifically designed to help you understand how to define the testing for the pallet calls.
This is very similar, and in fact a subset of your benchmarking template.

## Step 1: DYI Testing

In the first step of this exercise, you will be writing a DYI for your `pallet-template`.
First, study the logic in the pallet and familiarize yourself with it.

Then, proceed to writing tests.
This involves a `mock.rs` file, which contains your test setup, and a `tests.rs` file that contains the actual test.
The structure of this is up to you.
In fact, for a small pallet, you can bring them all in `lib.rs`, guarded by `#[cfg(test)] mod tests { .. }`.

To start covering your pallet, consider the following as a yardstick:

- Each extrinsic should have its own set of tests.
- For each extrinsic's potential failure (`Error::<T>`), you should have a test that ensures the extrinsic fails with the correct error.
  Make sure to use `assert_noop` to ensure storage has not changed!
- Make sure to check the state of all relevant storage items at the end of each test.
- Try and check the events at the end of each test.
- Try and create a `try_state` hook for the pallet, and check it after each test.

As for the structure, you can try and classify your tests into different clusters in `mod`s based on extrinsic.
For example:

```rust
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
```

_And more!_

## Step 2: Live Coding

Your instructor will walk you over the process of then enhancing your tests with some techniques and common practices.
By the end of this lecture, you should have covered:

- Creating a builder struct to build your initial state.
- Optionally, linking that builder struct to your `#[pallet::genesis_config]`.
- Using `derive_impl` to simplify the configuration of external pallets, if possible.
- How to read the events of your pallet from the system pallet.
- How to use transactional storage to check different outcomes, without needing to re-created the whole test setup.
- Creating and calling into a `try_state` hook.

## License

Licensed under the terms of the [GPL-3](./LICENSE.md) or later.
