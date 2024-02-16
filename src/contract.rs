 
pub mod query {
    use cosmwasm_std::{Deps, StdResult};
    use crate::state::COUNTER;

    use crate::msg::ValueResp;
 
    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }

    pub fn incremented(value: u64) -> ValueResp {
        ValueResp { value: value + 1 }
    }
}