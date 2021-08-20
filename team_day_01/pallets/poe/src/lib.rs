#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type MaxLength:Get<u8>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub(super) type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		(T::AccountId,T::BlockNumber)
	>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(T::AccountId,Vec<u8>),
		ClaimRevoked(T::AccountId,Vec<u8>),
		ClaimTransfered(T::AccountId,Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
		NoSameOwner,//
		NoMoreMaxLength,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub(super) fn create_claim(origin: OriginFor<T>, proof:Vec<u8>,) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&proof),Error::<T>::ProofAlreadyClaimed);

			ensure!(proof.len()<=T::MaxLength::get().into(),Error::<T>::NoMoreMaxLength);

			let current_block = frame_system::Pallet::<T>::block_number();

			Proofs::<T>::insert(&proof,(&sender,current_block));

			Self::deposit_event(Event::ClaimCreated(sender,proof));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub(super) fn revoke_claim(origin: OriginFor<T>,proof:Vec<u8>,) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;


			let (owner,_) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

			ensure!(sender==owner,Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);

			Self::deposit_event(Event::ClaimRevoked(sender,proof));

			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub(super) fn transfer_claim(origin: OriginFor<T>, proof:Vec<u8>,to: T::AccountId) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof),Error::<T>::NoSuchProof);

			let (owner,_) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

			ensure!(sender==owner,Error::<T>::NotProofOwner);

			let current_block = frame_system::Pallet::<T>::block_number();

			Proofs::<T>::insert(&proof,(&to,current_block));

			Self::deposit_event(Event::ClaimTransfered(to,proof));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
	}
}
