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

use crate::position::Position;
use crate::ui::event::{EventHandler, EventHandlers};
use crate::ui::spatial::Spatial;
use crate::ui::widget::{Context, Error, Scheduler, WidgetBuilder};

use std::rc::Rc;

///
/// This widget is the target of mouse events
///
pub struct MouseEventTarget<E: 'static> {
    handlers: EventHandlers<E>,
}

impl<E: 'static> MouseEventTarget<E> {
    ///
    /// Creates a new mouse event target
    ///
    fn new() -> MouseEventTarget<E> {
	MouseEventTarget {
	    handlers: EventHandlers::new(),
	}
    }

    ///
    /// Decorates a widget as a mouse button target
    ///
    pub fn decorate<'a>(builder: &mut WidgetBuilder<'a>) -> Result<(), Error> {
	if !builder.has_mouse_button_target()? {
	    Spatial::decorate(builder)?;
	    builder.set_mouse_button_target(MouseEventTarget::new());
	}
	Ok(())
    }

    ///
    /// Adds a handler
    ///
    pub fn add_handler(&mut self, handler: Rc<dyn EventHandler<E>>) {
	self.handlers.add(handler);
    }
    
    ///
    /// Notifies that the widget was the target of a mouse button event
    ///
    pub fn notify<'a>(&mut self, event: Rc<E>, scheduler: &mut Scheduler) {
	self.handlers.notify(event, scheduler);
    }
}

///
/// A mouse button event target component
///
pub type MouseButtonTarget = MouseEventTarget<MouseButtonEvent>;

///
/// Represents a mouse button
///
pub enum MouseButton {
    ///
    /// The left mouse button
    ///
    Left,

    ///
    /// The right mouse button
    ///
    Right,
}

///
/// What kind of event was it
///
pub enum MouseButtonEventKind {
    ///
    /// The mouse button was pressed
    ///
    Pressed,
    
    ///
    /// The mouse button was released
    ///
    Released,
}

///
/// A mouse button event
///
pub struct MouseButtonEvent {
    ///
    /// The kind of event
    ///
    pub kind: MouseButtonEventKind,
    
    ///
    /// Which button triggered the even 
    ///
    pub button: MouseButton,

    ///
    /// Where the event took place
    ///
    pub position: Position,
}

///
/// A mouse button event target component
///
pub type MouseMotionTarget = MouseEventTarget<MouseMotionEvent>;

///
/// A mouse motion event
///
pub struct MouseMotionEvent {
    ///
    /// Where the event originated
    ///
    position: Position,
}

///
/// A widget that can be hovered over
///
pub struct MouseOverTarget {
    ///
    /// Event handlers
    ///
    handlers: EventHandlers<MouseOverEvent>,
    ///
    /// Whether the mouse pointer was within bounds during the previous tick 
    ///
    within: bool,
}

impl MouseOverTarget {
    ///
    /// Creates a new mouse over target
    ///
    fn new() -> MouseOverTarget {
	MouseOverTarget {
	    handlers: EventHandlers::new(),
	    within: false,
	}
    }

    ///
    /// Decorates a widget with a mouse over target
    ///
    pub fn decorate<'a>(builder: &mut WidgetBuilder<'a>) -> Result<(), Error> {
	if !builder.has_mouse_over_target()? {
	    Spatial::decorate(builder)?;
	    MouseMotionTarget::decorate(builder)?;
	    builder.set_mouse_over_target(MouseOverTarget::new())?;
	    builder.mouse_motion_target_mut()?.handlers.add(Rc::new(CheckMouseOverHandler{}));
	}
	Ok(())
    }

    ///
    /// Adds a handler
    ///
    pub fn add_handler(&mut self, handler: Rc<dyn EventHandler<MouseOverEvent>>) {
	self.handlers.add(handler);
    }
}

///
/// Checks whether a mouse motion event should trigger a mouse over event
///
struct CheckMouseOverHandler {}

impl EventHandler<MouseMotionEvent> for CheckMouseOverHandler {
    ///
    /// Checks whether a mouse motion event should trigger
    ///
    fn handle_event<'a>(&self, event: &Rc<MouseMotionEvent>, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	let within = context.spatial(context.widget_id())?.bounds().contains_position(&event.position);
	let mouse_over = context.mouse_over_target_mut(context.widget_id())?;
	let old_within = mouse_over.within;
	if old_within && !within {
	    mouse_over.handlers.notify(Rc::from(MouseOverEvent {
		kind: MouseOverEventKind::Exited,
	    }), scheduler);
	} else if !old_within && within {
	    mouse_over.handlers.notify(Rc::from(MouseOverEvent {
		kind: MouseOverEventKind::Entered,
	    }), scheduler);
	}
	mouse_over.within = within;
	Ok(())
    }
}

///
/// Types of mouse over events
///
pub enum MouseOverEventKind {
    ///
    /// Mouse has entered
    ///
    Entered,

    ///
    /// Mouse has exited
    ///
    Exited,
}

///
/// An event for when the mouse pointer enters or exits a component
///
pub struct MouseOverEvent {
    ///
    /// The kind of event
    ///
    pub kind: MouseOverEventKind,
}
