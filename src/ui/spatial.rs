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

use crate::bounds::Bounds;
use crate::dimension::Dimension;
use crate::position::Position;
use crate::ui::event::{EventHandler, EventHandlers};
use crate::ui::widget::{Action, Context, Error, ListenerId, Listeners, Scheduler, WidgetBuilder, WidgetId};

use std::rc::Rc;

///
/// The spatial component to a widget
///
pub struct Spatial {
    ///
    /// The ID of the widget
    ///
    widget_id: WidgetId,
    
    ///
    /// The left bottom position of a widget
    ///
    position: Position,

    ///
    /// The preferred size of the component
    ///
    preferred_size: Dimension,

    ///
    /// The bounds of the component, calculated from the position and size
    ///
    bounds: Bounds,

    ///
    /// The bounds need to be updated
    ///
    update_bounds: bool,

    ///
    /// Handlers that have to be notified when the widget is moved
    ///
    move_handlers: EventHandlers<MoveEvent>,
}

impl Spatial {
    ///
    /// Creates a new spatial component
    ///
    fn new(widget_id: WidgetId) -> Spatial {
	Spatial {
	    widget_id,
	    position: Position::default(),
	    preferred_size: Dimension::default(),
	    bounds: Bounds::default(),
	    update_bounds: true,
	    move_handlers: EventHandlers::new(),
	}
    }

    ///
    /// Decorates a widget with a spatial component if it is not already present
    ///
    pub fn decorate<'a>(builder: &mut WidgetBuilder<'a>) -> Result<(), Error> {
	if !builder.has_spatial()? {
	    builder.set_spatial(Spatial::new(builder.widget_id()))?;
	}
	Ok(())
    }

    ///
    /// Returns the position of the widget
    ///
    pub fn position(&self) -> &Position {
	&self.position
    }

    ///
    /// Returns the preferred size
    ///
    pub fn preferred_size(&self) -> &Dimension {
	&self.preferred_size
    }
    
    ///
    /// Moves the widget
    ///
    pub fn set_position<'a>(&mut self, position: Position, scheduler: &mut Scheduler) {
	self.position = position.clone();
	self.move_handlers.notify(Rc::new(MoveEvent {
	    position: position,
	}), scheduler);
    }

    ///
    /// Returns a reference to the bounding box
    ///
    pub fn bounds(&self) -> &Bounds {
	&self.bounds
    }
	
    ///
    /// Adds a move handler
    ///
    pub fn add_move_handler(&mut self, handler: Rc<dyn EventHandler<MoveEvent>>) {
	self.move_handlers.add(handler);
    }
}

///
/// Signals a widget has been moved
///
pub struct MoveEvent {
    ///
    /// The new position
    ///
    position: Position,
}
