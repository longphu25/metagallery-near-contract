/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

// const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/png;base64,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";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Meta Gallery token".to_string(),
                symbol: "METAG".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
