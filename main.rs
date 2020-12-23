use std::collections::HashMap;

struct Pool {
  tokens: [u128; 2],
  invariant: u128,
  liquidity_providers: HashMap<String, u128>, // redo
}

impl Default for Pool {
  fn default() -> Pool {
    Pool {
      tokens: [0, 0],
      invariant: 0,
      liquidity_providers: HashMap::new(),
    }
  }
}

impl Pool {
  fn add_provider(&mut self, public_key: &str, lp_tokens: u128) {
    self.liquidity_providers.insert(public_key.to_string(), lp_tokens);
  }

  fn print(&self) {
    println!("  tokens: [{}], [{}]", self.tokens[0], self.tokens[1]);
    println!("  Invariant: {}", self.invariant);
    for (public_key, lp_tokens) in self.liquidity_providers.iter() {
      println!("  liquidity_provider: {} = {}", public_key, lp_tokens);
    }
  }
}

// -------------------------------------------------------------------------- //

struct ConstantProductMarketMaker {
  pools: HashMap<String, Pool>
}

impl Default for ConstantProductMarketMaker {
  fn default() -> ConstantProductMarketMaker {
    ConstantProductMarketMaker {
      pools: HashMap::new()
    }
  }
}

impl ConstantProductMarketMaker {
  fn create_pool(&mut self, pool_name: &str) {
    let pool: Pool = Pool{ ..Default::default() };
    self.pools.insert(pool_name.to_string(), pool);
  }

  fn join_pool(&mut self, pool_name: &str, adding_tokens: [u128; 2], sender_public_key: &str) {
    // increment how much the LP provided
    // self.pools.get(&pool_name).unwrap().join();
    let pool = self.pools.get_mut(pool_name).unwrap();
    let lp_tokens: u128 = adding_tokens[0] + adding_tokens[1];
    
    pool.tokens[0] = pool.tokens[0] + adding_tokens[0];
    pool.tokens[1] = pool.tokens[1] + adding_tokens[1];
    pool.invariant = (pool.tokens[0] * pool.tokens[1]) as u128;
    pool.add_provider(sender_public_key, lp_tokens);
    
    // contract.transfer({ to: sender_public_key, token: `${pool_name}-lp-token`, amount: x * y });
  }

  fn getApproximateSpotPrice(&mut self, pool_name: &str, offerToken: usize, offerAmount: u128, desiredToken: usize) -> u128 {
    let pool = self.pools.get_mut(pool_name).unwrap();
    let mut nextState: [u128; 2] = [
      pool.tokens[0],
      pool.tokens[1],
    ];

    nextState[offerToken] = nextState[offerToken] + offerAmount;
    nextState[desiredToken] = pool.invariant / nextState[offerToken];
    
    let estimate = pool.tokens[desiredToken] - nextState[desiredToken];
    return estimate;
  }

  fn swap(&mut self, pool_name: &str, offerToken: usize, offerAmount: u128, desiredToken: usize) {
    let pool = self.pools.get_mut(pool_name).unwrap();
    let mut nextState: [u128; 2] = [
      pool.tokens[0],
      pool.tokens[1],
    ];

    // calculate next state
    // todo: when converting to u256, multiply by 10 or 100, to reduce increase accuracy towards float math.println!
    // eg: 10/3*3 == 9. But if we multiply by 10, it will be 99, which can be treated as a 1% change in invariance.
    nextState[offerToken] = nextState[offerToken] + offerAmount;
    nextState[desiredToken] = pool.invariant / nextState[offerToken];

    // check to see if the invariant changed in the next state
    // note: we dont actually change the invariant on swaps, but the effective invariant can be calculated
    if pool.invariant / nextState[offerToken] * nextState[offerToken] != pool.invariant {
      println!("Change in invariant! Invalid swap");
      return;
    }

    // apply next state
    pool.tokens[0] = nextState[0];
    pool.tokens[1] = nextState[1];
    
    let outcome_tokens_to_user = pool.tokens[desiredToken] - nextState[desiredToken];
    // contract.transfer({ to: sender_public_key, token: `${pool_name}-outcome-${desiredToken}`, amount: outcome_tokens_to_user });
  }

  fn print(&mut self) {
    println!("+---- CPMM ----");

    for (poolName, pool) in self.pools.iter() {
      println!("Pool Name: '{}'", poolName);
      pool.print();
    }

    println!("+--------------");
  }
}

// -------------------------------------------------------------------------- //

fn test_estimate_only() {
  const USER_PUBLIC_KEY: &str = "000000001";
  const MARKET_NAME: &str = "Yes/No";

  let mut cpmm: ConstantProductMarketMaker = ConstantProductMarketMaker{ ..Default::default() };
  cpmm.create_pool(MARKET_NAME);
  cpmm.join_pool(MARKET_NAME, [10, 10], USER_PUBLIC_KEY);
  
  let estimate = cpmm.getApproximateSpotPrice(MARKET_NAME, 1, 20, 0);
  println!("estimate: {}", estimate);
}

fn test_two_swaps_low_liquidity() {
  const USER_PUBLIC_KEY: &str = "000000001";
  const MARKET_NAME: &str = "Yes/No";

  let mut cpmm: ConstantProductMarketMaker = ConstantProductMarketMaker{ ..Default::default() };
  cpmm.create_pool(MARKET_NAME);
  cpmm.join_pool(MARKET_NAME, [10, 10], USER_PUBLIC_KEY);

  cpmm.swap(MARKET_NAME, 1, 10, 0);
  cpmm.print();

  cpmm.swap(MARKET_NAME, 1, 10, 0);
  cpmm.print();
}

fn test_two_swaps_high_liquidity() {
  const USER_PUBLIC_KEY: &str = "000000001";
  const MARKET_NAME: &str = "Yes/No";

  let mut cpmm: ConstantProductMarketMaker = ConstantProductMarketMaker{ ..Default::default() };
  cpmm.create_pool(MARKET_NAME);
  cpmm.join_pool(MARKET_NAME, [10_000, 10_000], USER_PUBLIC_KEY);

  cpmm.swap(MARKET_NAME, 1, 10, 0);
  cpmm.print();

  cpmm.swap(MARKET_NAME, 1, 10, 0);
  cpmm.print();
}

fn test_swaps_with_changing_liquidity() {
  const USER_PUBLIC_KEY: &str = "000000001";
  const USER_PUBLIC_KEY_2: &str = "000000002";
  const MARKET_NAME: &str = "Yes/No";

  let mut cpmm: ConstantProductMarketMaker = ConstantProductMarketMaker{ ..Default::default() };
  cpmm.create_pool(MARKET_NAME);
  cpmm.join_pool(MARKET_NAME, [5_000, 5_000], USER_PUBLIC_KEY);

  cpmm.swap(MARKET_NAME, 1, 10, 0);
  cpmm.print();

  cpmm.join_pool(MARKET_NAME, [2_500, 2_500], USER_PUBLIC_KEY_2);

  cpmm.swap(MARKET_NAME, 1, 10, 0);
  cpmm.print();
}

fn main() {
  // test_estimate_only();
  test_two_swaps_low_liquidity();
  // test_two_swaps_high_liquidity();
  // test_swaps_with_changing_liquidity();
}
