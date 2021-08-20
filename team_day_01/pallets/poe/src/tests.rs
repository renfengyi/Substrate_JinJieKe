use crate::{mock::*};
use frame_support::{assert_ok,assert_noop};
use super::*;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let mut claim:Vec<u8> = vec![];
		for i in 0..8 {
			claim.push(i);
		}
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),Some((1,frame_system::Pallet::<Test>::block_number())));
	});
}

#[test]
fn create_claim_more_length_works() {
	new_test_ext().execute_with(|| {
		let mut claim:Vec<u8> = vec![];
		for i in 0..10 {
			claim.push(i);
		}
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::NoMoreMaxLength
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());


		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),None);
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());


		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		                                                   
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
	new_test_ext().execute_with(||{
		let claim:Vec<u8> = vec![0,1];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::NoSuchProof
		);
	});
}
