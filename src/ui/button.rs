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
    pub fn decorate<'a>(builder: &mut WidgetBuilder<'a>, label: String) {
	if !builder.has_button()? {
	    Spatial::decorate(builder)?;
	    MouseButtonTarget::decorate(builder)?;
	    MouseOverTarget::decorate(builder)?;
	    builder.set_button(Button::new(label));
	    builder.mouse_over_target_mut()?.add(Rc::from(HighlightHandler {}));
	    builder.mouse_button_target_mut()?.add(Rc::from(ClickHandler {}));
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
    fn handle_event(&self, event: &Rc<MouseOverEvent>, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	match event {
	    MouseOverEvent::Entered => {
		context.button_mut(context.widget_id).highlighted = true;
	    },
	    MouseOverEvent::Exited => {
		context.button_mut(context.widget_id).highlighted = false;	
	    },
	}
    }
}


///
/// Manages clicks
///
struct HighlightHandler {}

impl EventHandler<MouseOverEvent> for HighlightHandler {
    ///
    /// Sets highlight status
    ///
    fn handle_event(&self, event: &Rc<MouseOverEvent>, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	match event {
	    MouseOverEvent::Entered => {
		context.button_mut(context.widget_id).highlighted = true;
	    },
	    MouseOverEvent::Exited => {
		context.button_mut(context.widget_id).highlighted = false;	
	    },
	}
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
