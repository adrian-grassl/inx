// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::proto;

use bee_block_stardust as stardust;

/// Represents a new output in the ledger.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct LedgerOutput {
    pub output_id: stardust::output::OutputId,
    pub block_id: stardust::BlockId,
    pub milestone_index_booked: u32,
    pub milestone_timestamp_booked: u32,
    pub output: stardust::output::Output,
}

/// Represents a spent output in the ledger.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct LedgerSpent {
    pub output: LedgerOutput,
    pub transaction_id_spent: stardust::payload::transaction::TransactionId,
    pub milestone_index_spent: u32,
    pub milestone_timestamp_spent: u32,
}

/// Represents an update to ledger.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct LedgerUpdate {
    pub milestone_index: u32,
    pub created: Box<[LedgerOutput]>,
    pub consumed: Box<[LedgerSpent]>,
}

impl TryFrom<proto::OutputId> for stardust::output::OutputId {
    type Error = Error;

    fn try_from(value: proto::OutputId) -> Result<Self, Self::Error> {
        let bytes: [u8; stardust::output::OutputId::LENGTH] =
            value.id.try_into().map_err(|_| Error::InvalidBufferLength)?;
        stardust::output::OutputId::try_from(bytes).map_err(|_| Error::InvalidBufferLength)
    }
}

impl TryFrom<proto::LedgerOutput> for LedgerOutput {
    type Error = Error;

    fn try_from(value: proto::LedgerOutput) -> Result<Self, Self::Error> {
        Ok(LedgerOutput {
            output_id: value.output_id.ok_or(Error::MissingField("output_id"))?.try_into()?,
            block_id: value.block_id.ok_or(Error::MissingField("message_id"))?.try_into()?,
            milestone_index_booked: value.milestone_index_booked,
            milestone_timestamp_booked: value.milestone_timestamp_booked,
            output: value.output.ok_or(Error::MissingField("output"))?.try_into()?,
        })
    }
}

impl TryFrom<proto::LedgerSpent> for LedgerSpent {
    type Error = Error;

    fn try_from(value: proto::LedgerSpent) -> Result<Self, Self::Error> {
        Ok(LedgerSpent {
            output: value.output.ok_or(Error::MissingField("output"))?.try_into()?,
            transaction_id_spent: value
                .transaction_id_spent
                .ok_or(Error::MissingField("transaction_id"))?
                .try_into()?,
            milestone_index_spent: value.milestone_index_spent,
            milestone_timestamp_spent: value.milestone_timestamp_spent,
        })
    }
}

impl TryFrom<proto::LedgerUpdate> for LedgerUpdate {
    type Error = Error;

    fn try_from(value: proto::LedgerUpdate) -> Result<Self, Self::Error> {
        let mut created: Vec<LedgerOutput> = Vec::with_capacity(value.created.len());
        for c in value.created {
            created.push(c.try_into()?);
        }

        let mut consumed: Vec<LedgerSpent> = Vec::with_capacity(value.consumed.len());
        for c in value.consumed {
            consumed.push(c.try_into()?);
        }

        Ok(LedgerUpdate {
            milestone_index: value.milestone_index,
            created: created.into_boxed_slice(),
            consumed: consumed.into_boxed_slice(),
        })
    }
}
