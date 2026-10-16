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

const MINT_WHITELIST: [&'static str; 94] = [
    "HVbpJAQGNpkgBaYBZQBR1t7yFdvaYVp2vCQQfKKEN4tM", // USDP - Pax Dollar
    "FrBfWJ4qE5sCzKm3k3JaAtqZcXUh4LvJygDeketsrsH4", // ZUSD - Z.com USD
    "2u1tszSeqZ3qBWF3uNGPFc8TzMk2tdiwknnRMWGWjGWH", // USDG - Global Dollar
    "AUSD1jCcCyPLybk1YnvPWsHQSrZ46dxwoMniN4N2UEB9", // AUSD - AUSD
    "2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo", // PYUSD - Paypal USD
    "CASHx9KJUStyftLFWGvEVf59SGeG9sh5FfcnZMVPCASH", // CASH - Cash
    "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn", // Jito Staked SOL (JitoSOL)
    "oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp",  // ORE (ORE)
    "xorcaYqbXUNz3474ubUMJAdu2xgPsew3rUCe5ughT3N",  // Staked Orca (xORCA)
    "A7bdiYdS5GjqGFtxf17ppRHtDKPkkRqbKtR27dxvQXaS", // Zcash (ZEC)
    "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1",  // BlazeStake Staked SOL (bSOL) (bSOL)
    "5GgRAEmv8ZxF2PR5hY72Qs5x1bnQ6UK2RbTPoqJ3wSwW", // PAX Gold (PAXG)
    "AMC1qwR9KhiyrQBRPrxnfo4JfMeMZqEBvt5tgTytNNoc", // AMC - AMC Entertainment
    "AMD8XwJXgQ9WV45Wyj9yFLejxzf2J6VM1PJY8bJEjeES", // AMD - Advanced Micro Devices
    "ARMbSB1MBRQrY6PNMao1HdQ431VafC6JRJCsGZ2Yv3iJ", // ARM - Arm Holdings
    "BArimz1PcKZr8PcPh3tcZ2dg4S7FJLk3cw6R5F8GsHKg", // BA - The Boeing Company
    "BABANGA4JE7Kkam4nTrALAwAVgsNJUuFJnnkF7S16BZp", // BABA - Alibaba Group Holding
    "BBosJLw8ZzoATiEyywiifx7AgmrD2Cm3XjFWbhbRhChy", // BB - BlackBerry
    "BEBVfXZ8uCxRBuunEbW9wcq9r7W9ySNo14oJBJkDgpZx", // BE - Bloom Energy
    "BLKKYfF41c6xRPrtX1rdYbxLE1xxEdZUnxQsB5hcZ8Nf", // BLK - BlackRock
    "BoTx8y9ynfdxf5ZjWtCoBVkff52qKA82ysaLU8ZM6d8T", // BOT - RoboStrategy
    "BRVaZKg6J9iF2BEsdpsxJ9NyvN9PPxPZuoQUX2v8qqkk", // BROS - Dutch Bros
    "BULL151gUXcFV5wXEUqu9Am2L7Qt4bTJRLRuAUjkcspC", // BULL - Webull Corporation
    "CBRSFBbQPpp77NpMcwbTN6fhBHmZM94oFLJvruytw4Fi", // CBRS - Cerebras Systems
    "CzLTZppPdZtTjyq3WGpHLstoc3GLhu7zH5Zg6xUa6Gv5", // COPX - Global X Copper Miners ETF
    "CZEB3WNZuF2Yz1z2H81RcCk8T7fsw82KB33zqamASVsg", // COST - Costco Wholesale
    "CRWVJeR2yEZuDUKYfGuKCHvLz8ywn4LGvovHfy5WiFmi", // CRWV - CoreWeave
    "CYPHuMmCL1GxJWa2tsPhLKykC7GrHJTCHwbXD4g5uawK", // CYPH - Cypherpunk Technologies
    "DELL2aRKQz7DMq5DrKLtkn47ZCnbxXPZXrSGbkmd13wy", // DELL - Dell Technologies
    "DJTu7vi8norVzdVAffgvb39VP7wjKeTsgaMBJrzfxvoF", // DJT - Trump Media & Technology Group Corp. Common Stock
    "DKNGQFNGQmoBdXSRGKJ8tTu7uPDasw5JDcfMmWniNfow", // DKNG - DraftKings
    "DNUTsCvKbKwu2RM72cUuW3TD9YpzArzACcqYQssjPLSk", // DNUT - Krispy Kreme, Inc. Common Stock
    "DRAMjSWR7HRfJKjRkvQWYL2bcaejaVhuxEcjf4pAY4Cw", // DRAM - Roundhill Memory ETF
    "EWY4owSJYMpwN33qGDu5gGxpkQkpMJu8ZUsQJaNZG5dv", // EWY - iShares MSCI South Korea ETF
    "EWZzCki1igp2y73UyUM4RCG4QUVMQAtnrQUVNaDHzDxJ", // EWZ - iShares MSCI Brazil ETF
    "FLWSojG1gB5VStYR3Sb4nQFRt43UBYkqih1j2CpVLqgd", // FLWS - 1-800-FLOWERS.COM
    "FLYRq3en8r2Z69gN3KyAnDrvnitEJNkwPYY7favinHeD", // FLY - Firefly Aerospace
    "FWDtiB5fXHdVAewPqvHPL2dh4aBC1C6GacQbePoQXKjz", // FWDI - Forward Industries, Inc.
    "GPRR2u6NS5yBQHWGauoJ9HXgjrTH8dDsrBfTV5zAYvDH", // GPRO - GoPro
    "GRNDYDpqwpCm6jVxpbh4xT5AM4r3p391qYsKTHqgaET2", // GRND - Grindr
    "HiMSSzzwkZkrXJ4PGVJRdtfLaANeAztjjcgk5Dxe7Lwx", // HIMS - Hims & Hers Health
    "HooDYv5RewLRiMLnEVq3VJqdqxhuE6c5eYvqejMC3e9A", // HOOD - Robinhood Markets
    "HTZsLG4zqaNvWMwXSLHH3GG5KyJpKwpBRsKVdMG6hvzP", // HTZ - Hertz Global Holdings, Inc Common Stock
    "BMKdM4yUxX12moFqVk195k7coMbaybd4RUKCUdm7D1Sk", // IBM - International Business Machines
    "iNTCy1qTsUEZQe3DSocLz1ZXXai34Gdw8THQh5rxFaF",  // INTC - Intel
    "NQ5hSuXQZrbnrwcDVk2qN73njjd3E3v3badYHnj5thF",  // IONQ - IonQ
    "RENzhrJQgmAnfcLhU1U5XwAMc6TC15UA6jCbPBaasnj",  // IREN - IREN
    "JNJg1znKdF712Phe7L7z52AATAvEjEytBdN2w8Lnh1Y",  // JNJ - Johnson & Johnson
    "LLYuwZ33keFihgwoxXsBawy31AiRFLFSva32TYq5TvD",  // LLY - Eli Lilly and Company
    "LMT3i1BHgixFqPUgcyteJhnEz2dpy9i3cYy4pi9BoeV",  // LMT - Lockheed Martin
    "LULUmT9VMttkfAJE236LXJcYJ2tTP7nunrSWR5G1BdS",  // LULU - lululemon athletica inc.
    "LUV9GB51PNZNRyzzyYK3rtqFfvDvWtRiXZ34wVq2HrX",  // LUV - Southwest Airlines Co.
    "MGMuubtUEirmkhfEQdmGUh4pr7HuUdMWcZXFtpPbVJD",  // MGM - MGM Resorts International
    "MRNAzXzhNcaEXJPibHEn8cd4vyekCDiivTyEwswLUCT",  // MRNA - Moderna
    "MRVLSjkR2ceUBukujaD3xCyHP1H3B2SzpsNTZF546jo",  // MRVL - Marvell Technology
    "MSTRdWXMeZxdE8osAQy3fA4rvTY5rgummDSMEx6U7Nz",  // MSTR - Strategy
    "MUxEsUKSMACyw5fZf68wxf5FLnZVhtU9CwH8uNNGay1",  // MU - Micron Technology
    "NBiSF3UaVUFtRzHwAfxyHsBCAZWGEKnMpewAE4oh7BG",  // NBIS - Nebius Group N.V.
    "NFLX7qV57zuVxCoHy3s1jiGZyALraLNwttbxdvmYJLJ",  // NFLX - Netflix
    "NKEda5nHhNGgjrE9nDdMvaEmkmJ96qqxzBVZEcKmjSg",  // NKE - Nike
    "N7Q5fYX7YRnDQksfdBKnoUb3awm92n7QNAD35X3Rq1X",  // NOK - Nokia Oyj
    "NVDAVuiB7hwd3m5Wa1JuHNovPaPG6BH1QNztbKFxNjv",  // NVDA - Nvidia
    "PFER6ENqP8r8NF3CqVt4mFowxsin3V5MLidBNQFCC3x",  // PFE - Pfizer
    "PTNzAfFAB4LvoUQEUUGrFMyUoRLExMYjH6CcfyQfsVP",  // PTN - Palatin Technologies, Inc.
    "PUSAG1stksTcAoK9MjinEHUwAYRJsbovywYCk37m6hy",  // PUSA - Powerus Corporation
    "QUBTAD8C9bMU9LvmMNgKPhrmBGbHvxpu6vfWQtThxxw",  // QUBT - Quantum Computing
    "RACEyWiM2ztEZcJx2AHXU2eWjhxU57x3vXn92b39dLD",  // RACE - Ferrari
    "RBLXDGRD64AtRamHMFVcjqne3Ar7NLWtFtYNtsrf1cE",  // RBLX - Roblox
    "RDDTGbhHwVXfyCvQMXzzowKjf5qrYBZAnehoXW83ooh",  // RDDT - Reddit
    "RcZmt84VMJv9bDhKqmw1uWDahYrUT468VwAChTnfD8p",  // RIVN - Rivian Automotive
    "RKLBnAXGqv31iZomqsuAWkQm1aqC7JwwvbCfzGdqAhz",  // RKLB - Rocket Lab
    "RUMsPfFZFnN1ZmGANwP7FNMJMjKH4m9RiMePrtVtLe7",  // RUM - RUM Group Inc.
    "SCHHJ3jRdSjeFEVAaLrnYdx3Brphn92Ys7z1qkiCtPX",  // SCHH - Schwab U.S. REIT ETF
    "SH55hfaipFAbwT42nQYhRoM5o5t61QpkmJ6p62vXB3m",  // SHOP - Shopify
    "SKHYhSjuRWHgikq8eRKbtBbpABgJSkd7ytQV14i9EQ3",  // SKHY - SK Hynix
    "SNAPcESrvnH8yUdgeMF6xm1hym9b6hW6s8YeqeHdZFz",  // SNAP - Snap
    "SNDKbwMUQvZhnLnxLduradgLHG5KrPuKwpnrkkGRhfH",  // SNDK - Sandisk
    "SPCXxcqXj6e5dJDVNovHN8744zkbhM2bYudU45BimGb",  // SPCX - SpaceX
    "SPHRp8cZaSQBTp1KMNP4V1X821SXhXWt4Q2yLdyHzju",  // SPHR - Sphere Entertainment Co.
    "SQQQAa3gUxgnqcEsdjPA4RQNQBgB97TnG2hJhk2WaGE",  // SQQQ - ProShares UltraPro Short QQQ
    "TTWofwAge91oFhZs7kpQdyrVRkmevgM88xijGvQFbKo",  // TTWO - Take-Two Interactive Software
    "UPSqUeMHcWbkdg784XuBUEF9DtySSnW9ur5LAVdcuB9",  // UPS - United Parcel Service
    "URARfsinxCRw4JpvQhuT4CxavdZXZEMjv9ZwWmWpwag",  // URA - Global X Uranium ETF
    "USNv3NkKA27Dh4nsHJDPhW4VoEcTQmjoZyJt2dqdwFu",  // USO - United States Oil Fund, LP
    "WENAZ2WyPbmgvUcKfQ8hyMDfBQP9bZ65hsZ5KTFrRGZ",  // WEN - The Wendy's Company
    "WULFeyfrj1VJKD9HhRTcW8R4g5HefUA11HDEdBv2WxD",  // WULF - TeraWulf
    "AuzVdTnWMapP5B9PatcL7a2kW8Rym8SZabYSqfHPVo7w", // A - Agilent Technologies
    "AMBAHqGPjjtJaHPPSuvPu2mPCJdkGZpmhHQa5pkXbxrM", // AMBA - Ambarella, Inc. Ordinary Shares
    "GMETpjLMbTGTcJVhnxQLv1kxje2E7ZMR9Hnziu5Mt7oo", // GME - GameStop Corp.
    "LASEwYSQ9MrLY4kKNX5zwfndu1py1DwpBT98Eavr2Ju",  // LASE - Laser Photonics Corporation Common Stock
    "oURARB5TZqLn8iBGjvXjQyr1gfCfQV3c3YeUvduhcuZ",  // OURA - Oura
    "PENGTDQeQSXEjKcYw3CTQi9qznxcTHXAA7LMfUNrxLV",  // PENG - Penguin Solutions
    "PUMPPHYa9QQuYhDK1RjC2dmpvigT7LgtofHr6k3M4wt",  // PUMP - ProPetro Holding
    "XYZdVZ8jwqZCGN4XR7CiGkeWggo5huk7kNuXaCVs4nY"   // XYZ - Block
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
