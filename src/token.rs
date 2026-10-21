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

const MINT_WHITELIST: [Pubkey; 146] = [
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
