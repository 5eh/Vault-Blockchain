#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;
    extern crate alloc;
    use alloc::vec::Vec;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Product<T: Config> {
        pub name: Vec<u8>,
        pub owner: T::AccountId,
        pub price: BalanceOf<T>,
        pub quantity: u32,
        pub active: bool,
    }

    #[pallet::storage]
    pub type Products<T: Config> = StorageMap<_, Blake2_128Concat, u32, Product<T>>;

    #[pallet::storage]
    pub type NextId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProductRegistered {
            id: u32,
            owner: T::AccountId,
        },
        ProductUpdated {
            id: u32,
        },
        ProductTransferred {
            id: u32,
            from: T::AccountId,
            to: T::AccountId,
        },
        ProductPurchased {
            id: u32,
            buyer: T::AccountId,
            amount: u32,
        },
        ProductDeactivated {
            id: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ProductNotFound,
        NotOwner,
        NotActive,
        InsufficientQuantity,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn register(
            origin: OriginFor<T>,
            name: Vec<u8>,
            price: BalanceOf<T>,
            quantity: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let id = NextId::<T>::get();

            Products::<T>::insert(
                &id,
                Product {
                    name,
                    owner: who.clone(),
                    price,
                    quantity,
                    active: true,
                },
            );

            NextId::<T>::put(id + 1);
            Self::deposit_event(Event::ProductRegistered { id, owner: who });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn update(
            origin: OriginFor<T>,
            id: u32,
            name: Option<Vec<u8>>,
            price: Option<BalanceOf<T>>,
            quantity: Option<u32>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Products::<T>::try_mutate(&id, |maybe_product| -> DispatchResult {
                let product = maybe_product.as_mut().ok_or(Error::<T>::ProductNotFound)?;
                ensure!(product.owner == who, Error::<T>::NotOwner);

                if let Some(n) = name {
                    product.name = n;
                }
                if let Some(p) = price {
                    product.price = p;
                }
                if let Some(q) = quantity {
                    product.quantity = q;
                }

                Ok(())
            })?;

            Self::deposit_event(Event::ProductUpdated { id });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn purchase(origin: OriginFor<T>, id: u32, amount: u32) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            Products::<T>::try_mutate(&id, |maybe_product| -> DispatchResult {
                let product = maybe_product.as_mut().ok_or(Error::<T>::ProductNotFound)?;
                ensure!(product.active, Error::<T>::NotActive);
                ensure!(product.quantity >= amount, Error::<T>::InsufficientQuantity);

                let cost = product.price.saturating_mul(amount.into());
                T::Currency::transfer(
                    &buyer,
                    &product.owner,
                    cost,
                    frame_support::traits::ExistenceRequirement::KeepAlive,
                )?;

                product.quantity -= amount;
                Ok(())
            })?;

            Self::deposit_event(Event::ProductPurchased { id, buyer, amount });
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn transfer(origin: OriginFor<T>, id: u32, to: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Products::<T>::try_mutate(&id, |maybe_product| -> DispatchResult {
                let product = maybe_product.as_mut().ok_or(Error::<T>::ProductNotFound)?;
                ensure!(product.owner == who, Error::<T>::NotOwner);

                product.owner = to.clone();
                Ok(())
            })?;

            Self::deposit_event(Event::ProductTransferred { id, from: who, to });
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn deactivate(origin: OriginFor<T>, id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Products::<T>::try_mutate(&id, |maybe_product| -> DispatchResult {
                let product = maybe_product.as_mut().ok_or(Error::<T>::ProductNotFound)?;
                ensure!(product.owner == who, Error::<T>::NotOwner);

                product.active = false;
                Ok(())
            })?;

            Self::deposit_event(Event::ProductDeactivated { id });
            Ok(())
        }
    }
}
