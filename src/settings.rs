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

use clap::{Parser, ValueEnum};

///
/// Models application settings
///
#[derive(Debug, PartialEq)]
pub struct Settings {
    ///
    /// The log level
    ///
    log_level: LogLevel,
}

impl Settings {
    ///
    /// Loads the default settings and then applies configurations specified
    /// in the configuration files and on the command line (in that order) on top of that
    ///
    pub fn load() -> Result<Settings, Error> {
	let mut settings = Settings::default();
	settings.apply_command_line_arguments()?;
	Ok(settings)
    }

    ///
    /// Applies the command line arguments to the settings
    ///
    fn apply_command_line_arguments(&mut self) -> Result<(), Error> {
	let cli_settings = CLISettingsConfiguration::parse();
	if let Some(log_level) = cli_settings.log_level {
	    self.log_level = log_level;
	}
	Ok(())
    }
}

impl Default for Settings {
    ///
    /// Returns the default settings
    ///
    fn default() -> Settings {
	Settings {
	    log_level: LogLevel::Warning,
	}
    }
}

///
/// Models the log level of the application
///
#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum LogLevel {
    ///
    /// Debug messages get printed
    ///
    Debug,
    ///
    /// Info messages get printed
    ///
    Info,
    ///
    /// Non fatal errors get printed
    ///
    Warning,
    ///
    /// Fatal or serious errors get printed
    ///
    Error,
}

///
/// Errors that can occur loading or saving settings
///
#[derive(Debug, PartialEq)]
pub enum Error {}

///
/// The command line settings model
///
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLISettingsConfiguration {
    ///
    /// The log level
    ///
    #[arg(short, long, value_name = "LOG LEVEL")]
    log_level: Option<LogLevel>,
}
