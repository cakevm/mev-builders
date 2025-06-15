use mev_builders_macros::include_builders;

#[cfg(feature = "tools")]
pub mod tools;

/// List of known builders with their details, ordered by landed blocks.
pub static BUILDERS: &[Builder] = include_builders!("data/builders.json", "data/builders_stats.json");

/// Indicates if a builder requires signing for bundles using `X-Flashbots-Signature`.
///
/// All builder besides Flashbots have signing as optional or not supported.
/// If provided, the builder may give better priority to signed bundles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signing {
    /// Bundle gets rejected if not signed.
    Required,
    /// Signing is optional and may give better priority.
    Optional,
    /// Builder does not support signing.
    NotSupported,
}

impl Signing {
    /// Returns true if the builder requires signing for bundles.
    pub const fn is_required(&self) -> bool {
        matches!(self, Signing::Required)
    }
    /// Returns true if the builder supports signing, but it is optional.
    pub fn is_optional(&self) -> bool {
        matches!(self, Signing::Optional)
    }
    /// Returns true if the builder does not support signing.
    pub fn is_not_supported(&self) -> bool {
        matches!(self, Signing::NotSupported)
    }
}

/// Represents a builder with its details.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// The extra data provided by the builder in a block. Spaces at the start and end are trimmed. Be aware that everyone can impersonate the builder using this extra data.
    pub extra_data: Option<&'a str>,
    /// Indicates if the builder requires signing for bundles.
    pub signing: Signing,
    /// Requires account to use the RPC.
    pub account_required: bool,
    /// Number of blocks landed by this builder.
    pub blocks: u64,
}

impl<'a> Builder<'a> {
    /// Requires special handling for the builder.
    pub fn requires_extra_handling(&self) -> bool {
        // buildernet: requires custom cert or insecure connection. See: https://buildernet.org/docs/api#example-request-
        // bloxroute: requires an account to use the RPC.
        matches!(self.identifier, "buildernet" | "bloxroute")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_that_min_five_builders_are_present() {
        // Ensure that there are at least 5 builders defined
        assert!(BUILDERS.len() >= 5, "There should be at least 5 builders defined");
    }

    #[test]
    fn verify_builders_sorted_by_blocks() {
        // Verify that builders are sorted by blocks in descending order
        for i in 1..BUILDERS.len() {
            assert!(
                BUILDERS[i - 1].blocks >= BUILDERS[i].blocks,
                "Builders should be sorted by blocks in descending order. {} ({} blocks) comes before {} ({} blocks)",
                BUILDERS[i - 1].name,
                BUILDERS[i - 1].blocks,
                BUILDERS[i].name,
                BUILDERS[i].blocks
            );
        }
    }

    #[test]
    fn test_required_fields_not_empty() {
        for builder in BUILDERS {
            assert!(!builder.name.is_empty(), "Builder name should not be empty");
            assert!(!builder.identifier.is_empty(), "Builder identifier should not be empty");
            assert!(!builder.website.is_empty(), "Builder website should not be empty");
            assert!(!builder.searcher_rpc.is_empty(), "Builder searcher RPC should not be empty");
        }
    }

    #[test]
    fn test_identifier_format() {
        for builder in BUILDERS {
            assert!(
                builder.identifier.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
                "Builder identifier should be lowercase alphanumeric: {}",
                builder.identifier
            );
        }
    }

    #[test]
    fn test_urls_valid_format() {
        for builder in BUILDERS {
            // Check website URL
            assert!(
                builder.website.starts_with("https://") || builder.website.starts_with("http://"),
                "Builder website should start with 'https://' or 'http://': {}",
                builder.website
            );

            // Check searcher RPC URL
            assert!(
                builder.searcher_rpc.starts_with("https://") || builder.searcher_rpc.starts_with("http://"),
                "Builder searcher_rpc should start with 'https://' or 'http://': {}",
                builder.searcher_rpc
            );

            // Check MEV share RPC URL if present
            if let Some(mev_share_rpc) = builder.mev_share_rpc {
                assert!(
                    mev_share_rpc.starts_with("https://") || mev_share_rpc.starts_with("http://"),
                    "Builder mev_share_rpc should start with 'https://' or 'http://': {}",
                    mev_share_rpc
                );
            }
        }
    }

    #[test]
    fn test_unique_names() {
        let mut name_set = std::collections::HashSet::new();
        for builder in BUILDERS {
            assert!(name_set.insert(builder.name), "Duplicate builder name found: {}", builder.name);
        }
    }

    #[test]
    fn test_unique_identifiers() {
        let mut identifier_set = std::collections::HashSet::new();
        for builder in BUILDERS {
            assert!(identifier_set.insert(builder.identifier), "Duplicate builder identifier found: {}", builder.identifier);
        }
    }

    #[test]
    fn test_unique_websites() {
        let mut website_set = std::collections::HashSet::new();
        for builder in BUILDERS {
            assert!(website_set.insert(builder.website), "Duplicate builder website found: {}", builder.website);
        }
    }

    #[test]
    fn test_unique_rpc_endpoints() {
        let mut rpc_set = std::collections::HashSet::new();
        for builder in BUILDERS {
            assert!(rpc_set.insert(builder.searcher_rpc), "Duplicate searcher RPC found: {}", builder.searcher_rpc);
        }
    }

    #[test]
    fn test_unique_mev_share_endpoints() {
        let mut mev_share_rpc_set = std::collections::HashSet::new();
        for builder in BUILDERS {
            if let Some(mev_share_rpc) = builder.mev_share_rpc {
                assert!(mev_share_rpc_set.insert(mev_share_rpc), "Duplicate MEV share RPC found: {}", mev_share_rpc);
            }
        }
    }
}
