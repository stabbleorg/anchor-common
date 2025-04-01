use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_2022::spl_token_2022::{
        self,
        extension::{
            transfer_fee::{TransferFeeConfig, MAX_FEE_BASIS_POINTS},
            ExtensionType, StateWithExtensions,
        },
    },
    token_interface::{spl_token_2022::extension::BaseStateWithExtensions, Mint, TokenAccount},
};
use std::collections::HashSet;

const MINT_WHITELIST: [&'static str; 5] = [
    "HVbpJAQGNpkgBaYBZQBR1t7yFdvaYVp2vCQQfKKEN4tM", // USDP - Pax Dollar
    "FrBfWJ4qE5sCzKm3k3JaAtqZcXUh4LvJygDeketsrsH4", // ZUSD - Z.com USD
    "2u1tszSeqZ3qBWF3uNGPFc8TzMk2tdiwknnRMWGWjGWH", // USDG - Global Dollar
    "AUSD1jCcCyPLybk1YnvPWsHQSrZ46dxwoMniN4N2UEB9", // AUSD - AUSD
    "2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo", // PYUSD - Paypal USD
];

/// Check if the mint is supported for trade
pub fn is_supported_mint(mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
    let mint_info = mint_account.to_account_info();
    if *mint_info.owner == Token::id() {
        return Ok(true);
    }
    let mint_whitelist: HashSet<&str> = MINT_WHITELIST.into_iter().collect();
    if mint_whitelist.contains(mint_account.key().to_string().as_str()) {
        return Ok(true);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
    let extensions = mint.get_extension_types()?;
    for e in extensions {
        if e != ExtensionType::TransferFeeConfig
            && e != ExtensionType::MetadataPointer
            && e != ExtensionType::TokenMetadata
        {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Calculate the fee for input amount
pub fn get_transfer_fee(mint_info: &AccountInfo, pre_fee_amount: u64, epoch: u64) -> Result<u64> {
    if *mint_info.owner == Token::id() {
        return Ok(0);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        transfer_fee_config
            .calculate_epoch_fee(epoch, pre_fee_amount)
            .unwrap()
    } else {
        0
    };
    Ok(fee)
}

/// Calculate the fee for output amount
pub fn get_transfer_inverse_fee(
    mint_info: &AccountInfo,
    post_fee_amount: u64,
    epoch: u64,
) -> Result<u64> {
    if *mint_info.owner == Token::id() {
        return Ok(0);
    }
    if post_fee_amount == 0 {
        return Ok(0);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        let transfer_fee = transfer_fee_config.get_epoch_fee(epoch);
        if u16::from(transfer_fee.transfer_fee_basis_points) == MAX_FEE_BASIS_POINTS {
            u64::from(transfer_fee.maximum_fee)
        } else {
            transfer_fee_config
                .calculate_inverse_epoch_fee(epoch, post_fee_amount)
                .unwrap()
        }
    } else {
        0
    };
    Ok(fee)
}

/// Deserialize mint
pub fn try_deserialize_mint(account_info: &AccountInfo) -> Result<Mint> {
    let mut data: &[u8] = &account_info.try_borrow_data()?;
    Mint::try_deserialize(&mut data)
}

/// Deserialize token account
pub fn try_deserialize_token_account(account_info: &AccountInfo) -> Result<TokenAccount> {
    let mut data: &[u8] = &account_info.try_borrow_data()?;
    TokenAccount::try_deserialize(&mut data)
}
