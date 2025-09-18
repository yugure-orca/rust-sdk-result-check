use orca_whirlpools_client::{DynamicTickArray, FixedTickArray, Oracle, TickArray, Whirlpool, DYNAMIC_TICK_ARRAY_DISCRIMINATOR, FIXED_TICK_ARRAY_DISCRIMINATOR, ORACLE_DISCRIMINATOR, WHIRLPOOL_DISCRIMINATOR};
use orca_whirlpools_core::{swap_quote_by_input_token, TickArrays};

// SDK validity check for the following transaction:
// https://solscan.io/tx/4CKncoVgd3Gu6c9RiQqTiYRvi2zovPuAJZeaqpcetxbeRUiGKifQo2TyDKyJwerd3BywKx62txJzNzmJMAcRXhzr

// account state at the end of slot 365795240
// There is no transaction touching these pools between the end of slot 365795240 and the target transaction.
const END_OF_SLOT_365795240_ACCOUNT_DATA: &str = include_str!("eos365795240.dump.csv");
// block time of the next slot 365795241
const SLOT_365795241_BLOCK_TIME: u64 = 1757467270;

type AccountMap = std::collections::HashMap<String, Vec<u8>>;

fn main() {
    // csv into HashMap<String, Vec<u8>>
    let account_data: AccountMap = END_OF_SLOT_365795240_ACCOUNT_DATA
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ',');
            let pubkey = parts.next()?.trim().to_string();
            let b64data = parts.next()?.trim();
            let bytes = base64::decode(b64data).ok()?;
            Some((pubkey, bytes))
        })
        .collect();

    // pool#1 4HppGTweoGQ8ZZ6UcCgwJKfi5mJD9Dqwy6htCpnbfBLW
    let pool1 = whirlpool(&account_data, "4HppGTweoGQ8ZZ6UcCgwJKfi5mJD9Dqwy6htCpnbfBLW");
    let pool1_ta0 = tick_array(&account_data, "AdZ4qVAhjsN9r3QefbBLAmGBrNwpLMxYoaGTTvNzCJcB");
    let pool1_ta1 = tick_array(&account_data, "EJm5rgH6tTxTbF6JYV8PnqETSbMkXrvdqT4RqXcSjqeP");
    let pool1_ta2 = tick_array(&account_data, "6dy5p76UVHMD2Uj49rsAfd69Lg2K3eoaeJhYNZc2fYUJ");
    let pool1_oracle = oracle(&account_data, "31HfnCJfkAmdiAqXTRPVCPo9uDXLRstpK7tU2kk5zYB7");
    println!("pool1: ts={} fee={} tick={}, sp={}, liq={}", pool1.tick_spacing, pool1.fee_rate, pool1.tick_current_index, pool1.sqrt_price, pool1.liquidity);

    // pool#2 4VGetbiGroMGmFsqedF6t6PyYXXk9mzwxV8b7BSb3s9T
    let pool2 = whirlpool(&account_data, "4VGetbiGroMGmFsqedF6t6PyYXXk9mzwxV8b7BSb3s9T");
    let pool2_ta0 = tick_array(&account_data, "DnrVTkm5bceNdP54VNB82CxqozztCUh6oCEGRELLcWmp");
    let pool2_ta1 = tick_array(&account_data, "Bu9pPx5Av9JHJDF8VVcTtdrmXipbk7fphXmAdJB3CFeD");
    let pool2_ta2 = tick_array(&account_data, "A1Q48PG9WxipKzytjV2yhCGDAdCWQqgfHTnyc7dhcSf8");
    //let pool2_oracle = oracle(&account_data, "E3rmxqR1dLgo56NM9KzL6VApTWwSncjNiCsixH1kxgmn");
    println!("pool2: ts={} fee={} tick={}, sp={}, liq={}", pool2.tick_spacing, pool2.fee_rate, pool2.tick_current_index, pool2.sqrt_price, pool2.liquidity);

    // pool#3 2nJjE2ba3iGtefN4UNM1KN5FdYCwssU4Bzhc8URceqh6
    let pool3 = whirlpool(&account_data, "2nJjE2ba3iGtefN4UNM1KN5FdYCwssU4Bzhc8URceqh6");
    let pool3_ta0 = tick_array(&account_data, "8p1JGkGbFqMjXjUs7jvNS5n979JMVcHG49rGjFbS29TJ");
    let pool3_ta1 = tick_array(&account_data, "HUTdVj2dkCnSUUSuob54U1Y6JDFNK9eidWxULjbAxv9e");
    let pool3_ta2 = tick_array(&account_data, "FQWPc9jWvzf5V8LBTFFhpKcLud7p7cq2cxSycjdsKR45");
    //let pool3_oracle = oracle(&account_data, "HQdJ9Fuk3RLBJ3EFBEKMC7hhJ3L9QKcWGhwSbQoJA3QT");
    println!("pool3: ts={} fee={} tick={}, sp={}, liq={}", pool3.tick_spacing, pool3.fee_rate, pool3.tick_current_index, pool3.sqrt_price, pool3.liquidity);

    // pool#4 Hp53XEtt4S8SvPCXarsLSdGfZBuUr5mMmZmX2DRNXQKp
    let pool4 = whirlpool(&account_data, "Hp53XEtt4S8SvPCXarsLSdGfZBuUr5mMmZmX2DRNXQKp");
    let pool4_ta0 = tick_array(&account_data, "7nZrcnwtxqGeSsYgyaTZrwrwDFEe39CVwxcGgZhBjgLa");
    let pool4_ta1 = tick_array(&account_data, "H92dp72NGuaN1DX4i3yyGKZHYCZqW7ay54vSp2VN4tDo");
    let pool4_ta2 = tick_array(&account_data, "DL3jYPYmoEDmapAFHKWg4cFak1AGDGDhRtTHvgAGMmZB");
    //let pool4_oracle = oracle(&account_data, "");
    println!("pool4: ts={} fee={} tick={}, sp={}, liq={}", pool4.tick_spacing, pool4.fee_rate, pool4.tick_current_index, pool4.sqrt_price, pool4.liquidity);

    let slippage_tolerance_bps = 0u16;

    // swap on pool#1
    let amount = 20000000u64;
    let a_to_b = true;
    let pool1_quote = swap_quote_by_input_token(
        amount,
        a_to_b,
        slippage_tolerance_bps,
        pool1.into(),
        Some(pool1_oracle.into()),
        TickArrays::Three(
            pool1_ta0.into(),
            pool1_ta1.into(),
            pool1_ta2.into(),
        ),
        SLOT_365795241_BLOCK_TIME,
        None,
        None,
    ).expect("failed to get swap quote on pool#1");
    println!("pool#1 quote: {:#?}", pool1_quote);

    // swap on pool#2
    let amount = 4325437u64;
    let a_to_b = false;
    let pool2_quote = swap_quote_by_input_token(
        amount,
        a_to_b,
        slippage_tolerance_bps,
        pool2.into(),
        None,
        TickArrays::Three(
            pool2_ta0.into(),
            pool2_ta1.into(),
            pool2_ta2.into(),
        ),
        SLOT_365795241_BLOCK_TIME,
        None,
        None,
    ).expect("failed to get swap quote on pool#2");
    println!("pool#2 quote: {:#?}", pool2_quote);

    // swap on pool#3
    let amount = 76594545u64;
    let a_to_b = true;
    let pool3_quote = swap_quote_by_input_token(
        amount,
        a_to_b,
        slippage_tolerance_bps,
        pool3.into(),
        None,
        TickArrays::Three(
            pool3_ta0.into(),
            pool3_ta1.into(),
            pool3_ta2.into(),
        ),
        SLOT_365795241_BLOCK_TIME,
        None,
        None,
    ).expect("failed to get swap quote on pool#3");
    println!("pool#3 quote: {:#?}", pool3_quote);

    // swap on pool#4
    let amount = 16155704u64;
    let a_to_b = false;
    let pool4_quote = swap_quote_by_input_token(
        amount,
        a_to_b,
        slippage_tolerance_bps,
        pool4.into(),
        None,
        TickArrays::Three(
            pool4_ta0.into(),
            pool4_ta1.into(),
            pool4_ta2.into(),
        ),
        SLOT_365795241_BLOCK_TIME,
        None,
        None,
    ).expect("failed to get swap quote on pool#4");
    println!("pool#4 quote: {:#?}", pool4_quote);
}

