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
use crate::ui::widget::{Action, Error, ListenerId, Listeners, WidgetBuilder};

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
    /// Listeners that have to be notified when the widget is moved
    ///
    move_listeners: Listeners,
}

impl Spatial {
    ///
    /// Creates a new spatial component
    ///
    fn new() -> Spatial {
	Spatial {
	    position: Position::default(),
	    preferred_size: Dimension::default(),
	    bounds: Bounds::default(),
	    update_bounds: true,
	}
    }

    ///
    /// Decorates a widget with a spatial component if it is not already present
    ///
    pub fn decorate<'a>(builder: &mut WidgetBuilder<'a>) -> Result<(), Error> {
	if !builder.has_spatial() {
	    builder.set_spatial(Spatial::new());
	}
	Ok(())
    }

    ///
    /// Moves the widget
    ///
    pub fn set_position<'a>(&mut self, position: Position, context: &mut Context<'a>) {
	self.position = position;
	self.move_listeners.notify(context);
    }

    ///
    /// Schedule set position
    ///
    pub fn schedule_set_position(&self, position: Position, context: &mut Context<'a>) {
	context.schedule(widget_id, Rc::from(SetPosition {
	    position,
	}));
    }
	
    ///
    /// Adds a listener
    ///
    pub fn add_move_listener(&mut self, widget_id: WidgetId, action: Rc<dyn Action>) -> ListenerId {
	self.move_listeners.add(widget_id, action)
    }

    ///
    /// Removes a listener
    ///
    pub fn remove_move_listener(&mut self, id: ListenerId) -> bool {
	self.move_listeners.remove(id)
    }
}

///
/// Sets the position of a component
/// 
struct SetPosition {
    ///
    /// The position
    ///
    position: Position,
}

impl Action for SetPosition {
    ///
    /// Sets the position of a component
    ///
    fn execute<'a>(&self, context: &mut Context<'a>) -> Result<(), Error> {
	context.spatial_mut(context.widget_id())?.position = self.position.clone();
	Ok(())
    }
}
