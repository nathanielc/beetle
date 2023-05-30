use iroh_bitswap::BitswapEvent;
use libp2p::{
    autonat, dcutr, gossipsub, identify,
    kad::KademliaEvent,
    mdns, ping, relay,
    swarm::{behaviour::toggle::Toggle, dummy, NetworkBehaviour},
};

use super::peer_manager::PeerManagerEvent;

/// Event type which is emitted from the [`NodeBehaviour`].
///
/// [`NodeBehaviour`]: crate::behaviour::NodeBehaviour
#[derive(Debug)]
pub enum Event<B: NetworkBehaviour> {
    Ping(ping::Event),
    Identify(Box<identify::Event>),
    Kademlia(KademliaEvent),
    Mdns(mdns::Event),
    Bitswap(BitswapEvent),
    Autonat(autonat::Event),
    Relay(relay::Event),
    RelayClient(relay::client::Event),
    Dcutr(dcutr::Event),
    Gossipsub(gossipsub::Event),
    PeerManager(PeerManagerEvent),
    Custom(B::OutEvent),
}

impl<B: NetworkBehaviour> From<ping::Event> for Event<B> {
    fn from(event: ping::Event) -> Self {
        Event::Ping(event)
    }
}

impl<B: NetworkBehaviour> From<identify::Event> for Event<B> {
    fn from(event: identify::Event) -> Self {
        Event::Identify(Box::new(event))
    }
}

impl<B: NetworkBehaviour> From<KademliaEvent> for Event<B> {
    fn from(event: KademliaEvent) -> Self {
        Event::Kademlia(event)
    }
}

impl<B: NetworkBehaviour> From<mdns::Event> for Event<B> {
    fn from(event: mdns::Event) -> Self {
        Event::Mdns(event)
    }
}

impl<B: NetworkBehaviour> From<BitswapEvent> for Event<B> {
    fn from(event: BitswapEvent) -> Self {
        Event::Bitswap(event)
    }
}

impl<B: NetworkBehaviour> From<gossipsub::Event> for Event<B> {
    fn from(event: gossipsub::Event) -> Self {
        Event::Gossipsub(event)
    }
}

impl<B: NetworkBehaviour> From<autonat::Event> for Event<B> {
    fn from(event: autonat::Event) -> Self {
        Event::Autonat(event)
    }
}

impl<B: NetworkBehaviour> From<relay::Event> for Event<B> {
    fn from(event: relay::Event) -> Self {
        Event::Relay(event)
    }
}

impl<B: NetworkBehaviour> From<relay::client::Event> for Event<B> {
    fn from(event: relay::client::Event) -> Self {
        Event::RelayClient(event)
    }
}

impl<B: NetworkBehaviour> From<dcutr::Event> for Event<B> {
    fn from(event: dcutr::Event) -> Self {
        Event::Dcutr(event)
    }
}

impl<B: NetworkBehaviour> From<PeerManagerEvent> for Event<B> {
    fn from(event: PeerManagerEvent) -> Self {
        Event::PeerManager(event)
    }
}

// Implement this instance of the trait so that the Toggle<dummy::Behaviour>
// may be used when no custom behavior is desired.
impl From<void::Void> for Event<Toggle<dummy::Behaviour>> {
    fn from(event: void::Void) -> Self {
        Event::Custom(event)
    }
}
