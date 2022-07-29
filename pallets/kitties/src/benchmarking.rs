//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {


	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let reciever: T::AccountId = whitelisted_caller();
		let to: T::AccountId = account("reciever",0,0);
		let dna = vec![1,2,3];
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(),dna.clone() )?;
		let kitti_owner = Kitties::<T>::kitty_owned(caller.clone());
		let dna_hash = kitti_owner[0];
	}: _(RawOrigin::Signed(caller), reciever, dna_hash)


	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
