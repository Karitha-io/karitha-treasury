#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const REALM_ESDT_ID: &[u8] = b"REALM-8ead17";

#[elrond_wasm::contract]
pub trait TreasuryContract {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(withdrawRealmTo)]
    fn withdraw_realm_to(&self, to: ManagedAddress, #[var_args] amount: OptionalValue<BigUint>) {
        let token = TokenIdentifier::from_esdt_bytes(REALM_ESDT_ID);

        let amount = amount
            .into_option()
            .unwrap_or_else(|| self.blockchain().get_sc_balance(&token, 0));

        self.send().direct(&to, &token, 0, &amount, &[]);
    }

    #[only_owner]
    #[endpoint(withdrawEgldTo)]
    fn withdraw_egld_to(&self, to: ManagedAddress, #[var_args] amount: OptionalValue<BigUint>) {
        let self_addr = self.blockchain().get_sc_address();

        let amount = amount
            .into_option()
            .unwrap_or_else(|| self.blockchain().get_balance(&self_addr));

        self.send().direct_egld(&to, &amount, &[]);
    }
}
