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

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::DeserializeOwned;

pub fn load<M: DeserializeOwned>(path: &Path) -> Result<M, Error> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    Ok(serde_yaml::from_str::<M>(&buffer)?)
}

///
/// Errors that can occur loading configuration
///
#[derive(Debug)]
pub enum Error {
    ///
    /// an IO error occurred
    ///
    IO(std::io::Error),
    ///
    /// an error occurred when deserializing the configuration
    ///
    Parse(serde_yaml::Error),
}

impl From<std::io::Error> for Error {
    ///
    /// Creates a configuration for error
    ///
    fn from(e: std::io::Error) -> Error {
	Error::IO(e)
    }
}


impl From<serde_yaml::Error> for Error {
    ///
    /// Creates a configuration for error
    ///
    fn from(e: serde_yaml::Error) -> Error {
	Error::Parse(e)
    }
}
