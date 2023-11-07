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
use crate::ui::component::Id;
use crate::ui::error::Error;
use crate::ui::system::{Action, System};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// An event listener
///
pub trait Listener<E: 'static> {
    ///
    /// Notifies the handler that an event has been triggered
    ///
    fn notify(&self, event: Rc<E>) -> Result<(), Error>;
}

///
/// A set of event listeners
///
pub struct Listeners<E: 'static> {
    ///
    /// A set of listeners
    ///
    listeners: Arena<Rc<dyn Listener<E>>>,
}

impl<E: 'static> Listeners<E> {
    ///
    /// Creates a new set of listeners
    ///
    pub fn new() -> Listeners<E> {
	Listeners {
	    listeners: Arena::new(),
	}
    }

    ///
    /// Notifies all listeners
    ///
    pub fn notify(&self, event: Rc<E>) -> Result<(), Error> {
	self.listeners.iter().try_for_each(|l| l.notify(event.clone()))
    }

    ///
    /// Schedules event notification
    ///
    pub fn try_schedule_notify(&self, event: Rc<E>, system: &Weak<RefCell<System>>) -> Result<(), Error> {
	let mut system = system.upgrade().ok_or(Error::NoSystem)?;
	let mut system = system.borrow_mut();
	self.listeners.iter().for_each(|l| system.schedule(Box::from(NotifyAction {
	    event: event.clone(),
	    listener: Rc::downgrade(&l),
	})));
	Ok(())
    }

    ///
    /// Registers a listener
    ///
    pub fn register(&mut self, listener: Rc<dyn Listener<E>>) -> Id {
	self.listeners.insert(listener)
    }

    ///
    /// Unregisters a listener
    ///
    pub fn unregister(&mut self, id: Id) -> Option<Rc<dyn Listener<E>>> {
	self.listeners.remove(id)
    }
}

///
/// Wraps an event into an action
///
struct NotifyAction<E: 'static> {
    ///
    /// The event
    ///
    event: Rc<E>,

    ///
    /// The listener
    ///
    listener: Weak<dyn Listener<E>>,
}

impl<E: 'static> Action for NotifyAction<E> {
    fn execute(&self) -> Result<(), Error> {
	if let Some(listener) = self.listener.upgrade() {
	    listener.notify(self.event.clone())?;
	}
	Ok(())
    }
}

///
/// A basic event triggered by a component
///
pub struct ComponentEvent {
    ///
    /// The ID of the component that triggered the event
    ///
    pub id: Id,
}

impl ComponentEvent {
    ///
    ///
    ///
    pub fn new(id: Id) -> ComponentEvent {
	ComponentEvent {
	    id,
	}
    }
}
