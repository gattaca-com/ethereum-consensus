use crate::{
    deneb::mainnet::{BYTES_PER_BLOB, KZG_COMMITMENT_INCLUSION_PROOF_DEPTH},
    electra::spec,
    phase0::mainnet::MAX_COMMITTEES_PER_SLOT,
};

pub use crate::{
    capella::presets::mainnet::{
        BYTES_PER_LOGS_BLOOM, EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR,
        ETH1_DATA_VOTES_BOUND, HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS,
        MAX_BLS_TO_EXECUTION_CHANGES, MAX_BYTES_PER_TRANSACTION, MAX_DEPOSITS,
        MAX_EXTRA_DATA_BYTES, MAX_PROPOSER_SLASHINGS, MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_VALIDATORS_PER_COMMITTEE, MAX_VOLUNTARY_EXITS, MAX_WITHDRAWALS_PER_PAYLOAD,
        SLOTS_PER_HISTORICAL_ROOT, SYNC_COMMITTEE_SIZE, VALIDATOR_REGISTRY_LIMIT,
    },
    deneb::presets::mainnet::MAX_BLOB_COMMITMENTS_PER_BLOCK,
    electra::presets::Preset,
};

pub use spec::*;

pub const MIN_ACTIVATION_BALANCE: Gwei = 32 * 10u64.pow(9);
pub const MAX_EFFECTIVE_BALANCE_ELECTRA: Gwei = 2048 * 10u64.pow(9);
pub const MIN_SLASHING_PENALTY_QUOTIENT_ELECTRA: u64 = 4096;
pub const WHISTLEBLOWER_REWARD_QUOTIENT_ELECTRA: u64 = 4096;
pub const PENDING_DEPOSITS_LIMIT: usize = 2usize.pow(27);
pub const PENDING_PARTIAL_WITHDRAWALS_LIMIT: usize = 2usize.pow(27);
pub const PENDING_CONSOLIDATIONS_LIMIT: usize = 2usize.pow(18);
pub const MAX_ATTESTER_SLASHINGS_ELECTRA: usize = 1;
pub const MAX_ATTESTATIONS_ELECTRA: usize = 8;
pub const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize = 2;
pub const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize = 8192;
pub const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize = 16;
pub const MAX_PENDING_PARTIALS_PER_WITHDRAWALS_SWEEP: usize = 8;
pub const MAX_VALIDATORS_PER_SLOT: usize = 131072;

pub const PRESET: Preset = Preset {
    min_activation_balance: MIN_ACTIVATION_BALANCE,
    max_effective_balance_electra: MAX_EFFECTIVE_BALANCE_ELECTRA,
    min_slashing_penalty_quotient_electra: MIN_SLASHING_PENALTY_QUOTIENT_ELECTRA,
    whistleblower_reward_quotient_electra: WHISTLEBLOWER_REWARD_QUOTIENT_ELECTRA,
    pending_deposits_limit: PENDING_DEPOSITS_LIMIT,
    pending_partial_withdrawals_limit: PENDING_PARTIAL_WITHDRAWALS_LIMIT,
    pending_consolidations_limit: PENDING_CONSOLIDATIONS_LIMIT,
    max_attester_slashings_electra: MAX_ATTESTER_SLASHINGS_ELECTRA,
    max_attestations_electra: MAX_ATTESTATIONS_ELECTRA,
    max_consolidation_requests_per_payload: MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    max_deposit_requests_per_payload: MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    max_withdrawal_requests_per_payload: MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    max_pending_partials_per_withdrawals_sweep: MAX_PENDING_PARTIALS_PER_WITHDRAWALS_SWEEP,
};

pub type ExecutionPayload = spec::ExecutionPayload<
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
>;

pub type ExecutionPayloadHeader =
    spec::ExecutionPayloadHeader<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>;

pub type BlindedBeaconBlockBody = spec::BlindedBeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_COMMITTEES_PER_SLOT,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_VALIDATORS_PER_SLOT,
>;

pub type BlindedBeaconBlock = spec::BlindedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_COMMITTEES_PER_SLOT,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_VALIDATORS_PER_SLOT,
>;

pub type SignedBlindedBeaconBlock = spec::SignedBlindedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_COMMITTEES_PER_SLOT,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_VALIDATORS_PER_SLOT,
>;

pub type BeaconState = spec::BeaconState<
    SLOTS_PER_HISTORICAL_ROOT,
    HISTORICAL_ROOTS_LIMIT,
    ETH1_DATA_VOTES_BOUND,
    VALIDATOR_REGISTRY_LIMIT,
    EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR,
    MAX_VALIDATORS_PER_COMMITTEE,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    PENDING_DEPOSITS_LIMIT,
    PENDING_PARTIAL_WITHDRAWALS_LIMIT,
    PENDING_CONSOLIDATIONS_LIMIT,
>;

pub type BeaconBlockBody = spec::BeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_COMMITTEES_PER_SLOT,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_VALIDATORS_PER_SLOT,
>;

pub type BeaconBlock = spec::BeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_COMMITTEES_PER_SLOT,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_VALIDATORS_PER_SLOT,
>;

pub type SignedBeaconBlock = spec::SignedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_COMMITTEES_PER_SLOT,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_VALIDATORS_PER_SLOT,
>;

pub type Blob = spec::Blob<BYTES_PER_BLOB>;
pub type BlobSidecar = spec::BlobSidecar<BYTES_PER_BLOB, KZG_COMMITMENT_INCLUSION_PROOF_DEPTH>;
pub type BlobsBundle = spec::BlobsBundle<BYTES_PER_BLOB>;

pub type LightClientHeader = spec::LightClientHeader<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>;
pub type LightClientBootstrap =
    spec::LightClientBootstrap<SYNC_COMMITTEE_SIZE, BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>;
pub type LightClientUpdate =
    spec::LightClientUpdate<SYNC_COMMITTEE_SIZE, BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>;
pub type LightClientFinalityUpdate = spec::LightClientFinalityUpdate<
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
>;
pub type LightClientOptimisticUpdate = spec::LightClientOptimisticUpdate<
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
>;
