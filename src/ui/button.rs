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

use crate::ui::event::{EventHandler, EventHandlers};
use crate::ui::mouse::{MouseButtonEvent, MouseButtonEventKind, MouseButtonTarget, MouseOverEvent, MouseOverEventKind, MouseOverTarget};
use crate::ui::spatial::Spatial;
use crate::ui::widget::{Context, Error, Scheduler, WidgetBuilder, WidgetId};

use std::rc::Rc;

///
/// A button widget
///
pub struct Button {
    ///
    /// Whether the button is being pressed or not
    ///
    pressed: bool,
    
    ///
    /// Whether the button is highlighted or not
    ///
    highlighted: bool,
    
    ///
    /// The button label
    ///
    label: String,

    ///
    /// Event handlers
    ///
    handlers: EventHandlers<ButtonEvent>,
}

impl Button {
    ///
    /// Creates a new button
    ///
    fn new(label: String) -> Button {
	Button {
	    pressed: true,
	    highlighted: false,
	    label,
	    handlers: EventHandlers::new(),
	}
    }

    ///
    /// Decorates a widget with a button
    ///
    pub fn decorate<'a>(builder: &mut WidgetBuilder<'a>, label: String) -> Result<(), Error> {
	if !builder.has_button()? {
	    Spatial::decorate(builder)?;
	    MouseButtonTarget::decorate(builder)?;
	    MouseOverTarget::decorate(builder)?;
	    builder.set_button(Button::new(label));
	    builder.mouse_over_target_mut()?.add_handler(Rc::from(HighlightHandler {}));
	    builder.mouse_button_target_mut()?.add_handler(Rc::from(ClickHandler {}));
	}
	Ok(())
    }
}

///
/// Manages highlighting
///
struct HighlightHandler {}

impl EventHandler<MouseOverEvent> for HighlightHandler {
    ///
    /// Sets highlight status
    ///
    fn handle_event<'a>(&self, event: &Rc<MouseOverEvent>, context: &mut Context<'a>, _scheduler: &mut Scheduler) -> Result<(), Error> {
	match event.kind {
	    MouseOverEventKind::Entered => {
		context.button_mut(context.widget_id())?.highlighted = true;
	    },
	    MouseOverEventKind::Exited => {
		context.button_mut(context.widget_id())?.highlighted = false;	
	    },
	}
	Ok(())
    }
}


///
/// Manages clicks
///
struct ClickHandler {}

impl EventHandler<MouseButtonEvent> for ClickHandler {
    ///
    /// Sets pressed status
    ///
    fn handle_event<'a>(&self, event: &Rc<MouseButtonEvent>, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	match event.kind {
	    MouseButtonEventKind::Pressed => {
		if context.spatial(context.widget_id())?.bounds().contains_position(&event.position) {
		    context.button_mut(context.widget_id())?.pressed = true;
		}
	    },
	    MouseButtonEventKind::Released => {
		let widget_id = context.widget_id();
		let button = context.button_mut(context.widget_id())?;
		if button.pressed {
		    button.pressed = false;
		    button.handlers.notify(Rc::from(ButtonEvent {
			source_id: widget_id,
		    }), scheduler);
		}
	    },
	}
	Ok(())
    }
}

///
/// Button clicked event
///
pub struct ButtonEvent {
    ///
    /// The source widget ID
    ///
    pub source_id: WidgetId,
}
