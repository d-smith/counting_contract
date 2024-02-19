use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use msg::InstantiateMsg;

mod contract;
pub mod msg;
mod state;


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, msg.counter, msg.minimal_donation)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_json_binary(&query::value(deps)?),
        Incremented { value } => to_json_binary(&query::incremented(value)),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> StdResult<Response> {
    use contract::exec;
    use msg::ExecMsg::*;

    match msg {
        Donate {} => exec::donate(deps, info),
        Reset { counter } => exec::reset(deps, info, counter),
    } 
}

#[cfg(test)]
mod test {    
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::msg::{ExecMsg, InstantiateMsg,QueryMsg, ValueResp};
    use crate::{execute, instantiate, query};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 7,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 7 });
    }

    #[test]
    fn query_incremented() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 10,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Incremented { value: 1 })
            .unwrap();

        assert_eq!(resp, ValueResp { value: 2 });
    }
    

    #[test]
fn donate() {
    let mut app = App::default();
 
    let contract_id = app.store_code(counting_contract());
 
    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &InstantiateMsg {
                counter: 0,
                minimal_donation: coin(10, "atom"),
            },
            &[],
            "Counting contract",
            None,
        )
        .unwrap();
 
    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecMsg::Donate {},
        &[],
    )
    .unwrap();
 
    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Value {})
        .unwrap();
 
    assert_eq!(resp, ValueResp { value: 0 });
}

#[test]
fn donate_with_funds() {
    let sender = Addr::unchecked("sender");
 
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    });
 
    let contract_id = app.store_code(counting_contract());
 
    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &InstantiateMsg {
                counter: 0,
                minimal_donation: coin(10, "atom"),
            },
            &[],
            "Counting contract",
            None,
        )
        .unwrap();
 
    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecMsg::Donate {},
        &coins(10, "atom"),
    )
    .unwrap();
 
    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Value {})
        .unwrap();
 
    assert_eq!(resp, ValueResp { value: 1 });
}
}
