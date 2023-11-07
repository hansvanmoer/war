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

use crate::ui::error::Error;

///
/// The UI subsystem
///
pub struct System {
    scheduled: Vec<Box<dyn Action>>,
}

impl System {
    ///
    /// Schedules an action for later
    ///
    pub fn schedule(&mut self, action: Box<dyn Action>) {
	self.scheduled.push(action);
    }
}

///
/// An action to be executed later
///
pub trait Action {
    ///
    /// Executes the action
    ///
    fn execute(&self) -> Result<(), Error>;
}

macro_rules! impl_component {
    ($component_trait:ty, $component:ty, $widget:ty, $field:ident, $getter:ident, $mutator:ident) => {
	impl $component_trait for $widget {
	    ///
	    /// Gets the $component
	    ///
	    fn $getter(&self) -> &$component {
		&self.$field
	    }

	    ///
	    /// Gets a mutable reference to the $component
	    ///
	    fn $mutator(&mut self) -> &mut $component {
		&mut self.$field
	    }
	}
    }
}
