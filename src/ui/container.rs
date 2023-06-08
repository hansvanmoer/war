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
use crate::ui::spatial::{MoveEvent, Spatial};
use crate::ui::event::EventHandler;
use crate::ui::style::Margins;
use crate::ui::widget::{Action, Context, Error, Scheduler, WidgetBuilder, WidgetId};

use std::rc::Rc;

///
/// A container component for widgets that contain other widgets
///
pub struct Container {
    ///
    /// Inner margins
    ///
    margins: Margins,
    
    ///
    /// The rows and columns within the container
    ///
    rows: Vec<Vec<Column>>,

    ///
    /// Child positions and size has to be updated
    ///
    update: bool,
}

impl Container {
    ///
    /// Creates a new container
    ///
    fn new(margins: &Margins) -> Container {
	Container {
	    margins: margins.clone(),
	    rows: Vec::new(),
	    update: false,
	}
    }
    
    ///
    /// Decorates a widget with a container
    ///
    pub fn decorate(builder: &mut WidgetBuilder) -> Result<(), Error> {
	Spatial::decorate(builder)?;
	if !builder.has_container()? {
	    builder.set_container(Container::new(builder.style().container().margins()))?;
	    let widget_id = builder.widget_id();
	    builder.spatial_mut()?.add_move_handler(Rc::new(UpdateChildren {}));
	}
	Ok(())
    }

    ///
    /// Adds a new column
    ///
    pub fn add_column(&mut self, id: WidgetId, alignment: Alignment) {
	let column = Column {
	    widget_id: id,
	    alignment,
	};
	if self.rows.len() == 0 {
	    self.rows.push(vec![column]);
	} else {
	    let last_row_id = self.rows.len() - 1;
	    self.rows[last_row_id].push(column);
	}
    }

    ///
    /// Adds a new row
    ///
    pub fn add_row(&mut self) {
	self.rows.push(Vec::new());
    }
    
    ///
    /// Schedules an update of each child's position
    ///
    fn update_children<'a>(&self, context: &Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	let container_margins = &self.margins;
	let margins = context.style().container().margins();
	let spatial = context.spatial(context.widget_id())?;

	let (mut x, mut y) = spatial.position().coordinates();
	x += container_margins.left();
	y += container_margins.right();
	
	let width = spatial.preferred_size().width();
	for row in self.rows.iter() {
	    let mut height = 0.0;
	    let mut center_width = 0.0;
	    let mut right_width = 0.0;
	    for col in row.iter() {
		let mut size = context.spatial(col.widget_id)?.preferred_size();
		if size.height() > height {
		    height = size.height();
		}
		match col.alignment {
		    Alignment::Center => {
			center_width += size.width() + margins.horizontal();
		    },
		    Alignment::Right => {
			right_width += size.width() + margins.horizontal();
		    },
		    _ => {},
		}
	    }
	    let mut left = x;
	    let mut center = x + (width - center_width) / 2.0;
	    let mut right = x + width - right_width;
	    for col in row.iter() {
		let position = Position::new(match col.alignment {
		    Alignment::Left => {
			let x = left;
			left += context.spatial(col.widget_id)?.preferred_size().width() + margins.horizontal();
			x
		    },
		    Alignment::Center => {
			let x = center;
			center += context.spatial(col.widget_id)?.preferred_size().width() + margins.horizontal();
			x
		    },
		    Alignment::Right => {
			let x = right;
			right += context.spatial(col.widget_id)?.preferred_size().width() + margins.horizontal();
			x
		    }
		}, y);
		scheduler.schedule(col.widget_id, Rc::from(SetPosition {
		    widget_id: col.widget_id, 
		    position, 
		}))
	    }
	    if height > 0.0 {
		y += height + margins.vertical();
	    }
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

///
/// Sets the position of a component
/// 
struct SetPosition {
    ///
    /// The widget's ID
    ///
    widget_id: WidgetId,
    
    ///
    /// The position
    ///
    position: Position,
}

impl Action for SetPosition {
    ///
    /// Sets the position of a component
    ///
    fn execute<'a>(&self, context: &mut Context<'a>, scheduler: &mut Scheduler) -> Result<(), Error> {
	context.spatial_mut(self.widget_id)?.set_position(self.position.clone(), scheduler);
	Ok(())
    }
}
