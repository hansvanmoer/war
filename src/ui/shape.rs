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
use crate::ui::component::{Component, Id};
use crate::ui::event::{ComponentEvent, Listeners, Listener};
use crate::ui::error::Error;
use crate::ui::system::System;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// A component with a shape
///
pub struct Shape {
    ///
    /// A unique ID referencing the shape
    ///
    id: Id,
    
    ///
    /// A reference to the system that created this shape
    ///
    system: Weak<RefCell<System>>,
    
    ///
    /// The component's position
    ///
    position: Position,

    ///
    /// Triggered when the shape is moved
    ///
    on_move: Listeners<ComponentEvent>,
    
    ///
    /// The component's preferred size
    ///
    preferred_size: Dimension,

    ///
    /// Triggered when the shape is resized
    ///
    on_resize: Listeners<ComponentEvent>,
}

impl Shape {
    ///
    /// Creates a new shape
    ///
    pub fn new(system: &Rc<RefCell<System>>, id: Id, position: Position, preferred_size: Dimension) -> Rc<RefCell<Shape>> {
	Rc::from(RefCell::from(Shape {
	    id: id,
	    system: Rc::downgrade(system),
	    position,
	    on_move: Listeners::new(),
	    preferred_size,
	    on_resize: Listeners::new(),
	}))
    }
    
    ///
    /// Gets the position
    ///
    pub fn position(&self) -> &Position {
	&self.position
    }
    
    ///
    /// Sets the position of this shape
    ///
    pub fn set_position(&mut self, position: Position) -> Result<(), Error>{
	self.position = position;
	self.on_move.try_schedule_notify(Rc::from(MovedEvent::new(self.id)), &self.system)
    }


    ///
    /// Register on move listener
    ///
    pub fn register_on_move(&mut self, listener: Rc<dyn Listener<MovedEvent>>) -> Id {
	self.on_move.register(listener)
    }

    ///
    /// Unregister on move listener
    ///
    pub fn unregister_on_move(&mut self, id: Id) -> Option<Rc<dyn Listener<MovedEvent>>> {
	self.on_move.unregister(id)
    }
    
    ///
    /// Gets the preferred size
    ///
    pub fn preferred_size(&self) -> &Dimension {
	&self.preferred_size
    }

    ///
    /// Resizes the component
    ///
    pub fn set_preferred_size(&mut self, size: Dimension) -> Result<(), Error> {
	self.preferred_size = size;
	self.on_resize.try_schedule_notify(Rc::from(ResizedEvent::new(self.id)), &self.system)
    }

    ///
    /// Register on resize listener
    ///
    pub fn register_on_resize(&mut self, listener: Rc<dyn Listener<ResizedEvent>>) -> Id {
	self.on_resize.register(listener)
    }

    ///
    /// Unregister on resize listener
    ///
    pub fn unregister_on_resize(&mut self, id: Id) -> Option<Rc<dyn Listener<ResizedEvent>>> {
	self.on_resize.unregister(id)
    }
}

impl Component for Shape {
    fn id(&self) -> Id {
	self.id
    }
}

pub trait ShapeRef {

    ///
    /// Returns a reference to the underlying shape
    ///
    fn shape(&self) -> Rc<RefCell<Shape>>;

    ///
    /// Creates a new reference to the underlying shape
    ///
    fn create_reference(&self) -> Box<dyn ShapeRef>;
    
    ///
    /// Returns the position
    ///
    fn position(&self) -> Result<Position, Error> {
	Ok(self.shape().try_borrow()?.position().clone())
    }
    
    ///
    /// Sets the position of this shape
    ///
    fn set_position(&mut self, position: Position) -> Result<(), Error>{
	self.shape().try_borrow_mut()?.set_position(position);
	Ok(())
    }

    ///
    /// Gets the preferred size
    ///
    fn preferred_size(&self) -> Result<Dimension, Error> {
	Ok(self.shape().try_borrow()?.preferred_size().clone())
    }

    ///
    /// Sets the preferred size
    ///
    fn set_preferred_size(&mut self, size: Dimension) -> Result<(), Error> {
	self.shape().try_borrow_mut()?.set_preferred_size(size)
    }

    
    ///
    /// Register on resize listener
    ///
    fn register_on_resize(&mut self, listener: Rc<dyn Listener<ResizedEvent>>) -> Result<Id, Error> {
	Ok(self.shape().try_borrow_mut()?.on_resize.register(listener))
    }

    ///
    /// Unregister on resize listener
    ///
    fn unregister_on_resize(&mut self, id: Id) -> Result<Option<Rc<dyn Listener<ResizedEvent>>>, Error> {
	Ok(self.shape().try_borrow_mut()?.on_resize.unregister(id))
    }
}

///
/// The on move event
///
pub type MovedEvent = ComponentEvent;

///
/// The on resize event
///
pub type ResizedEvent = ComponentEvent;
