use md5;

pub struct Miner {
  secret: String
}

impl Miner {
  pub fn new(secret: &str) -> Self {
    Miner{
      secret: secret.to_string()
    }
  }

  pub fn mine(&self, prefix: &str) -> u64 {
    let secret = self.secret.clone();
    let mut iter = 0_u64;

    loop {
      let input = format!("{secret}{iter}");
      let digest = md5::compute(input.as_bytes());
      let hex = format!("{:x}", digest);

      if hex.starts_with(prefix) {
        return iter;
      } else {
        iter = iter + 1;
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_mine_five_zeroes() {
    let miner = Miner::new("abcdef");
    let solution = miner.mine("00000");
    assert_eq!(solution, 609043);

    let miner = Miner::new("pqrstuv");
    let solution = miner.mine("00000");
    assert_eq!(solution, 1048970);
  }
}
