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

use crate::dimension::Dimension;
use crate::position::Position;
use crate::ui::component::Id;
use crate::ui::error::Error;
use crate::ui::event::Listener;
use crate::ui::shape::{MovedEvent, ResizedEvent, Shape, ShapeRef};
use crate::ui::system::System;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// A container
///
pub struct Container {    
    ///
    /// A reference to the system that created this shape
    ///
    system: Weak<RefCell<System>>,

    ///
    /// A reference to the shape
    ///
    shape: Weak<RefCell<Shape>>,

    ///
    /// A reference to the container itself
    ///
    container: Weak<RefCell<Container>>,
    
    ///
    /// The rows in this container
    ///
    rows: Vec<Vec<Column>>,
}

impl Container {
    ///
    /// Creates a new container
    ///
    pub fn new(system: &Rc<RefCell<System>>, shape: &Rc<RefCell<Shape>>) -> Result<Rc<RefCell<Container>>, Error> {
	let shape = shape.clone();
	let container = Rc::from(RefCell::from(Container {
	    system: Rc::downgrade(system),
	    container: Weak::new(),
	    shape: Rc::downgrade(&shape),
	    rows: Vec::new(),
	}));
	let container_ref = Rc::downgrade(&container);
	shape.try_borrow_mut()?.register_on_move(Rc::from(ContainerMoved {
	    container: container_ref.clone(),
	}));
	container.borrow_mut().container = container_ref;
	Ok(container)
    }
    
    ///
    /// Add a row, if necessary
    ///
    pub fn add_row(&mut self) -> bool {
	let len = self.rows.len();
	if len == 0 || self.rows[len - 1].len() != 0 {
	    self.rows.push(Vec::new());
	    true
	} else {
	    false
	}
    }

    ///
    /// Add a column (and a row, if necessary)
    ///
    pub fn add_column(&mut self, mut child: Box<dyn ShapeRef>, alignment: Alignment) -> Result<(), Error> {
	let listener_id = child.register_on_resize(Rc::from(ChildResized {
	    container: self.container.clone(),
	}))?;
	let column = Column  {
	    child,
	    alignment,
	    listener_id,
	};
	let len = self.rows.len();
	if len == 0 {
	    self.rows.push(vec![column]);
	} else {
	    self.rows[len - 1].push(column);
	}
	Ok(())
    }

    ///
    /// Updates child components when this component is moved
    ///
    fn update_after_move(&mut self) -> Result<(), Error> {
	let shape_ref = Weak::upgrade(&self.shape).ok_or(Error::NoComponent)?;
	let shape = shape_ref.try_borrow_mut()?;
	let position = shape.position();
	let size = shape.preferred_size();
	
	let mut y = position.y;
	for row in self.rows.iter_mut() {
	    let mut row_height = 0.0;
	    let mut center_width = 0.0;
	    let mut right_width = 0.0;
	    for col in row.iter() {
		let size = col.child.preferred_size()?;
		let col_height = size.height();
		if col_height > row_height {
		    row_height = col_height;
		}
		match col.alignment {
		    Alignment::Center => {
			center_width += size.width();
		    },
		    Alignment::Right => {
			right_width += size.width();
		    },
		    _ => {}
		} 
	    }
	    let mut left_x = position.x;
	    let mut center_x = position.x + (size.width() - center_width) / 2.0;
	    let mut right_x = position.x + size.width() - right_width;
	    for col in row.iter_mut() {
		let col_size = col.child.preferred_size()?;
		let col_position = Position::new(match col.alignment {
		    Alignment::Left => {
			let x = left_x;
			left_x += col_size.width();
			x
		    },
		    Alignment::Center => {
			let x = center_x;
			center_x += col_size.width();
			x
		    },
		    Alignment::Right => {
			let x = right_x;
			right_x += col_size.width();
			x
		    },
		}, y + row_height - col_size.height());
		col.child.set_position(col_position)?;
		y += row_height;
	    }
	}
	Ok(())
    }

    ///
    /// Updates the component when one of its children has been resized
    ///
    fn update_after_resize(&mut self) -> Result<(), Error>{
	let shape_ref = Weak::upgrade(&self.shape).ok_or(Error::NoComponent)?;
	let mut shape = shape_ref.try_borrow_mut()?;
	let position = shape.position();
	let new_size = self.rows.iter().map(
	    |row| row.iter().map(Column::size).fold(Dimension::default(), Dimension::combine_horizontal)
	).fold(Dimension::default(), Dimension::combine_vertical);
	
	let mut y = position.y;
	for row in self.rows.iter_mut() {
	    let mut row_height = 0.0;
	    let mut center_width = 0.0;
	    let mut right_width = 0.0;
	    for col in row.iter() {
		let size = col.child.preferred_size()?;
		let col_height = size.height();
		if col_height > row_height {
		    row_height = col_height;
		}
		match col.alignment {
		    Alignment::Center => {
			center_width += size.width();
		    },
		    Alignment::Right => {
			right_width += size.width();
		    },
		    _ => {}
		} 
	    }
	    let mut left_x = position.x;
	    let mut center_x = position.x + (new_size.width() - center_width) / 2.0;
	    let mut right_x = position.x + new_size.width() - right_width;
	    for col in row.iter_mut() {
		let col_size = col.child.preferred_size()?;
		let col_position = Position::new(match col.alignment {
		    Alignment::Left => {
			let x = left_x;
			left_x += col_size.width();
			x
		    },
		    Alignment::Center => {
			let x = center_x;
			center_x += col_size.width();
			x
		    },
		    Alignment::Right => {
			let x = right_x;
			right_x += col_size.width();
			x
		    },
		}, y + row_height - col_size.height());
		col.child.set_position(col_position)?;
		y += row_height;
	    }
	}
	shape.set_preferred_size(new_size)
    }
}

///
/// Alignment of a child widget in a row
///
pub enum Alignment {
    ///
    /// Align to the left
    ///
    Left,

    ///
    /// Center
    ///
    Center,

    ///
    /// Align to the right
    ///
    Right,
}

///
/// A column containing a child widget
///
struct Column {
    ///
    /// The child component
    ///
    child: Box<dyn ShapeRef>,

    ///
    /// The alignment
    ///
    alignment: Alignment,
    
    ///
    /// The Id with which the resize listener is registered
    ///
    listener_id: Id,
}

impl Column {
    fn size(&self) -> Dimension {
	self.child.preferred_size().unwrap_or_else(|_| Dimension::default())
    }
}

impl Drop for Column {
    fn drop(&mut self) {
	let _result = self.child.unregister_on_resize(self.listener_id);
    }
}

///
/// Triggered when a child is resized
///
struct ChildResized {
    ///
    /// The container in which the child resides
    ///
    container: Weak<RefCell<Container>>,
}

impl Listener<ResizedEvent> for ChildResized {
    fn notify(&self, _: Rc<ResizedEvent>) -> Result<(), Error> {
	if let Some(container) = Weak::upgrade(&self.container) {
	    container.try_borrow_mut()?.update_after_resize()
	} else {
	    Ok(())
	}
    }
}


///
/// Triggered when a child is resized
///
struct ContainerMoved {
    ///
    /// The container in which the child resides
    ///
    container: Weak<RefCell<Container>>,
}

impl Listener<MovedEvent> for ContainerMoved {
    fn notify(&self, _: Rc<MovedEvent>) -> Result<(), Error> {
	if let Some(container) = Weak::upgrade(&self.container) {
	    container.try_borrow_mut()?.update_after_move()
	} else {
	    Ok(())
	}
    }
}
