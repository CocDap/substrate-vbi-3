
use crate::{mock::*, Error, Event as TemplateEvent};
use frame_support::{assert_noop, assert_ok};




fn last_event() -> TemplateEvent<Test> {
	println!("Events:{:?}", System::events());
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let Event::TemplateModule(inner) = e { Some(inner) } else { None })
		.last()
		.unwrap()
}


// Refer this link https://substrate.stackexchange.com/questions/3387/how-can-i-access-custom-pallet-event-data-in-a-test

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.

		// Lưu ý cần set block number
		System::set_block_number(1);
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		//assert_ok!(TemplateModule::do_something(Origin::signed(2), 100));

		System::assert_last_event(crate::Event::SomethingStored(42, 1).into());
		assert_eq!(last_event(), TemplateEvent::SomethingStored(42,1));
		println!("Event:{:?}", last_event());
		println!("Something:{}",TemplateModule::something().unwrap());
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
