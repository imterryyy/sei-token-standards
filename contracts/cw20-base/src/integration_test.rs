mod tests {
  use super::utils::*;
  use cosmwasm_std::Uint128;

  #[test]
  fn should_mint_token() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");
    let mint_amount = Uint128::from(100_000u128);

    let res = exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: mint_amount,
      })
    );

    assert!(res.is_ok());

    // Check recipient balance
    let recipient_balance: cw20::BalanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Balance {
        address: recipient.to_string(),
      })
    ).unwrap();

    assert_eq!(recipient_balance.balance, mint_amount);

    // Check total supply
    let token_info: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();
    assert_eq!(token_info.total_supply, mint_amount);
  }

  #[test]
  fn cant_mint_without_ownership() {
    let (mut app, token_addr) = load_prefix();
    let fake_minter = get_uaddr("fake_minter");
    let recipient = get_uaddr("recipient");

    let res = exec(
      &mut app,
      &fake_minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: Uint128::from(1000000u128),
      })
    );

    assert_err(res, crate::error::ContractError::Unauthorized {});
  }

  #[test]
  fn cant_mint_exeeds_max_supply() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");

    let res = exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: Uint128::from(1000000000000000001u128),
      })
    );

    assert_err(res, crate::error::ContractError::CannotExceedCap {});
  }

  #[test]
  fn should_burn_token() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");
    let burn_amount = Uint128::from(100_000u128);

    let res = exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    );

    assert!(res.is_ok());

    let total_supply_before_burn: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();

    let res = exec(
      &mut app,
      &recipient,
      &token_addr,
      &(crate::msg::ExecuteMsg::Burn {
        amount: burn_amount,
      })
    );

    assert!(res.is_ok());

    // Check recipient balance
    let recipient_balance: cw20::BalanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Balance {
        address: recipient.to_string(),
      })
    ).unwrap();

    assert_eq!(recipient_balance.balance, total_supply_before_burn.total_supply - burn_amount);

    // Check total supply
    let token_info: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();
    assert_eq!(token_info.total_supply, total_supply_before_burn.total_supply - burn_amount);
  }

  #[test]
  fn should_burn_from_token() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");
    let burner = get_uaddr("burner");
    let burn_amount = Uint128::from(100_000u128);

    let res = exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    );

    assert!(res.is_ok());

    let total_supply_before_burn: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();

    // Approve burner
    exec(
      &mut app,
      &recipient,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: burner.to_string(),
        amount: burn_amount,
        expires: None,
      })
    ).unwrap();

    // Burn from
    exec(
      &mut app,
      &burner,
      &token_addr,
      &(crate::msg::ExecuteMsg::BurnFrom {
        owner: recipient.to_string(),
        amount: burn_amount,
      })
    ).unwrap();

    assert!(res.is_ok());

    // Check recipient balance
    let recipient_balance: cw20::BalanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Balance {
        address: recipient.to_string(),
      })
    ).unwrap();

    assert_eq!(recipient_balance.balance, total_supply_before_burn.total_supply - burn_amount);

    // Check total supply
    let token_info: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();
    assert_eq!(token_info.total_supply, total_supply_before_burn.total_supply - burn_amount);
  }

  #[test]
  fn cant_burn_from_exceeds_allowance() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");
    let burner = get_uaddr("burner");
    let burn_amount = Uint128::from(100_000u128);
    let allowance = Uint128::from(50_000u128);

    let res = exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    );

    assert!(res.is_ok());

    // Approve burner
    exec(
      &mut app,
      &recipient,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: burner.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Burn from
    let res = exec(
      &mut app,
      &burner,
      &token_addr,
      &(crate::msg::ExecuteMsg::BurnFrom {
        owner: recipient.to_string(),
        amount: burn_amount,
      })
    );

    assert!(res.is_err())
  }

  #[test]
  fn should_transfer_tokens() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");
    let transfer_amount = Uint128::from(100_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    let total_supply_before_transfer: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();

    // Transfer
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Transfer {
        recipient: recipient.to_string(),
        amount: transfer_amount,
      })
    ).unwrap();

    // Check recipient balance
    let recipient_balance: cw20::BalanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Balance {
        address: recipient.to_string(),
      })
    ).unwrap();

    assert_eq!(recipient_balance.balance, transfer_amount);

    // Check total supply
    let token_info: cw20::TokenInfoResponse = query(&app, &token_addr, &(crate::msg::QueryMsg::TokenInfo {})).unwrap();
    assert_eq!(token_info.total_supply, total_supply_before_transfer.total_supply);
  }

  #[test]
  fn cant_transfer_exceeds_balance() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let recipient = get_uaddr("recipient");
    let transfer_amount = Uint128::from(100_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000u128),
      })
    ).unwrap();

    // Transfer
    let res = exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Transfer {
        recipient: recipient.to_string(),
        amount: transfer_amount,
      })
    );

    assert!(res.is_err())
  }

  #[test]
  fn should_approve_token() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let spender = get_uaddr("spender");
    let allowance = Uint128::from(100_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    // Approve
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Check allowance
    let allowance_response: cw20::AllowanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Allowance {
        owner: minter.to_string(),
        spender: spender.to_string(),
      })
    ).unwrap();

    assert_eq!(allowance_response.allowance, allowance);
  }

  #[test]
  fn should_increase_allowance() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let spender = get_uaddr("spender");
    let allowance = Uint128::from(100_000u128);
    let increase_amount = Uint128::from(50_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    // Approve
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Increase allowance
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: increase_amount,
        expires: None,
      })
    ).unwrap();

    // Check allowance
    let allowance_response: cw20::AllowanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Allowance {
        owner: minter.to_string(),
        spender: spender.to_string(),
      })
    ).unwrap();

    assert_eq!(allowance_response.allowance, allowance + increase_amount);
  }

  #[test]
  fn should_decrease_allowance() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let spender = get_uaddr("spender");
    let allowance = Uint128::from(100_000u128);
    let decrease_amount = Uint128::from(50_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    // Approve
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Decrease allowance
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::DecreaseAllowance {
        spender: spender.to_string(),
        amount: decrease_amount,
        expires: None,
      })
    ).unwrap();

    // Check allowance
    let allowance_response: cw20::AllowanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Allowance {
        owner: minter.to_string(),
        spender: spender.to_string(),
      })
    ).unwrap();

    assert_eq!(allowance_response.allowance, allowance - decrease_amount);
  }

  #[test]
  fn cant_decrease_allowance_below_zero() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let spender = get_uaddr("spender");
    let allowance = Uint128::from(100_000u128);
    let decrease_amount = Uint128::from(150_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    // Approve
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Decrease allowance
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::DecreaseAllowance {
        spender: spender.to_string(),
        amount: decrease_amount,
        expires: None,
      })
    ).unwrap();

    // Check allowance
    let allowance_response: cw20::AllowanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Allowance {
        owner: minter.to_string(),
        spender: spender.to_string(),
      })
    ).unwrap();

    assert_eq!(allowance_response.allowance, Uint128::zero());
  }

  #[test]
  fn should_transfer_from() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let spender = get_uaddr("spender");
    let recipient = get_uaddr("recipient");
    let allowance = Uint128::from(100_000u128);
    let transfer_amount = Uint128::from(50_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    // Approve
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Transfer from
    exec(
      &mut app,
      &spender,
      &token_addr,
      &(crate::msg::ExecuteMsg::TransferFrom {
        owner: minter.to_string(),
        recipient: recipient.to_string(),
        amount: transfer_amount,
      })
    ).unwrap();

    // Check recipient balance
    let recipient_balance: cw20::BalanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Balance {
        address: recipient.to_string(),
      })
    ).unwrap();

    assert_eq!(recipient_balance.balance, transfer_amount);

    // Check allowance
    let allowance_response: cw20::AllowanceResponse = query(
      &app,
      &token_addr,
      &(crate::msg::QueryMsg::Allowance {
        owner: minter.to_string(),
        spender: spender.to_string(),
      })
    ).unwrap();

    assert_eq!(allowance_response.allowance, allowance - transfer_amount);
  }

  #[test]
  fn cant_transfer_from_exceeds_allowance() {
    let (mut app, token_addr) = load_prefix();
    let minter = get_uaddr("minter");
    let spender = get_uaddr("spender");
    let recipient = get_uaddr("recipient");
    let allowance = Uint128::from(100_000u128);
    let transfer_amount = Uint128::from(150_000u128);

    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::Mint {
        recipient: minter.to_string(),
        amount: Uint128::from(1_000_000u128),
      })
    ).unwrap();

    // Approve
    exec(
      &mut app,
      &minter,
      &token_addr,
      &(crate::msg::ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: allowance,
        expires: None,
      })
    ).unwrap();

    // Transfer from
    let res = exec(
      &mut app,
      &spender,
      &token_addr,
      &(crate::msg::ExecuteMsg::TransferFrom {
        owner: minter.to_string(),
        recipient: recipient.to_string(),
        amount: transfer_amount,
      })
    );

    assert!(res.is_err())
  }
}

