// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use aleo_account::{Address, PrivateKey, ViewKey};

use rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Account {
    pub(crate) private_key: PrivateKey,
    pub(crate) view_key: ViewKey,
    pub(crate) address: Address,
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // pub fn new() -> Result<Self, JsValue> {
        let rng = &mut StdRng::from_entropy();
        let private_key = PrivateKey::new(rng).unwrap();
        let view_key = ViewKey::from(&private_key).unwrap();
        let address = Address::from(&private_key).unwrap();
        Self {
            private_key,
            view_key,
            address,
        }
    }

    #[wasm_bindgen]
    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();
        let view_key = ViewKey::from(&private_key).unwrap();
        let address = Address::from(&private_key).unwrap();
        Self {
            private_key,
            view_key,
            address,
        }
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        format!(
            "Account {{ private_key: {}, view_key: {}, address: {} }}",
            self.private_key, self.view_key, self.address
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn account_from_private_key_test() {
        let given_private_key = "APrivateKey1tvv5YV1dipNiku2My8jMkqpqCyYKvR5Jq4y2mtjw7s77Zpn";
        let given_view_key = "AViewKey1m8gvywHKHKfUzZiLiLoHedcdHEjKwo5TWo6efz8gK7wF";
        let given_address = "aleo1faksgtpmculyzt6tgaq26fe4fgdjtwualyljjvfn2q6k42ydegzspfz9uh";

        let account = Account::from_private_key(given_private_key);

        println!("{} == {}", given_private_key, account.private_key.to_string());
        assert_eq!(given_private_key, account.private_key.to_string());

        println!("{} == {}", given_view_key, account.view_key.to_string());
        assert_eq!(given_view_key, account.view_key.to_string());

        println!("{} == {}", given_address, account.address.to_string());
        assert_eq!(given_address, account.address.to_string());
    }

    // #[wasm_bindgen_test]
    // pub fn view_key_from_private_key_test() {
    //     let given_private_key = "APrivateKey1tvv5YV1dipNiku2My8jMkqpqCyYKvR5Jq4y2mtjw7s77Zpn";
    //     let given_view_key = "AViewKey1m8gvywHKHKfUzZiLiLoHedcdHEjKwo5TWo6efz8gK7wF";
    //
    //     let view_key = ViewKey::from_private_key(given_private_key);
    //
    //     println!("{} == {}", given_view_key, view_key.view_key.to_string());
    //     assert_eq!(given_view_key, view_key.view_key.to_string());
    // }
}
