use crate::{Builder, Signing};

/// List of known builders with their details, ordered by block production.
pub static BUILDERS: &[Builder] = &[
    // Blocks: 22,454
    Builder {
        name: "Titan Builder",
        identifier: "titan",
        website: "https://titanbuilder.xyz",
        searcher_rpc: "https://rpc.titanbuilder.xyz",
        mev_share_rpc: Some("https://rpc.titanbuilder.xyz"),
        extra_data: Some("Titan (titanbuilder.xyz)"),
        signing: Signing::Optional,
        account_required: false,
    },
    // Blocks: 12,121
    Builder {
        name: "Beaver Build",
        identifier: "beaverbuild",
        website: "https://beaverbuild.org",
        searcher_rpc: "https://rpc.beaverbuild.org",
        mev_share_rpc: Some("https://mevshare-rpc.beaverbuild.org"),
        extra_data: Some("beaverbuild.org"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 8,080
    Builder {
        name: "BuilderNet",
        identifier: "buildernet",
        website: "https://buildernet.org",
        searcher_rpc: "https://direct-us.buildernet.org",
        mev_share_rpc: None,
        extra_data: Some("BuilderNet"),
        signing: Signing::Required,
        account_required: false,
    },
    // Blocks: 4,336
    Builder {
        name: "Flashbots",
        identifier: "flashbots",
        website: "https://flashbots.net",
        searcher_rpc: "https://relay.flashbots.net",
        mev_share_rpc: Some("https://rpc.flashbots.net"),
        extra_data: Some("BuilderNet (Flashbots)"),
        signing: Signing::Required,
        account_required: false,
    },
    // Blocks: 1,982
    Builder {
        name: "Quasar Builder",
        identifier: "quasar",
        website: "https://quasar.win",
        searcher_rpc: "https://rpc.quasar.win",
        mev_share_rpc: Some("https://rpc.quasar.win"),
        extra_data: Some("Quasar (quasar.win)"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 1,057
    Builder {
        name: "BTCS Builder",
        identifier: "btcs",
        website: "https://btcs.com",
        searcher_rpc: "https://rpc.btcs.com",
        mev_share_rpc: Some("https://flashbots.btcs.com"),
        extra_data: Some("Builder+ www.btcs.com/builder"),
        signing: Signing::Optional,
        account_required: false,
    },
    // Blocks: 797
    Builder {
        name: "rsync Builder",
        identifier: "rsync",
        website: "https://rsync-builder.xyz",
        searcher_rpc: "https://rsync-builder.xyz",
        mev_share_rpc: Some("https://rsync-builder.xyz"),
        extra_data: Some("rsync-builder.xyz"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 101
    Builder {
        name: "Bob the Builder",
        identifier: "bobthebuilder",
        website: "https://bobthebuilder.xyz",
        searcher_rpc: "https://rpc.bobthebuilder.xyz",
        mev_share_rpc: None,
        extra_data: Some("bobTheBuilder.xyz"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 24
    Builder {
        name: "BlockBeelder",
        identifier: "blockbeelder",
        website: "https://blockbeelder.com",
        searcher_rpc: "https://blockbeelder.com/rpc",
        mev_share_rpc: Some("https://blockbeelder.com/rpc"),
        extra_data: Some("https://blockbeelder.com"),
        signing: Signing::Optional,
        account_required: false,
    },
    // Blocks: 18
    Builder {
        name: "Boba Builder",
        identifier: "bobabuilder",
        website: "https://blockscout.com",
        searcher_rpc: "https://boba-builder.com/searcher",
        mev_share_rpc: Some("https://boba-builder.com/searcher/bundle"),
        extra_data: Some("boba-builder.com"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 17
    Builder {
        name: "BuildAI",
        identifier: "buildai",
        website: "https://buildai.net",
        searcher_rpc: "https://Buildai.net",
        mev_share_rpc: Some("https://buildai.net"),
        extra_data: Some("BuildAI (https://buildai.net)"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 12
    Builder {
        name: "Eureka Builder",
        identifier: "eureka",
        website: "https://eurekabuilder.xyz",
        searcher_rpc: "https://rpc.eurekabuilder.xyz",
        mev_share_rpc: Some("https://rpc.eurekabuilder.xyz"),
        extra_data: Some("Eureka (eurekabuilder.xyz)"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    // Blocks: 8
    Builder {
        name: "bloXroute",
        identifier: "bloxroute",
        website: "https://bloxroute.com",
        searcher_rpc: "https://mev.api.blxrbdn.com",
        mev_share_rpc: Some("https://rpc-builder.blxrbdn.com"),
        extra_data: Some("Powered by bloXroute"),
        signing: Signing::NotSupported,
        account_required: false,
    },
];

/// Other builders without recent block production data.
pub static OTHER_BUILDERS: &[Builder] = &[];
