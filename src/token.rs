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

static MINT_WHITELIST: [Pubkey; 1528] = [
    pubkey!("HVbpJAQGNpkgBaYBZQBR1t7yFdvaYVp2vCQQfKKEN4tM"), // USDP - Pax Dollar
    pubkey!("FrBfWJ4qE5sCzKm3k3JaAtqZcXUh4LvJygDeketsrsH4"), // ZUSD - Z.com USD
    pubkey!("2u1tszSeqZ3qBWF3uNGPFc8TzMk2tdiwknnRMWGWjGWH"), // USDG - Global Dollar
    pubkey!("AUSD1jCcCyPLybk1YnvPWsHQSrZ46dxwoMniN4N2UEB9"), // AUSD - AUSD
    pubkey!("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo"), // PYUSD - Paypal USD
    pubkey!("CASHx9KJUStyftLFWGvEVf59SGeG9sh5FfcnZMVPCASH"), // CASH - Cash
    pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"), // Jito Staked SOL (JitoSOL)
    pubkey!("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp"),  // ORE (ORE)
    pubkey!("xorcaYqbXUNz3474ubUMJAdu2xgPsew3rUCe5ughT3N"),  // Staked Orca (xORCA)
    pubkey!("A7bdiYdS5GjqGFtxf17ppRHtDKPkkRqbKtR27dxvQXaS"), // Zcash (ZEC)
    pubkey!("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1"),  // BlazeStake Staked SOL (bSOL) (bSOL)
    pubkey!("5GgRAEmv8ZxF2PR5hY72Qs5x1bnQ6UK2RbTPoqJ3wSwW"), // PAX Gold (PAXG)
    pubkey!("AMC1qwR9KhiyrQBRPrxnfo4JfMeMZqEBvt5tgTytNNoc"), // AMC - AMC Entertainment
    pubkey!("AMD8XwJXgQ9WV45Wyj9yFLejxzf2J6VM1PJY8bJEjeES"), // AMD - Advanced Micro Devices
    pubkey!("ARMbSB1MBRQrY6PNMao1HdQ431VafC6JRJCsGZ2Yv3iJ"), // ARM - Arm Holdings
    pubkey!("BArimz1PcKZr8PcPh3tcZ2dg4S7FJLk3cw6R5F8GsHKg"), // BA - The Boeing Company
    pubkey!("BABANGA4JE7Kkam4nTrALAwAVgsNJUuFJnnkF7S16BZp"), // BABA - Alibaba Group Holding
    pubkey!("BBosJLw8ZzoATiEyywiifx7AgmrD2Cm3XjFWbhbRhChy"), // BB - BlackBerry
    pubkey!("BEBVfXZ8uCxRBuunEbW9wcq9r7W9ySNo14oJBJkDgpZx"), // BE - Bloom Energy
    pubkey!("BLKKYfF41c6xRPrtX1rdYbxLE1xxEdZUnxQsB5hcZ8Nf"), // BLK - BlackRock
    pubkey!("BoTx8y9ynfdxf5ZjWtCoBVkff52qKA82ysaLU8ZM6d8T"), // BOT - RoboStrategy
    pubkey!("BRVaZKg6J9iF2BEsdpsxJ9NyvN9PPxPZuoQUX2v8qqkk"), // BROS - Dutch Bros
    pubkey!("BULL151gUXcFV5wXEUqu9Am2L7Qt4bTJRLRuAUjkcspC"), // BULL - Webull Corporation
    pubkey!("CBRSFBbQPpp77NpMcwbTN6fhBHmZM94oFLJvruytw4Fi"), // CBRS - Cerebras Systems
    pubkey!("CzLTZppPdZtTjyq3WGpHLstoc3GLhu7zH5Zg6xUa6Gv5"), // COPX - Global X Copper Miners ETF
    pubkey!("CZEB3WNZuF2Yz1z2H81RcCk8T7fsw82KB33zqamASVsg"), // COST - Costco Wholesale
    pubkey!("CRWVJeR2yEZuDUKYfGuKCHvLz8ywn4LGvovHfy5WiFmi"), // CRWV - CoreWeave
    pubkey!("CYPHuMmCL1GxJWa2tsPhLKykC7GrHJTCHwbXD4g5uawK"), // CYPH - Cypherpunk Technologies
    pubkey!("DELL2aRKQz7DMq5DrKLtkn47ZCnbxXPZXrSGbkmd13wy"), // DELL - Dell Technologies
    pubkey!("DJTu7vi8norVzdVAffgvb39VP7wjKeTsgaMBJrzfxvoF"), // DJT - Trump Media & Technology Group Corp. Common Stock
    pubkey!("DKNGQFNGQmoBdXSRGKJ8tTu7uPDasw5JDcfMmWniNfow"), // DKNG - DraftKings
    pubkey!("DNUTsCvKbKwu2RM72cUuW3TD9YpzArzACcqYQssjPLSk"), // DNUT - Krispy Kreme, Inc. Common Stock
    pubkey!("DRAMjSWR7HRfJKjRkvQWYL2bcaejaVhuxEcjf4pAY4Cw"), // DRAM - Roundhill Memory ETF
    pubkey!("EWY4owSJYMpwN33qGDu5gGxpkQkpMJu8ZUsQJaNZG5dv"), // EWY - iShares MSCI South Korea ETF
    pubkey!("EWZzCki1igp2y73UyUM4RCG4QUVMQAtnrQUVNaDHzDxJ"), // EWZ - iShares MSCI Brazil ETF
    pubkey!("FLWSojG1gB5VStYR3Sb4nQFRt43UBYkqih1j2CpVLqgd"), // FLWS - 1-800-FLOWERS.COM
    pubkey!("FLYRq3en8r2Z69gN3KyAnDrvnitEJNkwPYY7favinHeD"), // FLY - Firefly Aerospace
    pubkey!("FWDtiB5fXHdVAewPqvHPL2dh4aBC1C6GacQbePoQXKjz"), // FWDI - Forward Industries, Inc.
    pubkey!("GPRR2u6NS5yBQHWGauoJ9HXgjrTH8dDsrBfTV5zAYvDH"), // GPRO - GoPro
    pubkey!("GRNDYDpqwpCm6jVxpbh4xT5AM4r3p391qYsKTHqgaET2"), // GRND - Grindr
    pubkey!("HiMSSzzwkZkrXJ4PGVJRdtfLaANeAztjjcgk5Dxe7Lwx"), // HIMS - Hims & Hers Health
    pubkey!("HooDYv5RewLRiMLnEVq3VJqdqxhuE6c5eYvqejMC3e9A"), // HOOD - Robinhood Markets
    pubkey!("HTZsLG4zqaNvWMwXSLHH3GG5KyJpKwpBRsKVdMG6hvzP"), // HTZ - Hertz Global Holdings, Inc Common Stock
    pubkey!("BMKdM4yUxX12moFqVk195k7coMbaybd4RUKCUdm7D1Sk"), // IBM - International Business Machines
    pubkey!("iNTCy1qTsUEZQe3DSocLz1ZXXai34Gdw8THQh5rxFaF"),  // INTC - Intel
    pubkey!("NQ5hSuXQZrbnrwcDVk2qN73njjd3E3v3badYHnj5thF"),  // IONQ - IonQ
    pubkey!("RENzhrJQgmAnfcLhU1U5XwAMc6TC15UA6jCbPBaasnj"),  // IREN - IREN
    pubkey!("JNJg1znKdF712Phe7L7z52AATAvEjEytBdN2w8Lnh1Y"),  // JNJ - Johnson & Johnson
    pubkey!("LLYuwZ33keFihgwoxXsBawy31AiRFLFSva32TYq5TvD"),  // LLY - Eli Lilly and Company
    pubkey!("LMT3i1BHgixFqPUgcyteJhnEz2dpy9i3cYy4pi9BoeV"),  // LMT - Lockheed Martin
    pubkey!("LULUmT9VMttkfAJE236LXJcYJ2tTP7nunrSWR5G1BdS"),  // LULU - lululemon athletica inc.
    pubkey!("LUV9GB51PNZNRyzzyYK3rtqFfvDvWtRiXZ34wVq2HrX"),  // LUV - Southwest Airlines Co.
    pubkey!("MGMuubtUEirmkhfEQdmGUh4pr7HuUdMWcZXFtpPbVJD"),  // MGM - MGM Resorts International
    pubkey!("MRNAzXzhNcaEXJPibHEn8cd4vyekCDiivTyEwswLUCT"),  // MRNA - Moderna
    pubkey!("MRVLSjkR2ceUBukujaD3xCyHP1H3B2SzpsNTZF546jo"),  // MRVL - Marvell Technology
    pubkey!("MSTRdWXMeZxdE8osAQy3fA4rvTY5rgummDSMEx6U7Nz"),  // MSTR - Strategy
    pubkey!("MUxEsUKSMACyw5fZf68wxf5FLnZVhtU9CwH8uNNGay1"),  // MU - Micron Technology
    pubkey!("NBiSF3UaVUFtRzHwAfxyHsBCAZWGEKnMpewAE4oh7BG"),  // NBIS - Nebius Group N.V.
    pubkey!("NFLX7qV57zuVxCoHy3s1jiGZyALraLNwttbxdvmYJLJ"),  // NFLX - Netflix
    pubkey!("NKEda5nHhNGgjrE9nDdMvaEmkmJ96qqxzBVZEcKmjSg"),  // NKE - Nike
    pubkey!("N7Q5fYX7YRnDQksfdBKnoUb3awm92n7QNAD35X3Rq1X"),  // NOK - Nokia Oyj
    pubkey!("NVDAVuiB7hwd3m5Wa1JuHNovPaPG6BH1QNztbKFxNjv"),  // NVDA - Nvidia
    pubkey!("PFER6ENqP8r8NF3CqVt4mFowxsin3V5MLidBNQFCC3x"),  // PFE - Pfizer
    pubkey!("PTNzAfFAB4LvoUQEUUGrFMyUoRLExMYjH6CcfyQfsVP"),  // PTN - Palatin Technologies, Inc.
    pubkey!("PUSAG1stksTcAoK9MjinEHUwAYRJsbovywYCk37m6hy"),  // PUSA - Powerus Corporation
    pubkey!("QUBTAD8C9bMU9LvmMNgKPhrmBGbHvxpu6vfWQtThxxw"),  // QUBT - Quantum Computing
    pubkey!("RACEyWiM2ztEZcJx2AHXU2eWjhxU57x3vXn92b39dLD"),  // RACE - Ferrari
    pubkey!("RBLXDGRD64AtRamHMFVcjqne3Ar7NLWtFtYNtsrf1cE"),  // RBLX - Roblox
    pubkey!("RDDTGbhHwVXfyCvQMXzzowKjf5qrYBZAnehoXW83ooh"),  // RDDT - Reddit
    pubkey!("RcZmt84VMJv9bDhKqmw1uWDahYrUT468VwAChTnfD8p"),  // RIVN - Rivian Automotive
    pubkey!("RKLBnAXGqv31iZomqsuAWkQm1aqC7JwwvbCfzGdqAhz"),  // RKLB - Rocket Lab
    pubkey!("RUMsPfFZFnN1ZmGANwP7FNMJMjKH4m9RiMePrtVtLe7"),  // RUM - RUM Group Inc.
    pubkey!("SCHHJ3jRdSjeFEVAaLrnYdx3Brphn92Ys7z1qkiCtPX"),  // SCHH - Schwab U.S. REIT ETF
    pubkey!("SH55hfaipFAbwT42nQYhRoM5o5t61QpkmJ6p62vXB3m"),  // SHOP - Shopify
    pubkey!("SKHYhSjuRWHgikq8eRKbtBbpABgJSkd7ytQV14i9EQ3"),  // SKHY - SK Hynix
    pubkey!("SNAPcESrvnH8yUdgeMF6xm1hym9b6hW6s8YeqeHdZFz"),  // SNAP - Snap
    pubkey!("SNDKbwMUQvZhnLnxLduradgLHG5KrPuKwpnrkkGRhfH"),  // SNDK - Sandisk
    pubkey!("SPCXxcqXj6e5dJDVNovHN8744zkbhM2bYudU45BimGb"),  // SPCX - SpaceX
    pubkey!("SPHRp8cZaSQBTp1KMNP4V1X821SXhXWt4Q2yLdyHzju"),  // SPHR - Sphere Entertainment Co.
    pubkey!("SQQQAa3gUxgnqcEsdjPA4RQNQBgB97TnG2hJhk2WaGE"),  // SQQQ - ProShares UltraPro Short QQQ
    pubkey!("TTWofwAge91oFhZs7kpQdyrVRkmevgM88xijGvQFbKo"),  // TTWO - Take-Two Interactive Software
    pubkey!("UPSqUeMHcWbkdg784XuBUEF9DtySSnW9ur5LAVdcuB9"),  // UPS - United Parcel Service
    pubkey!("URARfsinxCRw4JpvQhuT4CxavdZXZEMjv9ZwWmWpwag"),  // URA - Global X Uranium ETF
    pubkey!("USNv3NkKA27Dh4nsHJDPhW4VoEcTQmjoZyJt2dqdwFu"),  // USO - United States Oil Fund, LP
    pubkey!("WENAZ2WyPbmgvUcKfQ8hyMDfBQP9bZ65hsZ5KTFrRGZ"),  // WEN - The Wendy's Company
    pubkey!("WULFeyfrj1VJKD9HhRTcW8R4g5HefUA11HDEdBv2WxD"),  // WULF - TeraWulf
    pubkey!("AuzVdTnWMapP5B9PatcL7a2kW8Rym8SZabYSqfHPVo7w"), // A - Agilent Technologies
    pubkey!("AMBAHqGPjjtJaHPPSuvPu2mPCJdkGZpmhHQa5pkXbxrM"), // AMBA - Ambarella, Inc. Ordinary Shares
    pubkey!("GMETpjLMbTGTcJVhnxQLv1kxje2E7ZMR9Hnziu5Mt7oo"), // GME - GameStop Corp.
    pubkey!("LASEwYSQ9MrLY4kKNX5zwfndu1py1DwpBT98Eavr2Ju"), // LASE - Laser Photonics Corporation Common Stock
    pubkey!("oURARB5TZqLn8iBGjvXjQyr1gfCfQV3c3YeUvduhcuZ"), // OURA - Oura
    pubkey!("PENGTDQeQSXEjKcYw3CTQi9qznxcTHXAA7LMfUNrxLV"), // PENG - Penguin Solutions
    pubkey!("PUMPPHYa9QQuYhDK1RjC2dmpvigT7LgtofHr6k3M4wt"), // PUMP - ProPetro Holding
    pubkey!("XYZdVZ8jwqZCGN4XR7CiGkeWggo5huk7kNuXaCVs4nY"), // XYZ - Block
    pubkey!("Xsc9qvGR1efVDFGLrVsmkzv3qi45LTBjeUKSPmx9qEh"), // NVDAx - NVIDIA xStock
    pubkey!("XsgSaSvNSqLTtFuyWPBhK9196Xb9Bbdyjj4fH3cPJGo"), // AVGOx - Broadcom xStock
    pubkey!("XsXcJ6GZ9kVnjqGsjBnktRcuwMBmvKWh8S93RefZ1rF"), // AMDx - AMD xStock
    pubkey!("XsafvsGtzFqqHgTnA3aPC83EAMkacU5mcGtcSayhpVV"), // TSMx - TSMC xStock
    pubkey!("XshuHQ6o6SVpUNawvnnTMxsZ4tacZsNgVCLorv7TkFq"), // ASMLx - ASML xStock
    pubkey!("XswUFSYE5CWsZM3X3yo6e2pZvxcAzx912DonGvgUFka"), // ARMx - ARM xStock
    pubkey!("XspzcW1PRtgf6Wj92HCiZdjzKCyFekVD8P5Ueh3dRMX"), // MSFTx - Microsoft xStock
    pubkey!("Xs3eBt7uRfJX8QUs4suhyU8p2M6DoUDrJyWBa8LLZsg"), // AMZNx - Amazon.com xStock
    pubkey!("XsCPL9dNWBMvFtTmwcCA5v3xWPSMEBCszbQdiLLq6aN"), // GOOGLx - Alphabet xStock
    pubkey!("Xsa62P5mvPszXL1krVUnU5ar38bBSVcWAB6fmPCo5Zu"), // METAx - Meta xStock
    pubkey!("XsjFwUPiLofddX5cWFHW35GCbXcSu1BCUGfxoQAQjeL"), // ORCLx - Oracle xStock
    pubkey!("XsoBhf2ufR8fTyNSjqfU71DYGaE6Z3SUGAidpzriAA4"), // PLTRx - Palantir xStock
    pubkey!("XsQLZycSZ7QnBBdBXQaTbQdiUcbRqjNJgyBGAMzhHav"), // MUx - Micron Technology xStock
    pubkey!("XsnhgGRQwhExfS2bmWzR6EYddKGPRGDEjeJsatkmKqU"), // SKHYx - SK hynix xStock
    pubkey!("Xswbpc8UqU6e1j9QZEWCjBMjyvz4twqD7PCy6j2e7jj"), // SNDKx - Sandisk Corporation xStock
    pubkey!("XsQZdaWUAGC4R3fgD2N1fupKvJfJq6YM51ccnsLUWFA"), // AMATx - Applied Materials, Inc. xStock
    pubkey!("XsSN912SN4Whn2xn59vWZHt1uLbw9WnoNGp5MigmHFf"), // LRCXx - Lam Research Corporation xStock
    pubkey!("XsuxRGDzbLjnJ72v74b7p9VY6N66uYgTCyfwwRjVCJA"), // MRVLx - Marvell xStock
    pubkey!("XsrsM2RgtYxXqxmy4iWgxQJUkkHG1U5wzi74sVNUW8m"), // ANETx - Arista Networks, Inc. xStock
    pubkey!("Xsu7Tc5J2fVUE4H5vYAiSr34cvLJeCsYPMjAYnayQn6"), // DELLx - Dell Technologies Inc. xStock
    pubkey!("Xsii5eERa2sKFyTHQqdYxpxL5xoUSLVurzHeqBEMBho"), // NBISx - Nebius xStock
    pubkey!("XsLUiVEwYeoneKpgR1C2Q4DBUZhX4xDktSCfQqq8zmn"), // VRTx - Vertiv Holdings Co xStock
    pubkey!("Xs8S5L4HkJpeBWF1J4oyUX6rVwGHmG7GZ7PiChPt7nY"), // ETNx - Eaton Corporation plc xStock
    pubkey!("XswXzAsMV9kebQjCVtr1btvrQgQ7C4C9kKgH4QYAVzw"), // GEVx - GE Vernova Inc. xStock
    pubkey!("XsLR2VCGNzYVLfJGEwwNZsRYDZBGyBnoQH5rPGvwodA"), // PWRx - Quanta Services, Inc. xStock
    pubkey!("Xssu2cDLdZXZYrq17frTVrb3meumRCAzEf7pXyxoWVN"), // CEGx - Constellation Energy Corporation xStock
    pubkey!("XsT9Z9BYM5Bp2JAuKVgw1XLdENGyvQwC5C2gAuzeKyY"), // FCXx - Freeport-McMoRan xStock
    pubkey!("Xsq9sEQjYiUTSZ55RrbHAzVfz8HotwFGrqgxkgiv4LB"), // URAx - Global X Uranium ETF xStock
    pubkey!("Xsnuv4omNoHozR6EEW5mXkw8Nrny5rB3jVfLqi6gKMH"), // LLYx - Eli Lilly xStock
    pubkey!("XsfAzPzYrYjd4Dpa9BU3cusBsvWfVB9gBcyGC87S57n"), // NVOx - Novo Nordisk xStock
    pubkey!("XsgBcHP3frsQGHr5EoDdoPZMANk52Y4JPg5BYx98NwL"), // ISRGx - Intuitive Surgical xStock
    pubkey!("XsbEhLAtcf6HdfpFZ5xEMdqW8nfAvcsP5bdudRLJzJp"), // AAPLx - Apple xStock
    pubkey!("XsEH7wWfJJu2ZT3UCFeVfALnVA6CP5ur7Ee11KmzVpL"), // NFLXx - Netflix xStock
    pubkey!("XsDoVfqeBukxuZHWhdvWHBhgEHjGNst4MLodqsJHzoB"), // TSLAx - Tesla xStock
    pubkey!("XsYD72ntjj7ZwoFDZCDmN2gamTcLpnywqvG7PQN5vCN"), // SGOVx - iShares 0-3 Month Treasury Bond ETF xStock
    pubkey!("XsCAXu7xTaZMG9b9KJhNWYapuvNjxPuE4SysZq8uvMq"), // JPSTx - JPMorgan Ultra-Short Income xStock
    pubkey!("Xsv9hRk1z5ystj9MhnA7Lq4vjSsLwzL2nxrwmwtD3re"), // GLDx - Gold xStock
    pubkey!("Xs6B6zawENwAbWVi7w92rjazLuAr5Az59qgWKcNb45x"), // BRK.Bx - Berkshire Hathaway xStock
    pubkey!("XsGVi5eo1Dh2zUpic4qACcjuWGjNv8GCt3dm5XcX6Dn"), // JNJx - Johnson & Johnson xStock
    pubkey!("XszvaiXGPwvk2nwb3o9C1CX4K6zH8sez11E6uyup6fe"), // UNHx - UnitedHealth xStock
    pubkey!("XsYdjDjNUygZ7yGKfQaB6TxLh2gC6RRjzLtLAGJrhzV"), // PGx - Procter & Gamble xStock
    pubkey!("XsaBXg8dU5cPM6ehmVctMkVqoiRG2ZjMo1cyBJ3AykQ"), // KOx - Coca-Cola xStock
    pubkey!("Xs151QeqTCiuKtinzfRATnUESM2xTU6V9Wy8Vy538ci"), // WMTx - Walmart xStock
    pubkey!("Xs4f5EbxL1bB89N7ADF7smF8fXxifJa3t6SexZufBWA"), // NEEx - NextEra Energy xStock
    pubkey!("Xs3ZFkPYT2BN7qBMqf1j1bfTeTm1rFzEFSsQ1z3wAKU"), // AZNx - AstraZeneca xStock
    pubkey!("XsRvd1meWQ9kW1SrZPa1jokQqmBoPWWjd6wGTgdp5E6"), // CATx - Caterpillar xStock
    pubkey!("XsaHND8sHyfMfsWPj6kSdd5VwvCayZvjYgKmmcNL5qh"), // XOMx - Exxon Mobil xStock
    pubkey!("XsMAqkcKsUewDrzVkait4e5u4y8REgtyS7jWgCpLV2C"), // JPMx - JPMorgan Chase xStock
    pubkey!("XsoCS1TfEyfFhfvj8EtZ528L3CaKBDBRqRapnBbDF2W"), // SPYx - SP500 xStock
    pubkey!("Xs8S1uUs1zvS2p7iwtsG3b6fkhpvmwz4GYU3gWAmWHZ"), // QQQx - Nasdaq xStock
    pubkey!("XsLT5v4DAd1kwViPQh3SZiT5kzJfNLxzJxJN1dySZTe"), // VXUSx - Vanguard Total International Stock ETF xStock
    pubkey!("XsbZDdYJKxEBQASz4KBj23NRgymXw2NK6mbNQ6PYhC7"), // FEZx - SPDR EURO STOXX 50 xStock
    pubkey!("XsraCsjDB8cYn672MZSxCzZBhrodEPXD7pWd5EiRUhm"), // BITXx - Volatility Shares 2x Bitcoin Strategy xStock
    pubkey!("XsvFzSYaLgkoB2iHQiSMhf6e5bVcRThJZvRPiySbhjq"), // AAONx - AAON xStock
    pubkey!("XswbinNKyPmzTa5CskMbCPvMW6G5CMnZXZEeQSSQoie"), // ABBVx - AbbVie xStock
    pubkey!("XsDZMGEU8zadWFCkTtPBoPWYcUX3JHVmghnwf2Mve2q"), // ADBEx - Adobe xStock
    pubkey!("XsiXtK4niZXG2MFkDmgRcZskNYoetURTV9XhgRf1ESL"), // AEISx - Advanced Energy Industries xStock
    pubkey!("Xs8kHDEs5w8ERuTmefBXsJy66G5mh836VgDFfgpuRej"), // AEHRx - Aehr Test Systems xStock
    pubkey!("XsnNNkJDJz4T9t6KtFUXCwhD7ST39Yeizi3vncwLqih"), // AEOx - American Eagle Outfitters xStock
    pubkey!("XsBBTUAnNsSfXLAF9bte8S3947fyf4z5SFBqBGHxR5K"), // AVAVx - AeroVironment xStock
    pubkey!("XsKfV36es7HHpJFKrDqaaXSFC1dwsrwLit4N67JH5LV"), // AFRMx - Affirm xStock
    pubkey!("XsQ2JJPzG5EMvkRPSshA1mmpVvGPKBNYQmvuEnzpJhZ"), // AFLx - Aflac xStock
    pubkey!("Xs5drDwMFxkcChq5cU49EX2oTQUVWQ64qsSL5bVQoS3"), // Ax - Agilent Technologies xStock
    pubkey!("XsycyYwi9Kekp26jyGg3x5Fo1JDRr3XnTJmVaf5CRUb"), // AGNCx - AGNC Investment xStock
    pubkey!("XsdeDQyocwoXYq4rvm3eAxN4uDUqww78i9nKLABhp6h"), // APDx - Air Products and Chemicals xStock
    pubkey!("XsJW9EhZBVQawVj1aGFweU2LwQjZCvyHVYb3vXq8s99"), // AKAMx - Akamai Technologies xStock
    pubkey!("XsJ8inhc3J9kSWv436iEpfjiXUqULAP4oLMHzitGaz9"), // ALKx - Alaska Air xStock
    pubkey!("XsGea4488p7ydWQTQMYxtvCXDJiAJwgt4HsoKiKrGa3"), // ALBx - Albemarle xStock
    pubkey!("Xs2XDRGh6AhivmYgaVa8woDGKGccR1WnnGnw9vqSWD9"), // AAx - Alcoa xStock
    pubkey!("XsmMYEkQRXZ9CodFfqGEwNvtqVZdsDLHoYqPVm2EpHX"), // ALGNx - Align Technology xStock
    pubkey!("Xs1asXCeRWVFLE3mun2UZhaY9VEDYfyjUhp5mjkjX7b"), // ALNYx - Alnylam Pharmaceuticals xStock
    pubkey!("XsztStxbbGZE63p8DyD8YCpzyWFJwwy2PNcyaf3cZz8"), // MOx - Altria xStock
    pubkey!("XsP9JkJPYsrMKazFJVhYfYUx3hUdfSeh9qHKLXq6bbU"), // AMBAx - Ambarella xStock
    pubkey!("XsuDF3uVgT3SMSvX6kssq9FmChHZRBaK7TFdEapnxk5"), // AMCRx - Amcor xStock
    pubkey!("XssLKajmwH79hK14cqp65fFg2Eyteh23nmSaJpKRmzx"), // AEEx - Ameren xStock
    pubkey!("XsrisVGB3qU1p3KGR7fg4JuxHkeCpdPtByJpYLB4pbH"), // AALx - American Airlines xStock
    pubkey!("XsP8GpS93XjmDg5oVVCVqZsffkWadVCqZYp4BFFJnJm"), // AEPx - American Electric Power xStock
    pubkey!("XstYR6KGqZFchAuhHoEe4mL8W2kL9aqGW1EDBKRrmGB"), // AXPx - American Express xStock
    pubkey!("XsTg4SHooxeYYvQK5Q2wdKBKY46UdtZkF2LfNwbHEtu"), // AIGx - American International xStock
    pubkey!("XsvYPZkrDDg7259oDv5E76HSmeLWgHPgNEQ44KVbJJU"), // AMTx - American Tower xStock
    pubkey!("XsUQA7YBvJyBXDrQy23WNydKWdhVCPRVMxEF6MgXytd"), // AWKx - American Water Works xStock
    pubkey!("XsqWwT9y8ajA8LXQEy7msQmM6ExkP8Va3XYcr7oVhXg"), // AMPx - Ameriprise Financial xStock
    pubkey!("XsVNxwvuQghf6TYBmCck75ddYD3KiWozujR8C67bPXC"), // AMEx - AMETEK xStock
    pubkey!("XsgKwBMbv8LDzEYRgbKtyqsub6QB56PQVzgTGbiGaNe"), // AMGNx - Amgen xStock
    pubkey!("Xsqbg1tmAvNT5332hhX4hMSznqf2NJ1tz2cNWmAkRD3"), // AMKRx - Amkor Technology xStock
    pubkey!("XsvqNba1k4wF5ZEZ6Nrx8AXUDkKzAFuj4HaVDEemyYH"), // APHx - Amphenol xStock
    pubkey!("Xs3CytsvJshAYy1TQjq2o2yVWCyEGinNpvkB6f3Qs31"), // ADIx - Analog Devices xStock
    pubkey!("Xs61MkDiQRGDTskN6farB7Ehnj3KayM3PnuRjaWFHWi"), // AUx - Anglogold Ashanti xStock
    pubkey!("Xscms1jQDud7fsxLTKibMVxSTSQz3JFeJ1HDshRkPwc"), // ARx - Antero Resources xStock
    pubkey!("XsLd6TthuyfbYSP89Ze7DJRw1ZJmPSfFwaSZ1bP9Ejb"), // AONx - Aon xStock
    pubkey!("Xs5b8FFins32GgTH2FURV3Y873yA85jc4kZ6r8H9YyR"), // APAx - APA xStock
    pubkey!("XsosRSUc3n5pod3YNNyQ7go6811xWYd3YME2uEQohEx"), // APOx - Apollo Global Management xStock
    pubkey!("Xs2ZEuDVSQkNnXHqfqEYKVShLHpecyKfdfpEYwiHtQE"), // APLDx - Applied Digital Corporation xStock
    pubkey!("XsGHwSbPaUJu6r5dtJLHXkXenPgCQf4Sx2hb4e2sbCZ"), // AAOIx - Applied Optoelectronics xStock
    pubkey!("XsPdAVBi8Zc1xvv53k4JcMrQaEDTgkGqKYeh7AYgPHV"), // APPx - AppLovin xStock
    pubkey!("XsiKaTXpReUM51cT42hZFo8HQLjaN4vPxxGbUcdTRfe"), // APTVx - Aptiv xStock
    pubkey!("Xs7Dude9fBWENN3Vi25cwQmAF5BXR2zXrFMgPjKWmSS"), // ACGLx - Arch Capital xStock
    pubkey!("XsK6kBQe7QDQVmW1PupBZCc4oBpPbsRdPgvqCqvfCXX"), // ADMx - Archer-Daniels-Midland xStock
    pubkey!("Xsm7SVTuyeaeD5fwaWD1RFKX7Rr8WmNGz5j35r5npth"), // ARESx - Ares Management xStock
    pubkey!("Xszh2XpEww4ndD8CyFzD5DcmqXpw7HMkEdSU1Ngunbe"), // ARWRx - Arrowhead Pharmaceuticals xStock
    pubkey!("XsYka4UhXnzoqmdPCfqX8EZBQcN2MkrG4gJt4SxGvsN"), // AJGx - Arthur J. Gallagher xStock
    pubkey!("XsR4LAtaBgTKTRUhiijY1ba13nx4bepeEcag2Pr4dZ1"), // ASTSx - AST SpaceMobile xStock
    pubkey!("XsuJLDjTibUFnh1gNgnXeDSjQTVffnaWFKzUht5rVvc"), // ALABx - Astera Labs xStock
    pubkey!("XsqML71RLbyUM3CmY4EYN88zE6BJkpvJACnUKBdGD3x"), // Tx - AT&T xStock
    pubkey!("Xs7jmdBozN4RTu3YgSZeLcimiHJcsovXjvRtHhJbafT"), // ATIx - ATI xStock
    pubkey!("XsLhefHBtc8r9nStXgXtu9tHeEWYRXxSXrpbnfwTJ7v"), // TEAMx - Atlassian xStock
    pubkey!("Xsz1UqWKSjB4X7zcvXV6XVL5eBA2YaRqEjXZVP6GXwZ"), // ATOx - Atmos Energy xStock
    pubkey!("Xs2jBYo5VxMmuNjpBnMShsLB26dtK75bt42ag93ZdHU"), // ADSKx - Autodesk xStock
    pubkey!("XsCBDrb61PyQqTJp7uHhD4vj1MkD5UK6jpxUBbgG7i1"), // ADPx - Automatic Data Processing xStock
    pubkey!("XsfLtqmidw4ouMrdWvH5usSs3r77Usm6rXGygxhRTjz"), // AZOx - AutoZone xStock
    pubkey!("Xs1KurTeKA8Y6iYwqiEkc9uKLWyAUYMZWsE7wFJ6z4p"), // CARx - Avis Budget xStock
    pubkey!("Xshh7vNfHqyqbnfuvhn8o53wXcuUbBYKASPq8DEHPbx"), // AXONx - Axon Enterprise xStock
    pubkey!("XswiwGt897dzZU8eHoojgUu2ka34brmPGvWeVHcHho9"), // AXTIx - AXT xStock
    pubkey!("XsnStpqwfPD5mC1KRVrJ3w6kxEYVkUQivDHBSEr4uxf"), // BIDUx - Baidu xStock
    pubkey!("XsBE7iAzaJEBpSwG5JZ7zQTWNgTaerweBy7nJCachaJ"), // BKRx - Baker Hughes xStock
    pubkey!("XswsQk4duEQmCbGzfqUUWYmi7pV7xpJ9eEmLHXCaEQP"), // BACx - Bank of America xStock
    pubkey!("Xs3yrczxLidKAArdTKepU5NV8KUmwFM11NMyYbGqWDZ"), // BDXx - Becton Dickinson xStock
    pubkey!("Xsac5qkc4B254kpnb2qneeu6qsNX1fPtACeuNEiXWMR"), // BBYx - Best Buy xStock
    pubkey!("Xsv9aoiWTdvBzVpvetL7LBkuS9xvceDdbLkfqMbTVDx"), // BIIBx - Biogen xStock
    pubkey!("XsqgvqTs3e14JWGKCbLae3gvjxrVqnz5WmeVD9dodkx"), // BTDRx - Bitdeer Technologies xStock
    pubkey!("XsAGAMqiAySqkUDf5ZXCTR4v6hSgcBj7mQQy7w3Nja7"), // BJx - BJ's Wholesale Club xStock
    pubkey!("XsYtTeabUTJnkxWrHZUChwcKpNq6BbqPbWs9ZgLk4Xa"), // BXx - Blackstone xStock
    pubkey!("XsDP3d7dBNMVVZT6W5abD9jxZPGbrenPaj322sWn9Ct"), // XYZx - Block xStock
    pubkey!("Xsnb4PjGtFsvTyp7HJNWfJLEbK6MoSZV8o7xMo3PuBn"), // OWLx - Blue Owl Capital xStock
    pubkey!("XseQvfvuDT3KHKiwPNW1gcjfHbFjEMVTV5NJzDdNQmu"), // BKNGx - Booking xStock
    pubkey!("Xs34rXKEs63rPs3qWZeAubRmSVJNG5hSy6Ubt7Qag2F"), // BWAx - BorgWarner xStock
    pubkey!("Xs7vm4sZj1LiNLvnxUCyUTuxRMTt4WGJfdbCASTYhN9"), // BSXx - Boston Scientific xStock
    pubkey!("XsYKw8tSMPRNNJZyWV68MCEFbVkYRqhtbgqLFtUyRgq"), // BXBLx - BOXABL xStock
    pubkey!("XsRwmoXhovBAdH98UJUhzDtymh5HLA4sQj3SeEVuSch"), // BWETx - Breakwave Tanker Shipping xStock
    pubkey!("XsbJzb2V963yBoCKjRJqSLbNUeBsqy6uV4dVnCnNp4q"), // BBIOx - BridgeBio Pharma xStock
    pubkey!("Xsa3dm4UT6TdzPJV75UiuGUeGywYUnfSNhsV5nHPZEu"), // BMYx - Bristol-Myers Squibb xStock
    pubkey!("XsvEMALzsRVnPED4KEAd7pivT5ho9vgqzqZxQxNBkWW"), // BRx - Broadridge Financial Solutions xStock
    pubkey!("XsLdXScusw7mCSATbp7bWKfyBR1RVWEEayKgRcmJ8dZ"), // BAMx - Brookfield Asset Management xStock
    pubkey!("Xs9hw3a4acJr629NUzDbEC376rv3AhfmwvXm9BNnD9G"), // BROx - Brown & Brown xStock
    pubkey!("XsvHMmbDcd14DHHW16PkxPGW7ks77ehxUv1E9Zmxgj4"), // BTGOx - Bitgo xStock
    pubkey!("Xs15osmjEQrq49CkPigXhrRQ6kHkX2cjAPGvpkQFyY8"), // BLDRx - Builders FirstSource xStock
    pubkey!("XsTk9BmLSTUVGWA8snhsKvfAiJTZwqv4HjLwg5XosTg"), // BGx - Bunge Global xStock
    pubkey!("Xszn8sMqeBqfLDXAsDa9uEYo6WT3LYSD4jje2pGK6k5"), // BURLx - Burlington Stores xStock
    pubkey!("XsVpsBVdjW8SWAu4jAjqJ9qnGYzBmUzNDRcG6uiZ9xo"), // BWXTx - BWX Technologies xStock
    pubkey!("Xsm5zBfHeiGWjraGLwT8DwJhYGjUfhESs7QANcYVp8E"), // CHRWx - C.H. Robinson Worldwide xStock
    pubkey!("XsrB8RyBpbMWm8zCQutdT2n95BgNj4rJFfxuhHFjk2v"), // CDNSx - Cadence Design Systems xStock
    pubkey!("XsH2Wp6y922aQqAjvM8FFLSzvLqVDHry6dqgvCSFM29"), // CZRx - Caesars Entertainment xStock
    pubkey!("Xstd4ems8vhfGwTngpXCzgnSvJ4oD7E5gExUue2KEZX"), // COFx - Capital One Financial xStock
    pubkey!("XsrstNGy7STAR22txNru2ahbEwdBadFJwYRCBNEQYBS"), // CAHx - Cardinal Health xStock
    pubkey!("XsDPeNLGkVzqoCwWD6cPyAtdBu2yZa8j1u9JYS7oNri"), // CCLx - Carnival xStock
    pubkey!("Xs8FjYWcxTREaUj5hv77f4M2UCreGfP7HegLvqzX56X"), // CRSx - Carpenter Technology xStock
    pubkey!("XsvQh2FdVWMpbNsnYAbDNg38GtAWBEjcweJR42yRTgy"), // CARRx - Carrier Global xStock
    pubkey!("Xs8YT2AdFmjuG2HFjcvVTpXPwneiKNHMrR49muHY7z9"), // CVNAx - Carvana xStock
    pubkey!("XsDBYWqoxayY5cqFWs6jNJn4wv5vbP3RkTNqceEQRGW"), // CASYx - Casey's General Stores xStock
    pubkey!("XsPMh6LbBkmy9i8cCDDpHAku6XkeNdDTWF1Pnair5VK"), // CAVAx - Cava xStock
    pubkey!("XsTJV8FFx22kv41kMcuHawx8eoDybUnLxiiEscw317x"), // CBREx - CBRE xStock
    pubkey!("XsMmhKdFMiFdNFBcznYHx2zAU1SLtMnUL2G6bRzPpaY"), // CDWx - CDW xStock
    pubkey!("Xs6jmX6ggZVqEjdPy2eEi6pE27qrtrBMSni91KiXyk3"), // CELHx - Celsius xStock
    pubkey!("Xsst4VThLi3nvYTsNCVwJi1z23LfXrdyKmhArkrSn1n"), // CORx - Cencora xStock
    pubkey!("XsegJtRFtvC3qqhNYdN2QXxumd2fjvWQmithQZ33ehS"), // CNCx - Centene xStock
    pubkey!("Xsx9i1A6e4xc9r5Gxxy4uZV4YSmwAXBJScvGLRv4nH2"), // CNPx - CenterPoint Energy xStock
    pubkey!("XsH9yfX7GuRbCDnNvD4A7x8gvMc8sc8Htz1fQd7VYWw"), // CFx - CF Industries xStock
    pubkey!("XstvDoqmcKjMHZP8FoWy3Ee2EkCf4KZgqNtg21WFW3X"), // CHTRx - Charter Communications xStock
    pubkey!("XsYLZdcMYST418xZhTV6KaHqr8BWqaKS3J45ywqigF9"), // LNGx - Cheniere Energy xStock
    pubkey!("XsyXzuvyKkuWWJvm9ciFAzScF7e3sAHPgpT3CEbrAb4"), // CBx - Chubb xStock
    pubkey!("Xs712gQLCYkFSYJwdu3pE5itNfyKwsXQ1HEGZF8Esc5"), // CHDx - Church & Dwight xStock
    pubkey!("Xsim8883PnGBHyFMFYPqQQ1c9YxoRkiRrdEpe5HR1ND"), // CHYMx - Chime Financial xStock
    pubkey!("XsE5qZhg6oL1ypWwtwXPQtW1EpFSvRmLXk9ceX9TtZx"), // CIENx - Ciena xStock
    pubkey!("XszBTLtg8oMmEWsZHbJJwViiB6unNh2ei1e6miArqps"), // CTASx - Cintas xStock
    pubkey!("XsM1FstDXh1pA4uNXzvJdAop9SRJnnRdWt2o8A2mLpN"), // Cx - Citigroup xStock
    pubkey!("Xsp9avBowUQeip8NfLXcwfw5VT74kc72BafgYPXN74a"), // CFGx - Citizens Financial xStock
    pubkey!("Xsn3H7ACEpSF2ULxeiD6kW4jRZXpurh8ZPttyfoS56W"), // CLSKx - CleanSpark xStock
    pubkey!("XsXzwQgoZ1iyfyTh8ZhHGbrdSLsCkFNKQ6e8DWPWtyh"), // CLFx - Cleveland-Cliffs xStock
    pubkey!("XsR3LAMkzuPP8DnfYaiB2grTWqkBVy286gjJVErK1bi"), // NETx - Cloudflare xStock
    pubkey!("Xsx234fZF5KMZ5catYi7m9P682Pk9TWWHF5jAfYGZJk"), // CMEx - CME xStock
    pubkey!("Xs6Udev3M1atxRjjXCjAg24BdjYuLs6Qj7eTh7XjEjW"), // CMSx - CMS Energy xStock
    pubkey!("XszuEWDcqPz5xvXGJ1sGZd86GX8BrUrfKtt6eFEKB5U"), // CNHx - CNH Industrial xStock
    pubkey!("Xst5zttaWoyT8Td5jNhxwVQ1ku4bSRv8EVJe3SHoGdc"), // CDEx - Coeur Mining xStock
    pubkey!("XsMNmV7wVizphwXgMtXTVhLDpqmKPdfoYPK3JTXXnYW"), // CTSHx - Cognizant Technology Solutions xStock
    pubkey!("XsipFyePxrgwZJrX4s26RJ25cqwpkfn6ec8JLy26w5b"), // COHRx - Coherent xStock
    pubkey!("XsjFmYJCZnfMEVYwPt13fjZNmtFXqSKNd3PXz3pEBeK"), // COKEx - Coca-Cola Consolidated xStock
    pubkey!("XsWzjAeMsR9pMX1aKkq7uDKpr11daUHkmjXya15yCqd"), // CLx - Colgate-Palmolive xStock
    pubkey!("XsvKCaNsxg2GN8jjUmq71qukMJr7Q1c5R2Mk9P8kcS8"), // CMCSAx - Comcast xStock
    pubkey!("XsPTVwrSE3MMZF1bt15vxM7NuuRHdsBJ7mtGW4cttHX"), // FIXx - Comfort Systems USA xStock
    pubkey!("XspahpqdDTxoL1XKPysDSyeMKVaKerguZQeBPqtmFjM"), // CAGx - Conagra Brands xStock
    pubkey!("Xs58CxmUcgT17reeVV64se747XuRfjsJQnpb8DX9TDq"), // COPx - ConocoPhillips xStock
    pubkey!("XsESyqE7kRmE1RVQDmo4aiMGMXogeY1Sdo8FQKwAmco"), // EDx - Consolidated Edison xStock
    pubkey!("Xs6eATub8cqKdztx4asCRFMSzMwtNqAUWiDaUPXxUtD"), // STZx - Constellation Brands xStock
    pubkey!("XshHba6o5aAUPJFL19gmvSn4KRk6N44JVgiFt6UoRsB"), // CPRTx - Copart xStock
    pubkey!("Xs7YhGRdqb6nzTTPqViWue7TT3EXbmWwmD4XdrnW3Ux"), // CORTx - Corcept Therapeutics xStock
    pubkey!("XsBGEXxbBcuu8Nrokj14G8v4ezT3JYWWLneTzXK8t6Z"), // CORZx - Core Scientific xStock
    pubkey!("Xsg3UgvjxpUgV3WZx9Wt9deLx7qvKYp6ZLY6xMY2Dfq"), // GLWx - Corning xStock
    pubkey!("XsPtqd1RVDcZzSmeVwY253J4EHVUqMSrR9FcFmjZjqq"), // CPAYx - Corpay xStock
    pubkey!("XsJXYLby91CWhGhTXds9oxjoc8EdsLy2iuxmq55U1Y3"), // CTVAx - Corteva xStock
    pubkey!("XsJH1qPyJgbJiYo2KZXNirCQxC3QqQgYE1iNHY3Ecm2"), // CSGPx - CoStar xStock
    pubkey!("XsoVbMATH8hMemisZZvGCc9YEkve3Yiirr2VtinKTuw"), // CPNGx - Coupang xStock
    pubkey!("XsPZqWN9qCGwwasxuo3zSEnM6vQbBJ9imktKjUeBMyF"), // CRDOx - Credo Technology xStock
    pubkey!("XsSFnM7Nt2A6cBHWL4wvLDqQKECuodcBjRarpcicGLj"), // CRHx - CRH xStock
    pubkey!("Xs7xXqkcK7K8urEqGg52SECi79dRp2cEKKuYjUePYDw"), // CRWDx - CrowdStrike xStock
    pubkey!("XsbPN4VFi14geuC8D2Tb2u37kQM83VSkfMUyvXsWWoC"), // CCIx - Crown Castle xStock
    pubkey!("XskJzVZDKvqm999PGat2ifznPVmmDXn4Zi1LZ7Y5CVr"), // CSXx - CSX xStock
    pubkey!("XsqZ16VmoeszDniTfGTnwPBMv7j7uSDArddziagHWit"), // CMIx - Cummins xStock
    pubkey!("XsWCYadcnfTLH3W8sBbJZepwotUkfV1qg7QQmzVo8M8"), // CWx - Curtiss-Wright xStock
    pubkey!("XsKWyokwxJfb5aVBfDJYGte4wk2NPMNFo6SivouPeSy"), // CVSx - CVS Health xStock
    pubkey!("XsiNAvTHpvygKjMqb8Hz5Nqeo8NTAsWzT6DYLH287FA"), // DHIx - D.R. Horton xStock
    pubkey!("Xseo8tgCZfkHxWS9xbFYeKFyMSbWEvZGFV1Gh53GtCV"), // DHRx - Danaher xStock
    pubkey!("XsTNaRrDQtFJi56TAQbmk6tHzWQvihWGoafKSJzbJdY"), // DRIx - Darden Restaurants xStock
    pubkey!("XsFDH7DAPJ9q2KrU8itMbb4bWWSH5zm3Yu1aoWgjDH5"), // DARx - Darling Ingredients xStock
    pubkey!("XsQJQHNMLiZCBHgJfq2yK95BFraouUdSspv1aD8jCgQ"), // DDOGx - Datadog xStock
    pubkey!("XsS3GWSysqkDJZYFSuMDFZ2ELSjQAWwMH28vxLThnb4"), // DVAx - DaVita xStock
    pubkey!("XsbahiFKw3f3pgsAwREvQZAdERbV1NGUjWnkzUaBz91"), // DECKx - Deckers Outdoor xStock
    pubkey!("Xsh3Lt6pLpH65udstiFuZThJ6gyQTL6ryVXtNwEy5zd"), // DEx - Deere xStock
    pubkey!("XsQVuhV5fBEb3b8TauFuRimcXSJdVrKqFgQTVqLZiq9"), // DALx - Delta Air Lines xStock
    pubkey!("Xs8WNVWbYNsHsqEu9WTb9AazMfG7wFxFat11V6mVK1x"), // DVNx - Devon Energy xStock
    pubkey!("Xsspestq9YiZofYG2pvuHgYM1aKdRNocveBodzrGjbg"), // DXCMx - DexCom xStock
    pubkey!("XsdLjeamzdsWW5aBhsGVAaQFcZtCL6yF9nhtWfszGT3"), // FANGx - Diamondback Energy xStock
    pubkey!("Xs9aEtZqQDzTtb1PLVJ2W2d6NGJSSaJepTsQ2CZAjhn"), // DKSx - Dick's Sporting Goods xStock
    pubkey!("Xstp4CYu1181rhhi7w32Lj6wQZ2VDjFhQNKZwZARk5G"), // DLRx - Digital Realty Trust xStock
    pubkey!("XsKJGRCayh5LQ6bsET8ZVip8BaRZbWmj9fjSnCcy9P6"), // DOCNx - DigitalOcean xStock
    pubkey!("XsNZ125wJDfhqhdSuGMeVCWgP26pYsY5ZRtiAfKtU2d"), // TMFx - Direxion Daily 20 Year Treasury Bull 3X ETF xStock
    pubkey!("XsjZG9MgLMECjzSSxq3G7rabDceoaDqhN4C4zwce4i8"), // MUUx - Micron 2x Long ETF xStock
    pubkey!("XsCucuUESBi3ZjRxmjwUzGYuf6ZrtZDUvK6XhRA4RR3"), // SOXSx - 3x Short Semiconductor ETF xStock
    pubkey!("XsdZDkoMdUb6iKDAKKappuM7C1Q2HmTqC8jNujbfmCu"), // SOXLx - Direxion Semiconductor Bull 3X xStock
    pubkey!("Xs2LakxmT2YycpcxRDCJ7crPbFf91JKdAAWnfzANFmt"), // KORUx - Korea 3x Long ETF xStock
    pubkey!("XsWfu1svxj6DPqQZFAinCvtDa1EvgAZGx5qocUkrrA6"), // TSLLx - Direxion Daily TSLA Bull 2X Shares xStock
    pubkey!("XszH3SckdYS1Kbyj9wcmjttt9Lp8jMgU39rGL8b9Wcp"), // DOCUx - Docusign xStock
    pubkey!("Xsuex5gGigmPKMs6hsYpHbcg8HwBbBdWkGKub9MQbea"), // DGx - Dollar General xStock
    pubkey!("XsPWGRNQF1Thif9qaadTNtF8xfHTb8sjvw1NC46hkED"), // DLTRx - Dollar Tree xStock
    pubkey!("XsiYzgRPqPjAaJjpDMo2CaQ7c9khuCwD7Hj1po2dxSj"), // Dx - Dominion Energy xStock
    pubkey!("Xs6z8AmfhiHTGmfdcX6hG7hho5P7xnbE21ChtJ94oVj"), // DPZx - Domino's Pizza xStock
    pubkey!("XsuW1yxtbifjEYojFLmoavTWBT7hsuUXXeReAWK9X91"), // DASHx - DoorDash xStock
    pubkey!("XsXG3ytqNM8wMwxbZnbZjY3wFua4nU58ZR6Lwr5wSDp"), // DOVx - Dover xStock
    pubkey!("XsKWMeTN8wmhrkoWKy6tXAj6476Guj8zonzQu7jivqP"), // DOWx - Dow xStock
    pubkey!("XsVK4gk4X1ANBSKF9DGjy9WseKhBfMUuEmP8v7J8u2G"), // DTEx - DTE Energy xStock
    pubkey!("XswU7kXY6dGgMqpiWW2jKSYVZMsCWv5LKcLaFBgYzPv"), // DUKx - Duke Energy xStock
    pubkey!("Xs1CqRVt2mGVfMSHf7M6cDhkjpw8HDF14XbQNzDnGg4"), // DUOLx - Duolingo xStock
    pubkey!("XsHsqDrvsU85aJXNNmPBkxU4STbELJUeTV7zT4cJQD5"), // DYx - Dycom Industries xStock
    pubkey!("Xs8Jb7S3qqWbgkRGvwKmT62MD7gt1qYU7oUaR9oDC11"), // DTx - Dynatrace xStock
    pubkey!("XsDYSbgLh3T7oh59UmRX1F5WjDgJD3nmzACaG5LR1MB"), // EBAYx - eBay xStock
    pubkey!("XsPPAW5wuUa1ALN1NstEkUAajbwycXTRxtWoLfFriy4"), // ECLx - Ecolab xStock
    pubkey!("Xsm4TwT5UtCCjkcrj2eMZDPNCTdBeJacMmRCrxoXpZ8"), // EIXx - Edison International xStock
    pubkey!("XsSfT8eYeR2NqYFwCNLRNfDARHUTXvLeSnGbw2UKqrA"), // EWx - Edwards Lifesciences xStock
    pubkey!("XsJbC4TFupSLpxNwsfq9o1BB5tdTT7eZ3YNerVvE49H"), // ELVx - Elevance Health xStock
    pubkey!("XsHsi7FvQo4T7AkmtyszYkhNdKDCVgrJFS7CWGeaeM8"), // EMEx - EMCOR xStock
    pubkey!("Xskqq9tKh2gc1PBJHf2q6jMgBpt3AojT3HVkWhjoY5h"), // EMRx - Emerson Electric xStock
    pubkey!("XsfkWgfFJJmVZQr1GzRUXtaCvfEcP7PhPeLFrhJFpuS"), // ENPHx - Enphase Energy xStock
    pubkey!("XsD6kFBZCYumf4awQ9tgFFh8m2HGJBtTFdLYt7HcwRr"), // ENTGx - Entegris xStock
    pubkey!("XssZsJPrzg7n9Sc1stfPcB4vygcVeMfz5uZK2gPwNdv"), // ETRx - Entergy xStock
    pubkey!("XsfBx5h3LBhypgy7goD4i8nAfLU8ZZ5QBSQUtASRVox"), // EOGx - EOG Resources xStock
    pubkey!("Xs4q7KSys674dRgCRCX1ttBKEGTobCKRSiFq95KuxS4"), // EPAMx - EPAM Systems xStock
    pubkey!("XsvS1ZY358vAjNJ7s3fz13s64ZqYF36rgUDR8yVJkuA"), // EQTx - EQT xStock
    pubkey!("Xs3dfAaDSQMVrDcEVbxypK4UeXhCbDBK2AgAWCHwqmq"), // EFXx - Equifax xStock
    pubkey!("XsBo6pg8N9kk7YnL69NnBBSZBWDkF1gHWjTAXpYuAVo"), // EQIXx - Equinix xStock
    pubkey!("Xsf8f6sqb9p6JK9oR8UHXk7CmFDvofc1nxeink7AoGA"), // EQHx - Equitable xStock
    pubkey!("XsXarMSoXHF1zMcWM6e37h1S1JezEHizbn7XwNcM7Zh"), // ETSYx - Etsy xStock
    pubkey!("XsmTgboNTsssYV1VZrS4u1YMkcNWTKVMzUjghkXa3RF"), // EVRx - Evercore xStock
    pubkey!("XsqSsaDYsEAFwRwcwchPc9J1zX21VtHiXGJjFt5Ph5t"), // Px - Everpure xStock
    pubkey!("XsgPX8MYf23yPRv9DrT5ofHn4mThBZgkhboLHz5uSx7"), // ESx - Eversource Energy xStock
    pubkey!("XswJRi3vQhng81dGRXnaUjpyCzM89pwvYyHveT2xzEB"), // EXCx - Exelon xStock
    pubkey!("XsodbeGRpfyAzq1MMqZu7u1vCYNKNDuYrCtofaT1PJK"), // EXEx - Expand Energy xStock
    pubkey!("XsFj75v4Tw8TWSDaDoVTHF8RAjEEkmUeKgoRjB9DHsf"), // EXPEx - Expedia xStock
    pubkey!("XsJB61YjcXkEf114fDGMMnWhhFf8a1x12Z6hARUUeAM"), // EXPDx - Expeditors International of Washington xStock
    pubkey!("XseFccApcVhrqASiXFM97ZkX7a2uDWbUWjC4T82LgAT"), // EXRx - Extra Space Storage xStock
    pubkey!("XspZSVf3tmPqV7QXqNzCjmvv11nnAtQ4n4YwPxykt3U"), // FFIVx - F5 xStock
    pubkey!("XsJ6W21MaWMQCqhRLbLTD82427vcMVzxULkomGRg4gJ"), // FNx - Fabrinet xStock
    pubkey!("XsMQfkLnhK1ERGibdgGDHFaSHB8ETDtMiC5TtVZKMAv"), // FDSx - FactSet Research Systems xStock
    pubkey!("XsiZh3X1uThVm4bNh8PzyXVNe3v11oS6v2DCGdkboUL"), // FICOx - Fair Isaac xStock
    pubkey!("Xs58AdBQTkqkejRvfMrGMQMML1m5xMQKVfKgPmqNhwg"), // FASTx - Fastenal xStock
    pubkey!("XsccgtwEgwSxTh553v8iVLxNUrbHJfdnaJUjxgZfr48"), // FCNCAx - First Citizens BancShares xStock
    pubkey!("XsEmsxmtT12GBiqtvTxoNaipTBNEdUs6oq7pwHFHTb7"), // FDXx - FedEx xStock
    pubkey!("XsfdW27Z6gk5otDjYw5R2ad7PP1dKoR1Qx2ayurRBGc"), // FERGx - Ferguson Enterprises xStock
    pubkey!("XsTdkPyDNVtJYUjV7VhyZwtqQmA2Lg4DdTkRjYPD2gQ"), // FISx - Fidelity National Information Services xStock
    pubkey!("XsVsC5yPcJKdeuZDejjymo2KJTuE3zdXZoSVuTXA7qv"), // FITBx - Fifth Third Bancorp xStock
    pubkey!("XssfYT48PeXG2EmNdEaVtQcveNhxYnM42KTqdjmiBS9"), // FIGx - Figma xStock
    pubkey!("XsSbcq8MZso4DLAMgtRKjzCvgozdh4sje8PLj45kxJZ"), // FSLRx - First Solar xStock
    pubkey!("XsLpCFJxUb43H6da8YzrS4j79wuddw8Z4GETup7hKMc"), // FEx - FirstEnergy xStock
    pubkey!("XsXU2DbkVLWPRBG3QEpEXCTKS8Jv8VCxkB7e7ThdyLC"), // FISVx - Fiserv xStock
    pubkey!("Xs7zvvE6QUtYmpMBrz4oWJexS7WSJPwJs7vQsvMCpaG"), // FIVEx - Five Below xStock
    pubkey!("XsFYbD5dUBjCYxwF9oP2QTeJbPBt1mJybme62LGSyyF"), // FLEXx - Flex xStock
    pubkey!("Xs9L3oiA7VxA6STG9RNhRFtDo6G1aubRHJNsDR8PBf7"), // FLSx - Flowserve xStock
    pubkey!("Xsc5BxL1ucvrQNXZqW3CT8M9gWTko1LPSLLkSVzGe9h"), // FLNCx - Fluence Energy xStock
    pubkey!("XsfVkV6AtVAeR2hcpg5Bwwekq2LG3Thcds3oh4woMit"), // FLUTx - Flutter Entertainment xStock
    pubkey!("XsBNJXGu68cBH5hgFxdqZkeh8cMQv32jeEtZtYTYvfS"), // Fx - Ford Motor xStock
    pubkey!("XsvH4G2oumrBsL73QvVvc3pAw8KYWQJaACWsLtWrR9G"), // FPSx - Forgent Power Solutions xStock
    pubkey!("XsweEBLf9yJz58So3Z1WsJAzYFsbTQSdE4FeW7nQBH2"), // FORMx - FormFactor xStock
    pubkey!("XsDum7DWP825ChLAhSN5NYBpoCUsHJJdRKTPseA9VxR"), // FWONKx - Liberty Media xStock
    pubkey!("XsWrVR5F8eUgoienkVAQLwjvtzEAUoWWcwhcDzu3qRX"), // FTNTx - Fortinet xStock
    pubkey!("XsvL1QcWfqpUM7CQzcNsicatBTZK2dgJRTor3SHUXdD"), // FTVx - Fortive xStock
    pubkey!("Xs1fK3nQZurNve1sivRHEBPFTrrZiUn5R2XxYcWBEZa"), // FOXAx - Fox xStock
    pubkey!("XsX857TxHhoS3CbRt58me2xx8zcAn6n5kMJScZKrHJa"), // FTAIx - FTAI Aviation xStock
    pubkey!("XsWa1TvdA54zbHmNiiCrxJimWhuTXQLjCM64Nc6T68A"), // GRMNx - Garmin xStock
    pubkey!("XsfNvqLMCuDnxbtV3BhTm7ckCra1NpCVmokKHpvF9j3"), // ITx - Gartner xStock
    pubkey!("XsfsBjSZoXXKJsxoSt8TyLYsjhBAKFaDPZTCQbFRirP"), // GEx - GE Aerospace xStock
    pubkey!("XsQoxoLVogLGYRRRcFwjdcFQVLxFpCuYfDtvh3vJ2Ft"), // GEHCx - GE HealthCare Technologies xStock
    pubkey!("Xso2H7vbDS6HVokLUecSG5GvWAZXjH7PEHAH9rAYXUP"), // GENx - Gen Digital xStock
    pubkey!("XsAiJ9xc1eBoVEz4npaJqySHLrpCoyo1oFfJJpYSXPW"), // GNRCx - Generac xStock
    pubkey!("Xsgk6nv6V81zQms4mcvUiEtLDdVueXfWPRCaxayqkSN"), // GDx - General Dynamics xStock
    pubkey!("XsKfQxVh7wasRmzPQEs4qvnmayt5KTX1CqvQdJ8XcSJ"), // GISx - General Mills xStock
    pubkey!("XsSFgHPgZgYNSuz1ttNHNp8zUBBMZGngFcMMfKPK6JQ"), // GMx - General Motors xStock
    pubkey!("XsAB7oiJL94kURXya1i35Rv9ZoY9EoJSWM3dQ9mZp7t"), // GPCx - Genuine Parts xStock
    pubkey!("XsmAFboVqfDwGPgXfd7oCunvYAYSzzkuKE2BDRzwjGy"), // GILDx - Gilead Sciences xStock
    pubkey!("XsCkPFPzDHLXF7Mxksy1eDcVdYyMVPmwWNR7V7TE4Z6"), // GTLBx - GitLab xStock
    pubkey!("XsKbuY5Wyzz7Zzz6uxi69hHdxhrE53d746Sb1jBZyD1"), // GPNx - Global Payments xStock
    pubkey!("XsAzGv2quxQWSUphAKcmFeFyJqTgDkcetgp3DmAFVj4"), // SHLDx - Global X Defense Tech ETF xStock
    pubkey!("XsyGzGhqwnbfqeN6TQWK4SHnVJYgbxRdp4aH5L6RwEZ"), // QYLDx - Global X NASDAQ 100 Covered Call ETF xStock
    pubkey!("XsrTYnfAtuMDpQQgZtpQZreJWtjaRnRCTBeekcQi8AS"), // GFSx - GLOBALFOUNDRIES xStock
    pubkey!("Xs3c2aZenyRQwXjki5MDxJEJ2km27ef2rWQMFWx7QKJ"), // GLXYx - Galaxy Digital xStock
    pubkey!("XsZLNUcjAVzNSbU5Z198ngop2mmoinZnw5ijBeMgWhW"), // GDDYx - GoDaddy xStock
    pubkey!("Xs7zs57saNfGWUn3h5d2rFxxs43uVqzAY7iQVPHFbwE"), // INTWx - INTC 2x Long ETF xStock
    pubkey!("XsVEdRV17tPdXHamAi6TVFjyeSRHnCjdY5GXqAaevuk"), // MVLLx - Marvell 2x Long ETF xStock
    pubkey!("XsrH4d2jVBUbZx5YJ3CyPsiN84ugZQGKCbJAoDn9qEd"), // NVDLx - GraniteShares 2x Long NVDA Daily ETF xStock
    pubkey!("XsJwJiZkwUTaZE1q2J1hX6YTGv3xRdZGtqan9a2VsYW"), // GHx - Guardant Health xStock
    pubkey!("Xs6w8aJhwu1d5jKbmScXoitaDEE7HWhyujhkELeh4B5"), // GWREx - Guidewire Software xStock
    pubkey!("XsYq2q3UxkwdHmhxJf5wbuXfDWuy5mYP5xsDBVEDK8Q"), // HALx - Halliburton xStock
    pubkey!("XsJ5MSaKx3Y6bKEfmrRmovGEdRM53KWqEEHuaNUsQpD"), // HCAx - HCA Healthcare xStock
    pubkey!("XsgtMhyQgTevrWQNyLpHxwQeRAeaxQEtoiV1pNyxmfy"), // DOCx - Healthpeak Properties xStock
    pubkey!("Xsp5pDV28HxQDLcoC24G6G2yCNBvVpPJ3DZCjhs7YNR"), // HLx - Hecla Mining xStock
    pubkey!("XsjZ8V6qPBNbhXeFawPssBPUW8f7Sv39uxEnzxCboTq"), // HEIx - Heico xStock
    pubkey!("Xszp97B7zgeyNUFkymHdxXp3Y5KTZ1rzd2bAL9TW34t"), // HPEx - Hewlett Packard Enterprise xStock
    pubkey!("XsVuEGU83SBvgQiuynK2qtinyPVyKukoTCACdJg2767"), // DINOx - HF Sinclair xStock
    pubkey!("Xs66hB4NftAJmu73aeBc6RXQC4DX4LwU4xvnfsz581r"), // HLTx - Hilton Worldwide xStock
    pubkey!("XsR1381yatMKF6oUcBuk9rQEvsoipiZ7G19z9kCQmGb"), // HOGx - Harley-Davidson xStock
    pubkey!("XsYbh3CsZ4qCMcfPKEtHAYbePTywhwCkFvdN7Hx3bHA"), // HSTx - Host Hotels & Resorts xStock
    pubkey!("XsCX8NnXq3qx6zUxXxWwjV2yQnGc9C5y6UyXvE6RG7r"), // HWMx - Howmet Aerospace xStock
    pubkey!("Xs23PmjqLoJvcLJ2LyMzi17HmkFua1yZsmcxWd4q8Zz"), // HPQx - HP xStock
    pubkey!("XsCz2AUyFUuadE9zxfs7ywkbMvaQNeVTeNBK15L43Qc"), // HUBBx - Hubbell xStock
    pubkey!("Xs8sK7KXi6tWULhBq4a8197Ydz4zDdLxtKjK6gBF59x"), // HUBSx - HubSpot xStock
    pubkey!("XseFxRyaaJixiAzKBrSZAAKSLMQKVYceUGadwUAsQ7L"), // HUMx - Humana xStock
    pubkey!("XsuFbhEyBaWikFEfA3QM92rr9VVYm79Lip4cC5rnSp5"), // HBANx - Huntington Bancshares xStock
    pubkey!("XsG5QyZTQnVSpsXRpD92K5ZGoxXjsmfTyx7fW7r18DV"), // HUTx - Hut 8 xStock
    pubkey!("Xs2AVjDnFK6ShEvX6QfSFodHpy8znSBhimB4kvTFTa1"), // IDXXx - IDEXX Laboratories xStock
    pubkey!("XsepazYXhizTmBmiZWfGwFnyUUSepwzQkLRgQyBp2Zg"), // IESCx - IES xStock
    pubkey!("Xs7NEcjqek7SN135xwSafr721njNVyenBdd7NDG2cYo"), // ITWx - Illinois Tool Works xStock
    pubkey!("XsahzH43C2rZpZmYX6eCm2avkeYCzntC168xPdqrDzi"), // ILMNx - Illumina xStock
    pubkey!("XsRZVEBuuhuBU8NTztFVZGVz15Fumfo4UVWNbr3X6Po"), // IBRXx - ImmunityBio xStock
    pubkey!("XsXw4uaUyz6fHfK6qcet55Cg8b5rnf4D1TCJ96FUJqe"), // INFQx - Infleqtion xStock
    pubkey!("XsWx5RzivC8d5XwnLiMcgQqqhXiDnrTDwFCDap1o6NW"), // IRx - Ingersoll Rand xStock
    pubkey!("XsR6i1PnDGCbiV1KHsiMBdKz4MuDgpWHr3QSwC7zNMz"), // INSMx - Insmed xStock
    pubkey!("XshQUKnnPmz7DS8kUR6uXTZVYoa85GFVDAksXLzB9ud"), // PODDx - Insulet xStock
    pubkey!("XsTS8D9tBV6oQRqkixzNCThctMwFzxScetp4BA9ByTK"), // IBKRx - Interactive Brokers xStock
    pubkey!("XsS6AVyq5YpDA5yCUYuwxvkk5hvKvFxAt5tUJik9Y5H"), // IPx - International Paper xStock
    pubkey!("XsfodJMz7SyMNsSgATVDcbuMiQDqQoAiRTL1xuyHNUn"), // INTUx - Intuit xStock
    pubkey!("XsBHq5npA7gUemYgFkzHvpWznPvybEUmyuz3o3Dim11"), // LUNRx - Intuitive Machines xStock
    pubkey!("XsQFmCYjvPGgqYYAth5zNhW8NSCabdamgM1kXMorJs7"), // INVHx - Invitation Homes xStock
    pubkey!("Xstzb2X8HbtJVUephHsF7XknnwiBRYMxWoLyfUBWaoC"), // IQVx - IQVIA xStock
    pubkey!("Xs5jambBqEK8FK2FpxFTLBCQ3uR6q8CBpwCMmQri8TG"), // IRDMx - Iridium Communications xStock
    pubkey!("XssP4Vb5pahDsdKUXy74gD34DzotHCYKGXegd1KHdiA"), // IRMx - Iron Mountain xStock
    pubkey!("XsqD2nwDfGHYNUisQiPWmhQo2xAPLJY51tC2wyxDV4v"), // TLTx - iShares 20+ Year Treasury Bond xStock
    pubkey!("XsFnZawJdLdXfBSEt5Vw29K5vdBiHotdPLjUPafpfHs"), // IEMGx - Core MSCI Emerging Markets xStock
    pubkey!("Xs1neEMHNaDDSHqE9nvDGkmuX73ZASKcLtQqwmVAQde"), // EWQx - iShares MSCI France xStock
    pubkey!("XsaXKP7GaPDA3bLwtuFoZ2oGsaukfeS1v2U1xUeqwrp"), // EWJx - iShares MSCI Japan ETF xStock
    pubkey!("XsM8hCeX916xAiVLCeFwxmVpTNYq14rjM8h8Ha2UiTe"), // EWTx - iShares MSCI Taiwan ETF xStock
    pubkey!("XsbWCMXPzLdZMLpPtiiWE7M99fySmsnK5eFCWGWfDQ8"), // EWUx - iShares MSCI United Kingdom xStock
    pubkey!("XsbELVbLGBkn7xfMfyYuUipKGt1iRUc2B7pYRvFTFu3"), // IWMx - Russell 2000 xStock
    pubkey!("XsEsHeiNZf88sABxu74TPuXbQThdaQGdzPVZUBicjtQ"), // SOXXx - iShares Semiconductor xStock
    pubkey!("XsXoAR52Q2NYFkYiNqhCq4FauvyA1tdRsrmEYNf9fuh"), // ITAx - iShares U.S. Aerospace & Defense ETF xStock
    pubkey!("Xs5CSFf663gjowEA7ANX8ZhPe4AkLWMqLbriVu1JmUV"), // JBHTx - JB Hunt Transport Services xStock
    pubkey!("XsnZQ49Abxr3RJgRSxYd4azhUg4MQVT3amiyaq84oA1"), // JBLx - Jabil xStock
    pubkey!("XswZA2QB7YQBXTHBWoVBGk29pYNSvmqjwSJ7ojnQzJb"), // JACKx - Jack In The Box xStock
    pubkey!("XsEYQJtVa51Ww3y28jV1FwheMgmDj8BpxYiCPThcLWW"), // JAAAx - Janus Henderson AAA CLO xStock
    pubkey!("Xs3vKvZf5Rn9pMVM7ctBh1jb4EEH2fAVVmCBB4XDiEf"), // JAZZx - Jazz Pharmaceuticals xStock
    pubkey!("XstnmTaJXkobEbbs7AgmGLBsBeWyHTSJAFcJ6BF6ycs"), // JDx - JD.com xStock
    pubkey!("XsJjKxPyPx1oyqVoyekbphLCLEwEzszsYbZQXsGPDdE"), // JCIx - Johnson Controls International xStock
    pubkey!("XsDDkXqF1sr1ixW4Y93Geqfd2LbrWxokCjQf9poGN3c"), // KVUEx - Kenvue xStock
    pubkey!("XsdvuTpWkg6TSzpGJrdEwUwtWT7UrUzMxYTLhUZs9s9"), // KDPx - Keurig Dr Pepper xStock
    pubkey!("Xs5GgkWgze4XgU7NC5iXJqvbGufxQBJKH5JRN2MSPWQ"), // KEYx - KeyCorp xStock
    pubkey!("XsXzXSMoSdF3gczdPhECXHN3TwfkUpNkthUvpEJVbDn"), // KEYSx - Keysight Technologies xStock
    pubkey!("XsNnfg6KXzaRsH8PE9T9YEahq3MQpSZ3dQAa4hcL1SN"), // KMBx - Kimberly-Clark xStock
    pubkey!("XsQxeWjN2M3Spr5YHRQiM1oQ8fgpJWtdBdg1E2ZrjFg"), // KMIx - Kinder Morgan xStock
    pubkey!("XsTd1xJRAwAddguKdxYPGoS3qhctGsJb1ue4Rm6URFa"), // KKRx - KKR xStock
    pubkey!("Xsw2uU1i8tHjbgstUbtt3m6kg7BS7AgG5aj8z7ddmmN"), // KLACx - KLA Corporation xStock
    pubkey!("XsUyRqfxY2XtFvP99Lkn2DDWtxNNAY6w5CVDu1F68G1"), // KNXx - Knight-Swift Transportation xStock
    pubkey!("XsssrKt95uWKWRbDdbCeMubD7iiD74NqiXeNL1p3Gaj"), // KTOSx - Kratos Defense & Security Solutions xStock
    pubkey!("XsSXWwzGGZB1LyCFnWrHGfTVn6T7SXPycAXtTaA738o"), // KRMNx - Karman xStock
    pubkey!("XssE4tb3kc3f5V5cRL6dsuffP9wDm915W4bAsyNf62A"), // LHXx - L3Harris Technologies xStock
    pubkey!("Xs9bwo2tkrKRJxr9787rKj2bKcQUMFNtbzruYTrxdst"), // LVSx - Las Vegas Sands xStock
    pubkey!("Xs4Gzh8ZLhujFUZUcF1zaTc2dc9AZCEeWjiLb6XwYHd"), // LASEx - Laser Photonics xStock
    pubkey!("XsjaZAWtPNA1ZWqbPZxaDWtx1h9L23ucmRquwuSHQuf"), // LSCCx - Lattice Semiconductor xStock
    pubkey!("XsHjfVcciZyPmujpSYXKGyQZ3k4yyzxnUXE6CYdZ3AD"), // LENx - Lennar xStock
    pubkey!("Xs4MEQwg4mtEjAbtBzKsmrDccxX64Y4emF2yJptPJ3q"), // LIIx - Lennox International xStock
    pubkey!("Xst9jAZk2vxXtsttJdDh5pKvwfmDUPHAPqt3dxFi64G"), // SPCHx - Leverage Shares 2X Long SPCX Daily ETF xStock
    pubkey!("XsSr8anD1hkvNMu8XQiVcmiaTP7XGvYu7Q58LdmtE8Z"), // LINx - Linde xStock
    pubkey!("XsFU9pEZTmFTwGowKZkVtagbF5ivCJ4WTyPEZrzVFzY"), // LYVx - Live Nation Entertainment xStock
    pubkey!("XsixDLxXcKZBcRBddYiPZjB9FJcKMTLd4B7VBeKo3pB"), // LOWx - Lowe's Companies xStock
    pubkey!("Xsk7doh3Qsz51GJdGFUdXcW4PFSKgfj1KGMy6jjbk6J"), // LPLAx - LPL Financial xStock
    pubkey!("XsexQ9qqNbDkLE6XwCN9ceVhLo8Lxc7UheVR6eBkKyo"), // LITEx - Lumentum Holdings Inc. xStock
    pubkey!("Xs4EhDN7X1ZjuUqUwfQALufYZbnaGoZT5ibv8L8fsHp"), // LYFTx - Lyft xStock
    pubkey!("Xsir2zEBi9NoWA8r1oSdxS3RHzb38JuuAWWt3xqFW8w"), // LYBx - LyondellBasell Industries xStock
    pubkey!("Xs5xCbQ7r3tJrR1huRmNrAKLyDZQM4MadU1rkmrh8hX"), // LYTEx - Roundhill Photonics & Optics ETF xStock
    pubkey!("XsMXtu6uQkfvxCYURytUcQvhFCoqvphtoezVBh42CfZ"), // MTBx - M&T Bank xStock
    pubkey!("XsGkSoEbLmqH2GCixj1MdGYHDbRaJdgnnDPrYhMuLxA"), // MTSIx - MACOM Technology Solutions xStock
    pubkey!("XsguBZPkM9BDmxspmWe29EmrYZBv21ENcC27Pqh7grB"), // MARAx - MARA xStock
    pubkey!("XsyqGLaBLmGCZstuUrvo7iWhJkZgg4KEBtcxVFBsYSg"), // MPCx - Marathon Petroleum xStock
    pubkey!("Xs85zRMSv4KTRqSNjZSFWgEXEyb3esxUBzhTj377DH8"), // MARx - Marriott International xStock
    pubkey!("Xso2JLSTXQjpDXw1r7ZxbC2JCbB6iLGw65kUzQvgTwH"), // MRSHx - Marsh xStock
    pubkey!("Xsvi61uutcLdAsa5NxJ2c9geLgzUFM3tMtpJ44eoegt"), // MLMx - Martin Marietta Materials xStock
    pubkey!("XsNZFvmZv6zEeAxgF5L72Nt6A2K8Uo6u7aRDLm7Smvc"), // MASx - Masco xStock
    pubkey!("XsvBVwk5d7eU6prsSxHJPu2adEm5q6wx55mbhov2UnF"), // MTZx - MasTec xStock
    pubkey!("XscRMWo5xvLxenVLxmodRqZC3UJwACXoxAEEGQzrzKd"), // MKCx - McCormick xStock
    pubkey!("XsnoiYFyBemVokJpx3ddt6mPB3v2HepSevRLXSfj3Y5"), // MCKx - McKesson xStock
    pubkey!("XsxaiepK535fVqavwRvJVTC64KaCLTtCgQT5jpR3rJt"), // MDLNx - Medline xStock
    pubkey!("XsPdH3m4B33ZaRS5saL1XGQnUVYsx1yyEyAxmNfcpCb"), // MEDPx - Medpace xStock
    pubkey!("XsDgw22qRLTv5Uwuzn6T63cW69exG41T6gwQhEK22u2"), // MDTx - Medtronic xStock
    pubkey!("XsnQnU7AdbRZYe2akqqpibDdXjkieGFfSkbkjX1Sd1X"), // MRKx - Merck xStock
    pubkey!("XsGhGG2TNXuJLFL5q7pAomKtMVc2YvVqs4G1hyBZLfw"), // METx - MetLife xStock
    pubkey!("XsXb7oGeK1N5Jk9mm9VD4ZQrXsu9NULWP3C3n7hhDWp"), // MTDx - Mettler-Toledo International xStock
    pubkey!("Xs9GioXYhWaR1Pt7HLiNRgkHynWdPPWcdZWdUCHkZNj"), // MCHPx - Microchip Technology xStock
    pubkey!("XsVDmkYEQ3wwLkXKdkRmic3HroSbReejAcnY9eNmrRW"), // MKSIx - MKS xStock
    pubkey!("XsaiHwUCnDSFoBSZAuuQEWVnyjnzMAWdcbSBFYS2tkX"), // MODx - Modine Manufacturing xStock
    pubkey!("Xst1iLxrQ74L6Ai19KHJWGGmDYWpvjqWW6yQFCiPhnf"), // MOHx - Molina Healthcare xStock
    pubkey!("Xs3hGRD2NSyUUC1fjK5gKe8hXQgLoqxjR5WtYL1NBbg"), // MDLZx - Mondelez International xStock
    pubkey!("Xs12gqhuCeC9gpUdN6uDJQfpyKNf9Y4uK2TKY6c7NrZ"), // MDBx - MongoDB xStock
    pubkey!("Xs9YEeNWvRwWK4wPHPaitx9mFyfU6hbUY539BLJSHCN"), // MPWRx - Monolithic Power Systems xStock
    pubkey!("Xs72K1Ta1D5ccNy3RzSyQWGgZywWvphX78pL8WBk1Bo"), // MOOx - VanEck Agribusiness ETF xStock
    pubkey!("XseTVygt2sUPQsfYbYVMXzV1AeMRE6Usj9DEWwStMe3"), // MCOx - Moody's xStock
    pubkey!("XsUucbjDLYbER4WNNffn1eQ9cDnAcFVfLgSZeLHPDZS"), // MSx - Morgan Stanley xStock
    pubkey!("Xsa1gZ38panjmJwh6cr9QQ1YpRBEzVfxrykWSsiWvJe"), // MSIx - Motorola Solutions xStock
    pubkey!("XsFARXCfXKCdu6BxV2WACBt3Dk88nxLujoqPzyyBib9"), // MPx - MP Materials xStock
    pubkey!("Xs4BPW5YU7RsXMmCt6CyDphWH3YLtyaXDXEzZDmj33G"), // MSCIx - MSCI xStock
    pubkey!("XsxetxdredFJuPSRtp8s7zy4gC94a7Z1FDdtUbNJYQa"), // MUSAx - Murphy USA xStock
    pubkey!("XsLSwcujLXXgCPZWPZKRsksbZrWupPMNKmg6A71g2Ba"), // NDAQx - Nasdaq Inc. xStock
    pubkey!("Xsw75QJKjhyvcSFDnqrtXn7amNhCVgeoaoo3rp3sDak"), // NTRAx - Natera xStock
    pubkey!("XsjLc87ewHkMhsxhNRLy4TZFG64ycoZb23hLc2WW7iZ"), // NTAPx - NetApp xStock
    pubkey!("Xss5mnWWwgwVc6d7dibSqcb43fbmjXAnuRaJ6ZF2iXQ"), // NTESx - NetEase xStock
    pubkey!("XsCNTvYMAHYPuHc5r9WWhnYfvAZMLLsguv2NBLs8wrx"), // NBIXx - Neurocrine Biosciences xStock
    pubkey!("XssuE11NLek5vAeHNS2Jm8PFvT4PE7dtnpZwasWTGDT"), // NEMx - Newmont xStock
    pubkey!("XsMHaqEVV3ZekoRp1c3vARffHjJZCqk8FsciXHNghpj"), // NXTx - Nextpower xStock
    pubkey!("XsC3CiuRAFKN99m2ye6q7xrYMx4NuBdr1JR2gVWgFLD"), // NIx - NiSource xStock
    pubkey!("Xs4UsCDQKKJqHjidgoZBHUW9emLD4njpLPsYesp4ZpZ"), // NSCx - Norfolk Southern xStock
    pubkey!("XsRx9hY5BQWniDRahLNtV4uyyvQcnaRgrqw5ops842R"), // NTRSx - Northern Trust xStock
    pubkey!("Xs1eyZcwrzzYW3hDjfpm7PiJrC9RyFszsHpqfTeUTCE"), // NOCx - Northrop Grumman xStock
    pubkey!("XsWSgzrjyVsQJZKTv82uCcvVPxHWTzbxo8C4u2uRTzm"), // NCLHx - Norwegian Cruise Line xStock
    pubkey!("XsZ9qtReiaokaUfW3bEUx2AU4Zmcjk9Y95nEtFMqEXn"), // NRGx - NRG Energy xStock
    pubkey!("Xstn27LaCPFZashxd1eEn6NV4fVfuye1AM3MMsbuB97"), // NUx - Nu xStock
    pubkey!("XsvdeDvbEfZkvfEUixS9SydEfJopFFD6NJjXZN1haYr"), // NUEx - Nucor xStock
    pubkey!("XsrwdnwLHjVnyy7fgE2Pdwjp3czEm73jPjkSC5xzUBw"), // SMRx - NuScale Power Corporation xStock
    pubkey!("XsLsExw7d7cjbN65GwQzS7HaDxCtyE3rjywcNXqEtxL"), // NTNXx - Nutanix xStock
    pubkey!("XsVYCx1KtJEVZaHdQtQuPRRr6E7HTr9yfNbbuNaaBze"), // NVTx - nVent Electric xStock
    pubkey!("XsZCRtj2wRfksr19c4Bf4L2WBB5EYPmr1nBQS1yXApD"), // ORLYx - O'Reilly Automotive xStock
    pubkey!("XsRNiTxDeRvxTEMTV5BC98N7cTZXxjNdLnudR8YeB2b"), // OXYx - Occidental Petroleum xStock
    pubkey!("XsJb1p4Ks6VFggq68JyMz7S3kdRfkqh4wA6EMyxi4DD"), // OKLOx - Oklo xStock
    pubkey!("XsHXJXdfTGnrfygPVvnzb7xdsUiunEQZew7teAyvosL"), // OKTAx - Okta xStock
    pubkey!("XsuEQKUzuiMTMwFTm6MHBeTvknFU585gwu9u4CNKjQD"), // ODFLx - Old Dominion Freight Line xStock
    pubkey!("XscmXyJYjYbGNhxQsPjjRYsEwscg6uPFJzR6Af7Qi18"), // OMCx - Omnicom xStock
    pubkey!("XsQN2eDuuwQk3tuELtTeZ69jR4hpCUDPcjrK6RLACC3"), // ONONx - On xStock
    pubkey!("XsTzZxvfNdVESw8oEn7cNPnfQ4SGk7ncBSezfU6dxa7"), // ONx - ON Semiconductor xStock
    pubkey!("Xsgc9YTqdAH1GzDLBGQcScCMgkKsVBn4hBmqtNRospY"), // ONDSx - Ondas xStock
    pubkey!("XspDMNC416uCcUbngqyAwzk1phuRYdmm6AMsWo3EHd4"), // OKEx - ONEOK xStock
    pubkey!("Xsu33VJj7s2tMUZfAQ8bbGgpDEnfXdCsKHSC4p6C5tS"), // ONTOx - Onto Innovation xStock
    pubkey!("XsGtpmjhmC8kyjVSWL4VicGu36ceq9u55PTgF8bhGv6"), // OPENx - OPEN xStock
    pubkey!("Xsj1NnCroe64taUVsrxwWSzd1VgPnLtTD1K7W8MLXmd"), // OSCRx - Oscar Health xStock
    pubkey!("XszGT2Jo6bZz2SZBmhPstvo2xSs37q67nUJS7gtAfvx"), // OTISx - Otis Worldwide xStock
    pubkey!("XsUsHYstgit1UE4anT2thfJvTByEHmrgx4V1FS8EQu8"), // OUSTx - Ouster xStock
    pubkey!("Xs1b4xMzH16fHZuFUoXxni8THVafJ9t5dFL6kPUyG4k"), // OVVx - Ovintiv xStock
    pubkey!("Xs1rnpzumX77Zz3nVrRGbDYzTM4PM7vgvS4BLTG6ckQ"), // PCARx - PACCAR xStock
    pubkey!("Xstq2wirnUgqHdMvvWafGVbdtg9y845zxAy7iVixkRU"), // PKGx - Packaging Corp of America xStock
    pubkey!("XsQ6NfzzLH8nspjrB9X8R2K64Zz7Tnxqu12juDiMPMW"), // PANWx - Palo Alto Networks xStock
    pubkey!("XskVc7XiWmSqvzbjhA2LqGHQhvBdBNfatfeTpNru6KP"), // PHx - Parker Hannifin xStock
    pubkey!("Xsuo3Vd8T7MVAx3miKJ1beX4sGutzdL3GJsjk1yFbxA"), // PAYXx - Paychex xStock
    pubkey!("XsnHPnzymnDc3D7DWYD84cPt8cVSSAJ4XNoLc9kfJET"), // PAYCx - Paycom Software xStock
    pubkey!("XshWQWYVp5ff8CrAEsGmLVKD47nBWi3Ygn5v8wXK27G"), // PYPLx - PayPal xStock
    pubkey!("XsSdwPGhuiTBir2bpJTiJkEhfMEn5bG75XevuePua1q"), // PDDx - PDD xStock
    pubkey!("XsNKpPnrpzt3MRF7dA9EtAxWyrdivwN5xyvHUi2iweV"), // PENGx - Penguin Solutions xStock
    pubkey!("XsXh1W6QRHJBYdzVrngAxn4t4JQnJBYk7iM2S1hwqdT"), // PRx - Permian Resources xStock
    pubkey!("XsPq3VY73br4wpgi2a4b3jAdvuhstpknXVJ2fA4vNmk"), // PCGx - PG&E xStock
    pubkey!("Xsba6tUnSjDae2VcopDB6FGGDaxRrewFCDa5hKn5vT3"), // PMx - Philip Morris xStock
    pubkey!("XsYvZVxiNdjuHQQzdCmf4TMSTBgvarV2gLi2akooV8V"), // PSXx - Phillips 66 xStock
    pubkey!("XswgfhwvyLQ6ZdNsrwBGkBVpqXzQPpDykCDBmiti1o5"), // PINSx - Pinterest xStock
    pubkey!("XsMJtFbb8BwzQtck3oRXyAfs7SAPRuTXnSEDNd7BAVz"), // PLx - Planet Labs xStock
    pubkey!("XszWcNSHsYTXqxDbgK5GzNPxNfxSAKJBCkr2YtsjuPT"), // POETx - POET Technologies xStock
    pubkey!("XsqmkcmiAXxUmRf6bckJJtnVSynPYgjWFzUo8agEoq8"), // POWLx - Powell Industries xStock
    pubkey!("XsoCb51izF2MpU5XghroQDRxtJ2vtcMY9v5Mk1pqaBr"), // PPGx - PPG Industries xStock
    pubkey!("Xs12SMK4MA7sLVS8eTged2pXns736u3qe2a1a9io6he"), // PPLx - PPL xStock
    pubkey!("XsmWUJ5i951LNPSBDZkXzrZd7icvQQ2nnrhJGQ1fgKi"), // PRIMx - Primoris Services xStock
    pubkey!("XsnXCL5FtHLHH1jW5Nknd5BjuazxnnkCAGR6fWPwPJL"), // PLDx - Prologis xStock
    pubkey!("XsPhDoJMMgARN4eB8apaPNFUjVBKqx3WtRS4ddBwKVw"), // PRUx - Prudential Financial xStock
    pubkey!("Xs25CGQuYuoXrMMWUSxo9WmvnQZyFzBChaHbUPXzm6b"), // PTCx - PTC xStock
    pubkey!("XsUytoK53kCKfXC61Dn3ZM8sjyVBUpScF111pfMG69U"), // PEGx - Public Service Enterprise xStock
    pubkey!("XsBLm8wotoJMHjonHcr1btz9RwLa3EWjhtbs3DbQphg"), // PSAx - Public Storage xStock
    pubkey!("XsNcipuq5a8j8A79VDZZpPWuTpSj62Mm1NuPWzUJkhg"), // PHMx - PulteGroup xStock
    pubkey!("XsyQipBmDKeSy78zCSMp4BY91Cd46Dx3q4eBoSqbBDX"), // PCTx - PureCycle Technologies xStock
    pubkey!("XscLQvx6mzoarzvLF6CntvUPFa6ukUkFCvqzLxiihmw"), // PURRx - Hyperliquid Strategies xStock
    pubkey!("XsqUtYxpP7o3b2yupu7WZ2bi9u68W7VHQTFNR36mz96"), // Qx - Qnity Electronics xStock
    pubkey!("XsUUG8bjFN2KvzLTpzavvEKdAjMAeLTZiTeAQJ9uhvB"), // QCOMx - Qualcomm xStock
    pubkey!("XsGRoyBwemr8giBu8mgShoRGXHYbip7yWHDsaAueMvA"), // QNTx - Quantinuum xStock
    pubkey!("XsFyCPFNa7os3PvzuYQJcpws9WQS6bci2W8zqesHwPN"), // DGXx - Quest Diagnostics xStock
    pubkey!("XsuE5LvQcc9P67iWVqsysQ84DBDRWssbiKnwY8JtwpL"), // QUREx - uniQure xStock
    pubkey!("Xs3SstoaYf2Atbkfs8xvcoksUEaBT3q1RB2AbgPBdyH"), // QXOx - QXO xStock
    pubkey!("Xsm17DtqbHJnVmoEyDQwVmYRatZh4DjCE9csa766REy"), // RXTx - Rackspace Technology xStock
    pubkey!("XstJ1deKwoZVAzj1oF14s7g8ZPLumJKyiQXysym5duM"), // RLx - Ralph Lauren xStock
    pubkey!("Xs3VjsfwrH9mnKLo1z3TJqwtPjAEsHYBNsuzuZj76i5"), // RMBSx - Rambus xStock
    pubkey!("XsgQwodongKCYRux621fuQzxQRv3fyXQKyPzc2CJ7Ys"), // RJFx - Raymond James Financial xStock
    pubkey!("XsyMphR2LaL2YBiorsZYFhdN96xKTSi64L3pmiAHLc3"), // Ox - Realty Income xStock
    pubkey!("Xs6yCfUGbkjcvivWr4crLR7pJGxEXspsuuUMhxNEEUw"), // RCATx - Red Cat xStock
    pubkey!("Xs8LesEYZb7SZJrW85hMrrVzyeoyxKrCNVnpEURLzZp"), // RDWx - Redwire xStock
    pubkey!("XszxuQCqE72vU4ABTFDFwhs28AeSUHqzbWWVWDkhfYZ"), // RRXx - Regal Rexnord xStock
    pubkey!("XsWkTtvUXJgrvxvgGuHRRqk6iwDuviEQZw4uJFUUGgX"), // REGNx - Regeneron Pharmaceuticals xStock
    pubkey!("Xsk2PGWHCQweEwYWNqA72KbuUdNz518MBzHSaBDLowq"), // RFx - Regions Financial xStock
    pubkey!("XsUkTUDE9hCG9GLEpw3STiGuf58hYmpymiF3z1nPh3A"), // RSGx - Republic Services xStock
    pubkey!("XsjsQvjH8BEXCjrgNiVhmhFiYPcNRXGj262fDupfV1A"), // RMDx - ResMed xStock
    pubkey!("XsG3eLvLbHwYQP3r7tSznE6bEvcA2Ko1TpYwbyD6RQk"), // QSRx - Restaurant Brands International xStock
    pubkey!("XsCRJ3YusYCZ8ihZbmdao46y3ddSibxPK29UsitD9J9"), // RVMDx - Revolution Medicines xStock
    pubkey!("Xs82teACSWxcvGDbVJhREu5Q7cbKZpD3hFtkP8u623L"), // RHx - RH xStock
    pubkey!("XsmcgXdcTCXpPJwXPSs67ELi7iRkBzDZF1qcgEuUvTK"), // RKTx - Rocket Companies xStock
    pubkey!("XsnR8Rtrd8yFDGP3g2HwLdGiw58UcXjggXzgkL3pNYz"), // ROKx - Rockwell Automation xStock
    pubkey!("XsR5eHL5sXTwUckUcenkZyBdAkShFBcNDiYeecBMaCu"), // ROIVx - Roivant Sciences xStock
    pubkey!("Xss4k5HDz8CwyNnbZixMs3XgaXvnMoWtpxeqzFK6wvA"), // ROKUx - Roku xStock
    pubkey!("XsCbNmXqgaDVDTpXaATSrki9ATKwu1wzfhcYwPriMns"), // ROPx - Roper Technologies xStock
    pubkey!("XsPYJyoTnjc471VCHEERVBVrJ7xuQhDYWddPw6koNVV"), // ROSTx - Ross Stores xStock
    pubkey!("XsHDarKydU1cD6MWMGUgN1cSbWPNUoZHM3GE6YaC7A7"), // RCLx - Royal Caribbean xStock
    pubkey!("XsgiYbKzzyYzuxRj1R4E6rvfVccASCGepmWEMpJdFG2"), // RGLDx - Royal Gold xStock
    pubkey!("XsHkjFmgE1UtuNJXffYmTGMaF1rDU14BYBJbEGR4DVc"), // RPRXx - Royalty Pharma xStock
    pubkey!("XsN53tgjZA4QU8iuedPNHxxCbDqKcJqbrT1f8jBXzdC"), // RTXx - RTX xStock
    pubkey!("XsJRuk3Mn8wSkc3aTkG2oqxFJiZqC3YU3gmNZSFbLvb"), // RBRKx - Rubrik xStock
    pubkey!("XszWSGNpLFc6mvEipzspjBx9CJjvAHGgoRgbmNMce9c"), // RXRXx - Recursion Pharmaceuticals xStock
    pubkey!("XsTuW2zaiPbzgrnQ7b4Z99zeULMM7gPxppE61CtpyDj"), // SPGIx - S&P Global xStock
    pubkey!("XsdzBn1wxCibSdDZgaaWqGtdvo2JSFueo2CwHnA69Lf"), // SAIAx - Saia xStock
    pubkey!("XscePZ4mjiXqyEVv9ZZPGbT6L86LqnYPEBE4ViKSt3T"), // SAILx - SailPoint xStock
    pubkey!("XsczbcQ3zfcgAEt9qHQES8pxKAVG5rujPSHQEXi4kaN"), // CRMx - Salesforce xStock
    pubkey!("XsriZ58QT8GfKLep8Vof8oMGq3P9qPhowG8NfyLrLzQ"), // IOTx - Samsara xStock
    pubkey!("Xsk7NHyfTja9tK9bYfJxEtfPhVrvMiweB36rf9xK25F"), // SANMx - Sanmina xStock
    pubkey!("Xs3BuKwwLLSgKCKPBeSN5Wje5gP8McsHrt6pnqWzjhs"), // SBACx - SBA Communications xStock
    pubkey!("XsWAnFM77x6YvpdaZoos79R12o4Yj4r7EVkaTWddzhU"), // SCHFx - Schwab International Equity xStock
    pubkey!("XsDa2gxAwzix7mppKAG2KmnG6wwaxxJaR8iM4Q6Qy38"), // SECZx - Securitize xStock
    pubkey!("XsbicvsCUHE9ja8gTsjzruDxQTKeyij94i3phx3NuNf"), // SREx - Sempra xStock
    pubkey!("Xs9bdUnLdSWjQWeEsy8cde6t2L7d3adDRKcH8H51Uwr"), // SMTCx - Semtech xStock
    pubkey!("XsBnEFd2EtpwBRNVfN8qLZcVUnuqXRaGCSEEFChLfxL"), // NOWx - ServiceNow xStock
    pubkey!("Xs6Bdc9N4LM1ksoDuQigXwr8kc4Tt6fAJU2LZ71yFbK"), // SNx - SharkNinja xStock
    pubkey!("XsC6bBzDDABjHhKmUAxx7THK4jF5zbJgPA3kVox5okM"), // SIMOx - Silicon Motion Technology xStock
    pubkey!("XsUXFEjcDYrp7NuzHtC88JLyirmnLpA8nde1e8eRxEn"), // SPGx - Simon Property xStock
    pubkey!("XsRuL9oMSNpHehUVP1DQCphXVeQ5gRr1oiRo2cRKbz7"), // SITMx - SiTime xStock
    pubkey!("Xs6nGsox15dEqC77pSRsBUHhSoTuG4M9o5rK92GyHhM"), // SWKSx - Skyworks Solutions xStock
    pubkey!("Xssse6ey1U1D1NNbJHvep6MpvL16D4dqigsguvLGQqF"), // SLBx - SLB xStock
    pubkey!("Xs2p9MnR8wGJHUwtFu86KBHCzDcZJHufCCALZQfU7ty"), // SWx - Smurfit WestRock xStock
    pubkey!("Xs3QCTNpPWTFYRQ9yPv9gsvWKNJGk45UQ1bb1w7xznc"), // SNOWx - Snowflake xStock
    pubkey!("Xsipo31rLh5EqPMR2cArn6kVPAD83C6rxbmCrT9Wu5u"), // SOFIx - SoFi Technologies xStock
    pubkey!("XszuTk3Rr5HXpV7Nrfpg7KAZJfDccPAYLKBsRbLYRx4"), // SOLSx - Solstice Advanced Materials xStock
    pubkey!("XsU3RZd5ufTcCJneLjDhTrU47ma9ypnhXyvJACRZrTv"), // SGIx - Somnigroup International xStock
    pubkey!("XsEKTjrrYD3Yni2CEPZUn1H4wKsSXpZc8hBQLFy54TR"), // SONYx - Sony xStock
    pubkey!("XswM1GWRLNrUQWCXrcJDpwzpTFnzgB5U1NNpLGzTm1s"), // SOUNx - SoundHound AI xStock
    pubkey!("Xss3SfGpiR5nhJYsJ7gdd1qiNGxcsr5NX78rmGBDtND"), // SPOTx - Spotify Technology xStock
    pubkey!("Xst3tzrmqjzvGXZz2eE2h8CWWWxy4wzp6mXvTuG5Tqt"), // URNMx - Sprott Uranium Miners ETF xStock
    pubkey!("Xs4UYArNzo632wX8KUChpmS2PLU9fHskMyEpR8KC4Wp"), // SFMx - Sprouts Farmers Market xStock
    pubkey!("Xsq4wQVyggsJXzrjQK51M8bxK3duFaRegcX5Vxdbfcx"), // SSNCx - SS&C Technologies xStock
    pubkey!("Xs9gd8SGbYQn9kkUYQayn46BdqQbvvUshEF6ZpRAzM7"), // SBUXx - Starbucks xStock
    pubkey!("Xs6QWdM8Lgdf3gUfmhZfZDu8DKcoMyxjgyU46R28RUR"), // STTx - State Street xStock
    pubkey!("Xs54CrhmpVp6uxZXwgSTegrRH2kShh88XFPzgf4BExu"), // XLEx - Energy Select Sector SPDR Fund xStock
    pubkey!("XsNG7S8TMR3Xqa86GHjgtM8gj6Jkt8f62EvtG29B5yc"), // XBIx - State Street SPDR S&P Biotech ETF xStock
    pubkey!("XsAk6BoV4kBXUM6WXodKyM21CN92G9jArwAzFvbh3LX"), // XOPx - SPDR S&P Oil & Gas Exploration & Production ETF xStock
    pubkey!("Xsw7mKDicsyZ7dgSMfFxhX5nbgat3uG9V1nCxDy6Gvp"), // STLDx - Steel Dynamics xStock
    pubkey!("Xs2HTHR5KBVGz9HS1iMakbTs1zXZVRuaLDMApPbabwW"), // STEx - Steris xStock
    pubkey!("XsHLmetWDLLqs613YVRnkUsEZP4V1FSA3gTt33Dc66Y"), // STRLx - Sterling Infrastructure xStock
    pubkey!("XsAxB3xLnt7xto5ChiSbVNHBoXcXzKuzS7FieiNYwtY"), // SATAx - Strive, Inc. Series A Preferred xStock
    pubkey!("XshACd866qcdJYTotipkEHNKKnjDHJJkUciUxKVenE3"), // SYKx - Stryker xStock
    pubkey!("Xs1WLANCNaqzUa3Rtwh9nMA9Z1DKu8NhYokm9Av4wrR"), // SUNBx - Sunbelt Rentals xStock
    pubkey!("XsWMBE3QzwuS7pJET8GUyaGzjNJG49RToPBYWdGHUTG"), // SYFx - Synchrony Financial xStock
    pubkey!("XsfGN1hFHbigfAzUY1nLRjrECm3ohGzVcim5pMqgp9d"), // SNPSx - Synopsys xStock
    pubkey!("XszobR4az4oYsssmwsFov326662q84g7UPXzRgrGvfS"), // SYYx - Sysco xStock
    pubkey!("XswCi2U1G6Ppbw1QhG45yKb8UKuR1FKLJrquv2FZSD4"), // TMUSx - T-Mobile xStock
    pubkey!("XsiSfpU85Ee2YzXrprD6eNpdp8oo4eQMd7zPbqbAXSS"), // MSTUx - T-REX 2X Long MSTR Daily Target xStock
    pubkey!("Xs8urJdzBSFLaKqxZiMackbXDkyZbwPeAwz7PepQ1eD"), // TROWx - T. Rowe Price xStock
    pubkey!("Xs8yLZVjsYmQ5Tu4e5JDVQeo4cbRRDCVERgtSGJFn3q"), // TLNx - Talen Energy xStock
    pubkey!("XsMD39vkTVERi6vDmdr6VatnA8sruyVY6YdrfFz3p1c"), // TNGXx - Tango Therapeutics xStock
    pubkey!("Xs2zWfkYjSCCW4fdC1L36rz77YzTcQtji7ewjWPvuWW"), // TPRx - Tapestry xStock
    pubkey!("XsmhtZrbnheSvztwRW82ZqZqMjaRnAQqyWERFh1A4Kr"), // TRGPx - Targa Resources xStock
    pubkey!("XsjXR5jf4d4GNpJXwLvEBTMwK3ZYro2qZGgvUCzyJrx"), // TGTx - Target xStock
    pubkey!("Xsh64eWgeLab2YtKPRy15Zj7tXrUncjVwszGW9F5tWG"), // SNXx - TD Synnex xStock
    pubkey!("XsEn37JjZxj3m9YayAyCCJFaQKBPZc6aUEiLEus5igu"), // TEx - T1 Energy xStock
    pubkey!("XsoPv7MccAc4Zf1irRh3NLCSP59XMTaudu8ucRQR7S8"), // FTIx - TechnipFMC xStock
    pubkey!("XsBKXQehs7BBCZ8YbJ8X5B37BE1ymf9KyMJwbj6Jpoy"), // TDYx - Teledyne Technologies xStock
    pubkey!("XsBgbNRDujfznQq8P4NLKEkRTbPxUVt4v1xwkDzMMtQ"), // TEMx - Tempus AI xStock
    pubkey!("XsBYWM8wgvSJfvNybgc6qxGxkD584FSycCWRiCvwZHk"), // THCx - Tenet Healthcare xStock
    pubkey!("Xsbe4fwmjVQEWEPkzfyxqNdPUUK7X9dKfTJrZdDbNgx"), // TERx - Teradyne, Inc. xStock
    pubkey!("XsU4fHhJEFcrwNh4RYuHmWdaoCfsrWKEm4UEs3zTJwH"), // TXNx - Texas Instruments xStock
    pubkey!("XsNpiekiQLb87S2VqmaYjez6sNSrjoK78nCPAF8uaCF"), // TPLx - Texas Pacific Land xStock
    pubkey!("XsWFY8Ems96uFPnZJ1NwXsqeH6bczLQDDvvvab14crQ"), // TXRHx - Texas Roadhouse xStock
    pubkey!("XsEbV5ZDQvZP2Vifccymr8iaULJkJVZCPuQxnEFxvTh"), // AESx - The AES xStock
    pubkey!("Xs1kbMkmahb44LPLLdjscSYZ4c2HUu4WNfgk8P7ndQf"), // ALLx - Allstate xStock
    pubkey!("Xs2hXX5B6aYUn1Nf8RkspKQbBTjbsa1Co9SjpwgZbNr"), // BNYx - Bank of New York Mellon xStock
    pubkey!("XsCd1pXsmVgiacWR4oVuEghamgHPisFZmA5RdNs2NLM"), // CPBx - The Campbell's xStock
    pubkey!("XsK5U9qwT9ErPXnpgDzV9voTGASZbZnNTYTGVxCqPii"), // SCHWx - Charles Schwab xStock
    pubkey!("XsGfYyioN9bfk6M6RGKvucDq8grQehqkBz8ziEtT124"), // CIx - The Cigna xStock
    pubkey!("XshmkFbMrRMtVb2HoJw1nc9jGMKNjTWcutT86YpMbKi"), // CLXx - Clorox xStock
    pubkey!("XsTisPuTVM8VVHagEyeGR7CBt6TUD93AEhptETHWMsd"), // ELx - The Estee Lauder Companies xStock
    pubkey!("XsTqdbD9ghH3oAqqGQEkRXhNt2F9CGbnWE1ohiQf79Q"), // GAPx - Gap xStock
    pubkey!("XsgaUyp4jd1fNBCxgtTKkW64xnnhQcvgaxzsbAq5ZD1"), // GSx - Goldman Sachs xStock
    pubkey!("XsLw8Pw975zbR4QPQkAorVYM6qQq21jSztyDoskVz5x"), // HIGx - The Hartford Insurance xStock
    pubkey!("XscEuBenD5X7CMVdh6MKzTSBQLsKNqmsoLiXHrQMitH"), // HSYx - The Hershey xStock
    pubkey!("XszjVtyhowGjSC5odCqBpW1CtXXwXjYokymrk7fGKD3"), // HDx - Home Depot xStock
    pubkey!("XsqtzaHCKV2zsgRYitGJtcecfmBkdJsdrpSXZgQryHE"), // SJMx - J.M. Smucker xStock
    pubkey!("XszMPpjvc5nyynUGktkn7JkqkfFYxunE29Rxi4Vo8C5"), // KHCx - The Kraft Heinz xStock
    pubkey!("XsHyXCSuYHca6SxpSLb2taKiWkoJA48pVcnqyFguHf5"), // KRx - The Kroger xStock
    pubkey!("XsPti6oM1EuvpHdSxab8hTbiAtdye8ic7Zxobua3cdo"), // MOSx - The Mosaic xStock
    pubkey!("Xss1A59GFZcvyuTXLXMyYdJ7e9kpHq6AFKNjcVZ1rjA"), // PNCx - PNC Financial Services xStock
    pubkey!("XsGDHfnSNxofQ2qXzcty9r4HB5HFszUCrQhjN7mbEcw"), // PGRx - Progressive xStock
    pubkey!("XsfgitU15HiJX9UJN2fhcpbfRr5P8XJXvC1nuJs3jAr"), // SHWx - Sherwin-Williams xStock
    pubkey!("Xs5nNLYnMuwBTMQ7KnfCgfsDBa9WehDa1EZqzicarZm"), // SOx - Southern xStock
    pubkey!("XsPWZN3FYcrvpEnxrQqogbVZtxpRX9tFomymKw2dWJS"), // TJXx - TJX Companies xStock
    pubkey!("XsMheFQzTi63hVQDGnGeyrspB5Fn3mjJyXygaJCkMo9"), // TTDx - The Trade Desk xStock
    pubkey!("Xsxd1sqLaPsmmmYTZHr2GnNufXXiH5rZcahP99TyeDr"), // TRVx - The Travelers Companies xStock
    pubkey!("Xsg93jDV656ULQ5u9yT2x5DS9b4xGD8aDCtfESSW6Bb"), // DISx - The Walt Disney xStock
    pubkey!("Xso4umc25VxszLT64SYNT7TMeZudAC75xwNo7qFjJFu"), // WMBx - Williams Companies xStock
    pubkey!("XsXWXoWAzEo22ggADuXQeauMWxwgMS3em44tRBX2Vsx"), // TOSTx - Toast xStock
    pubkey!("XsCMQxyd94mNDqb9uXoxLh6hNFFcBygdBYerEKp2LZy"), // TSEMx - Tower Semiconductor xStock
    pubkey!("XsYMTZRVKs9he9redAPMhaBbCbmZvUxMTApsfCZsKzt"), // TSCOx - Tractor Supply xStock
    pubkey!("Xs3EFHHyP27EsAw3ftS8UFsN8kwkhFvbppji6bbkkPn"), // SNXXx - SNDK 2x Long ETF xStock
    pubkey!("XszyRH55GaDt8vYFw8p7KaLT8pedveg4B1QWcxy6VBV"), // TTx - Trane Technologies xStock
    pubkey!("XsaY4eSxkNswy2kg8j7CWS7v154y5kkxYRiqX2ffpVZ"), // TDGx - TransDigm xStock
    pubkey!("XspgPjzWCN6w4b8xHyfN8NRXrgdNMYx5nsWgiGGeK4R"), // TFCx - Truist Financial xStock
    pubkey!("Xs1PSc6DL1cdDPTbyJevgYYU7uA1wiJQESzrpWM7JRv"), // TTMIx - TTM Technologies xStock
    pubkey!("XsQVYwNXBY7UNaLonk5qTbr3RzFC87CECGXWWfJomt2"), // TWLOx - Twilio xStock
    pubkey!("XsMAsw5BADYr3m19HhYeXrG1qAieeP75DT8tKn42DFA"), // TWSTx - Twist Bioscience xStock
    pubkey!("XsiSQ5MUFoAwjrGB1x3FsXTthfwJnJgHVXj9tDKuvr8"), // TYLx - Tyler Technologies xStock
    pubkey!("XsWV8voFNehNEbuBW8cEAMWwjicXCYskUT4616PFZ6a"), // TSNx - Tyson Foods xStock
    pubkey!("Xsieiissncd7uzAW2kwNJPDKBfWcyvdibgxSMQindKF"), // USBx - U.S. Bancorp xStock
    pubkey!("XsLzbFA47Tm3Bs7MnSJrGbi6RnfbtCGA6pD3NHSzp6P"), // UIx - Ubiquiti xStock
    pubkey!("Xs2Gj37vGmjQKW82m7GiuHmZwcY21xyFNCdCqC9n8pQ"), // PATHx - UiPath xStock
    pubkey!("Xse5GAs99zAj5bzMAbwjooLn4AHn1Ny6nUcN6vt5vrR"), // ULTAx - Ulta Beauty xStock
    pubkey!("Xsehrw5hHiQ16Vfxxctj8ewUhtRCzbCrBYS278ctAAy"), // UNPx - Union Pacific xStock
    pubkey!("XsKq6Ac27X857u18jH8MrWQ3bEA5ZfUSGriawpyu8xw"), // UALx - United Airlines xStock
    pubkey!("XsLR9avpksMgLqB5Mw3jBNjzdvSDUKGw4onApxDGE6x"), // URIx - United Rentals xStock
    pubkey!("XsrazTRfp2iGrUcZkkoB8PpM9qApe77RbohmknHh9XJ"), // UTHRx - United Therapeutics xStock
    pubkey!("XsZygGhZ5GWq1TMrbBAibrH5Par33cKxrtZAuTQ9r9o"), // Ux - Unity Software xStock
    pubkey!("Xspjr5y1bLtPTeSg13NBzxcE3JKZjYQrdpHKxAGsQYs"), // USFDx - US Foods xStock
    pubkey!("XsJJneENiaBPqqcdK1gMsfwi9cbw111azigzRYHoctX"), // USARx - USA Rare Earth Inc. xStock
    pubkey!("XsmMjZH2ex52ecoCqBN1sMADoWQziXpe9j6z6QVmafF"), // VLOx - Valero Energy xStock
    pubkey!("XsVRhRg9eRE9PrsoPsAt5Mifa8mfjm9R3vdw6orp54j"), // GDXx - VanEck Gold Miners xStock
    pubkey!("XstuBvLo7soZzj3beCCPonHpR3eUfPNSeQzw35Swons"), // SMHx - VanEck Semiconductor ETF xStock
    pubkey!("XsTSfxoQry8okZoGMN5t6mExibDq3gA2NDLJeLztX7E"), // NLRx - VanEck Uranium and Nuclear xStock
    pubkey!("XsNVBwVGqtDqmA2Waoiux5mfykH8nepLK74z3ZoQWK2"), // VUGx - Vanguard Growth ETF xStock
    pubkey!("Xsd7TduTbjuYCFL7Uoujb8SbkZLmUsuYNLn7KdvX21x"), // VOOx - Vanguard S&P 500 xStock
    pubkey!("XsEdDDTcVGJU6nvdRdVnj53eKTrsCkvtrVfXGmUK68V"), // VTx - Vanguard Total World xStock
    pubkey!("XspuRH5i611hbZjGtYsiaLmhnGya23UU4jnwADWr9yV"), // VEEVx - Veeva Systems xStock
    pubkey!("XsGgFuo6ViEe9GT4QZJCkoGtu9skbti7HiDd4LBSzk2"), // VELOx - Velo3D xStock
    pubkey!("XsEvH8WR4t5FU5GT1cfsmPnvrmEknd4UXA68uJ1F25L"), // VTRx - Ventas xStock
    pubkey!("Xs6mp36rvKSo7vTM588J1HUZF1sbNjK3KSz12fNM7P5"), // VLTOx - Veralto xStock
    pubkey!("Xsy4NX4cCf4YjN6iwypnQ8crXtNkyA2K3jypAX5NfQ6"), // VRSNx - VeriSign xStock
    pubkey!("XsBDavbmVc22RNAgsHwUDEeC8f8VVCKLCcxFbfFUEzW"), // VRSKx - Verisk Analytics xStock
    pubkey!("Xs4VKGG8TTan8UN3mC1ufsDMamBtBLE4Ahfkyv2tfuy"), // VZx - Verizon Communications xStock
    pubkey!("XsgCWpLUC3pv6JmNAFtkYmuwSaikShy8QZRuripgBzn"), // VRTXx - Vertex Pharmaceuticals xStock
    pubkey!("Xs9uh84WXf2F4P4UVRLvNGwv753ArA69kUk6r4GKVoA"), // VTRSx - Viatris xStock
    pubkey!("Xsy8kJEvQN3B29UhEo1LXQdYCTo7egp5cayp1cnBWmf"), // VIAVx - Viavi Solutions xStock
    pubkey!("XsN9HmM3TqbY317obKz1fYbRvR81MT9qbL5eUSiGto3"), // VICIx - VICI Properties xStock
    pubkey!("XsrdBxdbyFTuZzQLs6isMzWAFiLVHdAzBA3mx5zVoyN"), // VICRx - Vicor xStock
    pubkey!("XsEWryPA68Tddfrw31uSv18aysuZ6gb4vszk856hqQ6"), // VIKx - Viking xStock
    pubkey!("Xs936JPaWKEXmpu54DZ149GETV7PmsnTqnMWxgcPNAj"), // VSTx - Vistra xStock
    pubkey!("XsAtnoftgxL6kJErDeCL8m2e364mAYgEbSfSKxi4ctJ"), // VMCx - Vulcan Materials xStock
    pubkey!("XsSuZjygy7yrbyZJNzBj78ysrKDFMv11HrPPwBUE2Dn"), // GWWx - W.W. Grainger xStock
    pubkey!("XscZ13Vr2dH9GV8vxVAgFsXz2YMoDCAz3QanuCcwsr5"), // WMx - Waste Management xStock
    pubkey!("XsGmxpiwpg9oBuRoyczLkXs7j82v9Rb4BTHwTkvXnNA"), // WATx - Waters xStock
    pubkey!("XsAdZrnayY88RC9cMEEaYnnKyBCALCwz6a4mmPTGUMT"), // Wx - Wayfair xStock
    pubkey!("Xsxq7WqXpiNFrdWSd6CA27LXmCsCrGyC9C3uaebmaqU"), // WECx - WEC Energy xStock
    pubkey!("XspHXejQ3A4VeimAqR1mAkvgTWgpmmoNR5CUhwCqzqr"), // WFCx - Wells Fargo xStock
    pubkey!("XsuKHFvcZLWvhk8XcprK4tiAZRefQ4nfTyNs6b9BArz"), // WELLx - Welltower xStock
    pubkey!("Xspj4rFiCCXJyymfNgkwBG9AXZbyp69HGyHwfz6WHTE"), // WCCx - WESCO International xStock
    pubkey!("XsptQPquxvzKZ7pZTXNhvtXRxFP8vvC6BmS6QL1PALe"), // WSTx - West Pharmaceutical Services xStock
    pubkey!("XsmopJuh6C6uNFJa3KaVoYqEtrf3Y7M5LYwRLKLER3h"), // WDCx - Western Digital xStock
    pubkey!("XsGFoZFgiRxDk49QCAQUArX84V88t7Bz3fsExp7HLuX"), // WABx - Westinghouse Air Brake Technologies xStock
    pubkey!("XsxjGDiXTcvRjbHgAkngyJgix766hJHKB3XLzNiRSro"), // WSMx - Williams-Sonoma xStock
    pubkey!("XsbkEG4hDnNbTAA5DNDaLG9jYPMTK9q19CuNuzhRR8b"), // WTWx - Willis Towers Watson xStock
    pubkey!("XsgZpBWek5D1KENwnJjP79KEWa7jbzVhoCqMVpqY5Qz"), // WINGx - Wingstop xStock
    pubkey!("Xs3vvmDWbAv1fJQrDSUSXvuTYJwVTDRPnzPDVgr9oMC"), // USFRx - WisdomTree Floating Rate Treasury xStock
    pubkey!("Xs1xSyecLzT2u188qGFs1JTBXeE9RmBX1pc11o5X6U2"), // WWDx - Woodward xStock
    pubkey!("XsDa7zkFPXdu1wz29JLEV2XtWvrLdb16s4yH1LcKQ5r"), // WDAYx - Workday xStock
    pubkey!("Xsgefxs1R4c5pYqhijeA3QLT42i38J7QpBtnt4GGk1f"), // WSOx - Watsco xStock
    pubkey!("XsLkMYrJVUPJv23tH3PB167BV6kpdubPe5F3KfVvSFN"), // WYFIx - Whitefiber xStock
    pubkey!("Xsr6MgLKmoEmN4aL78MQ2q98R8wJnZ3FaCZUMwkNbNU"), // XELx - Xcel Energy xStock
    pubkey!("XsQ2Jtw4SLMWmSwAuTspoDvGoAiR2iFA2JQNNxh9WPQ"), // XEx - X-Energy xStock
    pubkey!("XsJF1CN5uFwF5eD7b8htZDeYzY9o4kdepkvBdsa32ZT"), // XPOx - XPO xStock
    pubkey!("XsqFzD9CqfpnmTiFH631bcvQcDReVTMFZMSjk5aXQZL"), // XYLx - Xylem xStock
    pubkey!("XsNor8pbicnexAYvgvq3p8EXwyRj9UzjYu92ZnrJLqC"), // YUMx - Yum Brands xStock
    pubkey!("Xsxe5ypvCfynj9q9bJgB2Y88sZNCnxtEb9icgdbsPFT"), // ZBRAx - Zebra Technologies xStock
    pubkey!("XsE48wmot6dKRcCGVQUXFSrpnRZ4U4yC762ZktjkoGW"), // ZBHx - Zimmer Biomet xStock
    pubkey!("XsRPgsEQ3dFR84DhMv18jtob5D77FivAsTTsv1jCYmv"), // ZTSx - Zoetis xStock
    pubkey!("XsgDPEr2Zk1YkWnw1Mm77APoZpeTj8BQZeGSMVtBZGd"), // ZMx - Zoom Communications xStock
    pubkey!("Xsb1a2VbaT6rLLq48qcgMQ1iPbs5fW2JZZURBxKfhY6"), // ZSx - Zscaler xStock
    pubkey!("Xs3oZwbHvqis4NYcf4YKWmEia2eC84wSiVrcYcTqpH8"), // SPCXx - SpaceX xStock
    pubkey!("XsueG8BtpquVJX9LVLLEGuViXUungE6WmK5YZ3p3bd1"), // CRCLx - Circle xStock
    pubkey!("XsP7xzNPvEHS1m6qfanPUGjNmdnmsLKEoNAnHjdxxyZ"), // MSTRx - MicroStrategy xStock
    pubkey!("XsvNBAYkrDRNhA7wPHQfX3ZUXZyZLdnCQDfHZ56bzpg"), // HOODx - Robinhood xStock
    pubkey!("Xs7ZdzSHLU9ftNJsii5fCeJhoRWSC32SQGzGQtePxNu"), // COINx - Coinbase xStock
    pubkey!("XssvmC28hqLgtT5kf8R8eAv3mejy4XftJ6MXB8QywTa"), // TTWOx - Take-Two Interactive Software xStock
    pubkey!("XshPgPdXFRWB8tP1j82rebb2Q9rPgGX37RuqzohmArM"), // INTCx - Intel xStock
    pubkey!("XsESMQjWyDczCfiJ3QDjZEiaKwp1uXYp3TMZ6viuLcm"), // DRAMx - Storage ETF (Roundhill) xStock
    pubkey!("XsqE9cRRpzxcGKDXj1BJ7Xmg4GRhZoyY1KpmGSxAWT2"), // MCDx - McDonald's xStock
    pubkey!("Xsf9mBktVB9BSU5kf4nHxPq5hCBJ2j2ui3ecFGxPRGc"), // GMEx - Gamestop xStock
    pubkey!("Xs78JED6PFZxWc2wCEPspZW9kL3Se5J7L5TChKgsidH"), // STRCx - Strategy PP Variable xStock
    pubkey!("XsSPkcE4GbHiBacr8awTTHpwFBuznazcwEKgdvBCkoz"), // SQQQx - ProShares UltraPro Short QQQ xStock
    pubkey!("XscR1RVZydyrL6Wn7WqbW2k6EmkLuS8SqDzuyptwxoj"), // DJTx - Trump Media & Technology Group xStock
    pubkey!("XsiiNuNGVxvdpGqsSRLQYNdLkARwLBBRFLDLqCFck8y"), // CYPHx - Cypherpunk Technologies xStock
    pubkey!("XsprHSJzwz3qmHcEf7j7WcUk6hMUL4sPLuAdWaSY1oh"), // HIMSx - Hims & Hers xStock
    pubkey!("XsefS5bmqC24xdsZMhuHVQKPExckM3kpwJWy4pJQitA"), // RACEx - Ferrari xStock
    pubkey!("XsyusqQvb8RDULsY9szwvUv2CxrDKa642w7kecZPdRM"), // MNSTx - Monster xStock
    pubkey!("XsJBX1kSf58ymLVKFSm2MkbLtsPiVRMWTvkvF7TSiVT"), // BABAx - Alibaba xStock
    pubkey!("XsEkwEYVKnvqVF3sxsimXVEEB17jdT9pVA6SP3NBEYe"), // AMCx - AMC Entertainment xStock
    pubkey!("XscCMVe1Qu2rk1YoCDJGwMLZkfVnd69xcPER4DJ8tpB"), // DKNGx - DraftKings xStock
    pubkey!("Xss5RAku5EH6UViFdvW7ss9xQjwQLsrs2opPMhb3k43"), // RBLXx - Roblox xStock
    pubkey!("Xsi8P9r7ZWDBBnBj9JTfnKWqyKNV19X2CQPWQsGdjnS"), // BOTx - RoboStrategy xStock
    pubkey!("XsSE8X3ZCzVi4r6nebtfT5xHAcpwTqwXNzXRSDYjynX"), // SNAPx - Snap xStock
    pubkey!("XsjQP3iMAaQ3kQScQKthQpx9ALRbjKAjQtHg6TFomoc"), // TQQQx - TQQQ xStock
    pubkey!("XsDmtC5EpfzJoCzDN2SJP98xpCsSR2CqdZauJSvHNXT"), // LULUx - lululemon xStock
    pubkey!("XsGYpMvKbVt6ViHqRd7cF3s746dAMFBQWcC49hB9VVP"), // NKEx - Nike xStock
    pubkey!("XsXEEJX7WoXfre8ZqYx1M1yXnEwd1Ww3YhvfESA2DzE"), // RDDTx - Reddit xStock
    pubkey!("XsiRaD8qZm1P3KViowDYFS8rZ8BYT2g7ZEeyztthRye"), // COSTx - Costco xStock
    pubkey!("XspwhyYPdWVM8XBHZnpS9hgyag9MKjLRyE3tVfmCbSr"), // IBMx - International Business Machines xStock
    pubkey!("XsViHjyRbiSBskjizb1gszGTmZqtAH8TuqwJ7sB5AcP"), // MRNAx - Moderna xStock
    pubkey!("Xs4uNhvBDAcp2mz3g9XR5q3vzLgmF1ANWxJgWk2d5u3"), // WENx - Wendy's xStock
    pubkey!("Xs3N3dcHdWXaJ5dv2ysjMA8DS33vixYYzAgVzkG6L3T"), // GPROx - GoPro xStock
    pubkey!("XssULacY8D3z3a8HoQuLid8wdJHcZLGpSST9MkNFzox"), // LMTx - Lockheed Martin xStock
    pubkey!("XsRJiWgqGJrDaidERfWfdZkaxdA4VPjVEqN6XFctpp3"), // QUBTx - Quantum Computing xStock
    pubkey!("XswenHXJtDWYMh89uRYx2tZcABxwXSn7j3jidDPS1Yo"), // EWYx - iShares MSCI South Korea xStock
    pubkey!("Xs2yquAgsHByNzx68WJC55WHjHBvG9JsMB7CWjTLyPy"), // DFDVx - DFDV xStock
    pubkey!("XsRN7n3AZDs577pgQSGbrbgznY5T9xERcjr4dK6vHnX"), // SHOPx - Shopify xStock
    pubkey!("XsBcnKnZMsPaerLiUQ4eMFy4Fjysot4RugYXYDjiqCP"), // BAx - Boeing xStock
    pubkey!("XsAtbqkAP1HJxy7hFDeq7ok6yM43DQ9mQ1Rh861X8rw"), // PFEx - Pfizer xStock
    pubkey!("Xs3trfdPXSZuxBJsgau6HRfu8SdrCirkwHpPNgSpJz9"), // CRWVx - CoreWeave xStock
    pubkey!("XsmGSEqT6VXpVis3aVBDxaNwPgHNXbkjkVUCncsLkNB"), // BEx - Bloom Energy xStock
    pubkey!("XsKSh3QDynp6oms9yHjZXbZo3pKzUBoqUFKPHS2g9Bh"), // RKLBx - Rocket Lab xStock
    pubkey!("XsnqNkeDhvaK8C5B3Z4D1ciCupVAgkw1AdS3U2uEn26"), // UPSx - United Parcel Service xStock
    pubkey!("Xshh1dRsnxatP45yBfrzU9MrvrFCvxHQGTrWjgdA81E"), // IRENx - IREN xStock
    pubkey!("XsXJBom7kFSxA6hc2ArzUwe6o6zyRU1AdHTfvev1HkG"), // USOx - United States Oil Fund xStock
    pubkey!("Xs5ywCCaayf3TFdxFxuJCr25XMoqY6vdfJKkSHE4D6r"), // BLKx - BlackRock xStock
    pubkey!("Xsb9Hu9KkxJxMNoaogz7UYsNS3VHvLCqwT9S1mRyVGE"), // UVXYx - ProShares Ultra VIX Short-Term Futures ETF xStock
    pubkey!("XsBPCyiRanUFCw5zd3m9EUGWMZZRhtpavZ5SWPdTWM6"), // BBx - BlackBerry xStock
    pubkey!("XsRyxhErkd36CqMefXAqcFgcR2q4rPtrxXV72yJMg51"), // SPHRx - Sphere Entertainment xStock
    pubkey!("XsoxDMY8s3ZV7zD4Yd87wAvURyxA8VXkd7w6HNuQkzj"), // RIVNx - Rivian Automotive xStock
    pubkey!("XsQPrqFMbPDp3fa36oU9PT61DiQPJaGwEmox3CsXTqB"), // CHWYx - Chewy xStock
    pubkey!("XsuwUbQSzCJN2wZabD1Gxf1MER2Ypa7hMzVMYB2WawJ"), // WULFx - TeraWulf xStock
    pubkey!("Xs31mE5EiqjSHEaiX9QDKCN6NvSGCqpJ6f1FNq2wri5"), // RIOTx - Riot Platforms xStock
    pubkey!("XsvsSHRUz5dAmUGH9jvVnkEZRcLkgo7uJf1VWhSuVLk"), // EWZx - iShares MSCI Brazil ETF xStock
    pubkey!("Xs2hW1n8cBJAKUtDxZjJtFZBaxqTwczJY6n6XY8eV4o"), // MGMx - MGM Resorts International xStock
    pubkey!("XscSc1zjbVizEnhCzzehJ9fzztm3WRKdn9pjmriKDuN"), // ABNBx - Airbnb xStock
    pubkey!("XsqFgFeprZEpayi92MTQGheZiNFmTi2TRJRmrHjCzMx"), // IONQx - IonQ xStock
    pubkey!("XsTSUDFMukRGiSzCg79RNWXQbmCbBNZisW4GcDXhboo"), // TKOx - TKO xStock
    pubkey!("XsG6o69zE7mQCeFf6utyMx9Q89cJjGV4vTwi3wFHrkN"), // ICEx - Intercontinental Exchange xStock
    pubkey!("XsR1ktNCRYDoDNxhaSxF6NX2HyT4ACSQBgPRLpXTWmm"), // CMGx - Chipotle Mexican Grill xStock
    pubkey!("XsV4wp56vcokXjRWaeBxhf5PrYLtGtNxWDa9KddWGcx"), // HTZx - Hertz Global xStock
    pubkey!("XsybfiKkD4UmjkAGT2uR8X2sq9AWFtvGJM2KTffoALZ"), // COPXx - Global X Copper Miners xStock
    pubkey!("XsbXYrWioEcvy1wAjkzuV4QdN9vGNgyKFd5AzD5tpea"), // DNUTx - Krispy Kreme xStock
    pubkey!("Xstq9oUsBPd8LSyivHhnt8P8t9xK5RYoM9P6izLczvm"), // CBRSx - Cerebras Systems xStock
    pubkey!("XsQTDfyyWUBRbLnikwDoSqrcn3YKLG2EDRLHCoQ9e3a"), // FWDIx - Forward Industries xStock
    pubkey!("XsBugUzyEvG9tNCV82U3nA6R13LbfV9LzEsXe7ckhCz"), // BROSx - Dutch Bros xStock
    pubkey!("XseRUafvQDXZSddDFpsXQPBNggurszyztkh4TEnvEuM"), // LUVx - Southwest Airlines xStock
    pubkey!("XsZs72XGLgc7jzsT1SLntWcSS855Rir4Pd4SWAQsokR"), // GRNDx - Grindr xStock
    pubkey!("XsAss8V7MYP9Y36ELHHjHMKcxZBDcGwRaFb4X6VhBCb"), // FLWSx - 1-800-Flowers.com xStock
    pubkey!("Xsv99frTRUeornyvCfvhnDesQDWuvns1M852Pez91vF"), // PEPx - PepsiCo xStock
    pubkey!("XsrBCwaH8c46xiqXBChzobgufRKxQxAWUWbndgBNzFn"), // BMNRx - Bitmine xStock
    pubkey!("XsA5xNUbsqxyz22XSFnNeALLv5XsmBgq3SCaK6xFR5R"), // FLYx - Firefly Aerospace xStock
    pubkey!("XsNNMt7WTNA2sV3jrb1NNfNgapxRF5i4i6GcnTRRHts"), // CVXx - Chevron xStock
    pubkey!("XsqgsbXwWogGJsNcVZ3TyVouy2MbTkfCFhCGGGcQZ2p"), // Vx - Visa xStock
    pubkey!("XsAsZLF4MmsvS1sDxRMrUz7REjHfwbC9UAMXSRBqgEB"), // UBERx - Uber xStock
    pubkey!("XsHtf5RpxsQ7jeJ9ivNewouZKJHbPxhPoEy6yYvULr7"), // ABTx - Abbott xStock
    pubkey!("XsxAd6okt8y1RRK6gNg7iJaqiWNiq5Md5EDf3ZrF2dm"), // SLVx - iShares Silver Trust xStock
    pubkey!("XsssYEQjzxBCFgvYFFNuhJFBeHNdLWYeUSP8F45cDr9"), // VTIx - Vanguard xStock
    pubkey!("Xs5UJzmCRQ8DWZjskExdSQDnbE6iLkRu2jjrRAB1JSU"), // ACNx - Accenture xStock
    pubkey!("XsApJFV9MAktqnAc6jqzsHVujxkGm9xcSUffaBoYLKC"), // MAx - Mastercard xStock
    pubkey!("Xsr3pdLQyXvDJBFgpR5nexCEZwXvigb8wbPYp4YoNFf"), // CSCOx - Cisco xStock
    pubkey!("XsyZcb97BzETAqi9BoP2C9D196MiMNBisGMVNje2Thz"), // IJRx - S&P Small Cap xStock
    pubkey!("Xs8drBWy3Sd5QY3aifG9kt9KFs2K3PGZmx7jWrsrk57"), // TMOx - Thermo Fisher xStock
    pubkey!("XsTEo8W2L8JprN57bmJgatDT84z57ReJYizr5f32BfP"), // MMMx - 3M xStock
    pubkey!("XsMxAoJP47FQGLsVUvSS2QfBaHdNsd7DRU6nWRL8RSa"), // SMCIx - Super Micro Computer, Inc. xStock
    pubkey!("XsZUSqxAXKJkEimvD4CoVvEb4WUC92TFgj5zRtBxFeL"), // WBDx - Warner Bros. Discovery xStock
    pubkey!("XsosCAu1L8Ebpr4SdBDV1EboYDRTWCH8j79UnHYzvbN"), // VGKx - Vanguard FTSE Europe ETF xStock
    pubkey!("AA8fNXwHo2TFgzW77Qggq6J2vQZDYZ88fX3Rt1VAS69H"), // AA - Alcoa Corporation
    pubkey!("AALFWGekLbAgSW54dAmCwhCXKfX5z16N3wWNpMB5rdYK"), // AAL - American Airlines Group Inc.
    pubkey!("AASUXcQnUvyQi8KLTs42pm5i4oz3Nsmrnh1GB5XZtvHP"), // AAOI - Applied Optoelectronics, Inc.
    pubkey!("AAPLEDt8RpzPgXyhvFzkMBofvFSQw9gpeMCoUdPdLnB8"), // AAPL - Apple Inc.
    pubkey!("ABBVwWHRKef4fAGmcFgkXpD7K3BZgAufkHTVuP2DyaRd"), // ABBV - AbbVie Inc.
    pubkey!("ABNB56TteaTZG1S3DY2W8VMncQmHMvNCGaZgAba49CLP"), // ABNB - Airbnb, Inc.
    pubkey!("ABTsKmCVRdFcq9dgiCRAmgGhwM8miifXTinYaBaN9N6j"), // ABT - Abbott Laboratories
    pubkey!("ACGLeKsiaTBQHwYpBC6sNJVZ3Pv3NzYqSnQEDpg7V7eW"), // ACGL - Arch Capital Group Ltd.
    pubkey!("ACNiE9t6PQb38U5ETeUHiHKFc6bj1zjoDqLVvtqhjhPa"), // ACN - Accenture plc
    pubkey!("ADBEwXnAgdFVj42y3ZCBouUjZ133875hVrbdQGpQfRCk"), // ADBE - Adobe Inc.
    pubkey!("ADqtQq3fZLZzGHSMzhZwHx5v63dJes3wWRxRAHuDTJZH"), // ADI - Analog Devices, Inc.
    pubkey!("ADMoFqm8iw6vaaNXVekS3KEwB8ovSw3iYYnBgAf8soEK"), // ADM - Archer-Daniels-Midland Company
    pubkey!("ADPNB21qRB5SLAirm12jWjWJXvbmSyawoE79DoHV98YX"), // ADP - Automatic Data Processing, Inc.
    pubkey!("ADSKrpqTgdmeHV9a9kL45C7WjYzQqkcH2KHG6Hy5FjMX"), // ADSK - Autodesk, Inc.
    pubkey!("AEEsu9VtSVDwyUf2riugQyY66p9mvANYN5xRbw7ywDjD"), // AEE - Ameren Corporation
    pubkey!("AEHRtbxGNt1HwZtkZQFQoJMZMjAKLUw2rxLJFchGDKKs"), // AEHR - Aehr Test Systems
    pubkey!("AEo2DfHxKynPsqVcpY2zr5o9K9vceKBTrYLY6ooD2Cfb"), // AEIS - Advanced Energy Industries, Inc.
    pubkey!("AEk1M7ehtv5f26FwcgoDgTHDzfJsAaDnhogvQAtYAyQM"), // AEO - AEO.US
    pubkey!("AEPyq9AUEefKzKfx7p9ZuGe1LiMPQjZEiFpcbLCde9zE"), // AEP - American Electric Power Company, Inc.
    pubkey!("AESBn2kHDHZwZ7UjYr1aGWLWjCRntAtJjqBPoHEktLRa"), // AES - The AES Corporation
    pubkey!("AFLFg7qJAdxjsCxorMousboZwcTZTRDvsfSVBC4gxXaU"), // AFL - Aflac Incorporated
    pubkey!("AFRMNRdJz1vRPj67E5TsmW6AfztDrdBdsqEt1vyRA1ux"), // AFRM - Affirm Holdings, Inc.
    pubkey!("AGNCqhRqsTAWP3g1BPxp6adwwbwk4EFp2w1SJxi9M83E"), // AGNC - AGNC Investment Corp.
    pubkey!("AvrvV3iqmqhP3AAciUs3PWKsTVdw8VQAMiTSs2NwzcoG"), // AIG - American International Group, Inc.
    pubkey!("AJGqEi5nNJqfdhamsZnPMRC8Df3qwmoiuetpYqCTnQD3"), // AJG - Arthur J. Gallagher & Co.
    pubkey!("AKAMAkhYKChK6VgDtXGRpXhJxNLfSsvuGVkUvUpGqUfH"), // AKAM - Akamai Technologies, Inc.
    pubkey!("ALABN9xrge8LbvJmrE8xusXVXs7GNwtVxSuFQhXJnXSC"), // ALAB - Astera Labs, Inc.
    pubkey!("ALBdfxcfnNNWzsWVKSHTR2MA4NEzD44rTbqnBUci8S7F"), // ALB - Albemarle Corporation
    pubkey!("ALGNWvoG32YN9eNh75FvTLYAN9yRDLisAQsRVdww4cx3"), // ALGN - Align Technology, Inc.
    pubkey!("ALKipZLy4T38rBBF68dWWdWe5NFS3WEiQ4tHYPtaS5PN"), // ALK - Alaska Air Group, Inc.
    pubkey!("ALLbQs4TEHXkiQzxm4DmQbHWz5mf7WDXBaun81jAQoF1"), // ALL - The Allstate Corporation
    pubkey!("ALNYmBowb126w6eDVCPa8LtczBuA6q4XeQpuUimAaCAu"), // ALNY - Alnylam Pharmaceuticals, Inc.
    pubkey!("AMATEdemudkhfHEV3uWGqGHE8Dvg7ayBwqzC6dSoAH6r"), // AMAT - Applied Materials, Inc.
    pubkey!("AMCRvS2gMZsgtZpnYFK97wmbZmqfQqR5VaWRR2f4osSb"), // AMCR - Amcor plc
    pubkey!("AMEomrKvYw8j3FiRuT3wvZLZg3dLhVt6WX2vpAnY6C9d"), // AME - AMETEK, Inc.
    pubkey!("AMGNAYYQMJUhQbEyRPQ2kcBoeWbbmb8Nu6WUtCqNxADD"), // AMGN - Amgen Inc.
    pubkey!("AMKRt2fBGrQ9Jzut9Azvn6r8Xirv4nrU8tt7ZkGD3Cex"), // AMKR - Amkor Technology, Inc.
    pubkey!("AMPgArvW1qeBkkQnpiXqih2pRLnFXLgNJjZYRKEYm8Ke"), // AMP - Ameriprise Financial, Inc.
    pubkey!("AMTYhstMXWMeqq9bQMXTGfSFyor2Yk7B9iw2iyCcz6GP"), // AMT - American Tower Corporation
    pubkey!("AMZNw7cwU9L9c7ewZXxTws2PwQYxBGm2Rxg3FLDBJz9B"), // AMZN - Amazon
    pubkey!("ANETj2ubx2yiqx49rLC3gts7FjWFqb4xFc2tktSe4z6w"), // ANET - Arista Networks, Inc.
    pubkey!("AmZU58o1cSYiEhDFed2UaB2YXf5JcFY7ooZ6suYUSDGK"), // AON - Aon plc
    pubkey!("APA3D392HPL9AfyesZaeog3KFDjCDoN9NqEPH7P2bqp"),  // APA - APA Corporation
    pubkey!("APDPC3BLF6426ZE7ZjnFHocpXNXPoKNwNXUaLDpNQgft"), // APD - Air Products and Chemicals, Inc.
    pubkey!("APHwoyEXBi8ccyQdhwVzNcRuWbDD7nFmxM8FL7oTm6Hm"), // APH - Amphenol Corporation
    pubkey!("APLDFP4BWhChkUay6BpL8hbPBb2mbTq3gbnEucFDsoaY"), // APLD - Applied Digital Corporation
    pubkey!("AP1rVdcKZdPvLxVykKRkzsLCYdi25khvnVT9C3qYDBT6"), // APO - Apollo Global Management, Inc.
    pubkey!("APPZgX2B9xjAQGqsSmvaBbDJ9rjfZgSV2JWtCZNpvy7v"), // APP - AppLovin Corporation
    pubkey!("APTVytxeS4mbvmRv5L8Ht58itCK7k6eg5bUSGwDLbps9"), // APTV - Aptiv PLC
    pubkey!("ARPFWTURhTRMSBzsGCeEnRW6USufxfnkAAh6S7R91S9i"), // AR - Antero Resources Corporation
    pubkey!("ARESbYbTUyzSSvFASy7VR8doNip5jfVnkjg4KvJPGEHS"), // ARES - Ares Management Corporation
    pubkey!("ARWR5eX18w997S3i1XFVwMGr5CdMGzCUSvHXxaPkx4qX"), // ARWR - Arrowhead Pharmaceuticals, Inc.
    pubkey!("ASMLjWiCGtRVmKNA46MGAe4juDeZW9gxfgovAVuXBmpp"), // ASML - ASML Holding N.V.
    pubkey!("ASTSZrfAbXCZT8gtYyWQp4Lkqm6D5f3j9iN65YhT6fqj"), // ASTS - AST SpaceMobile, Inc.
    pubkey!("ATrVZn4NWhDY8MHaYaBF2mqv8Xd1VUTye14jVj3Q25qx"), // ATI - ATI Inc.
    pubkey!("ATvtt44TkqmwPGWVwkTgZiGke2aYpAeGF2yFY9ZqkTzp"), // ATO - Atmos Energy Corporation
    pubkey!("AUmbWqKRsPuj2wQeppbdzEnUn2WiAaDGuvqPBZ5r1JA6"), // AU - AngloGold Ashanti plc
    pubkey!("AVAVJCs6Huh8ArHDgG3ePDjjBcY9gP5pnsuBafd7ykAC"), // AVAV - AeroVironment, Inc.
    pubkey!("AVGoo4BQGsEfAmu4HU2deZTaTmLoqB3q3EzKXqAANGYD"), // AVGO - Broadcom Inc.
    pubkey!("AWKkmop5SpAERQBqf4zH2KtT6cPSfSzosExFjqJJVWBs"), // AWK - American Water Works Company, Inc.
    pubkey!("AXbkrudAq6VEzpS78PbCXFnzyRPH2XK2fKefDJ6oFJFy"), // AXON - Axon Enterprise, Inc.
    pubkey!("AXPgyGSi8MQSYxVZ9pUbGfE7k5pJwAyEtbPXQ7v332Bp"), // AXP - American Express Company
    pubkey!("AXTn8fibbGv2Zz6seBGZxZpfdJdZ2a6acop7gTonydAt"), // AXTI - AXT, Inc.
    pubkey!("AZNJrsjDM92nPpTSq6yFW1pgPhZZHEPxiJ8Cuf4qFhZs"), // AZN - AstraZeneca PLC
    pubkey!("AZBcv6eUTSgr2yuT2ZVsSf3CUKA6nThqEEZzPaJ8H1m4"), // AZO - AutoZone, Inc.
    pubkey!("BAC2PJa2dgSWXd3Yvncg1ZPK451CP4Cbp7xBgWss75iy"), // BAC - Bank of America Corporation
    pubkey!("BAM83dB8F5PxwskPupuxDq3wcsKpeP9p1871sb7eDTx5"), // BAM - Brookfield Asset Management Ltd.
    pubkey!("BBfn1tWQDLunkT8e1zHfVBL8GrMF9aT7fsoFj5L3Wn4"),  // BBIO - BridgeBio Pharma, Inc.
    pubkey!("BBYv731hfURjhuXYqXdy7MAqe6gQQrAV359Cfy5tr1qs"), // BBY - Best Buy Co., Inc.
    pubkey!("BDXhBw44NCRejBe2BK5pyQidnG4NQSbzzWTH6YPBqp11"), // BDX - Becton, Dickinson and Company
    pubkey!("BG8qFTCjsTw3QRj3k1oE7ASL8Lb2tfkS1sUoUYpxzTes"), // BG - Bunge Global SA
    pubkey!("BpSdWY3H8jpudcid8LJP46sPr3zJFGKVCWfKRxk1x2Ag"), // BIDU - Baidu, Inc.
    pubkey!("BpwnTWrLtW5L17dqdffPsNp9WbEAEMFvJxzyz5UMi6Gb"), // BIIB - Biogen Inc.
    pubkey!("BdJkAo8jPvGYwkqY1vdXRb5aBsopgjCJrDNX3ZdS26d7"), // BITX - 2x Bitcoin Strategy ETF
    pubkey!("BJk7WpBHvpf9TdcSVmi2RxRDPbd5S1QSVer3EqBWFLP9"), // BJ - BJ's Wholesale Club Holdings, Inc.
    pubkey!("BKNG3kewvECEGpwqzawVQU8eRGE7r3Wgc3ZbC5bUcpdJ"), // BKNG - Booking Holdings Inc.
    pubkey!("BKRExSfiSCmRXsRDugyBanPTh8XBqi9krSK4AuDMjjby"), // BKR - Baker Hughes Company
    pubkey!("BLDRWrosdxVvveM6KcbUfktEM5QydK5qwV277pfDpdiz"), // BLDR - Builders FirstSource, Inc.
    pubkey!("BMNRzxhtA7NyWGRiRnLkuvmrDkZt7FFEkYP3t2Zi41Zh"), // BMNR - Bitmine Immersion Technologies, Inc.
    pubkey!("BMYhp9hJc7m46bTgNjh83od5UHVxiQcMLLxv9PL1Upoi"), // BMY - Bristol-Myers Squibb Company
    pubkey!("BNY7BR2qBanHaqhCuYGP6N2VbnK53ApeNdLMgetX8Jkk"), // BNY - The Bank of New York Mellon Cor
    pubkey!("BRGuhSvNbLQHwuot5KSf4eJqh8ybfRSnw41wGUJuhHqH"), // BR - Broadridge Financial Solutions, Inc.
    pubkey!("BRKyxuqqxN4ZdGZ5JfQeP4uUCovKy71zqhLDFC3JXFNk"), // BRK.B - Berkshire Hathaway Inc. Class B
    pubkey!("BRHwXbeKEQAAMpRWScUpxTwHqX18mrkHuyBW4sxidoHb"), // BRO - Brown & Brown, Inc.
    pubkey!("BSXhttjaGskeFQwpvrnffCgUQgwFZ7GEp75bjQLCjp5s"), // BSX - Boston Scientific Corporation
    pubkey!("BTDRRc8dcj9nrpqcW1D7wsL9HR1yHKYpUR96TeDHpE8P"), // BTDR - Bitdeer Technologies Group
    pubkey!("BTGjnmoHWJqzTSiWqtZJLaY42dR6hUd7AY8bxUmmD9Nv"), // BTGO - BTGO.US
    pubkey!("BURLHNBfJGVxX7wkFWSb8cz5uLSvfzrdeNVyt1fKXECX"), // BURL - Burlington Stores, Inc.
    pubkey!("BWAoAbBqgStEtoBv6Nz5fC8g5dvbsk9XBkeGGpknxLRG"), // BWA - BorgWarner Inc.
    pubkey!("BWETDZqaiHUWremAeJ5w2EXM6htoADUjFzeYuJhAWQka"), // BWET - Breakwave Tanker Shipping ETF
    pubkey!("BWXT5bKeKSuoRtxxdo6wd9P9QhkCWvhekC8bmfzuR6PA"), // BWXT - BWX Technologies, Inc.
    pubkey!("BXj33ptJYSV1KiBLcVUR3RW61jKaM6A6rX3v2FnQazbu"), // BX - Blackstone Inc.
    pubkey!("BXBLRssB9cU9QxHZL9dGZXzXzXergzbpky3pLkb6wK9i"), // BXBL - BOXABL
    pubkey!("Coff5nQBSTwhEP7ASYtyWrWvLNkyniBb8uHUBp2CSX3f"), // C - Citigroup Inc.
    pubkey!("CAGUryFPxn1gJHft7HBbvu2V5rRrqeLYGAugzQqPxhzn"), // CAG - Conagra Brands, Inc.
    pubkey!("CAH9kC4D7YGf1tr6An9LKHEjCS1Tn637Buxon2GB821X"), // CAH - Cardinal Health, Inc.
    pubkey!("CAR5W1mspo5qPmeWFL6LVyne9gs2whsutKN8PLxbz8BF"), // CAR - Avis Budget Group, Inc.
    pubkey!("CARREcxzuVB8XsVd3Zn3xq6ptAbs94GFCMr25Lz8EJGL"), // CARR - Carrier Global Corporation
    pubkey!("CASY6PNpY61J9LVM6E5jLJL2Lwk3zzs8oatH7pg91K4N"), // CASY - Casey's General Stores, Inc.
    pubkey!("CATEfhKpcvMD5ucWvDE6Ki57BMZsaiQTK9EtrWwEmtPa"), // CAT - Caterpillar Inc.
    pubkey!("CAVAFLx8BYtLFNzswFM1NvnAMQPefe1K5BrPoWFy7Uby"), // CAVA - CAVA Group, Inc.
    pubkey!("CB2EefdQw6CgK9urPB8etEBy6y5NoV6ZuB23MKA2Z39q"), // CB - Chubb Limited
    pubkey!("CBREQNAXRrUuefWvFQZM2TsKxfyXoom7tVgtdvAiUxHc"), // CBRE - CBRE Group, Inc.
    pubkey!("CCtEyGQrRA5nvdmoG7QLFdhhTsKEE9677pbxzYrSry3T"), // CCI - Crown Castle Inc.
    pubkey!("CCLsUbmV9fzNQoJxgjYSRHfyr7gXYph9uugSBQ7cULYp"), // CCL - Carnival Corporation Ltd.
    pubkey!("CDEjPBp1YUjVHC7RtyzRHxWHDYtc6NEa44R3yrsdyv3f"), // CDE - Coeur Mining, Inc.
    pubkey!("CDNSSNxpkpkhdSr9KG3pGDf5Ye4nqYpNaQYD4ovTf5uH"), // CDNS - Cadence Design Systems, Inc.
    pubkey!("CDW6jMCAXfFMXVYjMZLmuPreJEiQTrXsfZKmNt1E77wm"), // CDW - CDW Corporation
    pubkey!("CEGcqNCttY9RStDXh8z3UVg8LS8EqQpoepDaiX7tqTLD"), // CEG - Constellation Energy Corporation
    pubkey!("CELHG9eu29cP95A2awRwKC61MbavZMyfiCL6NUNRJpbb"), // CELH - Celsius Holdings, Inc.
    pubkey!("CFiaTowoLFdAGeqkzQiWCzoefr7bNyPBkc9SQK3cPeB5"), // CF - CF Industries Holdings, Inc.
    pubkey!("CFGe6LRrjEvpuRs9nNqLMR7bwh2PZwfSAbi9Zi6Cyddz"), // CFG - Citizens Financial Group, Inc.
    pubkey!("CHDceCrqYQu4Sks6bARc3dmGm4UgVzUPAk4GW2sTskQ2"), // CHD - Church & Dwight Co., Inc.
    pubkey!("CHRWCoU2hNnxcREfVdGaKCXJoKsLWBkdtGy3XnrLhRyf"), // CHRW - C.H. Robinson Worldwide, Inc.
    pubkey!("CHTR3JuLHmkCf7S1MeFBdmAy67nA117Zkynn3P5T3Ya"),  // CHTR - Charter Communications, Inc.
    pubkey!("CHWYHz2zfKajomqPV9it26n7vu7AgZZfqGHXCWtGWuJg"), // CHWY - Chewy, Inc.
    pubkey!("CHYMS8ZqM3dvsRZAupVt4MQNbZGgACYYT5DavTiqHUGK"), // CHYM - CHYM.US
    pubkey!("CwVU7wcRbo4V6hARt9bgrqkH3sz7st1KWBvQZXGDeVSi"), // CI - The Cigna Group
    pubkey!("Cix4JuEZUyHrzEN3P37xhcUYA6TUBJFCwCEWrKx7mJBd"), // CIEN - Ciena Corporation
    pubkey!("CLfLxHmCPcVwoy3vbSApDw7CfXrbwXvVsuWCHNuZnEcm"), // CL - Colgate-Palmolive Company
    pubkey!("CLFP4vSg1xhUvPZuqdtuHKeoL32SUMBtedcokUviKBqT"), // CLF - Cleveland-Cliffs Inc.
    pubkey!("CLSKhHJSYZa4HVCXbkVKE4iz1JNzyPdjQEwBP4NRjLxA"), // CLSK - CleanSpark, Inc.
    pubkey!("CLX8ncxsK83wkcAAeNyKYVYD75bphg3xUT7XEPqA88Wn"), // CLX - The Clorox Company
    pubkey!("CMCSACPJXL54MyitbXKXMXLbLfaA4M9WksUwCtLtw3eH"), // CMCSA - Comcast Corporation
    pubkey!("CMEkCEMFMU3xDZZpTvsGS2dAhEJQomNrZm4LsTMEtmdj"), // CME - CME Group Inc.
    pubkey!("CMGi497VQA9RiHVWpQi5B847vN8RLaEwcMhB5kEUhkAQ"), // CMG - Chipotle Mexican Grill, Inc.
    pubkey!("CMmeyuLvrhwxuAueKJWvnXs3cP1tbW5PVJfKC5nkDGsV"), // CMI - Cummins Inc.
    pubkey!("CMSr1cpsADbtc6fH1JC5uDDE14XZLcJyh7cxcFp1tY4X"), // CMS - CMS Energy Corporation
    pubkey!("CNCZpdCCtZGWjUf6WXGUWo9RjnTVTtKLnG6yAXidr7yX"), // CNC - Centene Corporation
    pubkey!("CNHxgm1p3A6ZzRo3eKo5z3cAR6TxBgyPL1p3Q6bgw9QR"), // CNH - CNH Industrial N.V.
    pubkey!("CNPzttfrnjSiNzMpgdt2zXNXJcPPt73UZTr3m8Mg9pAe"), // CNP - CenterPoint Energy, Inc.
    pubkey!("CDdPVwkm8ueiC5FFBbEcNLFAHvSkp6vwJnxZycRVH3A3"), // COF - Capital One Financial Corporation
    pubkey!("CoHRpXjDRHiZ1EJKi6z5ugwpF8tcoCdWfdtMYgXGcR5X"), // COHR - Coherent Corp.
    pubkey!("CoiNzDC8qN2NiBy4ExBPHyVRFNq8J3Pvubq4HurSiVNq"), // COIN - Coinbase Global, Inc.
    pubkey!("Cn5mbEwoZSajR5SD2S1MXMrKyB79ZqFVDXqAdzpCMhRU"), // COKE - COKE.US
    pubkey!("C9yRvj4HV9nWFnC3wv83dtnoWzK8j7o7tUBYx86g2i4i"), // COP - ConocoPhillips
    pubkey!("CjJBRX2kmTWkyArMpu8Kt4kmWewBDvTD21YCzM7dy5gh"), // COR - Cencora, Inc.
    pubkey!("C1z7Wxuu7MmegSUeSVS53vWZBuBGx9f7exRbVVCQswLa"), // CORT - Corcept Therapeutics Incorporated
    pubkey!("C5pdJt8XQNMY9UUZRMqxf9mUveUje2PSoPf7SLMFRovK"), // CORZ - Core Scientific, Inc.
    pubkey!("CPAYhsNvTJ8d2SaC7biCUp3RhAiwit3mvqnFTtQDTukT"), // CPAY - Corpay, Inc.
    pubkey!("CPBw6Ak18mWSLripe34kc4wn4yUdDVHh5fqFGi9wWUF"),  // CPB - The Campbell's Company
    pubkey!("CPNGZ5AfiFKRDu4zH64RRpDSpbhYEbGa1cUEB9N5sNGz"), // CPNG - Coupang, Inc.
    pubkey!("CPRT2qaNF73w5pn18KeF9GaZy8B7rLLQoxhMCfmtahyn"), // CPRT - Copart, Inc.
    pubkey!("CRCL4kkDNmBCRjfrKJGqSnx7UsboEnjFpTTEvs5KmNmi"), // CRCL - Circle Internet Group
    pubkey!("CRDaBBjz1Mo3nybiB6TXVtPHZDqbfRUjTa9mEiKa1PLs"), // CRDO - Credo Technology Group Holding Ltd
    pubkey!("CRHnec5cFXKvCnuJFkg1eyD3cTcqdvzEBi88kz53AEQJ"), // CRH - CRH plc
    pubkey!("CRMuPKXij2Qnw9zUrEq3T12vPZAZzphYBaZ1Tn8Nbj6T"), // CRM - Salesforce, Inc.
    pubkey!("CRSxiWGMTVJjMyUHkBctm4qFTdqvaKbSAXzXMVcvWpH"), // CRS - Carpenter Technology Corporation
    pubkey!("CRWDjExP1yadeAjUQSUMP1FS3Gd6MbdKK7pNmUhjDihC"), // CRWD - CrowdStrike Holdings, Inc.
    pubkey!("CSCVFid86bhcdccpMUm1rFd7fyRaujxyTxgiDcLWUU5o"), // CSCO - Cisco Systems, Inc.
    pubkey!("CSGPnQh47GoZQ22r3Wf8gqVivtV7hM4cG8nRVAUANohu"), // CSGP - CoStar Group, Inc.
    pubkey!("CSXTLMfDN3voJ1Hw6z5CgbM42ZrTxA9nCcApGKd4cJxA"), // CSX - CSX Corporation
    pubkey!("CTAS1ycJqwCdEBvyrvRZgoF5n9Wfkbf1efG26GM5shgL"), // CTAS - Cintas Corporation
    pubkey!("CTSHVf83aa1BsbpbD7iSsJetjWXSheBnfYF375YymYDv"), // CTSH - Cognizant Technology Solutions Corporation
    pubkey!("CTVAUEdqbxmotwA5XxobHBgzDf8KHJC5V4kboPwycaR6"), // CTVA - Corteva, Inc.
    pubkey!("CVNA4766vEXGpXeo7ULFQHbYPBDzaUqgMFKJ8qwsSPob"), // CVNA - Carvana Co.
    pubkey!("CVSM6LaAWhyX56p7Ah2EjSYcYUw5ADgqtQgr335otrTG"), // CVS - CVS Health Corporation
    pubkey!("CVXU8f5BQ9k6TUR1v7Sxp7PTiGHFADXPqd7SkSPEzToX"), // CVX - Chevron Corporation
    pubkey!("CWUjJvQJmD5sK9a3rMpTdHRBZ8CFHTK3aFcKi3d74tir"), // CW - Curtiss-Wright Corporation
    pubkey!("CZRxUy2C4H95x5pU746NufG211aBPZ1Nz8BHktvuh6Fe"), // CZR - Caesars Entertainment, Inc.
    pubkey!("Di4CDW8A3evSjPhvidM1fwgTfUWZdNyeKfp42EV6M57o"), // D - Dominion Energy, Inc.
    pubkey!("DALSnBf5cEHos7XQ6SbQgiMk5piy9WwX5V3MFt14SueH"), // DAL - Delta Air Lines, Inc.
    pubkey!("DARXaiz4dph6bm5qkCUNhBgwMAEMpcjTVVypZpgPuhe7"), // DAR - Darling Ingredients Inc.
    pubkey!("DASHrA2rpN1B3Wy4ydFyZg3YBBSQvMSDXuNCpGV7cB1N"), // DASH - DoorDash, Inc.
    pubkey!("DDjmFXyk9jANEjJ9hdz7DmvN2guzPL8MqBF2vYbyDFZ6"), // DDOG - Datadog, Inc.
    pubkey!("DEUBdzpHpXx7fTgQxBX2KPCyrEJuuYNH7asH6TyQSRh1"), // DE - Deere & Company
    pubkey!("DECKrPBqLyJCaTYvHLNZFoLni12TBL47Bf6nszPWe87v"), // DECK - Deckers Outdoor Corporation
    pubkey!("DGvPCAdzQ3PJfKgA2mpBXeeKdXjCZafaKPBWoN2vetM6"), // DG - Dollar General Corporation
    pubkey!("DGXo9MDB2KVirKzCaoJxvVJTXa4Hfxsu9AswFAjsQqcv"), // DGX - Quest Diagnostics Incorporated
    pubkey!("DHb8Ridq2XiwhM3LNJrdCkij9KFEHuTLWVpSSFcodLNm"), // DHI - D.R. Horton, Inc.
    pubkey!("DHRy6QU6B3CthU9NAjTNtmV7pvuis3eF3TcKAze2YN2t"), // DHR - Danaher Corporation
    pubkey!("D2vnr1YuqfXUxV3FPxTfyz8T58gF3bQvEGd9hj4DzSVJ"), // DINO - HF Sinclair Corporation
    pubkey!("DfEtcVhiUZm5J3GySZjsZkkTMj79HdR18jxGiF7DvMP"),  // DIS - The Walt Disney Company
    pubkey!("DKSRa7qpE5icgNjxLDF1pmhRE7oDXjEJnCtN4NW7XAgE"), // DKS - DICK'S Sporting Goods, Inc.
    pubkey!("DLRu8ybVVH1waRj8EpA1qJA4MP7icURmKmBL9REJcSGM"), // DLR - Digital Realty Trust, Inc.
    pubkey!("DLTRPyz7HCGbFb65cKLB4MnUD6hd39CKz9CPDFxb1xEh"), // DLTR - Dollar Tree, Inc.
    pubkey!("DbcjtEyLiuWWKeewNkJcq6XYZB1sPgf771BU2YzMgpeC"), // DOC - Healthpeak Properties, Inc.
    pubkey!("D9Gf4BoSvWTZHrwwCkwi3eUDu7QmF52oR5VgXZMaHbVL"), // DOCN - DigitalOcean Holdings, Inc.
    pubkey!("Dtr1fHtZiBTA1MHC2F1RBJeDCsdFEp8wTcvFUbA34pHF"), // DOCU - DocuSign, Inc.
    pubkey!("DsGPWzQGqTEjABtF9zNtk6TUwbpJdi4ihGLfjj2ri69b"), // DOV - Dover Corporation
    pubkey!("DnbarPUnd3uCPwucJQqPryR2FcC99rP7z198nyc3QfLG"), // DOW - Dow Inc.
    pubkey!("DPZ3toRzoTz1W12n2o5zfq5AZ3kejupEQPpCa3EWMZBs"), // DPZ - Domino's Pizza, Inc.
    pubkey!("DRQ9mCpaDq2ihtbTvFtUwg9R5iB54FBUQ7iCyqkpHJPD"), // DRI - Darden Restaurants, Inc.
    pubkey!("DTyesYGpBdt9fapDXWCHd2uK2Lw43vR1rv9yaCF5SWNQ"), // DT - Dynatrace, Inc.
    pubkey!("DTEjrFkN4GUkfhPLyFgrUmZhmNCBaTtMvd5Wi453mVzB"), // DTE - DTE Energy Company
    pubkey!("DUKCeBTVehKce3R51vSFWeuUTSaAHzxVoGEwn1xAujZT"), // DUK - Duke Energy Corporation
    pubkey!("DUuD1sWHH7vyoMCjMbJ3oUXm7jNZqHMdDYmapUePBAEo"), // DUOL - Duolingo, Inc.
    pubkey!("DVAPMBrtHqVfpcXJJXKipuSn7rMpFMRsvT4ZzJyzHNcQ"), // DVA - DaVita Inc.
    pubkey!("DVNeaPA5UXXJhDks5LsMGr1UEfWQN8rRqnKkCZfZmozc"), // DVN - Devon Energy Corporation
    pubkey!("DXCMHb5dFChygUTctoMNqZERt4YQ4iScHqaW2sxGKW1z"), // DXCM - DexCom, Inc.
    pubkey!("DY3CAAdzSkocpvkyDuKZsnpDUR84PVPVPu9WzGXXeT9h"), // DY - Dycom Industries, Inc.
    pubkey!("EBAYcCycBkLvrGZWXFK8CAHy4hknWbkXEcmXmBaTH7zn"), // EBAY - eBay Inc.
    pubkey!("ECLPREQeJPDo8B7v3t8GomDesXtviYBUcssn7brSdgkg"), // ECL - Ecolab Inc.
    pubkey!("EDJCrRqrTF3ygfeY6CXJ6LxXXvUjdEP6cX6FWYQpC7n6"), // ED - Consolidated Edison, Inc.
    pubkey!("EFXfvjxacoDG722ZKSvS6gJjAVBaFpUSRZkPcLdnW5pb"), // EFX - Equifax Inc.
    pubkey!("EgVg5P8rS6enc7w946EYA89G71emJuymEXkBpgrzDRpt"), // EIX - Edison International
    pubkey!("ELxUXsuzEXqf3sRKrAWaAsn6THTwsPXYAdHwzcdLNoSZ"), // EL - The Estée Lauder Companies Inc.
    pubkey!("ELVGfMaKqkANKx6mdmovJ4r9b9vS6XweabvGRCQGELHD"), // ELV - Elevance Health, Inc.
    pubkey!("EMERSr4KyGWM3WF9qRyC6nnvN72HStiie2HJ1XFqAka5"), // EME - EMCOR Group, Inc.
    pubkey!("EMRc23btgR65DHfUHpSGq3GyDptVpprVYE2LGkuDqmxn"), // EMR - Emerson Electric Co.
    pubkey!("ENPHFsSCA7LNCeKCJ7cKrDKvW1UTB7WkKPieTRjt2ero"), // ENPH - Enphase Energy, Inc.
    pubkey!("ENTG1zo21N262JEKWvQTmzUHihH2kV2rTW3i4K8rMbS4"), // ENTG - Entegris, Inc.
    pubkey!("ExGapRi9M8pjrHybkGsn3qXA2LQD1ovCvDRiLTHwEkb"),  // EOG - EOG Resources, Inc.
    pubkey!("EPAMkaYMtas9nVGVeVQH6mZQ24YtohqHm4kFjPpCgded"), // EPAM - EPAM Systems, Inc.
    pubkey!("EQHAJ8BiEhyKu6AC4bxE2qLDGswGPmCfThmJRuyDKtiv"), // EQH - Equitable Holdings, Inc.
    pubkey!("EQmrWpnkCRqxVSPc2YAEDtJRWBPusUUr1eGipKrQPThq"), // EQIX - Equinix, Inc.
    pubkey!("EQTUSweFRDJdFfcoKUNAQ5kfVtWqs8Hecw38NzPyq3mQ"), // EQT - EQT Corporation
    pubkey!("ESq3ShM7NATF1Z4uL3ss1TmEjXhnhMhfzyNsR2y9RDBD"), // ES - Eversource Energy
    pubkey!("ETNXaiNJKYAys6FA4r8eSE1hKvVyZBtYwNcLYusHGups"), // ETN - Eaton Corporation plc
    pubkey!("ETR28rDFTovQdnqcNaw2qyeGL1g1u7WkmaSwWbsd15AT"), // ETR - Entergy Corporation
    pubkey!("ETSY7RJKdFRaWo3uqHmW3eyGXt67FFiDWoBkK1xbr4cS"), // ETSY - Etsy, Inc.
    pubkey!("EVRm72oxDZJapZm3c27C3cW99NBjK694UCod3pmj1GVV"), // EVR - Evercore Inc.
    pubkey!("EW9zFDVSGWiQ9HJbfhdev9KpwB1LJH6UMkjJmf3YjahN"), // EW - Edwards Lifesciences Corporation
    pubkey!("EWJQvQmWqKUpanexdCGEnNvy8PWnnEMQmSj9rbeGzGPs"), // EWJ - iShares MSCI Japan ETF
    pubkey!("EWQ5JtgnCVWDNzwx9Vo8irGSLn2hqkjPJHWKUzLbdUYA"), // EWQ - iShares MSCI France ETF
    pubkey!("EWT3HJsdofKQ67v3YyL9Z7M37LRtXnhuB1b5N5CTRahj"), // EWT - iShares MSCI Taiwan ETF
    pubkey!("EWUtBFqTVfkqfpEQ4Zf4AhK1PBS41PQibsttUTjZ6vxu"), // EWU - iShares MSCI United Kingdom ETF
    pubkey!("EXCoKTnD4TMaNzbkDJvjXcdfLq3FbZJio8BJ1y5Qb6WH"), // EXC - Exelon Corporation
    pubkey!("EXEt3Sromn77idGLyo53EU84ybaCfXNEVCWBJRY7cCPx"), // EXE - Expand Energy Corporation
    pubkey!("EXPDfecZYKKj6WeaKEYba2sWnQvu8HbAXfUbZWUpoFDn"), // EXPD - Expeditors International of Washington, Inc.
    pubkey!("EXPEVPqSMBpUXHWci9n3aYnfNMQVU7VcZCrDP27sSjbn"), // EXPE - Expedia Group, Inc.
    pubkey!("EXRZT5fPEauzHK1CzdhfH1ctt8A8o2BEjCDYWRaDxYCo"), // EXR - Extra Space Storage Inc.
    pubkey!("F7YtEZDj776vxYj9rgxbhnVy3qxVKXbTpmtdtVcaVtE1"), // F - Ford Motor Company
    pubkey!("FANGPsusTYf76fX5PukarCKq1MvFYyYsvH85Fi9HDRcK"), // FANG - Diamondback Energy, Inc.
    pubkey!("FASTx1PTvkcgAsVAWnSWMVRDw6gERghqvYFUWd28SByr"), // FAST - Fastenal Company
    pubkey!("FCNCAGnDHT7mUa5U1snd6Ru1jHtN7dNKbzmQZ2Zgxq6K"), // FCNCA - FCNCA.US
    pubkey!("FCXxXkNX4Q7yDvjvV2Xwd6Lq26mPimsFeX98r96yoZFd"), // FCX - Freeport-McMoRan Inc.
    pubkey!("FDSYjRArNuNEb6KrcUkkDVmSRXhqtuXzH9cMxp7NPfxs"), // FDS - FactSet Research Systems Inc.
    pubkey!("FDXWVe2vYw2Ek3DW9iNFnCQmq7qqkxcSDcrJiTeXCmnB"), // FDX - FedEx Corporation
    pubkey!("FEpGJc3uZEjigVExpXJhp8gJtptJWev8TVtds83ECLAg"), // FE - FirstEnergy Corp.
    pubkey!("FERGeRpyE1iHbuSK97r1JqEZxjNVm2wMmdht4aaba68e"), // FERG - Ferguson Enterprises Inc.
    pubkey!("FFNG4PzPhX2W6ijJD4Ub59YnNugrczWtMwbE9dw3r4JJ"), // FFIV - F5, Inc.
    pubkey!("FpZU1vq9GHQbGmTa4MzpywKkY7gEdfHFBWaSdb7fZY67"), // FICO - Fair Isaac Corporation
    pubkey!("F2PYZNM8vfWLoTLNLpY8Un8yb7jeQ1V62gpsZZZeL9gm"), // FIG - Figma, Inc.
    pubkey!("FwyfgwcnqPxY6ryxk2rA3xeCCjxf5fRUHbNg86aJm2e5"), // FIS - Fidelity National Information Services, Inc.
    pubkey!("Fa6jwouvCXSwAEE9HWZ1oJxK6c6fUfcsUCWUdwEyyhvQ"), // FISV - Fiserv, Inc.
    pubkey!("Ffr6TP7sbjyymZSwvyvwFCp4r7kCuHvtSrcNUN2w7xah"), // FITB - Fifth Third Bancorp
    pubkey!("Ff5z8z85ptfi4x5Neww1YTs9GumbxCJYnxSWY6tSfA7G"), // FIVE - Five Below, Inc.
    pubkey!("FsyYe5c8cCsGinW2cxHRSEAV9Wu4AUor67nUen1YV8By"), // FIX - Comfort Systems USA, Inc.
    pubkey!("FLEXqW5MbeGLiovfE2E1YsCbFDtNxKh2RbFx9xYwuTAx"), // FLEX - Flex Ltd.
    pubkey!("FLNC3gqX5ptQyGbrUbzRPRGmqZRqKU5AGBC4KokJmAqT"), // FLNC - Fluence Energy, Inc.
    pubkey!("FLSRAmBaz94KvJc3wiv9w3RF3w2JVsJ1s78vRnYoWYhV"), // FLS - Flowserve Corporation
    pubkey!("FLUTwQGHDUBcojEWhk6GzDenhi5dDn9eoBsTSooZ5u34"), // FLUT - Flutter Entertainment plc
    pubkey!("FN8cbVaUA5EaWk5Ni88UPoirzYpLxFTprnXDsxSwtVcY"), // FN - Fabrinet
    pubkey!("F89GtEieCda5bUHXCSz9GS3pf4o6Hg82aTnbhvxUt853"), // FORM - FormFactor, Inc.
    pubkey!("FwsrMphr5YSSdYEgjAiTEf5HyLNSe26PWMCR46V9VNWt"), // FOXA - Fox Corporation
    pubkey!("FPS2RUBGwaCMuRmoVfBYbNFWnHReUmvT14Pa1efY9WcS"), // FPS - Forgent Power Solutions, Inc.
    pubkey!("FSLR1GN2syPBV7GvsCT1EwKKQdR1NT62HQ1L5t9URLow"), // FSLR - First Solar, Inc.
    pubkey!("FTAFK7qD7CKFoE75L53gYF4yR8L4oFgqJZg2PRb89UWN"), // FTAI - FTAI Aviation Ltd.
    pubkey!("FTYNsyFrq17hrLEoXCKJ1dg1TMiRjZpKSFAZGrcR59n5"), // FTI - TechnipFMC plc
    pubkey!("FTNT4FzVP6ySTfLso1D6tCEPM3PHPES68iKKLiKGvWSL"), // FTNT - Fortinet, Inc.
    pubkey!("FTVNmN4ySYc8zbFoxaKEdQzPxj1eJ7WXxRKgXoUBeot4"), // FTV - Fortive Corporation
    pubkey!("FWd44X9rusTUzDipekfPKFJegxwVkDr7wp4cSaTuuksh"), // FWONK - Formula One Group
    pubkey!("GAPksXdAyNZFh5FKQcBPTezipikM25wo1ZqAAnANCFZ6"), // GAP - The Gap, Inc.
    pubkey!("GD71KuTAT8Bie2GD1TFcrRhprAMEwbspbKuPFpspwM7T"), // GD - General Dynamics Corporation
    pubkey!("GDDYHYq8KuG1voHPXtjef6unYr9HWA5PRMxTPq7q4jym"), // GDDY - GoDaddy Inc.
    pubkey!("GDX8bsP9nzmZC5nFbK68okhJytHXctmKmjcdkYLtJzxP"), // GDX - VanEck Gold Miners ETF
    pubkey!("GECH7BHzWQ6Rdaa2pDMwSP5XfVEiJCF7CqVBBn2ozTor"), // GE - GE Aerospace
    pubkey!("GEHCvZLrX1zZscF8B477CczxLxY6cvLwbhVjGThUkR2P"), // GEHC - GE HealthCare Technologies Inc.
    pubkey!("GENcF8nNEVqiHVrUHysga7nTMJMrdgtAZQYbrUw9QBwp"), // GEN - Gen Digital Inc.
    pubkey!("GEV2M2MUfmG3ZCsaccxsqfgn6mFev29GyVyhoPmUF9ty"), // GEV - GE Vernova Inc.
    pubkey!("GFSp4FSotxJfGC2pqbk3ejUADSAjM1F29Cq1v5gNQ6Bo"), // GFS - GLOBALFOUNDRIES Inc.
    pubkey!("GHsEAG13aadQssnGhS9iLjfCS9BcypEXt5RJqqNtkTf1"), // GH - Guardant Health, Inc.
    pubkey!("G1NXdieto2KxanUZrWRkaXUM83nELJNLQtipixmvXSty"), // GILD - Gilead Sciences, Inc.
    pubkey!("Gtb4FHX9FrRbkEaGATuSCSaq3mGZhd1Nb9kWd7fLH7rc"), // GIS - General Mills, Inc.
    pubkey!("GLDYTiasihdm4kjLCC8PFpjboMSj1UhJ8acmbtjC9DK5"), // GLD - SPDR Gold Shares
    pubkey!("GLWmypFYemSCedCNHjQCk4y7syGpvHG5Q9ts3phnNLnX"), // GLW - Corning Incorporated
    pubkey!("GLXYPRs6GE5547rc7mBzCZ9dFunctRtnNNj9apvLRVTt"), // GLXY - GLXY.US
    pubkey!("GMYvHj6jGLaTjxRtBTMZuNoptNRFcoLkPCREd4xL5UFy"), // GM - General Motors Company
    pubkey!("GNRCDdE85Vm21WqnBw8gxafvMai51dWN34PatWuJ8mr3"), // GNRC - Generac Holdings Inc.
    pubkey!("GooGLMvZtJZXHdNZy95RUrAMCCKxV6ghK3qE5ddBgVm5"), // GOOGL - Alphabet Inc.
    pubkey!("GPCbgnWBCUhYatwoTzEfDSsw2CwmuQcLFj6uvbovzP49"), // GPC - Genuine Parts Company
    pubkey!("GPNZWbPnLrsvPYpgY9SeutoJCUDZz3S34AnGipVMCjcS"), // GPN - Global Payments Inc.
    pubkey!("GRMN36ZxPGfQTT856H3FQVT4ftZRCLmqVJm41UERzzHu"), // GRMN - Garmin Ltd.
    pubkey!("GSsLRfhxVypZx9GNxs9CaFpeGFrWEnzTdtLQm1ifCTW9"), // GS - The Goldman Sachs Group, Inc.
    pubkey!("GTLBPa3zbXMaVute5JmhTkWw3a2y2FXWZtjCck3gZyHp"), // GTLB - GitLab Inc.
    pubkey!("GWREYwxTp4F3cwHeZRhWL3hVvNYTauNBeHpCBhiGFDzF"), // GWRE - Guidewire Software, Inc.
    pubkey!("GWWdJQVdNcxQ6j5aorEuR6rDCWnpiUmio3NsWjoA3q5G"), // GWW - W.W. Grainger, Inc.
    pubkey!("HALnQe1xqutKZHAW3ra4YY73qm5ZvhYRHz6me2KeCmpy"), // HAL - Halliburton Company
    pubkey!("HBANSb9NuwX3T2L1tqxmBBHm3ayxNyY8rRGpKe5P1UWS"), // HBAN - Huntington Bancshares Incorporated
    pubkey!("HCAuVcBNtXar6eMdUozUohx2AF3jw1Sno5C3hvc2X5r8"), // HCA - HCA Healthcare, Inc.
    pubkey!("HDH55GJYS5qJaUJxkYz1Qh6L768AEjYbKRYq3dmKBhj4"), // HD - The Home Depot, Inc.
    pubkey!("HEYRmPCVEmZNjbicGUCSFRNrboqo8p63epJTrYAxjCN4"), // HEI - HEICO Corporation
    pubkey!("HBoWufNmPNgZ7a9WZo5X4saE8NyPPAiazfYwdB5cLzZJ"), // HIG - The Hartford Insurance Group, Inc.
    pubkey!("HLsKtqeJy37Hgz881j9EWa8YmwCpWHzoKKwDMVCei34d"), // HL - Hecla Mining Company
    pubkey!("HLTaFahhasRXxYGmt6Jsh7uxGF2NP1D6SJwq95egKLe8"), // HLT - Hilton Worldwide Holdings Inc.
    pubkey!("HaZYCMj6FahfY93gFgtt5y2L6HrHorLpJZrE7myCB5FM"), // HOG - HOG.US
    pubkey!("HPErCtPmXfuqxevdqAM1WekpKCvRzwVPWrTcxMAwCShG"), // HPE - Hewlett Packard Enterprise Company
    pubkey!("HPQNkW9FYga8x5KzydWc165SwP5CFWN7GUUrJ9wFfebF"), // HPQ - HP Inc.
    pubkey!("HSTPL4W3hMTkco6nhWDcGGj62cYPc9M7NDSUhMYUW4eU"), // HST - Host Hotels & Resorts, Inc.
    pubkey!("HSYcL8rCvpZUMLZZuxarx1B1HqfRYeRvxhAMCDrLfaYq"), // HSY - The Hershey Company
    pubkey!("HUBBggMbYDVuZWKyMJDsG1sL3d7vKA26LhhJvJGtasXB"), // HUBB - Hubbell Incorporated
    pubkey!("HUBSZ8i7m1nYp7qQrE62zMvwdamH3XnPScotPG79o9bJ"), // HUBS - HubSpot, Inc.
    pubkey!("HUM8tdNMUwxEJx2rsdcbczysKovu3h4TYSCQ8HnM552g"), // HUM - Humana Inc.
    pubkey!("HUTEwGqjjKGyJ4nj5SafkgLytsZWFyLcckpxEUtz3JUk"), // HUT - Hut 8 Corp.
    pubkey!("HWM3cTNQQidduYS6YoL4tfDo8UUr1tpYPtaK4X2L4WB4"), // HWM - Howmet Aerospace Inc.
    pubkey!("BKRXeGXRregkRdqngccFwfDxhk79fP4G9DDTfBywKMFh"), // IBKR - Interactive Brokers Group, Inc.
    pubkey!("BRXLjsGEomj2fVt9GnwngzHCGuUWAx2QvPnY5GBtLuRg"), // IBRX - ImmunityBio, Inc.
    pubkey!("CEQnejxG1FPzMS6UiLbcxNiMKrZM3x6GTZucpJoSeByy"), // ICE - Intercontinental Exchange, Inc.
    pubkey!("DXXw8Hm6jTgGbq6XgKqFRn1gaAEn9m3XdPMLSSU7ygrh"), // IDXX - IDEXX Laboratories, Inc.
    pubkey!("EMGJhFXugTH4SzwJ9WSuJJs2mM1RxH5M2ztDXJVsD4cr"), // IEMG - iShares Core MSCI Emerging Markets ETF
    pubkey!("JRnw3hhd9g9dQSWHuUXTh28CEDkLsE5Ewk1Ef1jUaYu"),  // IJR - iShares Core S&P Small-Cap ETF
    pubkey!("LMNSk5AA6gZoJhau2TkzJiMdcHU1bbGgydEsTE8vrNb"),  // ILMN - Illumina, Inc.
    pubkey!("NFQ7NqEETEN7hka1JTSW1NncdeXcqhQFP7LSwK6aWeL"),  // INFQ - Infleqtion, Inc.
    pubkey!("NSMTe8TacDsKgC6TZXifkAiRwJZtazcbZGus5EfpC2b"),  // INSM - Insmed Incorporated
    pubkey!("NTUZKdCwAHYV2z2Eofe9mooiXZu6R481gWP1d3NSWvd"),  // INTU - Intuit Inc.
    pubkey!("NTWVLhY6ke8CJd3N7dCCtiLAuj8pWsMDJwn2HuTuQe7"), // INTW - GraniteShares 2x Long INTC Daily ETF
    pubkey!("NVH3ywrsu51WVUGZQKC84gVQVeX4hQMyfPuddmomcuF"), // INVH - Invitation Homes Inc.
    pubkey!("TLnmWKBdidPsc3HPqj71vQZtdJhTUhSkAmPv1TbLVZP"), // IOT - Samsara Inc.
    pubkey!("PJKijUUh4PHkz8J9vKRguQ1W4HgFprt9kMcui2aLVe3"), // IP - International Paper Company
    pubkey!("QVxz5uMSpecviWXDjBX9wHxEGCFxMjSGngAQKEFcn5H"), // IQV - IQVIA Holdings Inc.
    pubkey!("RzYx9FvE5A1gbbdPCGruxqaB6qvEkj4ZihWnLsJfgZk"), // IR - Ingersoll Rand Inc.
    pubkey!("RDMuRABxZUms4vHtDjpFA8c87zxb9UHdQURd3W6GA4Q"), // IRDM - Iridium Communications Inc.
    pubkey!("RMAEkxmnvwbQyUpVwAteGEedfssfJUqmSe1BKPzeHqA"), // IRM - Iron Mountain Incorporated
    pubkey!("SRGVvrNSz2r7ZiWkdfHFQnuoGJNSPqeHJHVvjUNeTix"), // ISRG - Intuitive Surgical, Inc.
    pubkey!("TS8ECpNTnizsZ2GKSNtT3HTooGm5QRqBKXzU3UYgvnn"), // IT - Gartner, Inc.
    pubkey!("TA4pQRGQV526WvDzzbroEn2yguNVKs1tuGKcAnrZtdz"), // ITA - iShares U.S. Aerospace & Defense ETF
    pubkey!("TW31tDqaWN5aZ9FazhXLkFueUoKFDFgDRh35e64HLjv"), // ITW - Illinois Tool Works Inc.
    pubkey!("WMo2fhRfSg83hTFYUjdTzGXwT9nPNiaJcmME6n2UB92"), // IWM - iShares Russell 2000 ETF
    pubkey!("JAAAXq4G7cGvGvyW6F1HjNDKnREFReptjXxszM7Yf6vM"), // JAAA - Janus Henderson AAA CLO ETF
    pubkey!("JACKYCfpZfPjTMB1xLS6Txh6YNQHZia155CmqeHLStTD"), // JACK - JACK.US
    pubkey!("JAZZyh7vRcUcXW3cGpcBgiDcyapkrBZ4amyrtHdNpFo7"), // JAZZ - Jazz Pharmaceuticals plc
    pubkey!("JBHT9f5F3o5F47i4SjFxxSykY1UDMS1Toz6cjKGKGUzN"), // JBHT - J.B. Hunt Transport Services, Inc.
    pubkey!("JBLPaRhqEjnYB9U5iWEAMzNbQXebb9v8FL6C6NttGsog"), // JBL - Jabil Inc.
    pubkey!("JCLwicU6hXyhWPReVS4hA73pM2qS4rJ1NXnoEXgRGsEi"), // JCI - Johnson Controls International plc
    pubkey!("JDw3eJqkTFisCzPNjQjDJQxHdGVPAk3KZd9QLUXR9W6K"), // JD - JD.com, Inc.
    pubkey!("JPM587siYyvY6uPonGQh98e8mYpBn9hmhYJgZgcHoXd"),  // JPM - JPMorgan Chase & Co.
    pubkey!("JPSTTDs3WxLVpQqRV4F54G3ZX9sGVmsxEy4bFvparKF"), // JPST - JPMorgan Ultra-Short Income ETF
    pubkey!("KDPoRzLY4prZRr7qQ1xFp83PzQBvWzmTDxCS8j33d4g"), // KDP - Keurig Dr Pepper Inc.
    pubkey!("KEY7grvK6hw3dMYwHEQMXFZs3acxGwFyBVYPLYcGFve"), // KEY - KeyCorp
    pubkey!("KEYSLDDAM4jv6SQtBnix7f89UQFMdCMULhw5uxtxkRq"), // KEYS - Keysight Technologies, Inc.
    pubkey!("KHC48yatqRbWR3q2SMZ5vP8TVxTTi5hTAExButVZ5bm"), // KHC - The Kraft Heinz Company
    pubkey!("KKRCSyuJRWL6YyDy2qFYpAhepDeGuck4NBTvwqBqfYP"), // KKR - KKR & Co. Inc.
    pubkey!("KLACAvKJ9h4sYHAjVU3T6GQYobb2MGGkwq5bxRPE2Rp"), // KLAC - KLA Corporation
    pubkey!("KMBZqRw8Kup8bjQtg91wKD99oKqBn7R1VVWhZoP7hHs"), // KMB - Kimberly-Clark Corporation
    pubkey!("KMfmjU22A5Rs6DBfFMiAL8eXURbzZpmExaFCni8FYkr"), // KMI - Kinder Morgan, Inc.
    pubkey!("KNXSxFDKVn8LLLs2GmxkumeUz14xqVLjiFLBcFdBhYc"), // KNX - Knight-Swift Transportation Holdings Inc.
    pubkey!("KkAjEZDUyr8DY7tyWt4bcZDgGa53m9R2zdEh18P7Jb3"), // KO - The Coca-Cola Company
    pubkey!("KQ2MdvVyodfDBjPE5vJxK8BpJcusoN68f2nQmBmuhju"), // KORU - Direxion Daily South Korea Bull 3X Shares
    pubkey!("KRepQi65Qnz8GNd2suWZyZ4na9utxKeyf5yfFfrZ591"), // KR - The Kroger Co.
    pubkey!("KTvwhANy8wc4sZiaGF7nLHhswqbj5C7dsrDB4tTPTyr"), // KTOS - Kratos Defense & Security Solutions, Inc.
    pubkey!("KVUECFXV67wggxHZJX8ftXnaFBktNkFqMrFNSfMNJvy"), // KVUE - Kenvue Inc.
    pubkey!("LENKZCBWZpLvQJPxX4ZKwFEtSMG9CPMhJAvUEoKdoo7"), // LEN - Lennar Corporation
    pubkey!("LHXKhCGfMgTtNAJqAs9a5dE8TYXru6qnJg5nNDtEURS"), // LHX - L3Harris Technologies, Inc.
    pubkey!("LFvT8ySRHjUZWEc1kag77HmRTiUHU2wh9RFG2H4AzyF"), // LII - Lennox International Inc.
    pubkey!("LmYGrbq7E5NWGKwPv2h2acCPNVfdHsDQwNNCaamMVDq"), // LIN - Linde plc
    pubkey!("LXbw9zC8bc9QB4ZX6bingMLQySoYaFS1fhXaYtmfRsA"), // LITE - Lumentum Holdings Inc.
    pubkey!("LNG9hfuAFWjhaxK2AH4Vu6a6hWB3xRT9EX9VnLZqTCo"), // LNG - Cheniere Energy, Inc.
    pubkey!("L2PmVSsJoPNUyEu8jeTd9t3rpAAop3LLyfcGkEuDGy1"), // LOW - Lowe's Companies, Inc.
    pubkey!("LPLA2noj3SfXcw3MM6mPhqCktRfZLdrPkWwmBH8AF2T"), // LPLA - LPL Financial Holdings Inc.
    pubkey!("LRCXrNs8Sg1x3y3CqauV9b2oqNwBmE3ux1SbonHrDRF"), // LRCX - Lam Research Corporation
    pubkey!("LSCCSRrD84hs9VAMGPSACFWcwsWFfDcQXT8PMRZ7Jnr"), // LSCC - Lattice Semiconductor Corporation
    pubkey!("LUNR4W8yKV2MYouHRxNtFzcKvMrf19QJHgAhD4a99T1"), // LUNR - Intuitive Machines, Inc.
    pubkey!("LVSKuHBtNaZPtXqPTAsF452AuvivSPrYxHWzydLg5y2"), // LVS - Las Vegas Sands Corp.
    pubkey!("LYBUMMAhG63Luyw6GLvYG2uA653pXsqFAMxMGoGsrmU"), // LYB - LyondellBasell Industries N.V.
    pubkey!("LYFTawk6kC2hFhcEspCbadPdMAufHkGubW8yA3oryum"), // LYFT - Lyft, Inc.
    pubkey!("LYVS7bCmJRg73e7eC6Evq3t4mDrPcE3jGvtDfHPFceQ"), // LYV - Live Nation Entertainment, Inc.
    pubkey!("MAmmpuFSY2QkdyFFe8YJmZ1rtJTYVx4B3cNCFxsPo2w"), // MA - Mastercard Incorporated
    pubkey!("MARiDmbAdLfCB8w6nwi6m1L51YbUraMVXih5fjsdyCW"), // MAR - Marriott International, Inc.
    pubkey!("MARARYmoJeRq9stLSaSWqj6fAdrBqUUZcoqXNKqoS7o"), // MARA - MARA Holdings, Inc.
    pubkey!("MASCcjK6T1YTWCWUWeBf712t9dAsWXjUjMZfMdupQcV"), // MAS - Masco Corporation
    pubkey!("MCDs354gYmhEm6RzQHpNAvEEFiN6Z5XAEqRshu4QbeU"), // MCD - McDonald's Corporation
    pubkey!("MCHPvisWGxoUNPRvQZAeHTRyqU33MQTSi8qxLdnCG1W"), // MCHP - Microchip Technology Incorporated
    pubkey!("MCKMkx6u6uBEYxCVmKpfgPrTjGJ7eeEBotDVvKmvGuL"), // MCK - McKesson Corporation
    pubkey!("MCUCA9haHvhkaNcHs2DxJvmrsHtJLdbLoubiyaVnFwX"), // MCO - Moody's Corporation
    pubkey!("MDBQdgGWpGDkQgGynvobL3nywmsYnaUAjQZiXeTzQC2"), // MDB - MongoDB, Inc.
    pubkey!("MDLNgWq3oTt8tKmtPmfab8BxJX4NqEKu7sj9yYkzDVj"), // MDLN - Medline Inc.
    pubkey!("MDLZUR4keHiKcH1qptnEtLoD3xJPBXnyKBVwFyamcsp"), // MDLZ - Mondelez International, Inc.
    pubkey!("MDTSkWYqUDXmzgh29yzkNBJBRaAaXJAnPeAtuoWqFd6"), // MDT - Medtronic plc
    pubkey!("MEDPoXghZfhbhbtJhz6TCib56xqM932NwHQXbc9Gt2k"), // MEDP - MEDP.US
    pubkey!("METreiyXJgFD2AdhnQ7zrzVZvUk8mrfT3EA5hderKWg"), // MET - MetLife, Inc.
    pubkey!("METAaH6MMZZgy2CTEJq3QdNi7AFVTV3NxHFBfFgLEDd"), // META - Meta Platforms, Inc.
    pubkey!("MKConb2imNQqeYMQVA1PpHTiP4g31k6HN2kWiWnnxBU"), // MKC - McCormick & Company, Incorporated
    pubkey!("MKS2MnwHGVmxm5eLP8VTHZEP3ba6pULH7tq4LbhCgoA"), // MKSI - MKS Inc.
    pubkey!("MLMoT9ESJSEykb5ghTq8sXhYCrTf5xJVjXSahCbVCQQ"), // MLM - Martin Marietta Materials, Inc.
    pubkey!("MMMaCbUcQHru6SmJgwB8gTKmwBJaBonXJPNDjrUigba"), // MMM - 3M Company
    pubkey!("MNSTy46haEN7sEaeXAWBXG6zHaRikCvqf3krAEDzxr2"), // MNST - Monster Beverage Corporation
    pubkey!("MzfS4jCbihsjBBXktuNNT8HvvE5ZujSTRHuCZNtfpZV"), // MO - Altria Group, Inc.
    pubkey!("MU1wSVLvFgBrSto3J6SeckQk4XfYH2RtjzfNFbMSzVm"), // MOD - Modine Manufacturing Company
    pubkey!("M9U3Y1PjYEdxBwTfRj56EgzbNvsriCrpNhLPSjwPUGu"), // MOH - Molina Healthcare, Inc.
    pubkey!("M1oa3XAaVSnZFTXWWscgdiB3VWNgZ9tagqQMB4H9osK"), // MOO - MOO.US
    pubkey!("M26JCpAEVY4mRM4SeScZV5XmwvAUr4x2y3YTKxYrZ2Q"), // MOS - The Mosaic Company
    pubkey!("MP6TSvyvm3JFhaQFzMacMS663HLvDjodCEuTuHoSVFK"), // MP - MP Materials Corp.
    pubkey!("MPCyzM15cGMU6qshXtte5x5Yz84cP8UGbhExSbGWJ5s"), // MPC - Marathon Petroleum Corporation
    pubkey!("MPWRuECCJp5LniCWpEWvDNu4H17Z7Hj7cbfF5m4MbDG"), // MPWR - Monolithic Power Systems, Inc.
    pubkey!("MRKnuF8xQHCffXJF1ps3iuuSXTt29c8oSEGwR9FpjEU"), // MRK - Merck & Co., Inc.
    pubkey!("MRSHcEr1CgpnseBpQMsdq7tJJrSkDtajhtSpfhh2ssy"), // MRSH - Marsh & McLennan Companies, Inc.
    pubkey!("MSZ5LmnjeSzCFZsXMsiy7RPDYc376vrcZR38BUTkbvc"), // MS - Morgan Stanley
    pubkey!("MSCQyZ5mKPYow8Y3KkS5UMKHdw4vXuR3JU1kJxEh5ep"), // MSCI - MSCI Inc.
    pubkey!("MSFThX899B1DddH5jkyMpdArWXzP85VXLYDrbGGcRJM"), // MSFT - Microsoft Corporation
    pubkey!("MSvo1Cw2QUjHgA9WLBkrg8u1BQpamBUtMmi4dnw6F5D"), // MSI - Motorola Solutions, Inc.
    pubkey!("MSTUsfiHsRhUdgVpicd8uV4xQFr56sDrczSbcVmUpkT"), // MSTU - T-Rex 2X Long MSTR Daily Target ETF
    pubkey!("MTBw91nEbXvmCCmcD1SvUhhgmp1KGq959NDUYzgvNnq"), // MTB - M&T Bank Corporation
    pubkey!("MTDZFWEc5fDCtpSaEDpqjnzYw1Y7nwxb39pK276BSsF"), // MTD - Mettler-Toledo International Inc.
    pubkey!("MTSk2BFA7EahtpRxxHuRuLVhSgz7HDHqQdMvTHFTEr3"), // MTSI - MACOM Technology Solutions Holdings, Inc.
    pubkey!("MTZZsxt3vPfptrfJ7Xfio3F1YyXdUdZRjJpJ8iAF7BL"), // MTZ - MasTec, Inc.
    pubkey!("MUSAuLu55PC1Kbma2ojM4hPBqMHgB4jFCook82U7LWQ"), // MUSA - MUSA.US
    pubkey!("MUUkoQzX89WAbSC46vCPoFT8rxZzB3EB1esR1xR35wj"), // MUU - Direxion Daily MU Bull 2X Shares
    pubkey!("MVLLCBDK8Y3KXV2eSMq1EUykH8LfY96oNd2Nv3J9svT"), // MVLL - GraniteShares 2x Long MRVL Daily ETF
    pubkey!("NBTMubhaX64uzrQPZaVtFUbUqoyf12niNeKXR1v9KSY"), // NBIX - Neurocrine Biosciences, Inc.
    pubkey!("NCLHWXp2QTReWtX7BijGFa1LwKyrHAprdAAxjYEynh8"), // NCLH - Norwegian Cruise Line Holdings Ltd.
    pubkey!("NDAQEvGbyRLMouKatcCApK8jTSkReH679JycJ3LBWym"), // NDAQ - Nasdaq, Inc.
    pubkey!("NEEe84jYQAyPwrb9CBGzr8KKVoamkjLFfd6LcJiDxhh"), // NEE - NextEra Energy, Inc.
    pubkey!("NEMjzgx5ciRu5J3d7WcpPEpAAKXQNKaCwSyukPCcQs4"), // NEM - Newmont Corporation
    pubkey!("NETqruv6aqCknkyHpuMCBiBM3kxkhTEMPqhqNNUWFUe"), // NET - Cloudflare, Inc.
    pubkey!("NsBWtdfYJxEFspiHmn2j59C73Q4QuUYcEVWEWR2bAu5"), // NI - NiSource Inc.
    pubkey!("NLRCB694hJyZCXFxSkozKsHd38MoNoKQ69pfqAtozfc"), // NLR - VanEck Uranium and Nuclear ETF
    pubkey!("NoThuJgtLDUXLBi68N7aDoYWZa4GWu1H7EGHAB52cW3"), // NOC - Northrop Grumman Corporation
    pubkey!("NrMZTkJA9fZYFZeipSUQ5cNruotesSiJTT17wtjhxCz"), // NOW - ServiceNow, Inc.
    pubkey!("NRGW7s6pXTSZRjkuLKnt4qAG2Qp1gqaMvVedp2EmzAC"), // NRG - NRG Energy, Inc.
    pubkey!("NSCwdRoiFFF1JFg5J3f9VuWvJJXaLFRSBqWkMFfYybQ"), // NSC - Norfolk Southern Corporation
    pubkey!("NTAPXuPrr5BzwET9U84HxS8SBpV9JmCNvdBvH1b3kML"), // NTAP - NetApp, Inc.
    pubkey!("NTES5D3VRWi73D1oqEJtubQbdYudPZe6zVPBidvp59d"), // NTES - NetEase, Inc.
    pubkey!("NTNXxCQf5StgdV1mXE8bA3j5siZSEVKuRbjEGFVpuYn"), // NTNX - Nutanix, Inc.
    pubkey!("NTRAq42D8WDtMoyQnHSPXQmeB3M7LQygPdTXi4oUzvx"), // NTRA - Natera, Inc.
    pubkey!("NTRSoozkPdRvoVa8tKgBzUm7psxmrqVN7YynMkJ6R8J"), // NTRS - Northern Trust Corporation
    pubkey!("NUD6HURM6jMmn374iHH1ebVbTdPijN4mLzvz4HFoXyf"), // NU - Nu Holdings Ltd.
    pubkey!("NUEfciH2qh2ie15BMsDSH2wvy5WFGju5ekHViMH6ZuL"), // NUE - Nucor Corporation
    pubkey!("NVDLaNQeLfbutQ7r1sXPjPkyY61qyksebVyFxzLBax3"), // NVDL - GraniteShares 2x Long NVDA Daily ETF
    pubkey!("NVodeeBFRSuk4iuRWisGy5kKdK4bqwHVj8tBy4eQHnd"), // NVO - Novo Nordisk A/S
    pubkey!("NVTaEY6kdbPGpi2SEv9rRxrMrupK8B9dRFUH3gmzUtC"), // NVT - nVent Electric plc
    pubkey!("NXTU9JBY4T8xcPVNuXZgr435KspvckkaxFVym7KBsCw"), // NXT - Nextpower Inc.
    pubkey!("7uYSiKC4z6k7xdoAXPWbs2h8JkFfwgLQVJZn2cgCDTGk"), // O - Realty Income Corporation
    pubkey!("DFL4vbeeRPtkYSwYYNWe2Rag8aZhU6X8LX4eZXcS46C7"), // ODFL - Old Dominion Freight Line, Inc.
    pubkey!("KExmhXPaFL2hnndo8vUGAhcLW4qSNpsMHYHdXQVGdDJ"),  // OKE - ONEOK, Inc.
    pubkey!("KLSpYKGrvVW5FWdUhjJSmwji9bUD18qmadywfj3DYVq"),  // OKLO - Oklo Inc.
    pubkey!("KTAceunndJRvBDEhw3DiwJmZN52rPX7C63RmrD5wpr6"),  // OKTA - Okta, Inc.
    pubkey!("MC6BJVVpjqgUyXAK16sY9HNorhna2MZZfez9R7tjE8g"),  // OMC - Omnicom Group Inc.
    pubkey!("NpJiUNwqmwhqKQiVjoNoKqsvL7LuicZSeqGC9hYYZn8"),  // ON - ON Semiconductor Corporation
    pubkey!("NDSLU35zwhc8wUy4Q4rQuSacDQ2GpFA7wEjfQiTpJKa"),  // ONDS - Ondas Inc.
    pubkey!("NJBYM6hH27ogMXirPT26Kw5oNNhTd8L54SdfriKnFLL"),  // ONON - On Holding AG
    pubkey!("NTJLGP593REWc1dSowKubgE36RTBVcyetAPXNxY4mqv"),  // ONTO - Onto Innovation Inc.
    pubkey!("PENc87HhnvgH49KPGRfRMsLsE364o3WXfqLB8niowwM"),  // OPEN - OPEN.US
    pubkey!("RCLiduhSAkYT5xZAbsx4Bsc9dN46cCG2w1HAEjjA4XT"),  // ORCL - Oracle Corporation
    pubkey!("RLYyUb1qUY1hcgAXxBRzTTGgKbmbmyH9cWMQndGRduR"),  // ORLY - O'Reilly Automotive, Inc.
    pubkey!("SCRmJJCqEhmHEDvX3LYXqwwddvUYxruL9QUkREL7ueS"),  // OSCR - Oscar Health, Inc.
    pubkey!("ThcapK73uVczLs3pCuHgxPyBrrFqU8M6Aj6HSW1sCKL"),  // OTIS - Otis Worldwide Corporation
    pubkey!("USTdE4WtVR6nhhePWH226afbsPYpkfv3t6dsrTnZnnK"),  // OUST - Ouster, Inc.
    pubkey!("VVvkzLu8dGfaX7bDX6WLLmZzHp8r8zLpXh7bBVPGZLW"),  // OVV - Ovintiv Inc.
    pubkey!("WLhuMS1x6LVbCehC9vdSAhux4StqmwJmfcxq15x8Bh1"),  // OWL - Blue Owl Capital Inc.
    pubkey!("XYX8eDbRJvRrfSwsHHMAFkue7Wadh8ZmvYawDtPuULN"), // OXY - Occidental Petroleum Corporation
    pubkey!("Pveif9xyV7oDNwvGvKEAyEMpMbLvHEzvWqb7pZgyrdC"), // P - Everpure, Inc.
    pubkey!("PANWLJWX6gJ9vbKTvSRZ9j9NPnQgNiCvMsNW6WDQaNb"), // PANW - Palo Alto Networks, Inc.
    pubkey!("PATHW4rTvrWBFhWDRX2Sorv971MF6cZUPTND9SBeYMj"), // PATH - UiPath, Inc.
    pubkey!("PAYC4HtreRrC35ofEhs178SkggGAduBqWJVJajBQSY3"), // PAYC - Paycom Software, Inc.
    pubkey!("PAYXon3BiMfLWD2Ru9C1DBv8gww2FZ6CSYLAJRnZq6N"), // PAYX - Paychex, Inc.
    pubkey!("PCARcepxrP8AGAwiPcHda6iaieyN5pGNDHeufsZh4qx"), // PCAR - PACCAR Inc
    pubkey!("PCGCeyvGsVYtfiXyvxYSk6jM2HN7k5FPh3U1anAdFXc"), // PCG - PG&E Corporation
    pubkey!("PCTatfY36uDS5SLANPC4rDSgvQ2ubhkwHHVGKbmTfs1"), // PCT - PureCycle Technologies, Inc.
    pubkey!("PDDRMSDjTn8qyW2yeB2ZSstEMNUGJ6emdQxp7F1Fh92"), // PDD - PDD Holdings Inc.
    pubkey!("PEG2NvDCeKdSsFGZdqy5iicB2auAncQd9GjAEHmzf6z"), // PEG - Public Service Enterprise Group Incorporated
    pubkey!("PEPMV9yEKnQzQ6iiQRKdLhu2RpUq4dCjN6iEYda8QFK"), // PEP - PepsiCo, Inc.
    pubkey!("PGdCducWZnA6B8njfhpK829M2GAtdpyHLtmnSwDEVEB"), // PG - The Procter & Gamble Company
    pubkey!("PGRLGTmrCuA4tvxESM78q8HXV8ZKKE5oco19ZN3TFKB"), // PGR - The Progressive Corporation
    pubkey!("PHHE5dmZVofbxer4HCFnhhph8mS8CBQZUEERNWR4VJD"), // PH - Parker-Hannifin Corporation
    pubkey!("PHMTCxgbYgmczvdUuM6KRNiRaFT4UyeRW9db7AgeDdL"), // PHM - PulteGroup, Inc.
    pubkey!("PoPnAxZCp4gQ9ojt7QEi383sNKBLi5zAoaX6PUAQwNp"), // PINS - Pinterest, Inc.
    pubkey!("PKG6tVhvFvYkLZCros3gXJSdjTEHMgc6LMgnFFHtU6F"), // PKG - Packaging Corporation of America
    pubkey!("PLJJy5t8B5B1ippJJNroYjbpF4ET6pmDkfqg3DeBk25"), // PL - Planet Labs PBC
    pubkey!("PLDQyGCzRmeh5XeCdGvg7viYNTvtPQFc5kJ6gEgQ1Hm"), // PLD - Prologis, Inc.
    pubkey!("PLTRu9F7TDWqMePPuRSopGFm2tQbrYtiJsFpHQJjafF"), // PLTR - Palantir Technologies Inc.
    pubkey!("PMabvcWVCaxEJgaeZHmUktfJCbqg3cCeHJ4QEtm5p6z"), // PM - Philip Morris International Inc.
    pubkey!("PNCUVKEKPi43iQrrDX7gfcq6DLXzW4doCSLicMaawdh"), // PNC - The PNC Financial Services Group, Inc.
    pubkey!("PaMsvi1iVaG4UC8U2YyRBqK9gp84nq2Sb8viXLXdugj"), // PODD - Insulet Corporation
    pubkey!("PeXYgyRwavgpMeLzdgXS5jBcs69GBvDadTUxJhoPASt"), // POET - POET Technologies Inc.
    pubkey!("PJxKuyev9rM62vHqjG5djSMCya2pPKAmxZG47TvhSg1"), // POWL - Powell Industries, Inc.
    pubkey!("PPGxaSDU5aovhXdzf4qrX9FbPsDwBaR7kM9KK1G3zHv"), // PPG - PPG Industries, Inc.
    pubkey!("PPLY2KezLa4ToFcRUXG94P525RrAD51HWkJgJcGCDvR"), // PPL - PPL Corporation
    pubkey!("PRr3RNYipsLdfWw1QGS9UiYq77EQt5PFZjei7TSQSci"), // PR - Permian Resources Corporation
    pubkey!("PRzTEsUC7pZL4PJQpaMBmS7mG16ReU6XWbUmxM7k5Gr"), // PRIM - Primoris Services Corporation
    pubkey!("PRUoUXqcQemdptbxRNxAdjMB1mxc6SsarFK4NN5iRSN"), // PRU - Prudential Financial, Inc.
    pubkey!("PSAXwYjg8P61T3hwXqYibMUUYyKt4YnfqFsZSvhiZfR"), // PSA - Public Storage
    pubkey!("PSXZ8m8Mpka44c5UFcf67dGDcauYwBcW1YMEd644wGy"), // PSX - Phillips 66
    pubkey!("PTCNMbJtGNimfRpGxmLex6km2brqwRjrq2qvcEMy7Y5"), // PTC - PTC Inc.
    pubkey!("PURRaGKXwD9rdZw851PnRDBMuDGopxrDGCM6GMHA9vh"), // PURR - PURR.US
    pubkey!("PWRE8MVduHzDfnnX57Sb4sUhNXRLuhyHJuaRThZ1f25"), // PWR - Quanta Services, Inc.
    pubkey!("PYPLP4YSiv8iqcuegh8XQ9grcemmJ9AgdawLsEXE3yc"), // PYPL - PayPal Holdings, Inc.
    pubkey!("QDhGwpuE8Q68iC12PbUGwoobj8gMFctV9pBCwr3bbwQ"), // Q - Qnity Electronics, Inc.
    pubkey!("QCtnKv4x6GPQhCUQuS1UxC4hP181LCDRBF73188MjLG"), // QCOM - QUALCOMM Incorporated
    pubkey!("QNT15PA1Kewqn6Jb1XiQwYktN9CasGtEn1srRCH51Gs"), // QNT - Quantinuum Inc
    pubkey!("QQQvDYxG7g11Rr4rnaJqDddmP9ocH74YhE3mDaG37R9"), // QQQ - Invesco QQQ Trust
    pubkey!("QSR4hzrgEkxjBrXsHFnRjzEzWh9CvhiCbHPCZ7FWJRF"), // QSR - Restaurant Brands International Inc.
    pubkey!("QUREqiwgHYwJiwyKrrxbhbqRPAU67cnXuC3y4yRv9tg"), // QURE - QURE.US
    pubkey!("QXu5zgHtSPQ2wrnigA8wTMeGZsVsKDXZJRzK3DA3Rz3"), // QXO - QXO, Inc.
    pubkey!("QYLDChCRUh9heR4hcgTKrpbrwe6VCBvKjNW5SsfLUQq"), // QYLD - Global X NASDAQ 100 Covered Call ETF
    pubkey!("RBRKJ7ju175WiveHwgisNxUQfgift3ZyMcMQ7WADeq9"), // RBRK - Rubrik, Inc.
    pubkey!("RCATPtK9PUzriFsjcJ42za8fSnt9CYd2aSB2AL6VByE"), // RCAT - Red Cat Holdings, Inc.
    pubkey!("RCLmSoeotojmzv8JwuH11XDvTS3S5jyRYfGiv7z38FQ"), // RCL - Royal Caribbean Cruises Ltd.
    pubkey!("RDWC5ArQfeKGKpY1FmS83c8WJxAeCz2E7v9RsdWzEMZ"), // RDW - Redwire Corporation
    pubkey!("REGNoMaVC4JTrrfni4zijJ8JGRWgro53oTua9fF7BR6"), // REGN - Regeneron Pharmaceuticals, Inc.
    pubkey!("RF2yR5AQhu4TS3juaxrN6y3615KnpESwXVE4e6SoZsG"), // RF - Regions Financial Corporation
    pubkey!("RGLDrbCR3kraMaZprw7MMzC2dPKHnEGuAtPA7GevEz8"), // RGLD - Royal Gold, Inc.
    pubkey!("RHXPJaGuTyms8g1vZHLZdyHyTBbEPR4iU9meCwRkXTq"), // RH - RH
    pubkey!("RUKx7nBgm94pwRPbVm99xYVdcTDTUyGs8Cw3VSYQXHf"), // RIOT - Riot Platforms, Inc.
    pubkey!("RJF41LLxz5MTGKXZAXUz49Yv65dZ4qUa5r8pTZSTWKH"), // RJF - Raymond James Financial, Inc.
    pubkey!("RKT7c4cStgcqabcQeyhYW5MRRjLfpChrEmU8ThQqqES"), // RKT - Rocket Companies, Inc.
    pubkey!("RLruzMMApPLBWAPDrHAmKsMeB8E1BD5HVAXBcbPzgWn"), // RL - Ralph Lauren Corporation
    pubkey!("RMBSujGaSNqGXHQtmpx98GoS9UnZgLD3mH5Msr2jRLa"), // RMBS - Rambus Inc.
    pubkey!("RMDtt9PdcVvJyfG1ytm2Lyc98c3WjtJ41QJ9uoBciZt"), // RMD - ResMed Inc.
    pubkey!("R37nFjJR3M1xi8ZaNRjDSRJ5BZgS4yxxNTzyqu35edq"), // ROIV - Roivant Sciences Ltd.
    pubkey!("RueeNcDYpdUQdQjGyKQsASbfd1znfmbZPcSx4XWdmeM"), // ROK - Rockwell Automation, Inc.
    pubkey!("RzHk5KnX5rFGNFPfHcxFi4enFgWejoUFG7UhcFfJfNY"), // ROKU - Roku, Inc.
    pubkey!("RsogRpzGQHeiWn4Mu6B17fJmcdVsMyw2VuLEQBbLd1L"), // ROP - Roper Technologies, Inc.
    pubkey!("R7Gjt1uaezTy7VMiPnbka64W38qpeYSLChug8PeaZo5"), // ROST - Ross Stores, Inc.
    pubkey!("RPRXDfNcZFY34EwmuS42YmpasL9Dg5DcorSuEfNPEks"), // RPRX - Royalty Pharma plc
    pubkey!("RRXR7yQHybAYW6emauq1CkbyhffKBYEpfCpvPAXLD3g"), // RRX - Regal Rexnord Corporation
    pubkey!("RSGDYYWX9RaJvoTJkaBxfwGAxxb9GW2shEifxx3mfWu"), // RSG - Republic Services, Inc.
    pubkey!("RTX6nQJSxdgCzmtUGMvK9is32e72yWnpL64os3akquV"), // RTX - RTX Corporation
    pubkey!("RVMDctWuKJUuHXpvpdTwrGo3LQZmYgWGfQsamJH5JFb"), // RVMD - Revolution Medicines, Inc.
    pubkey!("RXRXH6f3QEanL6LdLV13qpQ1om9oMc1JK7Fb6zJx49y"), // RXRX - RXRX.US
    pubkey!("RXTH4HNXXQjgfq11ZUnhdJr3Xg6ATZqoKMa67une1sU"), // RXT - Rackspace Technology, Inc.
    pubkey!("SAnqAzptC3eyEniPkdHkD1xJtqwpT36hX6U9KKR734X"), // SAIA - SAIA.US
    pubkey!("SAoPkNsdgkbwFE6ZPXujXDgawk6BRrCHcJwspMCA25x"), // SAIL - SailPoint, Inc.
    pubkey!("SANM2TPCh47JXnMThhnCFjWGMvpygi8eiZKo7KqPhoZ"), // SANM - Sanmina Corporation
    pubkey!("SBACjKeebuHgE4HxF1RUNSjNscVMJt9JUFYZvsTeFKa"), // SBAC - SBA Communications Corporation
    pubkey!("SBUXW9ENmLyWzihyLhUKnMbB1e7VvKPbphMiRFd57HQ"), // SBUX - Starbucks Corporation
    pubkey!("SCHFukTo7gVoJupU9ZDi4KY9ahEnpcnCKSz1zk59tL4"), // SCHF - Schwab International Equity ETF
    pubkey!("SCHW6oYwN6cfzk8t6DTjBH9wyTjEm2hjpm21ioGh2HX"), // SCHW - The Charles Schwab Corporation
    pubkey!("SECZyuGYsbkdpR6GLwNdDe3fNK8TFnqVwisnxj17Eg8"), // SECZ - Securitize
    pubkey!("SFMvRppMeQ1dEM5y824GTHQJbh8YWCs7G9ab5fooRhQ"), // SFM - Sprouts Farmers Market, Inc.
    pubkey!("SG3qmdBFoak5YwxH4CSmnPdyKDJQDkEosg5zin16PzA"), // SGI - Somnigroup International Inc.
    pubkey!("SGdhbWCK8UsE8DV3caknkQ7Lskkk5XmEsxVssHautPZ"), // SGOV - iShares 0-3 Month Treasury Bond ETF
    pubkey!("SHLDAhv7SYKtd2MPNbahLEXRr2475mTYocHxuf2hNRR"), // SHLD - Global X Defense Tech ETF
    pubkey!("SHW4wKvryuBeg996Fwfs6iFA12tmDKw4UDBJo346MSp"), // SHW - The Sherwin-Williams Company
    pubkey!("Sv3JpDxYv7sXa5AAQ1dbjcCCWzaJ66Xc5DbvgpQLtc5"), // SIMO - Silicon Motion Technology Corporation
    pubkey!("SueLbt57GdcaYheVMsRSYnCuSoCzpCZqyesDk8DraBg"), // SITM - SiTime Corporation
    pubkey!("SJMhWTBtCbjArxSPuy3oFR1JoMtBv7BzfiUvVFxvKBS"), // SJM - The J. M. Smucker Company
    pubkey!("SLBi3yDGZKrDNReJz2qSrASn3z67MaD6FBXhwrWseVg"), // SLB - SLB N.V.
    pubkey!("SLV6P4x3wPdstF1eDtBzGAXXHrEGjkcJVqvVxssqSKb"), // SLV - iShares Silver Trust
    pubkey!("SMCS8aokEhXZwz8KH1K2XAos7C4cn21eS6XTMhrYokz"), // SMCI - Super Micro Computer, Inc.
    pubkey!("SMH8eZrNZWrvQVJsFVNKGoRPPXJVdqFeBGffSqC6sve"), // SMH - VanEck Semiconductor ETF
    pubkey!("SMRJ2emvr4ZwPCuScrNENLMEcCRGG9JCyRtFeBxoPxd"), // SMR - NuScale Power Corporation
    pubkey!("SMTCNWhJ87b4JRmvVpUs28ktwsTszW13Ru5rC9iq97Z"), // SMTC - Semtech Corporation
    pubkey!("SNUkmy4MatTzNrhTxs94juuCDWSmxrdWo2d3UyvCLnn"), // SN - SharkNinja, Inc.
    pubkey!("SNfgYznvRyg5ZpbVUedbtuunXNH6S8ZqnT7mtSpyWBQ"), // SNOW - Snowflake Inc.
    pubkey!("SNPSe4KwWBiWjTrrcyDicvcgv9byzMSgezCb2cmEDKq"), // SNPS - Synopsys, Inc.
    pubkey!("SNX9JrTZbwjFLveJfcy41RjdugxSxR4jhqk1B7Cic6y"), // SNX - TD SYNNEX Corporation
    pubkey!("SNXXj12GyPcrki1uMTwcCu3KMbELrJ8oy1VeLgsbmQd"), // SNXX - Tradr 2X Long Sndk Daily ETF
    pubkey!("Sd89jJuWFfoGuJSPaa5Kz8syGNLDQ3GnaB9FVLdxv4J"), // SO - The Southern Company
    pubkey!("StRuUEQNAzj58atZ6sFu1KdPfJfmFBYvtDHh4QyQQKC"), // SOFI - SoFi Technologies, Inc.
    pubkey!("Sr8qU29bKPj1rgQGBN55Lgdap2argzBEQiaX2cdoUND"), // SOLS - Solstice Advanced Materials, Inc.
    pubkey!("SjYvzdx9bfoU1aQ1nbrV196H3QN7QzV9cHvXefs7og9"), // SONY - Sony Group Corporation
    pubkey!("Sf1PPwvhweuPgn8xuazddA98WoTKuXH2HKM3BkAqkre"), // SOUN - SoundHound AI, Inc.
    pubkey!("SoXLRsBe4uEwGPqqXsyaWJe8ecTpf5YMYtFgxwRU88n"), // SOXL - Direxion Daily Semiconductor Bull 3X Shares
    pubkey!("SoXSQS1y1d8UFUhJVr1WU9yUH5wnmok39GfRjASTnT4"), // SOXS - Direxion Daily Semiconductor Bear 3X Shares
    pubkey!("S6CuKP89TzYjWBs5Pocdxxi9Ei2DCd6obFELX6mGbLW"), // SOXX - iShares Semiconductor ETF
    pubkey!("SPCHvYz3NUyhawnvvLQbvARRW4sSjziFh7Tkmcw3h1b"), // SPCH - Leverage Shares 2X Long SPCX Daily ETF
    pubkey!("SPGaWibqhaBaqyPL2jRMZ63FK9YaJuxtTJxximyYeQm"), // SPG - Simon Property Group, Inc.
    pubkey!("SPGtKa6npLDg3hVoftGU2ktopFbNoWEkijAobcCBEZz"), // SPGI - S&P Global Inc.
    pubkey!("SPcu3qgqvbdV3x1CCM8kYp1gnSVTsrqyAYUAndhQwGw"), // SPOT - Spotify Technology S.A.
    pubkey!("SPYBo66VJPFjh1pXMb9Le53kDYWTK1zzYVDeVRWtsbi"), // SPY - State Street SPDR S&P 500 ETF Trust
    pubkey!("SREXRYRE3fcGydgLWRePKPRSLTxqbgwRaTSRPPYADjf"), // SRE - Sempra
    pubkey!("SSNCrVqnf1iQywT8MJ6M6p4CRSYASReFzdsM3z6eFq3"), // SSNC - SS&C Technologies Holdings, Inc.
    pubkey!("STEp67mM6Lt2451X31m58V1BtMcbrAgSEopzGosBFCX"), // STE - STERIS plc
    pubkey!("STLDkM9Rg3N56oZKSFzHAdgbX6n9Q11sNGjPMiEaeJV"), // STLD - Steel Dynamics, Inc.
    pubkey!("STRCfnmWSabEJzhJdT1zNYv1Aub82Zf1JqeyzCHCsPT"), // STRC - Strategy Inc
    pubkey!("STRLH2B1grNqwh1yPYJEmPoKzUeYK4Do8LHPnVDyvXk"), // STRL - Sterling Infrastructure, Inc.
    pubkey!("STT7o5FkME5SPBZU7b8Se48Ds6Tpdk13FQXSqgdQ7y9"), // STT - State Street Corporation
    pubkey!("STZHq1cQFV4WTHKDgyYiAWEr9V624RStJQcMThsWQHE"), // STZ - Constellation Brands, Inc.
    pubkey!("SUNB2Cbrbjvou3xpt9XtuGFCG9DkuzPu7soJsjF1Lgr"), // SUNB - Sunbelt Rentals Holdings, Inc.
    pubkey!("SWQ1oFkfLcysYdN91nMgK1nDzKitDk8ZvtRiHbBD5Rm"), // SW - Smurfit Westrock Plc
    pubkey!("SWKS1bb4KtB5pinZhzdvjSocqVxkeDXjHS8qwrLtcxH"), // SWKS - Skyworks Solutions, Inc.
    pubkey!("SYFSwd7KNWoy3pqBdNpP5CYyw3WueumSKskf3Y8EJke"), // SYF - Synchrony Financial
    pubkey!("SYKgL8nmGwuy54shU2vv2L3m6e4Q3BcszuQHCQA43PU"), // SYK - Stryker Corporation
    pubkey!("SYYtPcpPNdGt4GAqJi2tkEYhe5VyvouiRYVhC34ot3P"), // SYY - Sysco Corporation
    pubkey!("T48Qsk7MjNGaWXoTmuVSGJXYMmNuwrQ39hvyhw1nyzt"), // T - AT&T Inc.
    pubkey!("TDGksqtFvAniTtNHNior2ng3nB4HKcnzZ2eW9gCZmEc"), // TDG - TransDigm Group Incorporated
    pubkey!("TDYESqixLJVfzwYhVeUfjZvzwkhmqSPtiSQKcU4yCCK"), // TDY - Teledyne Technologies Incorporated
    pubkey!("TEjYQDnZawoyU4v46erPrfKTpxaCLZygMWgi61tGUfW"), // TE - TE.US
    pubkey!("TEAMK7bJG8NwisFKArs3LmodpTbBeVX5vQeYsabXcC6"), // TEAM - Atlassian Corporation
    pubkey!("TEMNmrsRhBgnYTnBTQB2hKNEULBotLW7ABiZDvr7up8"), // TEM - Tempus AI, Inc.
    pubkey!("TERWTMBnhLcC24sAzbhkexa6frY4FGJyTV2XAs1mSpZ"), // TER - Teradyne, Inc.
    pubkey!("TFCSaD8BnGnqKmyyqRbhtBRjPkXvk2Vr6zcxhBMtugg"), // TFC - Truist Financial Corporation
    pubkey!("TGTz2uEPMxMAp8kKMmtajQU527Vig7zKARWKzfNubbW"), // TGT - Target Corporation
    pubkey!("THCwHQKWZ68zBQZ5iY9zyEZsC2Lh3xxwDmm2vTfiA7u"), // THC - Tenet Healthcare Corporation
    pubkey!("TJXX9ZB8LPJJ384jr7gDmQxd8TWT8MmvE6dqhobtyK2"), // TJX - The TJX Companies, Inc.
    pubkey!("TKVC45Wshi9sjunhyTAyg7xHRVCSHoSe4AebGiFDkDC"), // TKO - TKO Group Holdings, Inc.
    pubkey!("TLNHN6sY71Mhcp5EKakmqnrbn9DWrJxeVhAhyqNfkZP"), // TLN - Talen Energy Corporation
    pubkey!("TLTf5WJrXuTJwtr2hFVC53b4nxy9oKiCxSdnLpCcS3G"), // TLT - iShares 20+ Year Treasury Bond ETF
    pubkey!("TMRsDh4kiN7LaVpdLhuhZzxkqaLPFbiPTEVs8aqqKMe"), // TMO - Thermo Fisher Scientific Inc.
    pubkey!("TMUSkRNxZapSQDjsrveHpiubGrXbnN3xDSDePhFuxDH"), // TMUS - T-Mobile US, Inc.
    pubkey!("TNGX8De17kaUiQ7FrVhgw1uC4kmCDR4BDYexjQzur7d"), // TNGX - Tango Therapeutics, Inc.
    pubkey!("T3iZ29JpENnPBZb5EvAEsh6vXBLwV3wLzWxneJ3A2Am"), // TOST - Toast, Inc.
    pubkey!("TPL8Ja7TEiaVVBEPuRQkJptpCe7ovsEcTJ6TMvs921L"), // TPL - Texas Pacific Land Corporation
    pubkey!("TPRhfuVdo1c4Scm2GuH9iTWBY85z89481hUnEFXLVFv"), // TPR - Tapestry, Inc.
    pubkey!("TQQQ1wNmXcwz21YK1mVBU3h8dPZ8LFi6DkEVskYxhgr"), // TQQQ - ProShares UltraPro QQQ
    pubkey!("TRGPEyX4uVDNAMFC3vG1uJS3th8yPSEnk7bRH6wLxNP"), // TRGP - Targa Resources Corp.
    pubkey!("TRsoUm2kGhv6pdkjm8CQBdHJju74e5ZBiUg9S3cXdFH"), // TROW - T. Rowe Price Group, Inc.
    pubkey!("TRVmmE95TT14vNUbngFBxcEcLi1HqtjwFNcfCVcZwdy"), // TRV - The Travelers Companies, Inc.
    pubkey!("TSCMboDKx2HnafH1dbmDPEjHVEgptvyLQNVgkTSphbE"), // TSCO - Tractor Supply Company
    pubkey!("TSEMtoh9M9UbHbR9brAcWaecZTadaCQjL9Du6okP4EM"), // TSEM - Tower Semiconductor Ltd.
    pubkey!("TSLAqBbv4CNCnzWFeB7LmydAyEiNMJtve7DYKLpdK4S"), // TSLA - Tesla, Inc.
    pubkey!("TSLLCJVZim9yaccKzy7eTaiZic3wUh9ExeYMtuCLfmx"), // TSLL - Direxion Daily TSLA Bull 2X Shares
    pubkey!("TSM2Hiuf7Z9REcwZFQvsBEekBYCYziWBABfFnj6V6gQ"), // TSM - Taiwan Semiconductor Manufacturing Company Limited
    pubkey!("TSNEL6GjtPWpppcGo4KYTW62PP44tKJH3Wi5t88Jryj"), // TSN - Tyson Foods, Inc.
    pubkey!("TTUXsreSvMYFQMzcoguUgNZ5g1f4epZZ8jJufb4mfJp"), // TT - Trane Technologies plc
    pubkey!("TTDHDNUg8TQ4vXGtuQJ2M3QhBRwVCphQWEHshLUo8kj"), // TTD - The Trade Desk, Inc.
    pubkey!("TTMdriwECnWSEcb7Hq9aZskMFRa6YJWT5CHrQrm6aQX"), // TTMI - TTM Technologies, Inc.
    pubkey!("TWL6zHbKcAXsNVd4rywYBNzBTSUMbZZ2neB5MJNTD7C"), // TWLO - Twilio Inc.
    pubkey!("TWSTM8Ecrbp4CaD78Bthyp7WWwovi9oKSR3MEXTyQn3"), // TWST - TWST.US
    pubkey!("TXNTEfKYtgfZSEf9uPCXGhsSe4iCqH27Jq5VwCzFLnj"), // TXN - Texas Instruments Incorporated
    pubkey!("TXRHzebkrGdrs9B6vM3PS8WaoTsSyVboG3igJHiKhxJ"), // TXRH - Texas Roadhouse, Inc.
    pubkey!("TYLfk2wRefZiGJuKzZqzURfJiGQKLSnnKrpCUGV6Dj2"), // TYL - Tyler Technologies, Inc.
    pubkey!("UiizsMX2fMb3Dds1L25hPpA4EEdEqMKwYWT2BZvu3Mg"), // U - Unity Software Inc.
    pubkey!("UALvvtm12XLqJns1qDHTD97XjiGJB8DfvgdmLvhW2jd"), // UAL - United Airlines Holdings, Inc.
    pubkey!("UBERfYx74rcUzBgr4pFaqgTBaS2AmejqQBX7oQupSFH"), // UBER - Uber Technologies, Inc.
    pubkey!("UUox82PLZ2Ggnkr1u9io9TnhRLQzik1NvSjFeSDFP9X"), // UI - UI.US
    pubkey!("ULTAxwNX7gdoJTg1B8ZY6518S6K3Uei81bsa4e5pyFD"), // ULTA - Ulta Beauty, Inc.
    pubkey!("UNH43aWWHQYq8G516u8hMVrH7KgrQ92Mk4vvm6yTfeV"), // UNH - UnitedHealth Group Incorporated
    pubkey!("UNPFVZq1WS2Aj6FB4VeP2uJYHj3ucJE4PG499xWxcXE"), // UNP - Union Pacific Corporation
    pubkey!("URiLXsXhv5yJaovZfavb4U6ammERcYYcaJQ1V4GwaBw"), // URI - United Rentals, Inc.
    pubkey!("URNMeMYERPfe9f72js1fHBShyRoLMkNiu2F4JPLffYv"), // URNM - Sprott Uranium Miners ETF
    pubkey!("USARDXfMvukvAWkhyC8Ymeo1H38zqnf7Q7VbtgJsBAT"), // USAR - USA Rare Earth, Inc.
    pubkey!("USBBXGk6ePs8EvntoHEdEoARuU1cDaH3qHXNkySGNZ7"), // USB - U.S. Bancorp
    pubkey!("USFDB5LM1ZZ6B9pSqbGbvi4Zs8w8RDsMvzoHRpHiLP5"), // USFD - US Foods Holding Corp.
    pubkey!("USFRKRSGNdATemuyyM4ywicecanJ43GsxWVhBcHSK87"), // USFR - WisdomTree Floating Rate Treasury Fund
    pubkey!("UTHRcUCdzG1RvtGsjyx4QJ3GHynTmSCCkBfS6hy4paX"), // UTHR - United Therapeutics Corporation
    pubkey!("UVXY4ojmAMTaPwUmrRmcuxWHjMHCbz3BxX2RBMzjYKx"), // UVXY - ProShares Ultra VIX Short-Term Futures ETF
    pubkey!("VjMYaf4KNiVCZsqEWzEtZBjSjRtVFEdgViewHQsDLnm"), // V - Visa Inc.
    pubkey!("VEEVNr8spyF8Yrvyk93CT5UHvkVBLGcaUrtAe5nQPkv"), // VEEV - Veeva Systems Inc.
    pubkey!("VELzqfybZtWqZ6t2XBpdMcTX8xKfwksgWJGsQrgim4Q"), // VELO - Velo3D, Inc.
    pubkey!("VGKJZPhZPTDRsrNh6CEepF9EUUdcQTdxrdDNTARtBbR"), // VGK - Vanguard FTSE Europe ETF
    pubkey!("Vfu63DxPevp42AH7U8eNyjqZUmQaPYTKHw7W27S46QV"), // VIAV - Viavi Solutions Inc.
    pubkey!("V5moim8hw6go7faj4TgYjn25JdaVzwXEssUaA9id81k"), // VICI - VICI Properties Inc.
    pubkey!("VU4KTnVUY47iqg6WWLiQbfNMeq8fAmFXNhsgUKQmnwc"), // VICR - Vicor Corporation
    pubkey!("VJBDa92zFbJNvF29dbou8cLn7oWJB1tdgeE2c8Hcsw4"), // VIK - Viking Holdings Ltd
    pubkey!("VLdVo49YFRLPn2YQYi4cKgKAJAnwjkJvse7kvMja7yz"), // VLO - Valero Energy Corporation
    pubkey!("VLTZXWQyiCuJMdf6Lwyqx8dkGYvrbVQ4TFeU2ivrfhD"), // VLTO - Veralto Corporation
    pubkey!("VMCNH64SRvLYTU3B1g3CWN5QaPWDQTyuWfx1nNtyoGA"), // VMC - Vulcan Materials Company
    pubkey!("Vv7EU4aoHNY6wFsFRUyXhGm2syKXBA2T6v2MZUZZo7E"), // VOO - Vanguard S&P 500 ETF
    pubkey!("VRSKgGjW4mCmH8ymTDYKG45xk6nRJdTQ6HLBfvkU5bP"), // VRSK - Verisk Analytics, Inc.
    pubkey!("VRSNXJiD6ffDkkt57yU4xSDcPxx3y4DmyjTHiFdTDhA"), // VRSN - VeriSign, Inc.
    pubkey!("VRTM8EB4P4MzMSj4BFK96P5BJxUB2b6LhQkPC2DjyCr"), // VRT - Vertiv Holdings Co
    pubkey!("VRTXPZ4kTYt7pRgkrwGEaK1N4NwKrgSzjjGd9JvVZyo"), // VRTX - Vertex Pharmaceuticals Incorporated
    pubkey!("VST744yvWsHsE7oLLj1x6rcogMvC9HiPbEVpaURqdRR"), // VST - Vistra Corp.
    pubkey!("VTyNWN9etB6PatBSg8ECF4PPr1HWBrb93tKMZaFkcf1"), // VT - Vanguard Total World Stock Index Fund ETF Shares
    pubkey!("VTs4SQUeefk93Lnfv7cAo3eo1pTQBq5niNDN7XuCMuE"), // VTI - Vanguard Total Stock Market Index Fund ETF Shares
    pubkey!("VTRfD64DsGk5Gzj3zPK3W3Giekqgxxedo9rBGVm7a4K"), // VTR - Ventas, Inc.
    pubkey!("VTRSQTidirassGwhq8QMeG4Kn8mQM8KLmwgJ58DoKP2"), // VTRS - Viatris Inc.
    pubkey!("VUGVKcKRfkEENKUc4ULEBngt1Dan4jrbPvC42KC8hFH"), // VUG - Vanguard Growth Index Fund ETF Shares
    pubkey!("VXUSpvu8iYbzFTJsufcN3kJ3bfwhiLe23TWPQB5AEA1"), // VXUS - Vanguard Total International Stock Index Fund ETF Shares
    pubkey!("VZz4S1h3psx7EZgrGTvJBPXgLYCMi47Qp4mnGcsxzys"), // VZ - Verizon Communications Inc.
    pubkey!("Wf8EmjnPVuczfsBLgB8viKk7upS5LxzZwtzj6i4LmZT"), // W - Wayfair Inc.
    pubkey!("WABzbPAP25yfK3tVezsvYmyWxWwRqu1K2NdLZLrJYBn"), // WAB - Westinghouse Air Brake Technologies Corporation
    pubkey!("WATrCzUqJPszw1t8G6S58txb4wo4YXgNodzn4S4xyNV"), // WAT - Waters Corporation
    pubkey!("WBDnbJXKrzVEBan2h4EyxJGiPLCK7qru6AZS21sizyv"), // WBD - Warner Bros. Discovery, Inc.
    pubkey!("WCCywne54TwgiQVKmiS3obovYuSGeaK5N5STzp9vrxr"), // WCC - WESCO International, Inc.
    pubkey!("WDAYPgyTpQKbGgKjgUtjtHCZLr9f1PYEx6EtVgYD5y8"), // WDAY - Workday, Inc.
    pubkey!("WDCAeonhdKy1STyAxvcFfmWsZknQTGnMPCz2Cw8tJzh"), // WDC - Western Digital Corporation
    pubkey!("WEC6FPzvPFYAtbpNcNNpfGJV2yKRVg6jJj6e5yfNtdz"), // WEC - WEC Energy Group, Inc.
    pubkey!("WELLHxRrKAhKE2pdHW85yLSbep4QeFduZYq1G9ZefPi"), // WELL - Welltower Inc.
    pubkey!("WFCNYPqA96ubSfLeTNWc9H2smZ2UygXS98YDjSLc5ez"), // WFC - Wells Fargo & Company
    pubkey!("WQLq6hKm1CjMdaP7zcd82xwibiVmPXdQvHmvCjbzsix"), // WING - Wingstop Inc.
    pubkey!("WMgE6RuicZ7paVvCrhtvYZ1H6rS22v9cmsDkEstSjjL"), // WM - Waste Management, Inc.
    pubkey!("WMBrvAcNBrFwApA444xmzKYSbT7npn7hLEDzbAvbdMA"), // WMB - The Williams Companies, Inc.
    pubkey!("WMTBkVDAn6Ngiq5XRzf1VBF43SkELuz5cSDrcZph18p"), // WMT - Walmart Inc.
    pubkey!("WSMtshuHzRRokNppki6r6JwS9CXSojbd8UzauZe9s9m"), // WSM - Williams-Sonoma, Inc.
    pubkey!("WSD8pYcFCMjFGkw2uPZj3RMLJWoPoeuhww7u575eJg5"), // WSO - WSO.US
    pubkey!("WSTCEUZ917TS3NgDgjuB34caS3gcasu5YNkVpcZzzEp"), // WST - West Pharmaceutical Services, Inc.
    pubkey!("WTWUsm2gLoMmPYwV8MPyK8KQPVi3rCL93bq3EUtuT3n"), // WTW - Willis Towers Watson Public Limited Company
    pubkey!("WWDcb6zgGWgonLNUayouH5CD7CU3AayFRyBGkRsbKbL"), // WWD - Woodward, Inc.
    pubkey!("WYFbpGaP3Kob3ffafjC5WPnqUKiZ5qECBmjTedLbh1L"), // WYFI - WYFI.US
    pubkey!("XBWFoNZrZ7YXnqxov2vXrnr6hZkYxpwpcP6Q67uDiuN"), // XBI - State Street SPDR S&P Biotech ETF
    pubkey!("XEPhvYWwrtH3f2WqirCtEBSS3DtjUy1pf99LhRiPSHL"), // XE - XE.US
    pubkey!("XELyakgZyEKXwmbSZiLAJ7F4NyFvGdUEJx76PsBG4P9"), // XEL - Xcel Energy Inc.
    pubkey!("XLETprc6d7MC6J9krNUj8gx16wpdkyJqjGQN1KPEixm"), // XLE - State Street Energy Select Sector SPDR ETF
    pubkey!("XnM2BArr2rpRJd8LvCt9dobUNUdrhVhjnPFMTPrJy45"), // XOM - Exxon Mobil Corporation
    pubkey!("XbFXpjPVeXYkDEY9UfhStkeVB1oyt2dPEpGWwFD92az"), // XOP - State Street SPDR S&P Oil & Gas Exploration & Production ETF
    pubkey!("XPeAf6dbLp9TdqmiGE9vs89G9XYRbrx1jYG7bm7KiC9"), // XPO - XPO, Inc.
    pubkey!("XYLQX7ySwfEYtfuqPhnKRvLnePLExzgBLiKEeemTFf9"), // XYL - Xylem Inc.
    pubkey!("YUMcsryaEugMCPVBSTgbwauY596ZPNz9JRtjraGLkp7"), // YUM - Yum! Brands, Inc.
    pubkey!("ZBHYqsojw7KV6vgTqdjW9QPBtcMqCtLWFHS8PzMYYJC"), // ZBH - Zimmer Biomet Holdings, Inc.
    pubkey!("ZBRAZb8NV6db5WEq1z99d6kMxevYekvc42x5iDP8ndb"), // ZBRA - Zebra Technologies Corporation
    pubkey!("ZMrnzqPBpgZ7j7d2pWDLwZHE5e5v2K26kqbx1Rnw3Mn"), // ZM - Zoom Communications, Inc.
    pubkey!("ZSqWgSzWr8UeixUcQM9is8WEMxY9hqLCN2NZ5sk8rgd"), // ZS - Zscaler, Inc.
    pubkey!("ZTSx8dAWAyLCn3gmq6KKidqrzyxS1kSotHoG15NQEst"), // ZTS - Zoetis Inc.
];

/// Check if the mint is supported for trade
pub fn is_supported_mint(mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
    let mint_info = mint_account.to_account_info();
    if *mint_info.owner == Token::id() {
        return Ok(true);
    }
    if MINT_WHITELIST.contains(&mint_account.key()) {
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
