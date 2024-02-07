use crate::ui::action::Scheduler;
use crate::ui::component::{Component, MovedEvent, ResizedEvent};
use crate::ui::error::Error;
use crate::ui::event::{Handler, HandlerId, Handlers};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// A component with child components
///
pub struct Container {
    ///
    /// A reference to the component 
    ///
    component: Weak<RefCell<Component>>,

    ///
    /// A self reference
    ///
    container: Weak<RefCell<Container>>,

    ///
    /// The child components as a list of rows
    ///
    rows: Vec<Vec<Column>>,
}

impl Container {
    ///
    /// Creates a new container
    ///
    pub fn new(component: &mut Rc<RefCell<Component>>) -> Rc<RefCell<Container>> {
	let mut container = Rc::from(RefCell::from(Container {
	    component: Rc::downgrade(&component),
	    container: Weak::new(),
	    rows: Vec::new(),
	}));
	container.borrow_mut().container = Rc::downgrade(&container);
	component.borrow_mut().add_moved_handler(Rc::from(ContainerMovedHandler {
	    container: Rc::downgrade(&container),
	}));
	container
    }

    ///
    /// Adds a child component
    ///
    pub fn add_column(&mut self, child: Rc<RefCell<Component>>, alignment: Alignment) -> Result<(), Error> {
	let resized_handler_id = child.try_borrow_mut()?.add_resized_handler(Rc::from(ChildResizedHandler {
	    container: self.container.clone(),
	}));
	let column = Column {
	    child: child,
	    alignment,
	    resized_handler_id,
	};
	let len = self.rows.len();
	if len == 0 {
	    self.rows.push(vec![column]);
	} else {
	    self.rows[len - 1].push(column);
	}
	Ok(())
    }

    ///
    /// Adds a row if the current row is not empty
    ///
    pub fn add_row(&mut self) -> bool {
	let len = self.rows.len();
	if len == 0 || self.rows[len - 1].len() != 0 {
	    self.rows.push(Vec::new());
	    true
	} else {
	    false
	}
    }
							
    ///
    /// Updates the position of all children
    ///
    fn update_child_positions(&mut self, scheduler: &mut Scheduler) -> Result<(), Error>{
	Ok(())
    }

    ///
    /// Updates the size of the container and all child positions
    ///
    fn update_size(&mut self, scheduler: &mut Scheduler) -> Result<(), Error> {
	Ok(())
    }
}

///
/// A column with a child component in it
///
struct Column {
    ///
    /// The child component
    ///
    child: Rc<RefCell<Component>>,
    
    ///
    /// Alignment of the child within the row
    ///
    alignment: Alignment,

    ///
    /// The ID of the resize handler
    ///
    resized_handler_id: HandlerId,
}

impl Drop for Column {
    fn drop(&mut self) {
	if let Ok(mut child) = self.child.try_borrow_mut() {
	    child.remove_resized_handler(self.resized_handler_id);
	}
    }
}

///
/// Component alignment
///
pub enum Alignment {
    ///
    /// Align to the left
    ///
    Left,

    ///
    /// Center in a row
    ///
    Center,

    ///
    /// Align to the right
    ///
    Right,
}

///
/// Updates the child positions when the container has been moved
///
struct ContainerMovedHandler {
    ///
    /// The container
    ///
    container: Weak<RefCell<Container>>,
}

impl Handler<MovedEvent> for ContainerMovedHandler {
    fn handle(&self, event: &Rc<MovedEvent>, scheduler: &mut Scheduler) -> Result<(), Error> {
	if let Some(container) = self.container.upgrade() {
	    container.try_borrow_mut()?.update_child_positions(scheduler)?;
	}
	Ok(())
    }
}

///
/// Updates the container size and child position when a child is resized
///
struct ChildResizedHandler {
    ///
    /// The container
    ///
    container: Weak<RefCell<Container>>,
}

impl Handler<ResizedEvent> for ChildResizedHandler {
    fn handle(&self, event: &Rc<ResizedEvent>, scheduler: &mut Scheduler) -> Result<(), Error> {
	if let Some(container) = self.container.upgrade() {
	    container.try_borrow_mut()?.update_size(scheduler)?;
	}
	Ok(())
    }
}
