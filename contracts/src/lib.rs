//! SpeedBet Arena - Head-to-Head Crypto Prediction Duels
//!
//! This crate defines the ABI (Application Binary Interface) for the SpeedBet Arena
//! application, which enables two players to bet against each other on 60-second
//! crypto price movements.

#![deny(missing_docs)]

pub mod operations;
pub mod state;
pub mod types;

use async_graphql::{Request, Response};
use linera_sdk::abi::{ContractAbi, ServiceAbi};
pub use operations::{Message, Operation};

/// The ABI definition for SpeedBet Arena
pub struct SpeedBetAbi;

impl ContractAbi for SpeedBetAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for SpeedBetAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Re-export commonly used types
pub use operations::InstantiationArgument;
pub use state::SpeedBetState;
pub use types::*;
