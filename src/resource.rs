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

use std::collections::HashMap;

///
/// A generic set of static resources
///
pub struct Resources<T> {
    ///
    /// The resource buffer
    ///
    buffer: Vec<T>,
    
    ///
    /// A lookup table to fetch resources by name
    ///
    by_name: HashMap<String, usize>,
}

impl<T> Resources<T> {
    ///
    /// Creates a new set of resources
    ///
    pub fn new() -> Resources<T> {
	Resources {
	    buffer: Vec::new(),
	    by_name: HashMap::new(),
	}
    }

    ///
    /// Add a resource to the set
    ///
    pub fn insert(&mut self, name: String, resource: T) -> Result<usize, Error> {
	if self.by_name.contains_key(&name) {
	    let id = self.buffer.len();
	    self.buffer.push(resource);
	    self.by_name.insert(name, id);
	    Ok(id)
	} else {
	    Err(Error::Duplicate)
	}
    }
    
    ///
    /// Adds a resource to the set created by the closure
    ///
    pub fn insert_from<E: From<Error>, F: FnOnce() -> Result<T, E>>(&mut self, name: String, create: F) -> Result<usize, E> {
	if self.by_name.contains_key(&name) {
	    Err(E::from(Error::Duplicate))
	} else {
	    let id = self.buffer.len();
	    self.buffer.push(create()?);
	    self.by_name.insert(name, id);
	    Ok(id)
	}
    }
}

///
/// Errors that can occur managing resources
///
#[derive(Debug)]
pub enum Error {
    ///
    /// A resource with this name already exists
    ///
    Duplicate,
}
