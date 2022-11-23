// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subxt is a library to **sub**mit e**xt**rinsics to a [substrate](https://github.com/paritytech/substrate) node via RPC.
//!
//! The generated Subxt API exposes the ability to:
//! - [Submit extrinsics](https://docs.substrate.io/v3/concepts/extrinsics/) (Calls)
//! - [Query storage](https://docs.substrate.io/v3/runtime/storage/) (Storage)
//! - [Query constants](https://docs.substrate.io/how-to-guides/v3/basics/configurable-constants/) (Constants)
//! - [Subscribe to events](https://docs.substrate.io/v3/runtime/events-and-errors/) (Events)
//!
//! # Initializing the API client
//!
//! To interact with a node, you'll need to construct a client.
//!
//! ```no_run
//! use subxt::{OnlineClient, PolkadotConfig};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
//! # }
//! ```
//!
//! This default client connects to a locally running node, but can be configured to point anywhere.
//! Additionally, an [`crate::OfflineClient`] is available to perform operations that don't require a
//! network connection to a node.
//!
//! The client takes a type parameter, here [`crate::PolkadotConfig`], which bakes in assumptions about
//! the structure of extrinsics and the underlying types used by the node for things like block numbers.
//! If the node you'd like to interact with deviates from Polkadot or the default Substrate node in these
//! areas, you'll need to configure them by implementing the [`crate::config::Config`] type yourself.
//!
//! # Generating runtime types
//!
//! Subxt can optionally generate types at compile time to help you interact with a node. These types are
//! generated using metadata which can be downloaded from a node using the [subxt-cli](https://crates.io/crates/subxt-cli)
//! tool. These generated types provide a degree of type safety when interacting with a node that is compatible with
//! the metadata that they were generated using. We also do runtime checks in case the node you're talking to has
//! deviated from the types you're using to communicate with it (see below).
//!
//! To generate the types, use the `subxt` macro and point it at the metadata you've downloaded, like so:
//!
//! ```ignore
//! #[subxt::subxt(runtime_metadata_path = "metadata.scale")]
//! pub mod node_runtime { }
//! ```
//!
//! For more information, please visit the [subxt-codegen](https://docs.rs/subxt-codegen/latest/subxt_codegen/)
//! documentation.
//!
//! You can opt to skip this step and use dynamic queries to talk to nodes instead, which can be useful in some cases,
//! but doesn't provide any type safety.
//!
//! # Interacting with the API
//!
//! Once instantiated, a client exposes four core functions:
//! - `.tx()` for submitting extrinsics/transactions. See [`crate::tx::TxClient`] for more details, or see
//!   the [balance_transfer](../examples/examples/balance_transfer.rs) example.
//! - `.storage()` for fetching and iterating over storage entries. See [`crate::storage::StorageClient`] for more details, or see
//!   the [fetch_staking_details](../examples/examples/fetch_staking_details.rs) example.
//! - `.constants()` for getting hold of constants. See [`crate::constants::ConstantsClient`] for more details, or see
//!   the [fetch_constants](../examples/examples/fetch_constants.rs) example.
//! - `.events()` for subscribing/obtaining events. See [`crate::events::EventsClient`] for more details, or see:
//!    - [subscribe_all_events](../examples/examples/subscribe_all_events.rs): Subscribe to events emitted from blocks.
//!    - [subscribe_one_event](../examples/examples/subscribe_one_event.rs): Subscribe and filter by one event.
//!    - [subscribe_some_events](../examples/examples/subscribe_some_events.rs): Subscribe and filter event.
//!
//! # Static Metadata Validation
//!
//! If you use types generated by the [`crate::subxt`] macro, there is a chance that they will fall out of sync
//! with the actual state of the node you're trying to interact with.
//!
//! When you attempt to use any of these static types to interact with a node, Subxt will validate that they are
//! still compatible and issue an error if they have deviated.
//!
//! Additionally, you can validate that the entirety of the statically generated code aligns with a node like so:
//!
//! ```no_run
//! use subxt::{OnlineClient, PolkadotConfig};
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! # #[tokio::main]
//! # async fn main() {
//! let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
//!
//! if let Err(_e) = polkadot::validate_codegen(&api) {
//!     println!("Generated code is not up to date with node we're connected to");
//! }
//! # }
//! ```
//! ## Opting out of static validation
//!
//! The static types that are used to query/access information are validated by default, to make sure that they are
//! compatible with the node being queried. You can generally call `.unvalidated()` on these static types to
//! disable this validation.
//!
//! # Runtime Updates
//!
//! The node you're connected to may occasionally perform runtime updates while you're connected, which would ordinarily
//! leave the runtime state of the node out of sync with the information Subxt requires to do things like submit
//! transactions.
//!
//! If this is a concern, you can use the `UpdateClient` API to keep the `RuntimeVersion` and `Metadata` of the client
//! synced with the target node.
//!
//! Please visit the [subscribe_runtime_updates](../examples/examples/subscribe_runtime_updates.rs) example for more details.

#![deny(
    bad_style,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_extern_crates,
    clippy::all
)]
#![allow(clippy::type_complexity)]

// Suppress an unused dependency warning because tokio is
// only used in example code snippets at the time of writing.
#[cfg(test)]
use tokio as _;

pub use subxt_macro::subxt;

// Used to enable the js feature for wasm.
#[cfg(target_arch = "wasm32")]
pub use getrandom as _;

#[cfg(all(feature = "jsonrpsee-ws", feature = "jsonrpsee-web"))]
std::compile_error!("Both the features `jsonrpsee-ws` and `jsonrpsee-web` are enabled which are mutually exclusive");

pub mod blocks;
pub mod client;
pub mod config;
pub mod constants;
pub mod dynamic;
pub mod error;
pub mod events;
pub mod metadata;
pub mod rpc;
pub mod storage;
pub mod tx;
pub mod utils;

// Expose a few of the most common types at root,
// but leave most types behind their respective modules.
pub use crate::{
    client::{
        OfflineClient,
        OnlineClient,
    },
    config::{
        Config,
        PolkadotConfig,
        SubstrateConfig,
    },
    error::Error,
    metadata::Metadata,
};

/// Re-export external crates that are made use of in the subxt API.
pub mod ext {
    pub use bitvec;
    pub use codec;
    pub use frame_metadata;
    pub use scale_value;
    pub use sp_core;
    pub use sp_runtime;
}
