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
use crate::ui::action::Scheduler;
use crate::ui::error::Error;
use crate::ui::event::{Handler, HandlerId, Handlers};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// A component
///
pub struct Component {
    ///
    /// A self reference
    ///
    component: Weak<RefCell<Component>>,

    ///
    /// The component's position
    ///
    position: Position,

    ///
    /// Moved event handlers
    ///
    moved_handlers: Handlers<MovedEvent>,
    
    ///
    /// The component's size
    ///
    size: Dimension,

    ///
    /// Resized handlers
    ///
    resized_handlers: Handlers<ResizedEvent>,
}

impl Component {
    ///
    /// Creates a new component
    ///
    pub fn new() -> Rc<RefCell<Component>> {
	let mut component = Rc::from(RefCell::from(Component {
	    component: Weak::new(),
	    position: Position::default(),
	    moved_handlers: Handlers::new(),
	    size: Dimension::default(),
	    resized_handlers: Handlers::new(),
	}));
	component.borrow_mut().component = Rc::downgrade(&component);
	component
    }

    ///
    /// Moves the component
    ///
    pub fn set_position(&mut self, position: Position, scheduler: &mut Scheduler) {
	let event = Rc::from(MovedEvent {
	    component: self.component.clone(),
	    original_position: self.position.clone(),
	    new_position: position.clone(),
	});
	self.moved_handlers.schedule(&event, scheduler);
	self.position = position;
    }

    ///
    /// Adds a handler that will be notified when the component is moved
    ///
    pub fn add_moved_handler(&mut self, handler: Rc<dyn Handler<MovedEvent>>) -> HandlerId {
	self.moved_handlers.add(handler)
    }

    ///
    /// Removed a moved handler
    ///
    pub fn remove_moved_handler(&mut self, id: HandlerId) -> Result<Rc<dyn Handler<MovedEvent>>, Error>{
	self.moved_handlers.remove(id)
    }
    
    ///
    /// Resizes the component
    ///
    pub fn set_size(&mut self, size: Dimension, scheduler: &mut Scheduler) {
	let event = Rc::from(ResizedEvent {
	    component: self.component.clone(),
	    original_size: self.size.clone(),
	    new_size: size.clone(),
	});
	self.resized_handlers.schedule(&event, scheduler);
	self.size = size;
    }
    
    ///
    /// Adds a handler that will be notified when the component is resized
    ///
    pub fn add_resized_handler(&mut self, handler: Rc<dyn Handler<ResizedEvent>>) -> HandlerId {
	self.resized_handlers.add(handler)
    }

    ///
    /// Removes a resized handler
    ///
    pub fn remove_resized_handler(&mut self, id: HandlerId) -> Result<Rc<dyn Handler<ResizedEvent>>, Error>{
	self.resized_handlers.remove(id)
    }
}

///
/// An event for when the component has been moved
///
pub struct MovedEvent {
    ///
    /// The component
    ///
    component: Weak<RefCell<Component>>,
    
    ///
    /// The original position
    ///
    original_position: Position,

    ///
    /// The new position
    ///
    new_position: Position,
}

///
/// An event for when the component has been resized
///
pub struct ResizedEvent {
    ///
    /// The component
    ///
    component: Weak<RefCell<Component>>,

    ///
    /// The original size
    ///
    original_size: Dimension,

    ///
    /// The new size
    ///
    new_size: Dimension,
}
