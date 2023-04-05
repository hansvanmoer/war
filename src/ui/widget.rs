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

use crate::arena::Arena;
use crate::ui::button::Button;
use crate::ui::container::Container;
use crate::ui::dialog::Dialog;
use crate::ui::mouse::{MouseButtonTarget, MouseMotionTarget};
use crate::ui::spatial::Spatial;

///
/// Manages the widgets and their component
///
pub struct Manager {
    ///
    /// All widgets managed by this instance
    ///
    widgets: Arena<Widget>,

    ///
    /// All buttons
    ///
    buttons: Area<Button>,
    
    ///
    /// All widgets that can contain other widgets
    ///
    containers: Arena<Container>,

    ///
    /// All dialogs
    ///
    dialogs: Arena<Dialog>,
    
    ///
    /// All widgets that listen to mouse button events
    ///
    mouse_button_targets: Arena<MouseButtonTarget>,

    ///
    /// All widgets that listen to mouse motion events
    ///
    mouse_motion_targets: Arena<MouseMotionTarget>,

    ///
    /// All widgets that have spatial coordinates
    ///
    spatials: Arena<Spatial>,
}

///
/// A widget
///
struct Widget {
    ///
    /// The ID of the button component
    ///
    button_id: Option<ComponentId>,

    ///
    /// The ID of the container component
    ///
    container_id: Option<ComponentId>,

    ///
    /// The ID of the dialog component
    ///
    dialog_id: Option<ComponentId>,

    ///
    /// The ID of the mouse button target component
    ///
    mouse_button_target_id: Option<ComponentId>,

    ///
    /// The ID of the mouse motion target component
    ///
    mouse_motion_target_id: Option<ComponentId>,

    ///
    /// The ID of the spatial component
    ///
    spatial_id: Option<ComponentId>,
}

///
/// An ID type for widgets
///
pub type WidgetId = usize;

///
/// An ID type for womponents
///
pub type ComponentId = usize;

///
/// A builder for a widget
///
pub struct WidgetBuilder<'a> {
    ///
    /// The manager
    ///
    manager: &'a mut Manager,

    ///
    /// The widget
    ///
    widget_id: WidgetId,
}

///
/// A context for widget and component actions
///
pub struct Context<'a> {
    ///
    /// The underlying manager
    ///
    manager: &'a mut Manager,

    ///
    /// The widget ID
    ///
    widget_id: WidgetId,
    
    ///
    /// Scheduled actions
    ///
    actions: LinkedList<ScheduledAction>,
}

impl<'a> Context<'a> {
    ///
    /// Schedules an action
    ///
    pub fn schedule(&mut self, target_id: WidgetId, action: Rc<dyn Action>) {
	self.actions.push_back(ScheduledActions {
	    source_id: self.widget_id,
	    target_id: target_id,
	    action,
	});
    }

    ///
    /// Schedules an action on the same widget
    ///
    pub fn schedule_for_self(&mut self, action: Rc<dyn Action>) {
	self.schedule(self.widget_id, action);
    }

    ///
    /// The ID of the widget currently being acted upon
    ///
    fn widget_id(&self) -> WidgetId {
	self.widget_id
    }
}

///
/// An action to be executed by a component
///
pub trait Action {
    ///
    /// Execute the action
    ///
    fn execute<'a>(&self, context: &mut Context<'a>) -> Result<(), Error>;
}

///
/// A scheduled action
///
pub struct ScheduledAction {
    ///
    /// The ID of the widget that scheduled the action
    ///
    source_id: WidgetId,

    ///
    /// The ID of the widget that must execute the action
    ///
    target_id: WidgetId,

    ///
    /// The action
    ///
    action: Rc<dyn Action>,
}

///
/// An ID type for a listener
///
pub type ListenerId = usize;

///
/// A listener
///
struct Listener {
    ///
    /// The listener ID
    ///
    id: ListenerId,
    
    ///
    /// The target widget ID,
    ///
    target_id: WidgetId,

    ///
    /// The action to schedule
    ///
    action: Rc<dyn Action>,
}

///
/// A set of listeners
///
pub struct Listeners {
    ///
    /// The entries
    ///
    listeners: Arena<Listener>,
}

impl Listeners {
    ///
    /// Triggers the listeners
    ///
    pub fn notify<'a>(&self, context: &mut Context<'a>) {
	self.listeners.iter().for_each(|entry| context.schedule(entry.target_id, entry.action.clone()));
    }

    ///
    /// Adds a listener
    ///
    pub fn add(&mut self, widget_id: WidgetId, action: Rc<dyn Action>) -> ListenerId {
	self.listeners.insert(Listener {
	    id,
	    target_id: widget_id,
	    actions,
	})
    }

    ///
    /// Removes a listener
    ///
    pub fn remove(&mut self, id: ListenerId) -> bool {
	self.listeners.remove(id).is_some()
    }
}

///
/// Errors that can occur when working widgets and components
///
pub enum Error {
    ///
    /// The widget does not exist
    ///
    NoWidget,

    ///
    /// The component does not exist
    ///
    NoComponent,
}

macro_rules! define_component {
    ($type:ident, $id:ident, $arena:ident, $has:ident, $get:ident, $mut:ident, $set:ident) => {
	impl<'a> WidgetBuilder<'a> {
	    ///
	    /// Adds a $type to the widget
	    ///
	    pub fn $set(component: $type) -> Result<(), Error> {
		let component_id = self.manager.$arena.insert(component);
		self.manager.widgets.get_mut(self.widget_id).ok_or(Error::NoWidget).$id = Some(component_id);
		Ok(())
	    }

	    ///
	    /// Checks whether a widget has a $type component
	    ///
	    pub fn $has() -> Result<bool, Error> {
		Ok(self.manager.widgets.get(self.widget_id)?.$id.is_some())
	    } 
	}

	impl<'a> Context<'a> {
	    ///
	    /// Gets a reference to a component
	    ///
	    pub fn $get(widget_id: WidgetId) -> Result<&'a $type, Error> {
		let component_id = self.manager.widgets.get(widget_id).ok_or(Error::NoWidget)?.$id.ok_or(Error::NoComponent);
		self.manager.$arena.get(component_id).ok_or(Error::NoComponent)
	    }

	    ///
	    /// Gets a reference to a component
	    ///
	    pub fn $mut(widget_id: WidgetId) -> Result<&'a mut $type, Error> {
		let component_id = self.manager.widgets.get(widget_id).ok_or(Error::NoWidget)?.$id.ok_or(Error::NoComponent);
		self.manager.$arena.get_mut(component_id).ok_or(Error::NoComponent)
	    }
	}
    }
}

define_component!(Button, button_id, buttons, has_button, get_button, mut_button, set_button);
define_component!(Container, container_id, containers, has_container, get_container, mut_container, set_container);
define_component!(Dialog, dialog_id, dialogs, has_dialog, get_dialog, mut_dialog, set_dialog);
define_component!(MouseButtonTarget, mouse_button_target_id, mouse_button_targets, has_mouse_button_target, get_mouse_button_target, mut_mouse_button_target, set_mouse_button_target);
define_component!(MouseMotionTarget, mouse_motion_target_id, mouse_motion_targets, has_mouse_motion_target, get_mouse_motion_target, mut_mouse_motion, set_mouse_motion);
define_component!(Spatial, spatial_id, spatials, has_spatial, get_spatial, mut_spatial, set_spatial);
