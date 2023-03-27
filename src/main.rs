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

mod engine;
mod graphics;
mod settings;

use crate::engine::Engine;
use crate::settings::Settings;

use log::{debug, info};

///
/// Main application entry point
///
fn main() {

    env_logger::init();
    info!("application started");
    
    // load settings
    let settings = Settings::load();
    debug!("settings loaded: {:?}", settings);

    let engine = Engine::new().expect("could not initialize engine subsystem");
    debug!("engine loaded");

    let _graphics = engine.create_graphics(&settings).expect("could not initialize graphics subsystem");
    debug!("graphics subsystem loaded");
}

