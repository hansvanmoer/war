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
/// An action to be executed at some later point
///
pub trait Action : 'static {
    ///
    /// Exectutes the action
    ///
    fn execute(&self, scheduler: &mut Scheduler) -> Result<(), Error>;
}

///
/// A scheduler can be used to execute a list of actions at a later date
///
pub struct Scheduler {
    ///
    /// The scheduled actions
    ///
    scheduled: Vec<Box<dyn Action>>,
}

impl Scheduler {
    ///
    /// Creates a new scheduler
    ///
    pub fn new() -> Scheduler {
	Scheduler {
	    scheduled: Vec::new(),
	}
    }

    ///
    /// Schedules an action
    ///
    pub fn schedule<A: Action>(&mut self, action: A) {
	self.scheduled.push(Box::new(action));
    }

    ///
    /// Whether there are scheduled actions
    ///
    fn has_scheduled(&self) -> bool {
	self.scheduled.len() != 0
    }

    ///
    /// Executes the scheduled actions
    ///
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<(), Error> {
	self.scheduled.iter().try_for_each(|action| action.execute(scheduler))?;
	self.scheduled.clear();
	Ok(())
    }
}

///
/// Provides schedulers and the way to execute the underlying actions
///
pub struct Actions {
    ///
    /// The active scheduler
    ///
    scheduler: Scheduler,

    ///
    /// The action buffer
    ///
    buffer: Scheduler,

    ///
    /// The maximum number of runs before a looping action is terminated
    ///
    max_runs: usize,
}

impl Actions {
    ///
    /// Creates a new actions scheduler pair
    ///
    pub fn new(max_runs: usize) -> Actions {
	Actions {
	    scheduler: Scheduler::new(),
	    buffer: Scheduler::new(),
	    max_runs,
	}
    }

    ///
    /// Gets a mutable reference to the active scheduler
    ///
    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
	&mut self.scheduler
    }

    ///
    /// Executes the scheduled actions
    ///
    pub fn execute(&mut self) -> Result<(), Error>{
	let mut run = 0;
	loop {
	    if run == self.max_runs {
		break Err(Error::ActionLoop)
	    }
	    if !self.scheduler.has_scheduled() {
		break Ok(())
	    }
	    std::mem::swap(&mut self.scheduler, &mut self.buffer);
	    self.buffer.execute(&mut self.scheduler)?;
	}
    }
}
