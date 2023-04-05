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

use std::collections::BTreeSet;

///
/// An arena
///
pub struct Arena<T> {
    ///
    /// The object buffer
    ///
    buffer: Vec<Option<T>>,
    ///
    /// The free list as an ordered set
    ///
    free: BTreeSet<usize>,
}

impl<T> Arena<T> {
    ///
    /// Creates a new, empty arena
    ///
    pub fn new() -> Arena<T> {
	Arena {
	    buffer: Vec::new(),
	    free: BTreeSet::new(),
	}
    }

    ///
    /// Inserts a new object into the arena
    ///
    pub fn insert(&mut self, object: T) -> usize {
	match self.free.pop_first() {
	    Some(id) => {
		self.buffer[id] = Some(object);
		id
	    },
	    None => {
		let id = self.buffer.len();
		self.buffer.push(Some(object));
		id
	    }
	}
    }

    ///
    /// Gets a reference to an object in the arena
    ///
    pub fn get(&self, id: usize) -> Option<&T> {
	match self.buffer.get(id) {
	    Some(value) => {
		value.as_ref()
	    },
	    None => None,
	}
    }
    
    ///
    /// Gets a mutable reference to an object in the arena
    ///
    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
	match self.buffer.get_mut(id) {
	    Some(value) => {
		value.as_mut()
	    },
	    None => None,
	}
    }

    ///
    /// Removes an object from the arena if it exists
    ///
    pub fn remove(&mut self, id: usize) -> Option<T> {
	if id < self.buffer.len() {
	    let mut value = None;
	    std::mem::swap(&mut value, &mut self.buffer[id]);
	    if value.is_some() {
		self.recycle_id(id);
	    }
	    value
	} else {
	    None
	}
    }

    ///
    /// Recycles an id
    ///
    fn recycle_id(&mut self, id: usize) {
	if id == self.buffer.len() - 1 {
	    self.buffer.pop();
	    while self.buffer.len() != 0 && self.buffer[self.buffer.len() - 1].is_none() {
		self.buffer.pop();
		self.free.pop_last();
	    }
	} else {
	    self.free.insert(id);
	}
    }

    ///
    /// Creates an iterator
    ///
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
	Iter {
	    arena: self,
	    next_id: 0,
	}
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Arena<T> {

    ///
    /// Formats the arena, omitting the free list
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "[")?;
	if self.buffer.len() != 0 {
	    write!(f, "{:?}", self.buffer[0])?;
	}
	for i in 1..self.buffer.len() {
	    write!(f, ", {:?}", self.buffer[i])?;
	}
	write!(f, "]")
    }
}

impl<T: PartialEq> PartialEq for Arena<T> {
    ///
    /// Two arena's are equal when their buffers are equal,
    /// give or take a tail of (None) entries at the end
    /// as this implies equal objects with equal ID's
    ///
    fn eq(&self, other: &Arena<T>) -> bool {
	self.buffer.eq(&other.buffer)
    }
}

///
/// An iterator for arenas
///
pub struct Iter<'a, T> {
    ///
    /// The arena
    ///
    arena: &'a Arena<T>,

    ///
    /// The next id
    ///
    next_id: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
	loop {
	    if self.next_id >= self.arena.buffer.len() {
		break None;
	    }
	    let value = &self.arena.buffer[self.next_id];
	    self.next_id += 1;
	    if value.is_some() {
		break value.as_ref();
	    }
	}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn arena_new() {
	let expected: Arena<i32> = Arena {
	    buffer: vec![],
	    free: BTreeSet::new(),
	};
	assert_eq!(expected, Arena::new());
    }
    
    #[test]
    fn arena_insert() {
	let expected: Arena<i32> = Arena {
	    buffer: vec![Some(3)],
	    free: BTreeSet::new(),
	};
	let mut arena = Arena::new();
	assert_eq!(0, arena.insert(3));
	assert_eq!(expected, arena);
    }

    #[test]
    fn arena_get() {
	let mut arena = Arena::new();
	arena.insert(3);
	assert_eq!(Some(&3), arena.get(0));
	assert_eq!(None, arena.get(1));
    }

    #[test]
    fn arena_get_mut() {
	let mut arena = Arena::new();
	arena.insert(3);
	assert_eq!(Some(&mut 3), arena.get_mut(0));
	assert_eq!(None, arena.get_mut(1));
	*arena.get_mut(0).unwrap() = 4;
	assert_eq!(Some(&mut 4), arena.get_mut(0));
    }
    
    #[test]
    fn arena_remove() {
	let expected: Arena<i32> = Arena {
	    buffer: vec![Some(5), Some(4)],
	    free: BTreeSet::new(),
	};

	let mut arena = Arena::new();
	arena.insert(3);
	arena.insert(4);
	assert_eq!(Some(3), arena.remove(0));
	assert_eq!(None, arena.remove(3));
	assert_eq!(0, arena.insert(5));
	assert_eq!(expected, arena);
    }

    #[test]
    fn arena_iter() {
	let mut arena = Arena::new();
	arena.insert(3);
	arena.insert(4);
	arena.insert(5);
	arena.remove(1);
	let mut i = arena.iter();
	assert_eq!(Some(&3), i.next());
	assert_eq!(Some(&5), i.next());
	assert_eq!(None, i.next());
	assert_eq!(None, i.next());
    }

    #[test]
    fn arena_eq() {
	let mut first = Arena::new();
	first.insert(1);

	let mut second = Arena::new();
	second.insert(1);

	assert_eq!(first, second);
	assert_eq!(second, first);

	let mut first = Arena::new();
	first.insert(1);
	
	let mut second = Arena::new();
	second.insert(2);

	assert!(!first.eq(&second));

	let mut first = Arena::new();
	first.insert(1);
	first.insert(2);
	first.remove(0);

	let mut second = Arena::new();
	second.insert(2);
	assert!(!first.eq(&second));
	
	let mut first = Arena::new();
	first.insert(3);
	first.insert(4);
	first.insert(5);
	first.insert(6);
	first.insert(7);
	first.remove(1);
	first.remove(3);
	first.remove(4);

	let mut second = Arena::new();
	second.insert(3);
	second.insert(4);
	second.insert(5);
	second.remove(1);

	assert_eq!(first, second);
	assert_eq!(second, first);
    }

    #[test]
    fn arena_debug() {
	let arena: Arena<i32> = Arena::new();
	assert_eq!("[]", format!("{:?}", arena));

	let mut arena = Arena::new();
	arena.insert(1);
	arena.insert(2);
	arena.insert(3);
	arena.insert(4);
	arena.remove(1);
	assert_eq!("[Some(1), None, Some(3), Some(4)]", format!("{:?}", arena));
    }
}
