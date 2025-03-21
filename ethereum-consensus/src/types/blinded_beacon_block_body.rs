//! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
use crate::{
    altair::SyncAggregate,
    bellatrix::blinded_beacon_block as bellatrix,
    capella::{blinded_beacon_block as capella, SignedBlsToExecutionChange},
    crypto::KzgCommitment,
    deneb::blinded_beacon_block as deneb,
    electra::{blinded_beacon_block as electra, ExecutionRequests},
    phase0::{Deposit, Eth1Data, ProposerSlashing, SignedVoluntaryExit},
    primitives::{BlsSignature, Bytes32},
    ssz::prelude::*,
    types::execution_payload_header::{ExecutionPayloadHeaderRef, ExecutionPayloadHeaderRefMut},
    Fork as Version,
};
#[derive(Debug, Clone, PartialEq, Eq, Serializable, HashTreeRoot, serde::Serialize)]
#[ssz(transparent)]
#[serde(untagged)]
pub enum BlindedBeaconBlockBody<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const MAX_COMMITTEES_PER_SLOT: usize,
    const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
    const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
    const MAX_ATTESTATIONS_ELECTRA: usize,
    const MAX_VALIDATORS_PER_SLOT: usize,
> {
    Bellatrix(
        bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ),
    Capella(
        capella::BlindedBeaconBlockBody<
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
        >,
    ),
    Deneb(
        deneb::BlindedBeaconBlockBody<
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
        >,
    ),
    Electra(
        electra::BlindedBeaconBlockBody<
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
        >,
    ),
}
impl<
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    BlindedBeaconBlockBody<
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
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn electra(
        &self,
    ) -> Option<
        &electra::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Electra(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn electra_mut(
        &mut self,
    ) -> Option<
        &mut electra::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Electra(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
            Self::Electra(_) => Version::Electra,
        }
    }
    pub fn randao_reveal(&self) -> &BlsSignature {
        match self {
            Self::Bellatrix(inner) => &inner.randao_reveal,
            Self::Capella(inner) => &inner.randao_reveal,
            Self::Deneb(inner) => &inner.randao_reveal,
            Self::Electra(inner) => &inner.randao_reveal,
        }
    }
    pub fn randao_reveal_mut(&mut self) -> &mut BlsSignature {
        match self {
            Self::Bellatrix(inner) => &mut inner.randao_reveal,
            Self::Capella(inner) => &mut inner.randao_reveal,
            Self::Deneb(inner) => &mut inner.randao_reveal,
            Self::Electra(inner) => &mut inner.randao_reveal,
        }
    }
    pub fn eth1_data(&self) -> &Eth1Data {
        match self {
            Self::Bellatrix(inner) => &inner.eth1_data,
            Self::Capella(inner) => &inner.eth1_data,
            Self::Deneb(inner) => &inner.eth1_data,
            Self::Electra(inner) => &inner.eth1_data,
        }
    }
    pub fn eth1_data_mut(&mut self) -> &mut Eth1Data {
        match self {
            Self::Bellatrix(inner) => &mut inner.eth1_data,
            Self::Capella(inner) => &mut inner.eth1_data,
            Self::Deneb(inner) => &mut inner.eth1_data,
            Self::Electra(inner) => &mut inner.eth1_data,
        }
    }
    pub fn graffiti(&self) -> &Bytes32 {
        match self {
            Self::Bellatrix(inner) => &inner.graffiti,
            Self::Capella(inner) => &inner.graffiti,
            Self::Deneb(inner) => &inner.graffiti,
            Self::Electra(inner) => &inner.graffiti,
        }
    }
    pub fn graffiti_mut(&mut self) -> &mut Bytes32 {
        match self {
            Self::Bellatrix(inner) => &mut inner.graffiti,
            Self::Capella(inner) => &mut inner.graffiti,
            Self::Deneb(inner) => &mut inner.graffiti,
            Self::Electra(inner) => &mut inner.graffiti,
        }
    }
    pub fn proposer_slashings(&self) -> &List<ProposerSlashing, MAX_PROPOSER_SLASHINGS> {
        match self {
            Self::Bellatrix(inner) => &inner.proposer_slashings,
            Self::Capella(inner) => &inner.proposer_slashings,
            Self::Deneb(inner) => &inner.proposer_slashings,
            Self::Electra(inner) => &inner.proposer_slashings,
        }
    }
    pub fn proposer_slashings_mut(
        &mut self,
    ) -> &mut List<ProposerSlashing, MAX_PROPOSER_SLASHINGS> {
        match self {
            Self::Bellatrix(inner) => &mut inner.proposer_slashings,
            Self::Capella(inner) => &mut inner.proposer_slashings,
            Self::Deneb(inner) => &mut inner.proposer_slashings,
            Self::Electra(inner) => &mut inner.proposer_slashings,
        }
    }
    // pub fn attester_slashings(
    //     &self,
    // ) -> &List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS> {
    //     match self {
    //         Self::Bellatrix(inner) => &inner.attester_slashings,
    //         Self::Capella(inner) => &inner.attester_slashings,
    //         Self::Deneb(inner) => &inner.attester_slashings,
    //         Self::Electra(inner) => &inner.attester_slashings,
    //     }
    // }
    // pub fn attester_slashings_mut(
    //     &mut self,
    // ) -> &mut List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS> {
    //     match self {
    //         Self::Bellatrix(inner) => &mut inner.attester_slashings,
    //         Self::Capella(inner) => &mut inner.attester_slashings,
    //         Self::Deneb(inner) => &mut inner.attester_slashings,
    //         Self::Electra(inner) => &mut inner.attester_slashings,
    //     }
    // }
    // pub fn attestations(
    //     &self,
    // ) -> &List<Attestation<MAX_VALIDATORS_PER_COMMITTEE, MAX_COMMITTEES_PER_SLOT>, MAX_ATTESTATIONS>
    // {
    //     match self {
    //         Self::Bellatrix(inner) => &inner.attestations,
    //         Self::Capella(inner) => &inner.attestations,
    //         Self::Deneb(inner) => &inner.attestations,
    //         Self::Electra(inner) => &inner.attestations,
    //     }
    // }
    // pub fn attestations_mut(
    //     &mut self,
    // ) -> &mut List<
    //     Attestation<MAX_VALIDATORS_PER_COMMITTEE, MAX_COMMITTEES_PER_SLOT>,
    //     MAX_ATTESTATIONS,
    // > {
    //     match self {
    //         Self::Bellatrix(inner) => &mut inner.attestations,
    //         Self::Capella(inner) => &mut inner.attestations,
    //         Self::Deneb(inner) => &mut inner.attestations,
    //         Self::Electra(inner) => &mut inner.attestations,
    //     }
    // }
    pub fn deposits(&self) -> &List<Deposit, MAX_DEPOSITS> {
        match self {
            Self::Bellatrix(inner) => &inner.deposits,
            Self::Capella(inner) => &inner.deposits,
            Self::Deneb(inner) => &inner.deposits,
            Self::Electra(inner) => &inner.deposits,
        }
    }
    pub fn deposits_mut(&mut self) -> &mut List<Deposit, MAX_DEPOSITS> {
        match self {
            Self::Bellatrix(inner) => &mut inner.deposits,
            Self::Capella(inner) => &mut inner.deposits,
            Self::Deneb(inner) => &mut inner.deposits,
            Self::Electra(inner) => &mut inner.deposits,
        }
    }
    pub fn voluntary_exits(&self) -> &List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS> {
        match self {
            Self::Bellatrix(inner) => &inner.voluntary_exits,
            Self::Capella(inner) => &inner.voluntary_exits,
            Self::Deneb(inner) => &inner.voluntary_exits,
            Self::Electra(inner) => &inner.voluntary_exits,
        }
    }
    pub fn voluntary_exits_mut(&mut self) -> &mut List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS> {
        match self {
            Self::Bellatrix(inner) => &mut inner.voluntary_exits,
            Self::Capella(inner) => &mut inner.voluntary_exits,
            Self::Deneb(inner) => &mut inner.voluntary_exits,
            Self::Electra(inner) => &mut inner.voluntary_exits,
        }
    }
    pub fn sync_aggregate(&self) -> &SyncAggregate<SYNC_COMMITTEE_SIZE> {
        match self {
            Self::Bellatrix(inner) => &inner.sync_aggregate,
            Self::Capella(inner) => &inner.sync_aggregate,
            Self::Deneb(inner) => &inner.sync_aggregate,
            Self::Electra(inner) => &inner.sync_aggregate,
        }
    }
    pub fn sync_aggregate_mut(&mut self) -> &mut SyncAggregate<SYNC_COMMITTEE_SIZE> {
        match self {
            Self::Bellatrix(inner) => &mut inner.sync_aggregate,
            Self::Capella(inner) => &mut inner.sync_aggregate,
            Self::Deneb(inner) => &mut inner.sync_aggregate,
            Self::Electra(inner) => &mut inner.sync_aggregate,
        }
    }
    pub fn execution_payload_header(
        &self,
    ) -> ExecutionPayloadHeaderRef<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES> {
        match self {
            Self::Bellatrix(inner) => From::from(&inner.execution_payload_header),
            Self::Capella(inner) => From::from(&inner.execution_payload_header),
            Self::Deneb(inner) => From::from(&inner.execution_payload_header),
            Self::Electra(inner) => From::from(&inner.execution_payload_header),
        }
    }
    pub fn execution_payload_header_mut(
        &mut self,
    ) -> ExecutionPayloadHeaderRefMut<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES> {
        match self {
            Self::Bellatrix(inner) => From::from(&mut inner.execution_payload_header),
            Self::Capella(inner) => From::from(&mut inner.execution_payload_header),
            Self::Deneb(inner) => From::from(&mut inner.execution_payload_header),
            Self::Electra(inner) => From::from(&mut inner.execution_payload_header),
        }
    }
    pub fn bls_to_execution_changes(
        &self,
    ) -> Option<&List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&inner.bls_to_execution_changes),
            Self::Electra(inner) => Some(&inner.bls_to_execution_changes),
        }
    }
    pub fn bls_to_execution_changes_mut(
        &mut self,
    ) -> Option<&mut List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&mut inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&mut inner.bls_to_execution_changes),
            Self::Electra(inner) => Some(&mut inner.bls_to_execution_changes),
        }
    }
    pub fn blob_kzg_commitments(
        &self,
    ) -> Option<&List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_kzg_commitments),
            Self::Electra(inner) => Some(&inner.blob_kzg_commitments),
        }
    }
    pub fn blob_kzg_commitments_mut(
        &mut self,
    ) -> Option<&mut List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.blob_kzg_commitments),
            Self::Electra(inner) => Some(&mut inner.blob_kzg_commitments),
        }
    }
    pub fn execution_requests(
        &self,
    ) -> Option<
        &ExecutionRequests<
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(_) => None,
            Self::Electra(inner) => Some(&inner.execution_requests),
        }
    }
    pub fn execution_requests_mut(
        &mut self,
    ) -> Option<
        &mut ExecutionRequests<
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(_) => None,
            Self::Electra(inner) => Some(&mut inner.execution_requests),
        }
    }
}
impl<
        'de,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    > serde::Deserialize<'de>
    for BlindedBeaconBlockBody<
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
    >
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Electra(inner));
        }
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Deneb(inner));
        }
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Capella(inner));
        }
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Bellatrix(inner));
        }
        Err(serde::de::Error::custom("no variant could be deserialized from input"))
    }
}
#[derive(Debug, PartialEq, Eq, HashTreeRoot)]
#[ssz(transparent)]
pub enum BlindedBeaconBlockBodyRef<
    'a,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const MAX_COMMITTEES_PER_SLOT: usize,
    const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
    const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
    const MAX_ATTESTATIONS_ELECTRA: usize,
    const MAX_VALIDATORS_PER_SLOT: usize,
