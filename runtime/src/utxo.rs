use codec::{Decode, Encode};
use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::{DispatchResult, Vec},
    ensure,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{
    crypto::Public as _,
    sr25519::{Public, Signature},
    H256, H512,
};
use sp_runtime::{
    traits::{BlakeTwo256, Hash, SaturatedConversion},
    transaction_validity::{TransactionLongevity, ValidTransaction},
};
use sp_std::collections::btree_map::BTreeMap;
// use super::{block_author::BlockAuthor, issuance::Issuance};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Debug, Hash)]
pub struct TransactionInput {
    pub outpointt: H256,
    pub sigscript: H512,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Debug, Hash)]
pub struct TransactionOutput {
    pub value: Value,
    pub pubkey: H256,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Debug, Hash)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

pub trait Trait: frame_system::Trait {
    /// The ubiquitous Event type
    type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;

    /// A source to determine the block author
    type BlockAuthor: BlockAuthor;

    /// A source to determine the issuance portion of the block reward
    type Issuance: Issuance<<Self as frame_system::Trait>::BlockNumber, Value>;
}

pub type Value = u128;

decl_storage! {
    trait Store for Module<T: Trait> as Utxo {
      UtxoStore build(|config: &GenesisConfig| {
      config.genesis_utxos
        .iter()
        .cloned()
        .map(|u| (BlakeTwo256::hash_of(&u, u)))
        .collect::<Vec<_>>()
      }) map hasher(identity) H256 => Option<TransactionOutput>;
    }

  add_extra_genesis {
    config(genesis_utxos): Vec<TransactionOutput>;
  }
}

// External functions: callable by the end user
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn spend(_origin, transaction: Transaction) -> DispatchResult {
          
          Self::update_storage(&transaction)?;
          Self::deposit_event(Event::TranactionSuccess);

          Ok(())
        }
    }
}

decl_event! {
    pub enum Event {
      TransactionSuccess(Transaction),
    }
}

impl<T: Trait> Module<T> {
    
  fn update_storage(transaction: Transaction) -> DispatchResult {
    for input in &transaction.inputs {
      <UtxoStore>::remove(input.output);
    }
    let index: u64 = 0;
    for output in &transaction.outputs {
      let key = BlakeTwo256::hash_of((transaction.encode(), index));
      index = index.checked_add(1).ok_or("output index overflow")?;
      <UtxoStore>::insert(key, output);
    }
    Ok(()) 
  }
}

/// Tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use frame_support::{
        assert_noop, assert_ok, impl_outer_origin, parameter_types, weights::Weight,
    };
    use sp_core::testing::SR25519;
    use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};
    // use sp_core::traits::KeystoreExt;

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const MaximumBlockWeight: Weight = 1024;
            pub const MaximumBlockLength: u32 = 2 * 1024;
            pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }
    impl frame_system::Trait for Test {
        type BaseCallFilter = ();
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
        type DbWeight = ();
        type BlockExecutionWeight = ();
        type ExtrinsicBaseWeight = ();
        type MaximumExtrinsicWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
        type PalletInfo = ();
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
    }

    impl Trait for Test {
        type Event = ();
        type BlockAuthor = ();
        type Issuance = ();
    }

    type Utxo = Module<Test>;
}
