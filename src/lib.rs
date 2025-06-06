/// Indicates if a builder requires signing for bundles using `X-Flashbots-Signature`.
///
/// All builder besides Flashbots have signing as optional or not supported.
/// If provided, the builder may give better priority to signed bundles.
pub enum Signing {
    /// Bundle gets rejected if not signed.
    Required,
    /// Signing is optional and may give better priority.
    Optional,
    /// Builder does not support signing.
    NotSupported,
}

/// Represents a builder with its details.
pub struct Builder<'a> {
    /// Human-readable name of the builder.
    pub name: &'a str,
    /// Unique identifier for the builder.
    pub identifier: &'a str,
    /// Website URL for the builder.
    pub website: &'a str,
    /// RPC endpoint for the searcher.
    pub searcher_rpc: &'a str,
    /// Optional RPC endpoint for MEV share.
    pub mev_share_rpc: Option<&'a str>,
    /// Indicates if the builder requires signing for bundles.
    pub signing: Signing,
    /// Requires account to use the RPC.
    pub account_required: bool,
}

/// List of known builders with their details.
pub static BUILDERS: &[Builder] = &[
    Builder {
        name: "Flashbots",
        identifier: "flashbots",
        website: "https://flashbots.net",
        searcher_rpc: "https://relay.flashbots.net",
        mev_share_rpc: Some("https://rpc.flashbots.net"),
        signing: Signing::Required,
        account_required: false,
    },
    Builder {
        name: "Beaver Build",
        identifier: "beaverbuild",
        website: "https://beaverbuild.org",
        searcher_rpc: "https://rpc.beaverbuild.org",
        mev_share_rpc: Some("https://mevshare-rpc.beaverbuild.org"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Titan Builder",
        identifier: "titan",
        website: "https://titanbuilder.xyz",
        searcher_rpc: "https://rpc.titanbuilder.xyz",
        mev_share_rpc: Some("https://rpc.titanbuilder.xyz"),
        // If provided there is a priority see: https://docs.titanbuilder.xyz/authentication
        signing: Signing::Optional,
        account_required: false,
    },
    Builder {
        name: "rsync Builder",
        identifier: "rsync",
        website: "https://rsync-builder.xyz",
        searcher_rpc: "https://rsync-builder.xyz",
        mev_share_rpc: Some("https://rsync-builder.xyz"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "BTCS Builder",
        identifier: "btcs",
        website: "https://btcs.com",
        searcher_rpc: "https://rpc.btcs.com",
        mev_share_rpc: Some("https://flashbots.btcs.com"),
        signing: Signing::Optional,
        account_required: false,
    },
    Builder {
        name: "BlockBeelder",
        identifier: "blockbeelder",
        website: "https://blockbeelder.com",
        searcher_rpc: "https://blockbeelder.com/rpc",
        mev_share_rpc: Some("https://blockbeelder.com/rpc"),
        signing: Signing::Optional,
        account_required: false,
    },
    Builder {
        name: "BuildAI",
        identifier: "buildai",
        website: "https://buildai.net",
        searcher_rpc: "https://Buildai.net",
        mev_share_rpc: Some("https://buildai.net"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Bob the Builder",
        identifier: "bobthebuilder",
        website: "https://bobthebuilder.xyz",
        searcher_rpc: "https://rpc.bobthebuilder.xyz",
        mev_share_rpc: None,
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "JetBuilder",
        identifier: "jetbuilder",
        website: "https://jetbuilder.xyz",
        searcher_rpc: "https://rpc.mevshare.jetbldr.xyz",
        mev_share_rpc: None,
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Gigabuilder",
        identifier: "gigabuilder",
        website: "https://gigabuilder.io",
        searcher_rpc: "https://rpc.gigabuilder.io",
        mev_share_rpc: None,
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "SmithBot",
        identifier: "smithbot",
        website: "https://smithbot.xyz",
        searcher_rpc: "https://smithbot.xyz",
        mev_share_rpc: None,
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Builder0x69",
        identifier: "builder0x69",
        website: "https://builder0x69.io",
        searcher_rpc: "https://builder0x69.io",
        mev_share_rpc: Some("https://builder0x69.io"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Gmbit Builder",
        identifier: "gmbit",
        website: "https://gmbit.co",
        searcher_rpc: "https://builder.gmbit.co/rpc",
        mev_share_rpc: Some("https://builder.gmbit.co"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Quasar Builder",
        identifier: "quasar",
        website: "https://quasar.win",
        searcher_rpc: "https://rpc.quasar.win",
        mev_share_rpc: Some("https://rpc.quasar.win"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Eureka Builder",
        identifier: "eureka",
        website: "https://eurekabuilder.xyz",
        searcher_rpc: "https://rpc.eurekabuilder.xyz",
        mev_share_rpc: Some("https://rpc.eurekabuilder.xyz"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "bloXroute",
        identifier: "bloxroute",
        website: "https://bloxroute.com",
        searcher_rpc: "https://mev.api.blxrbdn.com",
        mev_share_rpc: Some("https://rpc-builder.blxrbdn.com"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "n!",
        identifier: "blocknative",
        website: "https://nfactorial.xyz",
        searcher_rpc: "https://nfactorial.xyz",
        mev_share_rpc: None,
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "ETH builder",
        identifier: "blockscout",
        website: "https://eth-builder.com",
        searcher_rpc: "https://eth-builder.com",
        mev_share_rpc: None,
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "EigenPhi",
        identifier: "eigenphi",
        website: "https://eigenphi.io",
        searcher_rpc: "https://builder.eigenphi.io",
        mev_share_rpc: Some("https://builder.eigenphi.io"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Penguinbuild",
        identifier: "penguinbuild",
        website: "https://penguinbuild.org/",
        searcher_rpc: "https://rpc.penguinbuild.org",
        mev_share_rpc: Some("https://rpc.penguinbuild.org"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Boba Builder",
        identifier: "bobabuilder",
        website: "https://blockscout.com",
        searcher_rpc: "https://boba-builder.com/searcher",
        mev_share_rpc: Some("https://boba-builder.com/searcher/bundle"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "tbuilder",
        identifier: "tbuilder",
        website: "https://tbuilder.xyz",
        searcher_rpc: "https://rpc.tbuilder.xyz",
        mev_share_rpc: Some("https://flashbots.rpc.tbuilder.xyz"),
        signing: Signing::NotSupported,
        account_required: false,
    },
    Builder {
        name: "Payload",
        identifier: "payload",
        website: "https://payload.de",
        searcher_rpc: "https://rpc.payload.de",
        mev_share_rpc: Some("https://rpc.payload.de"),
        signing: Signing::NotSupported,
        account_required: false,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builders() {
        for builder in BUILDERS {
            // Ensure all required fields are populated
            assert!(!builder.name.is_empty(), "Builder name should not be empty");
            assert!(!builder.identifier.is_empty(), "Builder identifier should not be empty");
            assert!(!builder.website.is_empty(), "Builder website should not be empty");
            assert!(!builder.searcher_rpc.is_empty(), "Builder searcher RPC should not be empty");

            // Check if the identifier is a valid lowercase alphanumeric string
            assert!(
                builder.identifier.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
                "Builder identifier should be lowercase alphanumeric: {}",
                builder.identifier
            );

            // Starts with "https://" or "http://"
            assert!(
                builder.website.starts_with("https://") || builder.website.starts_with("http://"),
                "Builder website should start with 'https://' or 'http://': {}",
                builder.website
            );

            // Check if rpc starts with "https://" or "http://"
            assert!(
                builder.searcher_rpc.starts_with("https://") || builder.searcher_rpc.starts_with("http://"),
                "Builder searcher_rpc should start with 'https://' or 'http://': {}",
                builder.searcher_rpc
            );

            // Check if mev_share_rpc is None or starts with "https://" or "http://"
            if let Some(mev_share_rpc) = builder.mev_share_rpc {
                assert!(
                    mev_share_rpc.starts_with("https://") || mev_share_rpc.starts_with("http://"),
                    "Builder mev_share_rpc should start with 'https://' or 'http://': {}",
                    mev_share_rpc
                );
            }

            // Check that the name is unique
            let mut name_set = std::collections::HashSet::new();
            assert!(name_set.insert(builder.name), "Duplicate builder name found: {}", builder.name);

            // Check that the identifier is unique
            let mut identifier_set = std::collections::HashSet::new();
            assert!(identifier_set.insert(builder.identifier), "Duplicate builder identifier found: {}", builder.identifier);

            // Check that the website is unique
            let mut website_set = std::collections::HashSet::new();
            assert!(website_set.insert(builder.website), "Duplicate builder website found: {}", builder.website);

            // Check that rpc endpoints are unique
            let mut rpc_set = std::collections::HashSet::new();
            assert!(rpc_set.insert(builder.searcher_rpc), "Duplicate searcher RPC found: {}", builder.searcher_rpc);

            // Check that mev_share_rpc endpoints are unique
            let mut mev_share_rpc_set = std::collections::HashSet::new();
            if let Some(mev_share_rpc) = builder.mev_share_rpc {
                assert!(mev_share_rpc_set.insert(mev_share_rpc), "Duplicate MEV share RPC found: {}", mev_share_rpc);
            }
        }
    }
}