fn whirlpool(account_data: &AccountMap, address: &str) -> Whirlpool {
    let data = account_data.get(address).expect("Whirlpool account not found");
    assert_eq!(&data[0..8], WHIRLPOOL_DISCRIMINATOR);
    Whirlpool::from_bytes(data).expect("failed to parse Whirlpool account data")
}

fn tick_array(account_data: &AccountMap, address: &str) -> TickArray {
    let data = account_data.get(address).expect("TickArray account not found");
    match &data[0..8] {
        FIXED_TICK_ARRAY_DISCRIMINATOR => {
            TickArray::FixedTickArray(FixedTickArray::from_bytes(data).expect("failed to parse FixedTickArray"))
        },
        DYNAMIC_TICK_ARRAY_DISCRIMINATOR => {
            TickArray::DynamicTickArray(DynamicTickArray::from_bytes(data).expect("failed to parse DynamicTickArray"))
        },
        _ => panic!("unknown TickArray discriminator"),
    }
}

fn oracle(account_data: &AccountMap, address: &str) -> Oracle {
    let data = account_data.get(address).expect("Oracle account not found");
    assert_eq!(&data[0..8], ORACLE_DISCRIMINATOR);
    Oracle::from_bytes(data).expect("failed to parse Oracle account data")
}

/*

OUTPUT

pool1: ts=4 fee=400 tick=-15309, sp=8580620861527422605, liq=17043360106242
pool2: ts=128 fee=10000 tick=-28845, sp=4361122162976020549, liq=71362182707
pool3: ts=8 fee=500 tick=-15559, sp=8474104210733383005, liq=5272818749069
pool4: ts=1 fee=100 tick=-2062, sp=16640347035777708922, liq=141520545218031008
pool#1 quote: ExactInSwapQuote {
    token_in: 20000000,
    token_est_out: 4325437,
    token_min_out: 4325437,
    trade_fee: 9100,
    trade_fee_rate_min: 455,
    trade_fee_rate_max: 455,
}
pool#2 quote: ExactInSwapQuote {
    token_in: 4325437,
    token_est_out: 76594545,
    token_min_out: 76594545,
    trade_fee: 43255,
    trade_fee_rate_min: 10000,
    trade_fee_rate_max: 10000,
}
pool#3 quote: ExactInSwapQuote {
    token_in: 76594545,
    token_est_out: 16155704,
    token_min_out: 16155704,
    trade_fee: 38298,
    trade_fee_rate_min: 500,
    trade_fee_rate_max: 500,
}
pool#4 quote: ExactInSwapQuote {
    token_in: 16155704,
    token_est_out: 19851673,
    token_min_out: 19851673,
    trade_fee: 1616,
    trade_fee_rate_min: 100,
    trade_fee_rate_max: 100,
}

*/