mod utils {
  use std::fmt::Debug;

  use cosmwasm_std::{ Empty, Addr, Uint128, StdError };
  use cw20::MinterResponse;
  use cw_multi_test::{ Contract, ContractWrapper, App, Executor, AppResponse };
  use serde::{ Serialize, de::DeserializeOwned };

  pub fn get_uaddr(str: &str) -> Addr {
    Addr::unchecked(str.to_string())
  }

  pub fn exec(app: &mut App, sender: &Addr, contract_addr: &Addr, msg: &(impl Serialize + Debug)) -> Result<AppResponse, anyhow::Error> {
    app.execute_contract(sender.clone(), contract_addr.clone(), &msg, &[])
  }

  pub fn query<T: DeserializeOwned>(app: &App, contract_addr: &Addr, msg: &(impl Serialize + Debug)) -> Result<T, StdError> {
    app.wrap().query_wasm_smart(contract_addr.clone(), &msg)
  }

  pub fn assert_err(res: Result<AppResponse, anyhow::Error>, expected_err: crate::error::ContractError) {
    assert!(
      res.is_err_and(|_err| {
        match _err.downcast_ref::<crate::error::ContractError>() {
          Some(err) => err == &expected_err,
          None => false,
        }
      })
    );
  }

  pub fn cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(crate::contract::execute, crate::contract::instantiate, crate::contract::query);
    Box::new(contract)
  }

  pub fn load_prefix() -> (App, Addr) {
    let mut app = App::default();
    let contract = cw20();
    let code_id = app.store_code(contract);

    let owner = get_uaddr("owner");
    let minter = get_uaddr("minter");
    let admin = get_uaddr("admin");
    let max_supply = Uint128::from(1_000_000u128);

    let token_contract_addr = app
      .instantiate_contract(
        code_id,
        owner,
        &(crate::msg::InstantiateMsg {
          name: "Test".to_string(),
          symbol: "TEST".to_string(),
          decimals: 6,
          initial_balances: vec![],
          mint: Some(MinterResponse {
            minter: minter.to_string(),
            cap: Some(max_supply),
          }),
          marketing: None,
        }),
        &[],
        "Test Token".to_string(),
        Some(admin.to_string())
      )
      .unwrap();

    (app, token_contract_addr)
  }
}
