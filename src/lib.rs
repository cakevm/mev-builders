mod builders;
pub use builders::{BUILDERS, OTHER_BUILDERS};

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
