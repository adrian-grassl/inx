// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::proto;

use bee_block_stardust as stardust;

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum LedgerInclusionState {
    NoTransaction,
    Included,
    Conflicting,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum ConflictReason {
    None,
    InputAlreadySpent,
    InputAlreadySpentInThisMilestone,
    InputNotFound,
    InputOutputSumMismatch,
    InvalidSignature,
    TimelockNotExpired,
    InvalidNativeTokens,
    ReturnAmountNotFulfilled,
    InvalidInputUnlock,
    InvalidInputsCommitment,
    InvalidSender,
    InvalidChainStateTransition,
    SemanticValidationFailed,
}

/// The metadata for a block with a given [`BlockId`](stardust::BlockId).
#[derive(Clone, Debug, PartialEq)]
pub struct BlockMetadata {
    /// The id of the block.
    pub block_id: stardust::BlockId,
    /// The parents of the messsage.
    pub parents: Vec<stardust::BlockId>,
    /// Status of the solidification process.
    pub is_solid: bool,
    /// Indicates that the block should be promoted.
    pub should_promote: bool,
    /// Indicates that the block should be reattached.
    pub should_reattach: bool,
    /// The milestone that referenced the block.
    pub referenced_by_milestone_index: u32,
    /// The corresponding milestone index.
    pub milestone_index: u32,
    /// Indicates if a block is part of the ledger state or not.
    pub ledger_inclusion_state: LedgerInclusionState,
    /// Indicates if a conflict occured, and if so holds information about the reason for the conflict.
    pub conflict_reason: ConflictReason,
}

impl TryFrom<proto::BlockMetadata> for BlockMetadata {
    type Error = Error;

    fn try_from(value: proto::BlockMetadata) -> Result<Self, Self::Error> {
        let ledger_inclusion_state = value.ledger_inclusion_state().into();
        let conflict_reason = value.conflict_reason().into();

        let mut parents = Vec::with_capacity(value.parents.len());
        for parent in value.parents {
            parents.push(parent.try_into()?);
        }

        Ok(BlockMetadata {
            block_id: value.block_id.ok_or(Error::MissingField("block_id"))?.try_into()?,
            parents,
            is_solid: value.solid,
            should_promote: value.should_promote,
            should_reattach: value.should_reattach,
            referenced_by_milestone_index: value.referenced_by_milestone_index,
            milestone_index: value.milestone_index,
            ledger_inclusion_state,
            conflict_reason,
        })
    }
}

impl From<proto::block_metadata::LedgerInclusionState> for LedgerInclusionState {
    fn from(value: proto::block_metadata::LedgerInclusionState) -> Self {
        match value {
            proto::block_metadata::LedgerInclusionState::NoTransaction => LedgerInclusionState::NoTransaction,
            proto::block_metadata::LedgerInclusionState::Included => LedgerInclusionState::Included,
            proto::block_metadata::LedgerInclusionState::Conflicting => LedgerInclusionState::Conflicting,
        }
    }
}

impl From<proto::block_metadata::ConflictReason> for ConflictReason {
    fn from(value: proto::block_metadata::ConflictReason) -> Self {
        match value {
            proto::block_metadata::ConflictReason::None => ConflictReason::None,
            proto::block_metadata::ConflictReason::InputAlreadySpent => ConflictReason::InputAlreadySpent,
            proto::block_metadata::ConflictReason::InputAlreadySpentInThisMilestone => {
                ConflictReason::InputAlreadySpentInThisMilestone
            }
            proto::block_metadata::ConflictReason::InputNotFound => ConflictReason::InputNotFound,
            proto::block_metadata::ConflictReason::InputOutputSumMismatch => ConflictReason::InputOutputSumMismatch,
            proto::block_metadata::ConflictReason::InvalidSignature => ConflictReason::InvalidSignature,
            proto::block_metadata::ConflictReason::TimelockNotExpired => ConflictReason::TimelockNotExpired,
            proto::block_metadata::ConflictReason::InvalidNativeTokens => ConflictReason::InvalidNativeTokens,
            proto::block_metadata::ConflictReason::ReturnAmountNotFulfilled => ConflictReason::ReturnAmountNotFulfilled,
            proto::block_metadata::ConflictReason::InvalidInputUnlock => ConflictReason::InvalidInputUnlock,
            proto::block_metadata::ConflictReason::InvalidInputsCommitment => ConflictReason::InvalidInputsCommitment,
            proto::block_metadata::ConflictReason::InvalidSender => ConflictReason::InvalidSender,
            proto::block_metadata::ConflictReason::InvalidChainStateTransition => {
                ConflictReason::InvalidChainStateTransition
            }
            proto::block_metadata::ConflictReason::SemanticValidationFailed => ConflictReason::SemanticValidationFailed,
        }
    }
}
