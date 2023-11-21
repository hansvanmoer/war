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
use crate::ui::system::System;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// An ID type for components
///
pub type Id = usize;

///
/// All generic component fields go here
///
pub struct Component {
    ///
    /// A unique ID identifying the component
    ///
    id: Id,

    ///
    /// A weak self reference, used to construct more references
    ///
    reference: Box<dyn ComponentWeakRef + 'static>,

    ///
    /// A reference to the underlying system
    ///
    system: Weak<RefCell<System>>,
    
    ///
    /// A reference to the parent component, if any
    ///
    parent: Option<Box<dyn ComponentWeakRef + 'static>>,
}

impl Component {

    ///
    /// Creates a new component
    ///
    pub fn new(system: &Rc<RefCell<System>>) -> Result<Rc<RefCell<Component>>, Error> {
	Ok(Rc::from(RefCell::from(Component {
	    id: system.try_borrow_mut()?.create_id(),
	    reference: NoComponentWeakRef::new(),
	    system: Rc::downgrade(system),
	    parent: None, 
	})))
    }

    ///
    /// Returns the component's unique ID
    ///
    pub fn id(&self) -> Id {
	self.id
    }
    
    ///
    /// Returns a reference to the system
    ///
    pub fn system(&self) -> Result<Rc<RefCell<System>>, Error> {
	self.system.upgrade().ok_or(Error::NoSystem)
    }
    
    ///
    /// Sets the reference to the component itself
    ///
    pub fn set_reference(&mut self, reference: Box<dyn ComponentWeakRef>) {
	self.reference = reference;
    }
    
}

///
/// The reference to a generic component
///
pub trait ComponentRef: {
    ///
    /// Returns a reference to the underlying component
    ///
    fn component(&self) -> Rc<RefCell<Component>>;

    ///
    /// Downgrade this reference to a weak reference
    ///
    fn downgrade(self) -> Result<Box<dyn ComponentWeakRef + 'static>, Error>;    
    
    ///
    /// Creates a new reference to this component
    ///
    fn reference(&self) -> Result<Box<dyn ComponentRef + 'static>, Error> {
	self.component().try_borrow()?.reference.upgrade().ok_or(Error::NoComponent)
    }

    ///
    /// The ID of the widget to which this component belongs
    ///
    fn id(&self) -> Result<Id, Error> {
	Ok(self.component().try_borrow()?.id)
    }
    
    ///
    /// A reference to the parent component, if any
    ///
    fn parent(&self) -> Result<Option<Box<dyn ComponentRef + 'static>>, Error> {
	let component = self.component();
	let component = component.try_borrow()?;
	match component.parent {
	    Some(ref p) => Ok(p.upgrade()),
	    None => Ok(None),
	}
    }
}

///
/// A weak reference to a component
///
pub trait ComponentWeakRef {
    ///
    /// Upgrades the 
    ///
    fn upgrade(&self) -> Option<Box<dyn ComponentRef + 'static>>;
}
			       
pub struct NoComponentWeakRef {}

impl NoComponentWeakRef {
    fn new() -> Box<NoComponentWeakRef> {
	Box::from(NoComponentWeakRef {})
    }
}

impl ComponentWeakRef for NoComponentWeakRef {
    fn upgrade(&self) -> Option<Box<dyn ComponentRef + 'static>> {
	None
    }
}
