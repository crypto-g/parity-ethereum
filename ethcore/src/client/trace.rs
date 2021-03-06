// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Bridge between Tracedb and Blockchain.

use ethereum_types::H256;
use header::BlockNumber;
use trace::DatabaseExtras as TraceDatabaseExtras;
use blockchain::{BlockChain, BlockProvider, TransactionAddress};
pub use types::trace_filter::Filter;

impl TraceDatabaseExtras for BlockChain {
	fn block_hash(&self, block_number: BlockNumber) -> Option<H256> {
		(self as &BlockProvider).block_hash(block_number)
	}

	fn transaction_hash(&self, block_number: BlockNumber, tx_position: usize) -> Option<H256> {
		(self as &BlockProvider).block_hash(block_number)
			.and_then(|block_hash| {
				let tx_address = TransactionAddress {
					block_hash: block_hash,
					index: tx_position
				};
				self.transaction(&tx_address)
			})
			.map(|tx| tx.hash())
	}
}
