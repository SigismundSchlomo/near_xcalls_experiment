use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, Promise, AccountId, Gas, log};
use near_sdk::serde_json::json;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Contract {

}

#[near_bindgen]
impl Contract {

    pub fn call_low_level(&mut self) {
        let account_id = validate_account_id("ref-finance-101.testnet".to_string());
        let promise_id = env::promise_batch_create(&account_id);
        // deposit
        // env::promise_batch_action_function_call(
        //     promise_id,
        //     "ft_transfer_call",
        //     &json!({ "receiver_id": "ref-finance-101.testnet".to_string(), "amount": "110000000000000000000".to_string(), "msg": ""})
        //         .to_string()
        //         .into_bytes(),
        //     1,
        //     Gas(50_000_000_000_000),
        // );

        //get pool
        env::promise_batch_action_function_call(
            promise_id,
            "get_pool",
            &json!({"pool_id": 0u64})
                .to_string()
                .into_bytes(),
            0,
            Gas(50_000_000_000_000)
        );

        env::promise_batch_action_function_call(
            promise_id,
            "swap",
            &json!({"actions": [{"pool_id": 0u64,
                    "token_in": "wrap.testnet",
                    "amount_in": "500000000000000000",
                    "token_out": "rft.tokenfactory.testnet",
                    "min_amount_out": "1"}]
                })
                .to_string()
                .into_bytes(),
            0,
            Gas(50_000_000_000_000)
        );




        env::promise_return(promise_id);
    }

    pub fn call_mid_level(&mut self) -> Promise {
        let account_id = validate_account_id("ref-finance-101.testnet".to_string());
        let cross_contract_call = Promise::new(
            account_id
        )
        // deposit
        //     .function_call(
        //     "ft_transfer_call".to_string(),
        //     json!({ "receiver_id": "ref-finance-101.testnet".to_string(), "amount": "110000000000000000000".to_string(), "msg": "".to_string()})
        //         .to_string()
        //         .into_bytes(),
        //     1,
        //     Gas(50_000_000_000_000)
        // );
            .function_call(
                "get_pool".to_string(),
                json!({"pool_id": 0u64})
                    .to_string()
                    .into_bytes(),
                0,
                Gas(50_000_000_000_000)
            )
            .function_call(
                "swap".to_string(),
                json!({"actions": [{"pool_id": 0u64,
                    "token_in": "wrap.testnet",
                    "amount_in": "500000000000000000",
                    "token_out": "rft.tokenfactory.testnet",
                    "min_amount_out": "1"}]
                })
                    .to_string()
                    .into_bytes(),
                0,
                Gas(50_000_000_000_000)
            );

        cross_contract_call
    }

    pub fn my_callback(&mut self) {
        log!("Callback was called")
    }

}

pub fn validate_account_id(string: String) -> AccountId {
    if let Ok(account_id) = AccountId::try_from(string) {
        account_id
    } else {
        env::panic_str("ACCOUNT_ID_IS_INVALID")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
