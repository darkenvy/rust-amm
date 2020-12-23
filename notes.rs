// impl ConstantProductMarketMaker {
//   fn trade(&mut self, x: f64, y: f64, desired_token: usize, estimate_only: bool) -> f64 {
//     let state: [f64; 2] = [
//       self.x,
//       self.y,
//     ];
//     let mut next_state: [f64; 2] = [
//       self.x + x,
//       self.y + y,
//     ];

//     next_state[desired_token] = self.invariant / next_state[(desired_token + 1) % 2];
//     let sending_token_amount: f64 = state[desired_token] - next_state[desired_token];

//     // If in get_estimate mode, quit.
//     if estimate_only {
//       return sending_token_amount;
//     }

//     // ensure invariance
//     if next_state[0] * next_state[1] != self.invariant {
//       println!("Invariant doesn't match! {}", next_state[0] * next_state[1]);
//       return 0.0;  
//     }

//     // if contract.transfer is reducing the amount, will have to factor in. Cant double-reduce
//     self.x = next_state[0];
//     self.y = next_state[1];
//     // contract.transfer({ to: '', tokens: sending_token_amount });

//     sending_token_amount
//   }

//   fn get_estimate(&mut self, x: f64, y: f64, desired_token: usize) -> f64 {
//     self.trade(x, y, desired_token, true)
//   }

//   fn add_liquidity(&mut self, x: f64, y: f64) {
//     self.x = self.x + x;
//     self.y = self.y + y;
//     self.invariant = self.x * self.y;
//   }

//   fn print(&mut self) {
//     println!("+---- CPMM ----");
//     println!("x: {}", self.x);
//     println!("y: {}", self.y);
//     println!("invariant: {}", self.invariant);
//     println!("+--------------");
//   }
// }