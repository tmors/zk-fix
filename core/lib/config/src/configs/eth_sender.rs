// Built-in uses
use std::time::Duration;
// External uses
use serde::Deserialize;
// Workspace uses
use zksync_basic_types::{Address, H256};
// Local uses
use crate::envy_load;

/// Configuration for the Ethereum sender crate.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ETHSenderConfig {
    /// Options related to the Ethereum sender directly.
    pub sender: SenderConfig,
    /// Options related to the `GasAdjuster` submodule.
    pub gas_adjuster: GasAdjusterConfig,
}

impl ETHSenderConfig {
    pub fn from_env() -> Self {
        Self {
            sender: envy_load!("eth_sender", "ETH_SENDER_SENDER_"),
            gas_adjuster: envy_load!("eth_sender.gas_adjuster", "ETH_SENDER_GAS_ADJUSTER_"),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum ProofSendingMode {
    OnlyRealProofs,
    OnlySampledProofs,
    SkipEveryProof,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SenderConfig {
    pub aggregated_proof_sizes: Vec<usize>,
    /// Private key of the operator account.
    pub operator_private_key: H256,
    /// Address of the operator account.
    pub operator_commit_eth_addr: Address,
    /// mount of confirmations required to consider L1 transaction committed.
    pub wait_confirmations: u64,
    /// Node polling period in seconds.
    pub tx_poll_period: u64,
    /// Aggregate txs polling period in seconds.
    pub aggregate_tx_poll_period: u64,
    /// The maximum number of unconfirmed Ethereum transactions.
    pub max_txs_in_flight: u64,
    /// The mode in which proofs are sent.
    pub proof_sending_mode: ProofSendingMode,

    pub max_aggregated_tx_gas: u32,
    pub max_eth_tx_data_size: usize,
    pub max_aggregated_blocks_to_commit: u32,
    pub max_aggregated_blocks_to_execute: u32,
    pub aggregated_block_commit_deadline: u64,
    pub aggregated_block_prove_deadline: u64,
    pub aggregated_block_execute_deadline: u64,
    pub timestamp_criteria_max_allowed_lag: usize,

    /// L1 batches will only be executed on L1 contract after they are at least this number of seconds old.
    /// Note that this number must be slightly higher than the one set on the contract,
    /// because the contract uses block.timestamp which lags behind the clock time.
    pub l1_batch_min_age_before_execute_seconds: Option<u64>,
}

impl SenderConfig {
    /// Converts `self.tx_poll_period` into `Duration`.
    pub fn tx_poll_period(&self) -> Duration {
        Duration::from_secs(self.tx_poll_period)
    }
    /// Converts `self.aggregate_tx_poll_period` into `Duration`.
    pub fn aggregate_tx_poll_period(&self) -> Duration {
        Duration::from_secs(self.aggregate_tx_poll_period)
    }
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct GasAdjusterConfig {
    /// Priority Fee to be used by GasAdjuster
    pub default_priority_fee_per_gas: u64,
    /// Number of blocks collected by GasAdjuster from which base_fee median is taken
    pub max_base_fee_samples: usize,
    /// Parameter of the transaction base_fee_per_gas pricing formula
    pub pricing_formula_parameter_a: f64,
    /// Parameter of the transaction base_fee_per_gas pricing formula
    pub pricing_formula_parameter_b: f64,
    /// Parameter by which the base fee will be multiplied for internal purposes
    pub internal_l1_pricing_multiplier: f64,
    /// If equal to Some(x), then it will always provide `x` as the L1 gas price
    pub internal_enforced_l1_gas_price: Option<u64>,
    /// Node polling period in seconds
    pub poll_period: u64,
}

impl GasAdjusterConfig {
    /// Converts `self.poll_period` into `Duration`.
    pub fn poll_period(&self) -> Duration {
        Duration::from_secs(self.poll_period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::test_utils::{addr, hash, set_env};

    fn expected_config() -> ETHSenderConfig {
        ETHSenderConfig {
            sender: SenderConfig {
                aggregated_proof_sizes: vec![1, 5],
                aggregated_block_commit_deadline: 30,
                aggregated_block_prove_deadline: 3_000,
                aggregated_block_execute_deadline: 4_000,
                max_aggregated_tx_gas: 4_000_000,
                max_eth_tx_data_size: 120_000,

                timestamp_criteria_max_allowed_lag: 30,
                max_aggregated_blocks_to_commit: 3,
                max_aggregated_blocks_to_execute: 4,
                wait_confirmations: 1,
                tx_poll_period: 3,
                aggregate_tx_poll_period: 3,
                max_txs_in_flight: 3,
                operator_private_key: hash(
                    "27593fea79697e947890ecbecce7901b0008345e5d7259710d0dd5e500d040be",
                ),
                operator_commit_eth_addr: addr("de03a0B5963f75f1C8485B355fF6D30f3093BDE7"),
                proof_sending_mode: ProofSendingMode::SkipEveryProof,
                l1_batch_min_age_before_execute_seconds: Some(1000),
            },
            gas_adjuster: GasAdjusterConfig {
                default_priority_fee_per_gas: 20000000000,
                max_base_fee_samples: 10000,
                pricing_formula_parameter_a: 1.5,
                pricing_formula_parameter_b: 1.0005,
                internal_l1_pricing_multiplier: 0.8,
                internal_enforced_l1_gas_price: None,
                poll_period: 15,
            },
        }
    }

    #[test]
    fn from_env() {
        let config = r#"
ETH_SENDER_SENDER_WAIT_CONFIRMATIONS="1"
ETH_SENDER_SENDER_TX_POLL_PERIOD="3"
ETH_SENDER_SENDER_AGGREGATE_TX_POLL_PERIOD="3"
ETH_SENDER_SENDER_MAX_TXS_IN_FLIGHT="3"
ETH_SENDER_SENDER_OPERATOR_PRIVATE_KEY="0x27593fea79697e947890ecbecce7901b0008345e5d7259710d0dd5e500d040be"
ETH_SENDER_SENDER_OPERATOR_COMMIT_ETH_ADDR="0xde03a0B5963f75f1C8485B355fF6D30f3093BDE7"
ETH_SENDER_SENDER_PROOF_SENDING_MODE="SkipEveryProof"
ETH_SENDER_GAS_ADJUSTER_DEFAULT_PRIORITY_FEE_PER_GAS="20000000000"
ETH_SENDER_GAS_ADJUSTER_MAX_BASE_FEE_SAMPLES="10000"
ETH_SENDER_GAS_ADJUSTER_PRICING_FORMULA_PARAMETER_A="1.5"
ETH_SENDER_GAS_ADJUSTER_PRICING_FORMULA_PARAMETER_B="1.0005"
ETH_SENDER_GAS_ADJUSTER_INTERNAL_L1_PRICING_MULTIPLIER="0.8"
ETH_SENDER_GAS_ADJUSTER_POLL_PERIOD="15"
ETH_SENDER_WAIT_FOR_PROOFS="false"
ETH_SENDER_SENDER_AGGREGATED_PROOF_SIZES="1,5"
ETH_SENDER_SENDER_MAX_AGGREGATED_BLOCKS_TO_COMMIT="3"
ETH_SENDER_SENDER_MAX_AGGREGATED_BLOCKS_TO_EXECUTE="4"
ETH_SENDER_SENDER_AGGREGATED_BLOCK_COMMIT_DEADLINE="30"
ETH_SENDER_SENDER_AGGREGATED_BLOCK_PROVE_DEADLINE="3000"
ETH_SENDER_SENDER_AGGREGATED_BLOCK_EXECUTE_DEADLINE="4000"
ETH_SENDER_SENDER_TIMESTAMP_CRITERIA_MAX_ALLOWED_LAG="30"
ETH_SENDER_SENDER_MAX_AGGREGATED_TX_GAS="4000000"
ETH_SENDER_SENDER_MAX_ETH_TX_DATA_SIZE="120000"
ETH_SENDER_SENDER_L1_BATCH_MIN_AGE_BEFORE_EXECUTE_SECONDS="1000"
        "#;
        set_env(config);

        let actual = ETHSenderConfig::from_env();
        assert_eq!(actual, expected_config());
    }

    /// Checks the correctness of the config helper methods.
    #[test]
    fn methods() {
        let config = expected_config();

        assert_eq!(
            config.sender.tx_poll_period(),
            Duration::from_secs(config.sender.tx_poll_period)
        );
    }
}
