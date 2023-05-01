Things to figure out:
1. Should we use the blocksignature on the block_data object, or get it from somewhere else? Why is mobilecoind getting blocksignatures from the watcher instead of the block_data?
The watcher is getting the signatures from the block_data, so it seems likely that the block_data signature is the source of truth for the signature.
2. What structure do we need to relay?
Block_data includes one signature, one block metadata and block contents.
What signatures do we need? We need one per node in our quorum set, and watch all of those in order to get signatures from all of them.
reqwest transaction fetcher is fetching from multiple source urls.
https://github.com/mobilecoinfoundation/mobilecoin/blob/master/ledger/sync/src/ledger_sync/ledger_sync_service.rs#L135
This function specifically gets the blocks from every node. Normally the reqwest transaction fetcher just round robins requests between the source urls.
This bit of code checks whether the quorum condition has been satisfied:
https://github.com/mobilecoinfoundation/mobilecoin/blob/master/ledger/sync/src/network_state/scp_network_state.rs#L70
https://github.com/mobilecoinfoundation/mobilecoin/blob/master/ledger/sync/src/ledger_sync/ledger_sync_service.rs

identify_safe_blocks <- does NOT check the signatures. Checks the hashes

We need to check for each of the responders, there is a signature from the claimed responder. e.g.
[BlockData1, .... BlockData9] <- 


This blockdata comes with signatures. We need to verify that the signatures correspond to our known responder ids.
Then we need to verify that the responderids form a quorum slice for our quorum set.
This function checks the signatures: https://github.com/mobilecoinfoundation/mobilecoin/blob/master/api/src/convert/archive_block.rs#L50
Ask Chris: Where is the verify being called?


Notes: 
- The function (validate_append_block())[https://github.com/mobilecoinfoundation/mobilecoin/blob/617115f70740bf4030825167e3dc66c168fcd8f3/ledger/db/src/ledger_db.rs], called when the ledger writes a block to the database, makes a bunch of checks that the block is formatted correctly, not missing fields, etc., but does **not** check signatures.

Configure with a burn address with a particular memo type to watch for.
When seeing a relevant burn, sends the following:
BlockData for all nodes
relevant txo

there are two keys floating around: one is the message signing key, controlled by the node host and used in SCP messages, and the other is controlled by the enclave. The one controlled by the enclave is reset every time the enclave restarts, so would not be good for the light client. Since the BlockSignature signing key is part of the VerificationReport, which is signed by the enclave, it might be the enclave's key that signs blocks?
 Chris wasn't sure which one is the key appearing in the BlockSignature stuff - to chase this down, we should look into the consensus code where the signatures are actually made. If it is the enclave key, we will need to change the blockchain to add a signature from the message signing key as well.

This seems to be where consensus verifies signatures:
https://github.com/mobilecoinfoundation/mobilecoin/blob/master/peers/src/consensus_msg.rs#L182
https://github.com/mobilecoinfoundation/mobilecoin/blob/master/consensus/service/src/api/peer_api_service.rs#L185

The block is signed here:
https://github.com/mobilecoinfoundation/mobilecoin/blob/86fda4d27e445b66dca8661dbb72c75199bed002/consensus/enclave/impl/src/lib.rs#L488

This is where responder id is set:
https://github.com/mobilecoinfoundation/mobilecoin/blob/2c26aec103e8b76b86268412d17efc6ff4fe714e/consensus/enclave/impl/src/lib.rs#L481

The block signing key is ultimately read into the enclave from a file specified in the enclave config:
https://github.com/mobilecoinfoundation/mobilecoin/blob/e57b6902aee60be45b78b496c1bef781746e4389/consensus/service/src/bin/main.rs#L53

Example of view key matching the burn address:
https://github.com/mobilecoinfoundation/mobilecoin/blob/2c26aec103e8b76b86268412d17efc6ff4fe714e/mobilecoind/src/service.rs#L4791

The key that is read from disk is not the message signing key. Because that is sealed before writing to disk that still doesn't survive the enclave reboots.

Msg signing key is in the metadata as node signing key:
Pr where metadata was modified: https://github.com/mobilecoinfoundation/mobilecoin/pull/2058

Metadata validator can be used to validate the metadata signature: https://github.com/mobilecoinfoundation/mobilecoin/blob/2c26aec103e8b76b86268412d17efc6ff4fe714e/blockchain/validators/src/metadata/mod.rs#L35

it is also checked on conversion: https://github.com/mobilecoinfoundation/mobilecoin/blob/2c26aec103e8b76b86268412d17efc6ff4fe714e/blockchain/types/src/block_metadata.rs#L114

The metadatavalidator: https://github.com/mobilecoinfoundation/mobilecoin/pull/2131

Archive block verifies the metadata node signing key as well:https://github.com/mobilecoinfoundation/mobilecoin/blob/2c26aec103e8b76b86268412d17efc6ff4fe714e/api/src/convert/archive_block.rs#L56

ReqwestTransactionsFetcher's methods get_origin_block_and_transactions, get_block_by_index, etc. try to pull the block from one specific url (the most recently used): https://github.com/mobilecoinfoundation/mobilecoin/blob/e57b6902aee60be45b78b496c1bef781746e4389/ledger/sync/src/reqwest_transactions_fetcher.rs#L266