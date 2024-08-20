use std::str::FromStr;

use lampo_common::ldk::routing::gossip::NodeId;
use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::Builder;

fn main() -> anyhow::Result<()> {
    lampo_common::logger::init("debug", None)?;

    let mut builder = Builder::new();
    builder.set_network(Network::Bitcoin);
    builder.set_esplora_server("https://blockstream.info/api".to_string());
    builder.set_log_level(ldk_node::LogLevel::Gossip);
    builder.set_log_dir_path("/home/vincent/ldk".to_owned());
    let node = builder.build()?;

    node.start()?;

    // We noted that ldk can have problem to fetch gossip from
    // core lightning.
    //
    // 03e2408a49f07d2f4083a47344138ef89e7617e63919202c92aa8d49b574a560ae@65.108.246.14:19736
    let add = SocketAddress::from_str("8.210.134.135:26658").unwrap();
    let node_id =
        PublicKey::from_str("0294ac3e099def03c12a37e30fe5364b1223fd60069869142ef96580c8439c2e0a")
            .unwrap();
    node.connect(node_id, add, true)?;
    loop {
        let channels = node.network_graph().list_channels();
        log::info!("number of network chennel update `{}`", channels.len());
        let bruce_node =
            NodeId::from_str("029ef2ce43571727104099576c633b2233bfeb8dc18b476f93540a32207da9e9a4")
                .unwrap();
        let bruce_channel = node.network_graph().node(&bruce_node);
        log::info!("{:?}", bruce_channel);
    }
}
