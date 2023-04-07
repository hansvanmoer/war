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

use crate::ui::spatial::Spatial;
use crate::ui::widget::{Action, Context, Error, Scheduler, WidgetBuilder, WidgetId};

use std::rc::Rc;

///
/// A container component for widgets that contain other widgets
///
pub struct Container {
    ///
    /// The rows and columns within the container
    ///
    children: Vec<Vec<WidgetId>>,
}

impl Container {
    ///
    /// Creates a new container
    ///
    fn new() -> Container {
	Container {
	    children: Vec::new(),
	}
    }
    
    ///
    /// Decorates a widget with a container
    ///
    fn decorate(builder: &mut WidgetBuilder) -> Result<(), Error> {
	Spatial::decorate(builder)?;
	if !builder.has_container()? {
	    builder.set_container(Container::new())?;
	    let widget_id = builder.widget_id();
	    builder.spatial_mut()?.add_move_listener(widget_id, Rc::new(UpdateChildren {}));
	}
	Ok(())
    }
}

///
/// Update children action
///
struct UpdateChildren {}

impl Action for UpdateChildren {
    ///
    /// Updates child positions after move
    ///
    fn execute<'a>(&self, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	Ok(())
    }
}
