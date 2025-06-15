use proc_macro::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct BuilderJson {
    name: String,
    identifier: String,
    website: String,
    searcher_rpc: String,
    mev_share_rpc: Option<String>,
    extra_data: Option<String>,
    signing: String,
    account_required: bool,
}

#[proc_macro]
pub fn include_builders(_input: TokenStream) -> TokenStream {
    // Get the manifest directory (where Cargo.toml is)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // Read both JSON files
    let builders_path = Path::new(&manifest_dir).join("data/builders.json");
    let stats_path = Path::new(&manifest_dir).join("data/builders_stats.json");

    let builders_json = fs::read_to_string(&builders_path).expect(&format!("Failed to read builders.json from {:?}", builders_path));
    let stats_json = fs::read_to_string(&stats_path).expect(&format!("Failed to read builders_stats.json from {:?}", stats_path));

    // Parse JSON using structs
    let builders_data: Vec<BuilderJson> = serde_json::from_str(&builders_json).expect("Failed to parse builders.json");
    let stats_data: HashMap<String, u64> = serde_json::from_str(&stats_json).expect("Failed to parse builders_stats.json");

    // Create a vec of (builder, blocks) tuples
    let mut builders_with_blocks: Vec<(BuilderJson, u64)> = builders_data
        .into_iter()
        .map(|builder| {
            let blocks = builder.extra_data.as_ref().and_then(|extra_data| stats_data.get(extra_data)).copied().unwrap_or(0);
            (builder, blocks)
        })
        .collect();

    // Sort builders by blocks count (descending)
    builders_with_blocks.sort_by(|a, b| b.1.cmp(&a.1));

    // Generate tokens for each builder
    let builder_tokens: Vec<_> = builders_with_blocks
        .iter()
        .map(|(builder, blocks)| {
            let name = &builder.name;
            let identifier = &builder.identifier;
            let website = &builder.website;
            let searcher_rpc = &builder.searcher_rpc;

            let mev_share_rpc_tokens = match &builder.mev_share_rpc {
                Some(rpc) => quote! { Some(#rpc) },
                None => quote! { None },
            };

            let extra_data_tokens = match &builder.extra_data {
                Some(data) => quote! { Some(#data) },
                None => quote! { None },
            };

            let signing_tokens = match builder.signing.as_str() {
                "Required" => quote! { crate::Signing::Required },
                "Optional" => quote! { crate::Signing::Optional },
                _ => quote! { crate::Signing::NotSupported },
            };

            let account_required = builder.account_required;

            quote! {
                crate::Builder {
                    name: #name,
                    identifier: #identifier,
                    website: #website,
                    searcher_rpc: #searcher_rpc,
                    mev_share_rpc: #mev_share_rpc_tokens,
                    extra_data: #extra_data_tokens,
                    signing: #signing_tokens,
                    account_required: #account_required,
                    blocks: #blocks,
                }
            }
        })
        .collect();

    let expanded = quote! {
        &[
            #(#builder_tokens),*
        ]
    };

    TokenStream::from(expanded)
}
