use anyhow::Ok;
use near_sdk::json_types::U128;
use near_units::{parse_gas, parse_near};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::result::CallExecutionDetails;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const DEFI_WASM_FILEPATH: &str =
    "../../contract/target/wasm32-unknown-unknown/release/fungible_token.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environment
    let worker = workspaces::sandbox().await?;

    //deploy contracts
    let token_wasm = std::fs::read(DEFI_WASM_FILEPATH)?;
    let token_contract = worker.dev_deploy(&token_wasm).await?;

    // Create Accounts
    let owner = worker.root_account().unwrap();
    let jay = owner
        .create_subaccount(&worker, "jay")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let george = owner
        .create_subaccount(&worker, "george")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let dennoh = owner
        .create_subaccount(&worker, "dennoh")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let gichuru = owner
        .create_subaccount(&worker, "gichuru")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // Initialize Contracts
    token_contract
        .call(&worker, "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": owner.id(),
            "total_supply": parse_near!("1,000,000,000 N").to_string(),
        }))?
        .transact()
        .await?;

    //begin test
    //TODO: ADD tests here
    test_total_supply(&owner, &token_contract, &worker).await?;
    test_simple_transfer(&owner, &jay, &token_contract, &worker).await?;

    Ok(())
}

async fn test_total_supply(
    owner: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let initial_bal = U128::from(parse_near!("1,000,000,000 N"));
    let res: U128 = owner
        .call(&worker, contract.id(), "ft_total_supply")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    assert_eq!(res, initial_bal);
    println!(" Passed test_total_supply");
    Ok(())
}

async fn test_simple_transfer(
    owner: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let transfer_amount = U128::from(parse_near!("1,000 N"));

    // register user
    user.call(&worker, contract.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": user.id(),
        }))?
        .deposit(parse_near!("0.008 N"))
        .transact()
        .await?;

    // Transfer fungible token
    owner
        .call(&worker, contract.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": user.id(),
            "amount": transfer_amount
        }))?
        .deposit(1)
        .transact()
        .await?;

    let root_balance: U128 = owner
        .call(&worker, contract.id(), "ft_balance_of")
        .args_json(serde_json::json!({
            "account_id": owner.id()
        }))?
        .transact()
        .await?
        .json()?;

    let jay_balance: U128 = owner
        .call(&worker, contract.id(), "ft_balance_of")
        .args_json(serde_json::json!({
            "account_id": user.id()
        }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(root_balance, U128::from(parse_near!("999,999,000 N")));
    assert_eq!(jay_balance, transfer_amount);

    println!("Passed :white_check_mark: test_simple_transfer");

    Ok(())
}
