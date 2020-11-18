#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use codec::{Encode, Decode};
use hex_literal::hex;
use sp_std::vec::Vec;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Foo {
    id: u32,
    data: Vec<u8>,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + pallet_contracts::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		FooStore get(fn foo): Option<Foo>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),

		// A smart contract was called from the runtime.
		ContractCalled(AccountId, bool),

		// A smart contract storage was queried.
		ContractQueried(AccountId, bool),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// Stores a custom struct in the runtime storage.
		/// Used for querying from smart contract.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn store_foo(origin, data: Vec<u8>, id: u32) -> dispatch::DispatchResult {
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
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn call_contract(origin, address: T::AccountId, selector: Vec<u8>, flag: bool, val: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let encoded_bool = bool::encode(&flag);
			let encoded_int = u32::encode(&val);
			let input_data = [&selector[..], &encoded_bool[..], &encoded_int[..]].concat();

			let exec_result = <pallet_contracts::Module<T>>::bare_call(who, address.clone(), 0.into(), 500000, input_data);
			match exec_result.0 {
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
		#[weight = 10_000 + T::DbWeight::get().reads(1)]
		pub fn get_contract_storage(origin, address: T::AccountId) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;
			// The key is derived from the metadata.json file of the smart contract.
			// The storage section of the metadata.json contains this value under key.
			let key_bool: [u8; 32] = hex!("0000000000000000000000000000000000000000000000000000000000000000");
			let key_int: [u8; 32] = hex!("0100000000000000000000000000000000000000000000000000000000000000");
			let res_bool = <pallet_contracts::Module<T>>::get_storage(address.clone(), key_bool);
			let res_int = <pallet_contracts::Module<T>>::get_storage(address.clone(), key_int);
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

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
