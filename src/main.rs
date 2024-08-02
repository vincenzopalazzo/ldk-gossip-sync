use std::str::FromStr;

use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::Builder;

fn main() -> anyhow::Result<()> {
    lampo_common::logger::init("debug", None)?;

    let mut builder = Builder::new();
    builder.set_network(Network::Bitcoin);
    builder.set_esplora_server("https://blockstream.info/api".to_string());

    let node = builder.build()?;

    node.start()?;

    // 03e2408a49f07d2f4083a47344138ef89e7617e63919202c92aa8d49b574a560ae@65.108.246.14:19736
    let add = SocketAddress::from_str("65.108.246.14:19736").unwrap();
    let node_id =
        PublicKey::from_str("03e2408a49f07d2f4083a47344138ef89e7617e63919202c92aa8d49b574a560ae")
            .unwrap();
    node.connect(node_id, add, true)?;
    loop {
        let channels = node.network_graph().list_channels();
        log::info!("number of network chennel update `{}`", channels.len());
    }
}
