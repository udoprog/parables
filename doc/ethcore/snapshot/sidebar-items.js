initSidebarItems({"enum":[["Error","Snapshot-related errors."],["RestorationStatus","Statuses for restorations."]],"fn":[["chunk_secondary","Create and write out all secondary chunks to disk, returning a vector of all the hashes of secondary chunks created."],["chunk_state","Walk the given state database starting from the given root, creating chunks and writing them out."],["take_snapshot","Take a snapshot using the given blockchain, starting block hash, and database, writing into the given writer."],["verify_old_block","Verify an old block with the given header, engine, blockchain, body. If `always` is set, it will perform the fullest verification possible. If not, it will take a random sample to determine whether it will do heavy or light verification."]],"mod":[["io","Snapshot i/o. Ways of writing and reading snapshots. This module supports writing and reading snapshots of two different formats: packed and loose. Packed snapshots are written to a single file, and loose snapshots are written to multiple files in one directory."],["service","Snapshot network service implementation."]],"struct":[["BasicAccount","Basic account type."],["ManifestData","Manifest data."],["PoaSnapshot","Snapshot creation and restoration for PoA chains. Chunk format:"],["PowRebuilder","Rebuilder for proof-of-work chains. Does basic verification for all blocks, but `PoW` verification for some. Blocks must be fed in-order."],["PowSnapshot","Snapshot creation and restoration for PoW chains. This includes blocks from the head of the chain as a loose assurance that the chain is valid."],["Progress","A progress indicator for snapshots."],["StateRebuilder","Used to rebuild the state trie piece by piece."],["Watcher","A `ChainNotify` implementation which will trigger a snapshot event at certain block numbers."]],"trait":[["Rebuilder","Restore from secondary snapshot chunks."],["SnapshotComponents","Components necessary for snapshot creation and restoration."],["SnapshotService","The interface for a snapshot network service. This handles:    - restoration of snapshots to temporary databases.    - responding to queries for snapshot manifests and chunks"]],"type":[["ChunkSink","A sink for produced chunks."]]});