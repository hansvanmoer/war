/*
 * This file is part of 'The Hundred Years War'.
 * 'The Hundred Years War' is free software: you can redistribute it and/or modify it under the terms of
 * the GNU General Public License as published by the Free Software Foundation,
 * either version 3 of the License, or (at your option) any later version.
 * 'The Hundred Years War' is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or 
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for 
 * more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with 'The Hundred Years War'. If not, see <https://www.gnu.org/licenses/>. 
 *
 */

use std::cell::{BorrowError, BorrowMutError};

///
/// All errors that can occur in the UI subsystem
///
pub enum Error {
    ///
    /// The system no longer exists
    ///
    NoSystem,

    ///
    /// The component no longer exists
    ///
    NoComponent,
    
    ///
    /// An error occurred borrowing a value, this indicates a programming error
    ///
    BorrowError,
}

impl From<BorrowError> for Error {
    fn from(_: BorrowError) -> Error {
	Error::BorrowError
    }
}

impl From<BorrowMutError> for Error {
    fn from(_: BorrowMutError) -> Error {
	Error::BorrowError
    }
}
