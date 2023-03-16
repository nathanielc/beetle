//! A p2p instance listening on a memory rpc channel.
use iroh_p2p::config::Config;
use iroh_p2p::{DiskStorage, Keychain, Node, PeerId};
use iroh_rpc_types::p2p::P2pAddr;
use tokio::task;
use tokio::task::JoinHandle;
use tracing::error;

/// Starts a new p2p node, using the given mem rpc channel.
pub async fn start(rpc_addr: P2pAddr, config: Config) -> anyhow::Result<(PeerId, JoinHandle<()>)> {
    let kc = Keychain::<DiskStorage>::new(config.key_store_path.clone()).await?;

    let mut p2p = Node::new(config, rpc_addr, kc).await?;
    let peer_id = p2p.local_peer_id().to_owned();

    // Start services
    let p2p_task = task::spawn(async move {
        if let Err(err) = p2p.run().await {
            error!("{:?}", err);
        }
    });

    Ok((peer_id, p2p_task))
}
