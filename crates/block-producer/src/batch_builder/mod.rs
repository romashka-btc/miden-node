use std::{cmp::min, collections::BTreeSet, sync::Arc, time::Duration};

use async_trait::async_trait;
use miden_objects::{notes::NoteId, transaction::OutputNote};
use tokio::time;
use tracing::{debug, info, instrument, Span};

use crate::{block_builder::BlockBuilder, ProvenTransaction, SharedRwVec, COMPONENT};

#[cfg(test)]
mod tests;

pub mod batch;
pub use batch::TransactionBatch;
use miden_node_utils::formatting::{format_array, format_blake3_digest};

use crate::{errors::BuildBatchError, store::Store};

// BATCH BUILDER
// ================================================================================================

/// Abstraction over batch proving of transactions.
///
/// Transactions are aggregated into batches prior to being added to blocks. This trait abstracts
/// over this responsibility. The trait's goal is to be implementation agnostic, allowing for
/// multiple implementations, e.g.:
///
/// - in-process cpu based prover
/// - out-of-process gpu based prover
/// - distributed prover on another machine
#[async_trait]
pub trait BatchBuilder: Send + Sync + 'static {
    /// Start proving of a new batch.
    async fn build_batch(&self, txs: Vec<ProvenTransaction>) -> Result<(), BuildBatchError>;
}

// DEFAULT BATCH BUILDER
// ================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultBatchBuilderOptions {
    /// The frequency at which blocks are created
    pub block_frequency: Duration,

    /// Maximum number of batches in any given block
    pub max_batches_per_block: usize,
}

pub struct DefaultBatchBuilder<S, BB> {
    store: Arc<S>,

    block_builder: Arc<BB>,

    options: DefaultBatchBuilderOptions,

    /// Batches ready to be included in a block
    ready_batches: SharedRwVec<TransactionBatch>,
}

impl<S, BB> DefaultBatchBuilder<S, BB>
where
    S: Store,
    BB: BlockBuilder,
{
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns an new [BatchBuilder] instantiated with the provided [BlockBuilder] and the
    /// specified options.
    pub fn new(store: Arc<S>, block_builder: Arc<BB>, options: DefaultBatchBuilderOptions) -> Self {
        Self {
            store,
            block_builder,
            options,
            ready_batches: Default::default(),
        }
    }

    // BATCH BUILDER STARTER
    // --------------------------------------------------------------------------------------------
    pub async fn run(self: Arc<Self>) {
        let mut interval = time::interval(self.options.block_frequency);

        info!(target: COMPONENT, period_ms = interval.period().as_millis(), "Batch builder started");

        loop {
            interval.tick().await;
            self.try_build_block().await;
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Note that we call `build_block()` regardless of whether the `ready_batches` queue is empty.
    /// A call to an empty `build_block()` indicates that an empty block should be created.
    #[instrument(target = "miden-block-producer", skip_all)]
    async fn try_build_block(&self) {
        let mut batches_in_block: Vec<TransactionBatch> = {
            let mut locked_ready_batches = self.ready_batches.write().await;

            let num_batches_in_block =
                min(self.options.max_batches_per_block, locked_ready_batches.len());

            locked_ready_batches.drain(..num_batches_in_block).collect()
        };

        match self.block_builder.build_block(&batches_in_block).await {
            Ok(_) => {
                // block successfully built, do nothing
            },
            Err(_) => {
                // Block building failed; add back the batches at the end of the queue
                self.ready_batches.write().await.append(&mut batches_in_block);
            },
        }
    }

    /// Returns a list of IDs for unauthenticated notes which are not output notes of any ready
    /// transaction batch or the candidate batch itself.
    async fn find_dangling_notes(&self, txs: &[ProvenTransaction]) -> Vec<NoteId> {
        // TODO: We can optimize this by looking at the notes created in the previous batches

        // build a set of output notes from all ready batches and the candidate batch
        let mut all_output_notes: BTreeSet<NoteId> = txs
            .iter()
            .flat_map(|tx| tx.output_notes().iter().map(OutputNote::id))
            .chain(
                self.ready_batches
                    .read()
                    .await
                    .iter()
                    .flat_map(|batch| batch.output_notes().iter().map(OutputNote::id)),
            )
            .collect();

        // from the list of unauthenticated notes in the candidate batch, filter out any note
        // which is also an output note either in any of the ready batches or in the candidate
        // batch itself
        txs.iter()
            .flat_map(|tx| tx.get_unauthenticated_notes().map(|note| note.id()))
            .filter(|note_id| !all_output_notes.remove(note_id))
            .collect()
    }
}

#[async_trait]
impl<S, BB> BatchBuilder for DefaultBatchBuilder<S, BB>
where
    S: Store,
    BB: BlockBuilder,
{
    #[instrument(target = "miden-block-producer", skip_all, err, fields(batch_id))]
    async fn build_batch(&self, txs: Vec<ProvenTransaction>) -> Result<(), BuildBatchError> {
        let num_txs = txs.len();

        info!(target: COMPONENT, num_txs, "Building a transaction batch");
        debug!(target: COMPONENT, txs = %format_array(txs.iter().map(|tx| tx.id().to_hex())));

        // make sure that all unauthenticated notes in the transactions of the proposed batch
        // have been either created in any of the ready batches (or the batch itself) or are
        // already in the store
        //
        // TODO: this can be optimized by first computing dangling notes of the batch itself,
        //       and only then checking against the other ready batches
        let dangling_notes = self.find_dangling_notes(&txs).await;
        let found_unauthenticated_notes = match dangling_notes.is_empty() {
            true => Default::default(),
            false => {
                let stored_notes =
                    match self.store.get_note_authentication_info(dangling_notes.iter()).await {
                        Ok(stored_notes) => stored_notes,
                        Err(err) => return Err(BuildBatchError::NotePathsError(err, txs)),
                    };
                let missing_notes: Vec<_> = dangling_notes
                    .into_iter()
                    .filter(|note_id| !stored_notes.contains_note(note_id))
                    .collect();

                if !missing_notes.is_empty() {
                    return Err(BuildBatchError::UnauthenticatedNotesNotFound(missing_notes, txs));
                }

                stored_notes
            },
        };

        let batch = TransactionBatch::new(txs, found_unauthenticated_notes)?;

        info!(target: COMPONENT, "Transaction batch built");
        Span::current().record("batch_id", format_blake3_digest(batch.id()));

        let num_batches = {
            let mut write_guard = self.ready_batches.write().await;
            write_guard.push(batch);
            write_guard.len()
        };

        info!(target: COMPONENT, num_batches, "Transaction batch added to the batch queue");

        Ok(())
    }
}
