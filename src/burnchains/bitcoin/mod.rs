/*
 copyright: (c) 2013-2018 by Blockstack PBC, a public benefit corporation.

 This file is part of Blockstack.

 Blockstack is free software. You may redistribute or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License or
 (at your option) any later version.

 Blockstack is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY, including without the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with Blockstack. If not, see <http://www.gnu.org/licenses/>.
*/

// This module is concerned with the implementation of the BitcoinIndexer
// structure and its methods and traits.

pub mod address;
pub mod bits;
pub mod blocks;
pub mod messages;
pub mod keys;
pub mod indexer;
pub mod network;
pub mod rpc;
pub mod spv;

use std::fmt;
use std::io;
use std::error;
use std::sync::Arc;

use std::sync::mpsc::SyncSender;

use bitcoin::network::serialize::Error as btc_serialize_error;
use bitcoin::util::hash::HexError as btc_hex_error;

use jsonrpc::Error as jsonrpc_error;

use burnchains::BurnchainBlock;
use burnchains::bitcoin::address::BitcoinAddress;
use burnchains::bitcoin::keys::BitcoinPublicKey;

use chainstate::burn::db::Error as burndb_error;

pub type PeerMessage = Arc<bitcoin::network::message::NetworkMessage>;
pub type BlockSender = SyncSender<Arc<BurnchainBlock<BitcoinAddress, BitcoinPublicKey>>>;

// Borrowed from Andrew Poelstra's rust-bitcoin 

/// Network error
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(io::Error),
    /// Socket mutex was poisoned
    SocketMutexPoisoned,
    /// Not connected to peer
    SocketNotConnectedToPeer,
    /// Serialization error 
    SerializationError(btc_serialize_error),
    /// Invalid Message to peer
    InvalidMessage(PeerMessage),
    /// Invalid Reply from peer
    InvalidReply,
    /// Invalid magic 
    InvalidMagic,
    /// Unhandled message 
    UnhandledMessage(PeerMessage),
    /// Functionality not implemented 
    NotImplemented,
    /// Connection is broken and ought to be re-established
    ConnectionBroken,
    /// Connection could not be (re-)established
    ConnectionError,
    /// general filesystem error
    FilesystemError(io::Error),
    /// Hashing error
    HashError(btc_hex_error),
    /// Non-contiguous header 
    NoncontiguousHeader,
    /// Missing header
    MissingHeader,
    /// Invalid target 
    InvalidPoW,
    /// RPC error with bitcoin 
    JSONRPCError(jsonrpc_error),
    /// Thread pipeline error (i.e. a receiving thread died)
    PipelineError,
    /// Wrong number of bytes for constructing an address
    InvalidByteSequence,
    /// Configuration error 
    ConfigError(String),
    /// Tried to synchronize to a point above the chain tip
    BlockchainHeight,
    /// Burn database error 
    DBError(burndb_error),
    /// Invalid argument 
    InvalidArgument
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            Error::SocketMutexPoisoned | Error::SocketNotConnectedToPeer => f.write_str(error::Error::description(self)),
            Error::SerializationError(ref e) => fmt::Display::fmt(e, f),
            Error::InvalidMessage(ref msg) => f.write_str(error::Error::description(self)),
            Error::InvalidReply => f.write_str(error::Error::description(self)),
            Error::InvalidMagic => f.write_str(error::Error::description(self)),
            Error::UnhandledMessage(ref msg) => f.write_str(error::Error::description(self)),
            Error::NotImplemented => f.write_str(error::Error::description(self)),
            Error::ConnectionBroken => f.write_str(error::Error::description(self)),
            Error::ConnectionError => f.write_str(error::Error::description(self)),
            Error::FilesystemError(ref e) => fmt::Display::fmt(e, f),
            Error::HashError(ref e) => fmt::Display::fmt(e, f),
            Error::NoncontiguousHeader => f.write_str(error::Error::description(self)),
            Error::MissingHeader => f.write_str(error::Error::description(self)),
            Error::InvalidPoW => f.write_str(error::Error::description(self)),
            Error::JSONRPCError(ref e) => fmt::Display::fmt(e, f),
            Error::PipelineError => f.write_str(error::Error::description(self)),
            Error::InvalidByteSequence => f.write_str(error::Error::description(self)),
            Error::ConfigError(ref e_str) => fmt::Display::fmt(e_str, f),
            Error::BlockchainHeight => f.write_str(error::Error::description(self)),
            Error::DBError(ref e) => fmt::Display::fmt(e, f),
            Error::InvalidArgument => f.write_str(error::Error::description(self))
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::SocketMutexPoisoned | Error::SocketNotConnectedToPeer => None,
            Error::SerializationError(ref e) => Some(e),
            Error::InvalidMessage(ref msg) => None,
            Error::InvalidReply => None,
            Error::InvalidMagic => None,
            Error::UnhandledMessage(ref msg) => None,
            Error::NotImplemented => None,
            Error::ConnectionBroken => None,
            Error::ConnectionError => None,
            Error::FilesystemError(ref e) => Some(e),
            Error::HashError(ref e) => Some(e),
            Error::NoncontiguousHeader => None,
            Error::MissingHeader => None,
            Error::InvalidPoW => None,
            Error::JSONRPCError(ref e) => Some(e),
            Error::PipelineError => None,
            Error::InvalidByteSequence => None,
            Error::ConfigError(ref e_str) => None,
            Error::BlockchainHeight => None,
            Error::DBError(ref e) => Some(e),
            Error::InvalidArgument => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::SocketMutexPoisoned => "socket mutex was poisoned",
            Error::SocketNotConnectedToPeer => "not connected to peer",
            Error::SerializationError(ref e) => e.description(),
            Error::InvalidMessage(ref msg) => "Invalid message to send",
            Error::InvalidReply => "invalid reply for given message",
            Error::InvalidMagic => "invalid network magic",
            Error::UnhandledMessage(ref msg) => "Unhandled message",
            Error::NotImplemented => "functionality not implemented",
            Error::ConnectionBroken => "connection to peer node is broken",
            Error::ConnectionError => "connection to peer could not be (re-)established",
            Error::FilesystemError(ref e) => e.description(),
            Error::HashError(ref e) => e.description(),
            Error::NoncontiguousHeader => "Non-contiguous header",
            Error::MissingHeader => "Missing header",
            Error::InvalidPoW => "Invalid proof of work",
            Error::JSONRPCError(ref e) => e.description(),
            Error::PipelineError => "Pipeline broken",
            Error::InvalidByteSequence => "Invalid sequence of bytes",
            Error::ConfigError(ref e_str) => e_str.as_str(),
            Error::BlockchainHeight => "Value is beyond the end of the blockchain",
            Error::DBError(ref e) => e.description(),
            Error::InvalidArgument => "Invalid argument"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitcoinNetworkType {
    mainnet,
    testnet,
    regtest
}

