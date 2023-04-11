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

use crate::ui::spatial::{MoveEvent, Spatial};
use crate::ui::event::EventHandler;
use crate::ui::widget::{Action, Context, Error, Scheduler, WidgetBuilder, WidgetId};

use std::rc::Rc;

///
/// A container component for widgets that contain other widgets
///
pub struct Container {
    ///
    /// The rows and columns within the container
    ///
    rows: Vec<Vec<Column>>,
}

impl Container {
    ///
    /// Creates a new container
    ///
    fn new() -> Container {
	Container {
	    rows: Vec::new(),
	}
    }
    
    ///
    /// Decorates a widget with a container
    ///
    pub fn decorate(builder: &mut WidgetBuilder) -> Result<(), Error> {
	Spatial::decorate(builder)?;
	if !builder.has_container()? {
	    builder.set_container(Container::new())?;
	    let widget_id = builder.widget_id();
	    builder.spatial_mut()?.add_move_handler(Rc::new(UpdateChildren {}));
	}
	Ok(())
    }
    
    ///
    /// Schedules an update of each child's position
    ///
    fn update_children<'a>(&self, context: &Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	for row in self.rows.iter() {
	    
	}
	Ok(())
    }
}

///
/// A layout column
///
struct Column {
    ///
    /// The widget ID
    ///
    widget_id: WidgetId,

    ///
    /// Alignment
    ///
    alignment: Alignment,
}

///
/// Widget alignment
///
pub enum Alignment {
    ///
    /// Aligns the widget to the left
    ///
    Left,

    ///
    /// Aligns the widget in the center
    ///
    Center,

    ///
    /// Aligns the widget to the right
    ///
    Right,
}

///
/// Updates the position of each child when the parent widget moves
///
struct UpdateChildren {}

impl EventHandler<MoveEvent> for UpdateChildren {
    ///
    /// Updates child positions after move
    ///
    fn handle_event<'a>(&self, event: &Rc<MoveEvent>, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	context.container(context.widget_id())?.update_children(context, scheduler)
    }
}
