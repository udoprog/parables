initSidebarItems({"enum":[["ErrorKind","The kind of an error."],["Readiness","Transaction readiness."]],"mod":[["scoring","A transactions ordering abstraction."]],"struct":[["Error","The Error type."],["LightStatus","Light pool status. This status is cheap to compute and can be called frequently."],["NoopListener","A no-op implementation of `Listener`."],["Options","Transaction Pool options."],["PendingIterator","An iterator over all pending (ready) transactions. NOTE: the transactions are not removed from the queue. You might remove them later by calling `cull`."],["Pool","A transaction pool."],["Status","A full queue status. To compute this status it is required to provide `Ready`. NOTE: To compute the status we need to visit each transaction in the pool."],["Transaction","Internal representation of transaction."]],"trait":[["Listener","Transaction pool listener."],["Ready","A readiness indicator."],["VerifiedTransaction","Already verified transaction that can be safely queued."],["Verifier","Transaction verification."]]});