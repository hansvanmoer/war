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

use crate::ui::widget::{Action, Context, Error, Scheduler};

use std::collections::BTreeMap;
use std::rc::Rc;

///
/// An event handler
///
pub trait EventHandler<E> {
    ///
    /// Executes an event action
    ///
    fn handle_event<'a>(&self, event: &Rc<E>, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error>;

}

///
/// An adapter from actions to event handlers
///
pub struct EventAction<E> {
    ///
    /// The underlying event handler
    ///
    event_handler: Rc<dyn EventHandler<E>>,

    ///
    /// The event
    ///
    event: Rc<E>,
}

impl<E> EventAction<E> {
    ///
    /// Creates a new event action
    ///
    pub fn new(event: Rc<E>, event_handler: Rc<dyn EventHandler<E>>) -> EventAction<E> {
	EventAction {
	    event_handler,
	    event,
	}
    }
}

impl<E> Action for EventAction<E> {
    ///
    /// Triggers the event handler
    ///
    fn execute<'a>(&self, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	self.event_handler.handle_event(&self.event, context, scheduler)
    }
}

///
/// A list of event handlers
///
pub struct EventHandlers<E: 'static> {
    ///
    /// The list of event handlers
    ///
    handlers: BTreeMap<usize, Rc<dyn EventHandler<E>>>,
    ///
    /// The next ID
    ///
    next_id: usize,
}

impl<E: 'static> EventHandlers<E> {
    ///
    /// Creates a new list of event handlers
    ///
    pub fn new() -> EventHandlers<E> {
	EventHandlers {
	    handlers: BTreeMap::new(),
	    next_id: 0,
	}
    }

    pub fn add(&mut self, event_handler: Rc<dyn EventHandler<E>>) -> EventHandlerId {
	let id = self.next_id;
	self.next_id += 1;
	self.handlers.insert(id,event_handler);
	id
    }

    pub fn remove(&mut self, id: EventHandlerId) {
	self.handlers.remove(&id);
    }

    pub fn notify<'a>(&self, event: Rc<E>, scheduler: &mut Scheduler) {
	self.handlers.iter()
	    .for_each(|(_, handler) |
		      scheduler.schedule_for_self(Rc::from(EventAction::new(event.clone(), handler.clone()))));
    }
}

///
/// An ID type for event handlers
///
pub type EventHandlerId = usize;
