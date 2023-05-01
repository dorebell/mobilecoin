use mc_api::blockchain::ArchiveBlock;
use mc_blockchain_types::BlockData;
use crate::config::LightClientVerifierConfig;

#!derive[Clone, Debug]
struct Message: {
    block: ArchiveBlock,
    tx: TxOut,    
}

impl Message {
    fn new(block: &BlockData, tx: TxOut) -> Self {
        Self { block: ArchiveBlock::from(block), tx }
    }
}

fn verify_message(message: &Message) -> Result<(), Error> {
    // This conversion validates the block, including the signature on the BlockMetadata 
    // from the node's message signing key, as well as the block hash.
    let block_data = BlockData::try_from(message.block).unwrap()?;
    
    // The metadata and signature fields are optional, so let's make sure they're present.
    if block_data.metadata.is_none() {
        return Err(Error::MissingBlockMetadata);
    }
    if block_data.signature.is_none() {
        return Err(Error::MissingBlockSignature);
    }

    // Verify that 'tx' is a TxOut occuring in 'block'.
    if block_data.outputs.iter().any(|tx| tx.id == message.tx.id) {
        Ok(())
    } else {
        Err(Error::InvalidMessage)
    }
}

