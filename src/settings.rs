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

use clap::Parser;
use log::{info, warn};
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use log::debug;
use std::path::PathBuf;

///
/// Models application settings
///
#[derive(Debug, PartialEq)]
pub struct Settings {
    ///
    /// The data path contains all the static application data
    ///
    data_path: PathBuf,
    ///
    /// The window width in pixels
    ///
    window_width: u32,
    ///
    /// The window height in pixels
    ///
    window_height: u32,
}

impl Settings {
    ///
    /// Loads the default settings and then applies configurations specified
    /// in the configuration files and on the command line (in that order) on top of that
    ///
    pub fn load() -> Settings {
	let mut settings = Settings::default();
	settings.apply_file();
	settings.apply_command_line_arguments();
	settings
    }

    ///
    /// Returns the window width in pixels
    ///
    pub fn window_width(&self) -> u32 {
	self.window_width
    }

    ///
    /// Returns the window width in pixels
    ///
    pub fn window_height(&self) -> u32 {
	self.window_height
    }
    
    ///
    /// Applies the command line arguments to the settings
    ///
    fn apply_command_line_arguments(&mut self) {
	let config = CLISettingsConfiguration::parse();
	if let Some(data_path) = config.data_path {
	    self.data_path = data_path;
	}
	if let Some(window_width) = config.window_width {
	    self.window_width = window_width;
	}
	if let Some(window_height) = config.window_height {
	    self.window_height = window_height;
	}
    }

    ///
    /// Tries to find a settings file and applies it to the settings
    ///
    fn apply_file(&mut self) {
	match Settings::find_settings_file() {
	    Some(path) => {
		let mut buffer = String::new();
		match File::open(path.clone()) {
		    Ok(mut file) => {
			match file.read_to_string(&mut buffer) {
			    Ok(_) => {
				match serde_yaml::from_str::<FileSettingsConfiguration>(&buffer) {
				    Ok(config) => {
					if let Some(data_path) = config.data_path {
					    self.data_path = data_path;
					};
					self.window_width = config.window_width;
					self.window_height = config.window_height;
				    },
				    Err(e) => {
					warn!("could not read settings file ({:?}): {:?}", path, e);
				    }
				}
			    },
			    Err(e) => {
				warn!("could not read settings file ({:?}): {:?}", path, e);
			    }
			}
		    },
		    Err(e) => {
			warn!("could not open settings file ({:?}): {:?}", path, e);
		    }   
		}
	    },
	    None => {
		info!("settings file not found");
	    }
	}
    }

    ///
    /// Checks whether the path is the data directory
    ///
    fn is_data_dir(path: &mut PathBuf) -> bool {
	path.push(".data-dir");
	let result = path.is_file();
	path.pop();
	result
    }
    
    ///
    /// Finds the data directory, containing all immutable data
    ///
    fn find_data_dir() -> Option<PathBuf> {
	debug!("trying to find data path");
	let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::new());
	loop {
	    path.push("data");
	    debug!("trying x{:?}", path);
	    if Settings::is_data_dir(&mut path) {
		debug!("found data path: '{:?}'", path);
		return Some(path);
	    }
	    path.pop();
	    if !path.pop() {
		return None;
	    }
	}
    }

    ///
    /// Finds the user data path, containing all mutable user specific data
    ///
    fn find_user_data_dir() -> Option<PathBuf> {
	match home::home_dir() {
	    Some(mut path) => {
		path.push(".hundredyearswar");
		if path.is_dir() {
		    Some(path)
		} else {
		    None
		}
	    },
	    None => None
	}
    }
    
    ///
    /// Tries to find the settings file
    ///
    fn find_settings_file() -> Option<PathBuf> {
	match Settings::find_user_data_dir() {
	    Some(mut path) => {
		path.push("settings.yaml");
		if path.is_file() {
		    Some(path)
		} else {
		    None
		}
	    },
	    None => None,
	}
    }
}

impl Default for Settings {
    ///
    /// Returns the default settings
    ///
    fn default() -> Settings {
	Settings {
	    data_path: Settings::find_data_dir().unwrap_or_else(PathBuf::new),
	    window_width: 800,
	    window_height: 600,
	}
    }
}

///
/// The command line settings model
///
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLISettingsConfiguration {
    ///
    /// The data path
    ///
    #[arg(short, long)]
    data_path: Option<PathBuf>,
    ///
    /// The window width in pixels
    ///
    window_width: Option<u32>,
    ///
    /// The window height in pixels
    ///
    window_height: Option<u32>,
}

///
/// The settings file model
///
#[derive(Deserialize, Serialize)]
pub struct FileSettingsConfiguration {
    ///
    /// The data path
    ///
    data_path: Option<PathBuf>,
    ///
    /// The window width in pixels
    ///
    window_width: u32,
    ///
    /// The window height in pixels
    ///
    window_height: u32,
}