> {
    Bellatrix(
        &'a bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ),
    Capella(
        &'a capella::BlindedBeaconBlockBody<
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
        >,
    ),
    Deneb(
        &'a deneb::BlindedBeaconBlockBody<
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
        >,
    ),
    Electra(
        &'a electra::BlindedBeaconBlockBody<
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
        >,
    ),
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    BlindedBeaconBlockBodyRef<
        'a,
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
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn electra(
        &self,
    ) -> Option<
        &electra::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Electra(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
            Self::Electra(_) => Version::Electra,
        }
    }
    pub fn randao_reveal(&self) -> &BlsSignature {
        match self {
            Self::Bellatrix(inner) => &inner.randao_reveal,
            Self::Capella(inner) => &inner.randao_reveal,
            Self::Deneb(inner) => &inner.randao_reveal,
            Self::Electra(inner) => &inner.randao_reveal,
        }
    }
    pub fn eth1_data(&self) -> &Eth1Data {
        match self {
            Self::Bellatrix(inner) => &inner.eth1_data,
            Self::Capella(inner) => &inner.eth1_data,
            Self::Deneb(inner) => &inner.eth1_data,
            Self::Electra(inner) => &inner.eth1_data,
        }
    }
    pub fn graffiti(&self) -> &Bytes32 {
        match self {
            Self::Bellatrix(inner) => &inner.graffiti,
            Self::Capella(inner) => &inner.graffiti,
            Self::Deneb(inner) => &inner.graffiti,
            Self::Electra(inner) => &inner.graffiti,
        }
    }
    pub fn proposer_slashings(&self) -> &List<ProposerSlashing, MAX_PROPOSER_SLASHINGS> {
        match self {
            Self::Bellatrix(inner) => &inner.proposer_slashings,
            Self::Capella(inner) => &inner.proposer_slashings,
            Self::Deneb(inner) => &inner.proposer_slashings,
            Self::Electra(inner) => &inner.proposer_slashings,
        }
    }
    // pub fn attester_slashings(
    //     &self,
    // ) -> &List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS> {
    //     match self {
    //         Self::Bellatrix(inner) => &inner.attester_slashings,
    //         Self::Capella(inner) => &inner.attester_slashings,
    //         Self::Deneb(inner) => &inner.attester_slashings,
    //         Self::Electra(inner) => &inner.attester_slashings,
    //     }
    // }
    // pub fn attestations(
    //     &self,
    // ) -> &List<Attestation<MAX_VALIDATORS_PER_COMMITTEE, MAX_COMMITTEES_PER_SLOT>, MAX_ATTESTATIONS>
    // {
    //     match self {
    //         Self::Bellatrix(inner) => &inner.attestations,
    //         Self::Capella(inner) => &inner.attestations,
    //         Self::Deneb(inner) => &inner.attestations,
    //         Self::Electra(inner) => &inner.attestations,
    //     }
    // }
    pub fn deposits(&self) -> &List<Deposit, MAX_DEPOSITS> {
        match self {
            Self::Bellatrix(inner) => &inner.deposits,
            Self::Capella(inner) => &inner.deposits,
            Self::Deneb(inner) => &inner.deposits,
            Self::Electra(inner) => &inner.deposits,
        }
    }
    pub fn voluntary_exits(&self) -> &List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS> {
        match self {
            Self::Bellatrix(inner) => &inner.voluntary_exits,
            Self::Capella(inner) => &inner.voluntary_exits,
            Self::Deneb(inner) => &inner.voluntary_exits,
            Self::Electra(inner) => &inner.voluntary_exits,
        }
    }
    pub fn sync_aggregate(&self) -> &SyncAggregate<SYNC_COMMITTEE_SIZE> {
        match self {
            Self::Bellatrix(inner) => &inner.sync_aggregate,
            Self::Capella(inner) => &inner.sync_aggregate,
            Self::Deneb(inner) => &inner.sync_aggregate,
            Self::Electra(inner) => &inner.sync_aggregate,
        }
    }
    pub fn execution_payload_header(
        &self,
    ) -> ExecutionPayloadHeaderRef<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES> {
        match self {
            Self::Bellatrix(inner) => From::from(&inner.execution_payload_header),
            Self::Capella(inner) => From::from(&inner.execution_payload_header),
            Self::Deneb(inner) => From::from(&inner.execution_payload_header),
            Self::Electra(inner) => From::from(&inner.execution_payload_header),
        }
    }
    pub fn bls_to_execution_changes(
        &self,
    ) -> Option<&List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&inner.bls_to_execution_changes),
            Self::Electra(inner) => Some(&inner.bls_to_execution_changes),
        }
    }
    pub fn blob_kzg_commitments(
        &self,
    ) -> Option<&List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_kzg_commitments),
            Self::Electra(inner) => Some(&inner.blob_kzg_commitments),
        }
    }
    pub fn execution_requests(
        &self,
    ) -> Option<
        &ExecutionRequests<
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(_) => None,
            Self::Electra(inner) => Some(&inner.execution_requests),
        }
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    >
    for BlindedBeaconBlockBodyRef<
        'a,
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
    >
{
    fn from(
        value: &'a bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ) -> Self {
        Self::Bellatrix(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a capella::BlindedBeaconBlockBody<
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
        >,
    >
    for BlindedBeaconBlockBodyRef<
        'a,
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
    >
{
    fn from(
        value: &'a capella::BlindedBeaconBlockBody<
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
        >,
    ) -> Self {
        Self::Capella(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a deneb::BlindedBeaconBlockBody<
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
        >,
    >
    for BlindedBeaconBlockBodyRef<
        'a,
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
    >
{
    fn from(
        value: &'a deneb::BlindedBeaconBlockBody<
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
        >,
    ) -> Self {
        Self::Deneb(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a electra::BlindedBeaconBlockBody<
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
        >,
    >
    for BlindedBeaconBlockBodyRef<
        'a,
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
    >
{
    fn from(
        value: &'a electra::BlindedBeaconBlockBody<
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
        >,
    ) -> Self {
        Self::Electra(value)
    }
}
#[derive(Debug, PartialEq, Eq, HashTreeRoot)]
#[ssz(transparent)]
pub enum BlindedBeaconBlockBodyRefMut<
    'a,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const MAX_COMMITTEES_PER_SLOT: usize,
    const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
    const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
    const MAX_ATTESTATIONS_ELECTRA: usize,
    const MAX_VALIDATORS_PER_SLOT: usize,
> {
    Bellatrix(
        &'a mut bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ),
    Capella(
        &'a mut capella::BlindedBeaconBlockBody<
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
        >,
    ),
    Deneb(
        &'a mut deneb::BlindedBeaconBlockBody<
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
        >,
    ),
    Electra(
        &'a mut electra::BlindedBeaconBlockBody<
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
        >,
    ),
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    BlindedBeaconBlockBodyRefMut<
        'a,
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
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn electra(
        &self,
    ) -> Option<
        &electra::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Electra(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn electra_mut(
        &mut self,
    ) -> Option<
        &mut electra::BlindedBeaconBlockBody<
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
        >,
    > {
        match self {
            Self::Electra(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
            Self::Electra(_) => Version::Electra,
        }
    }
    pub fn randao_reveal(&self) -> &BlsSignature {
        match self {
            Self::Bellatrix(inner) => &inner.randao_reveal,
            Self::Capella(inner) => &inner.randao_reveal,
            Self::Deneb(inner) => &inner.randao_reveal,
            Self::Electra(inner) => &inner.randao_reveal,
        }
    }
    pub fn randao_reveal_mut(&mut self) -> &mut BlsSignature {
        match self {
            Self::Bellatrix(inner) => &mut inner.randao_reveal,
            Self::Capella(inner) => &mut inner.randao_reveal,
            Self::Deneb(inner) => &mut inner.randao_reveal,
            Self::Electra(inner) => &mut inner.randao_reveal,
        }
    }
    pub fn eth1_data(&self) -> &Eth1Data {
        match self {
            Self::Bellatrix(inner) => &inner.eth1_data,
            Self::Capella(inner) => &inner.eth1_data,
            Self::Deneb(inner) => &inner.eth1_data,
            Self::Electra(inner) => &inner.eth1_data,
        }
    }
    pub fn eth1_data_mut(&mut self) -> &mut Eth1Data {
        match self {
            Self::Bellatrix(inner) => &mut inner.eth1_data,
            Self::Capella(inner) => &mut inner.eth1_data,
            Self::Deneb(inner) => &mut inner.eth1_data,
            Self::Electra(inner) => &mut inner.eth1_data,
        }
    }
    pub fn graffiti(&self) -> &Bytes32 {
        match self {
            Self::Bellatrix(inner) => &inner.graffiti,
            Self::Capella(inner) => &inner.graffiti,
            Self::Deneb(inner) => &inner.graffiti,
            Self::Electra(inner) => &inner.graffiti,
        }
    }
    pub fn graffiti_mut(&mut self) -> &mut Bytes32 {
        match self {
            Self::Bellatrix(inner) => &mut inner.graffiti,
            Self::Capella(inner) => &mut inner.graffiti,
            Self::Deneb(inner) => &mut inner.graffiti,
            Self::Electra(inner) => &mut inner.graffiti,
        }
    }
    pub fn proposer_slashings(&self) -> &List<ProposerSlashing, MAX_PROPOSER_SLASHINGS> {
        match self {
            Self::Bellatrix(inner) => &inner.proposer_slashings,
            Self::Capella(inner) => &inner.proposer_slashings,
            Self::Deneb(inner) => &inner.proposer_slashings,
            Self::Electra(inner) => &inner.proposer_slashings,
        }
    }
    pub fn proposer_slashings_mut(
        &mut self,
    ) -> &mut List<ProposerSlashing, MAX_PROPOSER_SLASHINGS> {
        match self {
            Self::Bellatrix(inner) => &mut inner.proposer_slashings,
            Self::Capella(inner) => &mut inner.proposer_slashings,
            Self::Deneb(inner) => &mut inner.proposer_slashings,
            Self::Electra(inner) => &mut inner.proposer_slashings,
        }
    }
    // pub fn attester_slashings(
    //     &self,
    // ) -> &List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS> {
    //     match self {
    //         Self::Bellatrix(inner) => &inner.attester_slashings,
    //         Self::Capella(inner) => &inner.attester_slashings,
    //         Self::Deneb(inner) => &inner.attester_slashings,
    //         Self::Electra(inner) => &inner.attester_slashings,
    //     }
    // }
    // pub fn attester_slashings_mut(
    //     &mut self,
    // ) -> &mut List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS> {
    //     match self {
    //         Self::Bellatrix(inner) => &mut inner.attester_slashings,
    //         Self::Capella(inner) => &mut inner.attester_slashings,
    //         Self::Deneb(inner) => &mut inner.attester_slashings,
    //         Self::Electra(inner) => &mut inner.attester_slashings,
    //     }
    // }
    // pub fn attestations(
    //     &self,
    // ) -> &List<Attestation<MAX_VALIDATORS_PER_COMMITTEE, MAX_COMMITTEES_PER_SLOT>, MAX_ATTESTATIONS>
    // {
    //     match self {
    //         Self::Bellatrix(inner) => &inner.attestations,
    //         Self::Capella(inner) => &inner.attestations,
    //         Self::Deneb(inner) => &inner.attestations,
    //         Self::Electra(inner) => &inner.attestations,
    //     }
    // }
    // pub fn attestations_mut(
    //     &mut self,
    // ) -> &mut List<
    //     Attestation<MAX_VALIDATORS_PER_COMMITTEE, MAX_COMMITTEES_PER_SLOT>,
    //     MAX_ATTESTATIONS,
    // > {
    //     match self {
    //         Self::Bellatrix(inner) => &mut inner.attestations,
    //         Self::Capella(inner) => &mut inner.attestations,
    //         Self::Deneb(inner) => &mut inner.attestations,
    //         Self::Electra(inner) => &mut inner.attestations,
    //     }
    // }
    pub fn deposits(&self) -> &List<Deposit, MAX_DEPOSITS> {
        match self {
            Self::Bellatrix(inner) => &inner.deposits,
            Self::Capella(inner) => &inner.deposits,
            Self::Deneb(inner) => &inner.deposits,
            Self::Electra(inner) => &inner.deposits,
        }
    }
    pub fn deposits_mut(&mut self) -> &mut List<Deposit, MAX_DEPOSITS> {
        match self {
            Self::Bellatrix(inner) => &mut inner.deposits,
            Self::Capella(inner) => &mut inner.deposits,
            Self::Deneb(inner) => &mut inner.deposits,
            Self::Electra(inner) => &mut inner.deposits,
        }
    }
    pub fn voluntary_exits(&self) -> &List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS> {
        match self {
            Self::Bellatrix(inner) => &inner.voluntary_exits,
            Self::Capella(inner) => &inner.voluntary_exits,
            Self::Deneb(inner) => &inner.voluntary_exits,
            Self::Electra(inner) => &inner.voluntary_exits,
        }
    }
    pub fn voluntary_exits_mut(&mut self) -> &mut List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS> {
        match self {
            Self::Bellatrix(inner) => &mut inner.voluntary_exits,
            Self::Capella(inner) => &mut inner.voluntary_exits,
            Self::Deneb(inner) => &mut inner.voluntary_exits,
            Self::Electra(inner) => &mut inner.voluntary_exits,
        }
    }
    pub fn sync_aggregate(&self) -> &SyncAggregate<SYNC_COMMITTEE_SIZE> {
        match self {
            Self::Bellatrix(inner) => &inner.sync_aggregate,
            Self::Capella(inner) => &inner.sync_aggregate,
            Self::Deneb(inner) => &inner.sync_aggregate,
            Self::Electra(inner) => &inner.sync_aggregate,
        }
    }
    pub fn sync_aggregate_mut(&mut self) -> &mut SyncAggregate<SYNC_COMMITTEE_SIZE> {
        match self {
            Self::Bellatrix(inner) => &mut inner.sync_aggregate,
            Self::Capella(inner) => &mut inner.sync_aggregate,
            Self::Deneb(inner) => &mut inner.sync_aggregate,
            Self::Electra(inner) => &mut inner.sync_aggregate,
        }
    }
    pub fn execution_payload_header(
        &self,
    ) -> ExecutionPayloadHeaderRef<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES> {
        match self {
            Self::Bellatrix(inner) => From::from(&inner.execution_payload_header),
            Self::Capella(inner) => From::from(&inner.execution_payload_header),
            Self::Deneb(inner) => From::from(&inner.execution_payload_header),
            Self::Electra(inner) => From::from(&inner.execution_payload_header),
        }
    }
    pub fn execution_payload_header_mut(
        &mut self,
    ) -> ExecutionPayloadHeaderRefMut<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES> {
        match self {
            Self::Bellatrix(inner) => From::from(&mut inner.execution_payload_header),
            Self::Capella(inner) => From::from(&mut inner.execution_payload_header),
            Self::Deneb(inner) => From::from(&mut inner.execution_payload_header),
            Self::Electra(inner) => From::from(&mut inner.execution_payload_header),
        }
    }
    pub fn bls_to_execution_changes(
        &self,
    ) -> Option<&List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&inner.bls_to_execution_changes),
            Self::Electra(inner) => Some(&inner.bls_to_execution_changes),
        }
    }
    pub fn bls_to_execution_changes_mut(
        &mut self,
    ) -> Option<&mut List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&mut inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&mut inner.bls_to_execution_changes),
            Self::Electra(inner) => Some(&mut inner.bls_to_execution_changes),
        }
    }
    pub fn blob_kzg_commitments(
        &self,
    ) -> Option<&List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_kzg_commitments),
            Self::Electra(inner) => Some(&inner.blob_kzg_commitments),
        }
    }
    pub fn blob_kzg_commitments_mut(
        &mut self,
    ) -> Option<&mut List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.blob_kzg_commitments),
            Self::Electra(inner) => Some(&mut inner.blob_kzg_commitments),
        }
    }
    pub fn execution_requests(
        &self,
    ) -> Option<
        &ExecutionRequests<
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(_) => None,
            Self::Electra(inner) => Some(&inner.execution_requests),
        }
    }
    pub fn execution_requests_mut(
        &mut self,
    ) -> Option<
        &mut ExecutionRequests<
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(_) => None,
            Self::Electra(inner) => Some(&mut inner.execution_requests),
        }
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a mut bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    >
    for BlindedBeaconBlockBodyRefMut<
        'a,
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
    >
{
    fn from(
        value: &'a mut bellatrix::BlindedBeaconBlockBody<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ) -> Self {
        Self::Bellatrix(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a mut capella::BlindedBeaconBlockBody<
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
        >,
    >
    for BlindedBeaconBlockBodyRefMut<
        'a,
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
    >
{
    fn from(
        value: &'a mut capella::BlindedBeaconBlockBody<
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
        >,
    ) -> Self {
        Self::Capella(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a mut deneb::BlindedBeaconBlockBody<
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
        >,
    >
    for BlindedBeaconBlockBodyRefMut<
        'a,
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
    >
{
    fn from(
        value: &'a mut deneb::BlindedBeaconBlockBody<
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
        >,
    ) -> Self {
        Self::Deneb(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
    >
    From<
        &'a mut electra::BlindedBeaconBlockBody<
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
        >,
    >
    for BlindedBeaconBlockBodyRefMut<
        'a,
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
    >
{
    fn from(
        value: &'a mut electra::BlindedBeaconBlockBody<
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
        >,
    ) -> Self {
        Self::Electra(value)
    }
}
