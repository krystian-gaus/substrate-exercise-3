#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	StorageValue,
	StorageDoubleMap,
	traits::Randomness,
	RuntimeDebug,
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Doggy(pub [u8; 16]);

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as Doggies {
		pub Doggies get(fn doggies): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Doggy>;
		pub NextDoggyId get(fn next_doggy_id): u32;
	}
}

decl_event! {
	pub enum Event<T> where <T as frame_system::Config>::AccountId,
	{
		DoggyCreated(AccountId, u32, Doggy), //(owner, doggy_id, doggy)
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		DoggiesIdOverflow,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 1000] // weight = gues how long it will need to run (only for development a fixed number)
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			// Generate a random 128bit value
			let payload = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<frame_system::Module<T>>::extrinsic_index(),
			);
			let dna = payload.using_encoded(blake2_128);

			// Create and store doggy
			let doggy = Doggy(dna);
			let doggy_id = Self::next_doggy_id();
			Doggies::<T>::insert(&sender, doggy_id, doggy.clone());
			NextDoggyId::put(doggy_id + 1);

			// Emit event
			Self::deposit_event(RawEvent::DoggyCreated(sender, doggy_id, doggy))
		}
	}
}