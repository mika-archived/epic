use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeHistory {
    #[serde(alias = "oldestBlock")]
    pub oldest_block: String,

    #[serde(alias = "baseFeePerGas")]
    pub base_fee_per_gas: Vec<String>,

    #[serde(alias = "gasUsedRatio")]
    pub gas_used_ratio: Vec<f64>,

    #[serde(alias = "reward")]
    pub reward: Option<Vec<Vec<String>>>,
}

impl std::fmt::Display for FeeHistory {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        println!("Oldest Block: {}", self.oldest_block);
        println!("Base fee per Gas:");

        for (i, fee) in self.base_fee_per_gas.iter().enumerate() {
            let fee = u128::from_str_radix(&fee[2..], 16).unwrap();
            println!("    {}: {}", i, fee);
        }

        println!("Gas used ratio:");
        for (i, ratio) in self.gas_used_ratio.iter().enumerate() {
            let ratio = format!("{:.2}", ratio * 100.0);
            println!("    {}: {}", i, ratio);
        }

        if self.reward.is_some() {
            println!("Reward:");

            for (i, reward) in self.reward.as_ref().unwrap().iter().enumerate() {
                let reward1 = u128::from_str_radix(&reward.get(0).unwrap()[2..], 16).unwrap();
                let reward2 = u128::from_str_radix(&reward.get(1).unwrap()[2..], 16).unwrap();

                println!("    {}: {}, {}", i, reward1, reward2);
            }
        }

        Ok(())
    }
}
