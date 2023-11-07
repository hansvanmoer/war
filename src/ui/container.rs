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

use crate::dimension::Dimension;
use crate::position::Position;
use crate::ui::component::{Component, Id};
use crate::ui::error::Error;
use crate::ui::event::Listener;
use crate::ui::shape::{MovedEvent, ResizedEvent, Shape, ShapeRef};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// A container
///
pub struct Container {
    
}

pub enum Alignment {
    Left,
    Center,
    Right,
}

struct Column {
    component: Box<dyn ShapeRef>,
}
