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

use crate::arena::Arena;
use crate::ui::action::{Action, Scheduler};
use crate::ui::error::Error;

use std::rc::{Rc, Weak};

///
/// An event handler
///
pub trait Handler<E: 'static> {
    ///
    /// Handles an event
    ///
    fn handle(&self, event: &Rc<E>, scheduler: &mut Scheduler) -> Result<(), Error>;
}

///
/// An ID type for a handler
///
pub type HandlerId = usize;

///
/// A set of handlers
///
pub struct Handlers<E: 'static> {
    ///
    /// The handlers
    ///
    handlers: Arena<Rc<dyn Handler<E>>>,
}

impl<E: 'static> Handlers<E> {
    ///
    /// Constructs a new set of handlers 
    ///
    pub fn new() -> Handlers<E> {
	Handlers {
	    handlers: Arena::new(),
	}
    }

    ///
    /// Schedules an event
    ///
    pub fn schedule(&self, event: &Rc<E>, scheduler: &mut Scheduler) {
	self.handlers.iter().for_each(|handler| scheduler.schedule(EventAction {
	    event: event.clone(),
	    handler: Rc::downgrade(handler),
	}));
    }

    ///
    /// Add a handler
    ///
    pub fn add(&mut self, handler: Rc<dyn Handler<E>>) -> HandlerId {
	self.handlers.insert(handler)
    }

    ///
    /// Removes a handler
    ///
    pub fn remove(&mut self, id: HandlerId) -> Result<Rc<dyn Handler<E>>, Error> {
	self.handlers.remove(id).ok_or(Error::NoHandler)
    }
}

///
/// An action that triggers an event
///
struct EventAction<E: 'static> {
    ///
    /// The event handler
    ///
    handler: Weak<dyn Handler<E>>,

    ///
    /// The event
    ///
    event: Rc<E>,
}

impl<E: 'static> Action for EventAction<E> {
    fn execute(&self, scheduler: &mut Scheduler) -> Result<(), Error> {
	if let Some(handler) = self.handler.upgrade() {
	    handler.handle(&self.event, scheduler)?;
	}
	Ok(())
    }
}


