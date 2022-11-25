elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait TokenModule {
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(self.nft_token().is_empty(), "Token already issued");

        let issue_cost = self.call_value().egld_value();

        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback())
            .call_and_exit();
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(!self.nft_token().is_empty(), "Token is not issued");

        let token = &self.nft_token().get_token_id();
        let roles = [EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn, EsdtLocalRole::NftUpdateAttributes];

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &token,
                (&roles[..]).into_iter().cloned(),
            )
            .async_call()
            .call_and_exit();
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token().set_token_id(&token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let (token_id, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
                if token_id.is_egld() && returned_tokens > 0 {
                    self.send().direct(&caller, &token_id, 0, &returned_tokens);
                }
            }
        }
    }

    #[view(getNftTokenID)]
    #[storage_mapper("nftTokenID")]
    fn nft_token(&self) -> NonFungibleTokenMapper<Self::Api>;
}
