use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn it_works_for_create_kitties() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		
		System::assert_last_event(mock::Event::KittiesModule(
			crate::Event::KittyCreate(1,0)
		));

		assert_eq!(KittiesCount::<Test>::get(),1);

		assert_eq!(Owner::<Test>::get(0),Some(1));

		assert_eq!(Balances::reserved_balance(1),1_000);
	});
}

#[test]
fn can_create_fail_money_no_enough(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::create(Origin::signed(100)),
			Error::<Test>::MoneyNoEnough
		);
	});
}

#[test]
fn can_transfer_ok(){
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesCount::<Test>::get(),1);

		assert_eq!(Balances::reserved_balance(1),1_000);

		assert_ok!(KittiesModule::transfer(Origin::signed(1),2,0));

		System::assert_last_event(mock::Event::KittiesModule(
			crate::Event::KittyTransfer(1,2,0)
		));

		assert_eq!(Balances::reserved_balance(1),0);

		assert_eq!(Balances::reserved_balance(2),1_000);

	});
}

#[test]
fn transfer_failed_no_owner(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1),2,99),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn transfer_money_no_enough(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::create(Origin::signed(99)),
			Error::<Test>::MoneyNoEnough
		);
	});
}

#[test]
fn transfer_already_owner(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1),1,0),
			Error::<Test>::AlreadyOwned
		);
	});
}

#[test]
fn can_bread_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesCount::<Test>::get(),2);
		assert_eq!(Owner::<Test>::get(0),Some(1));
		assert_eq!(Owner::<Test>::get(1),Some(1));
		assert_eq!(Balances::reserved_balance(1),2_000);

		assert_ok!(KittiesModule::breed(Origin::signed(1),0,1));
		System::assert_last_event(
			mock::Event::KittiesModule(crate::Event::KittyCreate(1,2))
		);

		assert_eq!(KittiesCount::<Test>::get(),3);

		assert_eq!(Owner::<Test>::get(2),Some(1));

		assert_eq!(Balances::reserved_balance(1),3_000);
	});
}

#[test]
fn can_bread_fail_same_parent(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::breed(Origin::signed(1),1,1),
			Error::<Test>::SameParentIndex
		);
	});
}

#[test]
fn can_bread_fail_invaild_kittyindex(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::breed(Origin::signed(1),0,1),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn can_sale_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_ok!(KittiesModule::sale(Origin::signed(1),0,Some(2_000)));

		System::assert_last_event(mock::Event::KittiesModule(
			crate::Event::KittySale(1,0,Some(2_000))
		));
	});
}

#[test]
fn can_sale_fail_no_owner(){
	new_test_ext().execute_with(|| {
		assert_noop!{
			KittiesModule::sale(Origin::signed(1),0,Some(2_000)),
			Error::<Test>::NotOwner
		}
	});
}

#[test]
fn can_buy_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sale(Origin::signed(1),0,Some(2_000)));
		assert_eq!(Owner::<Test>::get(0),Some(1));
		assert_eq!(KittyPrices::<Test>::get(0),Some(2_000));


		assert_ok!(KittiesModule::buy(Origin::signed(2),0));
		System::assert_last_event(mock::Event::KittiesModule(
			crate::Event::KittyBuy(2,0,Some(2_000))
		));

		assert_eq!(Balances::free_balance(1),10_000+2_000);

		assert_eq!(Balances::free_balance(2),20_000-2_000-1_000);

		assert_eq!(Balances::reserved_balance(1),0);
		assert_eq!(Balances::reserved_balance(2),1_000);

		assert_eq!(Owner::<Test>::get(0),Some(2));
		assert_eq!(KittyPrices::<Test>::get(0),None);
	});
}

#[test]
fn can_buy_fail_no_owner(){
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::buy(Origin::signed(1),99),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn can_buy_fail_no_sale(){
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(2),0),
			Error::<Test>::NoSale
		);
	});
}

#[test]
fn can_buy_fail_already_owner(){
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sale(Origin::signed(1),0,Some(3_000)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(1),0),
			Error::<Test>::AlreadyOwned
		);
	});
}