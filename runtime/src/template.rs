/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use rstd::prelude::*;
use support::{decl_module, decl_storage, decl_event, dispatch::Result};
use system::ensure_signed;
use codec::{Decode, Encode};
use hex_literal::hex;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Foo {
  id: u32,
  data: Vec<u8>,
}

/// The module's configuration trait.
pub trait Trait: system::Trait + contracts::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(fn something): Option<u32>;

		FooStore get(fn foo): Option<Foo>;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Stores a custom struct in the runtime storage.
		/// Used for querying from smart contract.
		pub fn store_foo(origin, data: Vec<u8>, id: u32) -> Result {
			let _who = ensure_signed(origin)?;

			let foo = Foo {
				id,
				data,
			  };

			FooStore::put(foo);
			Ok(())
		}
		
		/// Calls a Substrate smart contract using its address and ABI.
		/// input_data is the bytes representation of contract function/message name
		/// and scale encoded parameter value.
		pub fn call_contract(origin, address: T::AccountId, selector: Vec<u8>, flag: bool, val: u32) -> Result {
			let who = ensure_signed(origin)?;
			let encoded_bool = bool::encode(&flag);
			let encoded_int = u32::encode(&val);
			let input_data = [&selector[..], &encoded_bool[..], &encoded_int[..]].concat();
		
			let exec_result = <contracts::Module<T>>::bare_call(who, address.clone(), 0.into(), 500000, input_data);
			match exec_result {
				Ok(v) => {
					let result_val = bool::decode(&mut &v.data[..]);
					match result_val {
						Ok(b) => {
							Self::deposit_event(RawEvent::ContractCalled(address, b));
						},
						Err(_) => { },
					}
				},
				Err(_) => { },
			}
			
			Ok(())
		}

		/// Query smart contract storage from runtime.
		pub fn get_contract_storage(origin, address: T::AccountId) -> Result {
			let _who = ensure_signed(origin)?;
			// The key is derived from the metadata.json file of the smart contract.
			// The storage section of the metadata.json contains this value under the `range.offset` key. 
			let key_bool: [u8; 32] = hex!("0000000000000000000000000000000000000000000000000000000000000000");
			let key_int: [u8; 32] = hex!("0000000000000000000000000000000000000000000000000000000000000001");
			let res_bool = <contracts::Module<T>>::get_storage(address.clone(), key_bool);
			let res_int = <contracts::Module<T>>::get_storage(address.clone(), key_int);
			match res_bool {
				Ok(Some(v)) => {
					let result_val = bool::decode(&mut &v[..]);
					match result_val {
						Ok(b) => {
							Self::deposit_event(RawEvent::ContractQueried(address, b));
						},
						Err(_) => { },
					}
				},
				Ok(None) => { },
				Err(_) => { },
			}
			match res_int {
				Ok(Some(v)) => {
					let result_val = u32::decode(&mut &v[..]);
					match result_val {
						Ok(u) => {
							Something::put(u);
						},
						Err(_) => { },
					}
				},
				Ok(None) => { },
				Err(_) => { },
			}
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),

		// A smart contract was called from the runtime.
		ContractCalled(AccountId, bool),

		// A smart contract storage was queried.
		ContractQueried(AccountId, bool),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use primitives::H256;
	use support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
