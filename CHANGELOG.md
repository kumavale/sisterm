# Change Log

## [3.0.1] - 2023-02-09
### Fixed
- Fixed bug about Can't exit on windows with german keyboard layout (#18)

## [3.0.0] - 2022-09-03
### Added
- Supported SSH
- Get default TerminalType from environment variable
- Generate config (`config.toml`) on build
- Make it Asynchronous

### Fixed
- Fixed bug about cannot connect with telnet

--------------------------------------------------------------------------------

## [2.4.0] - 2020-10-31
### Added
- Supported the terminal type ANSI
- Added a config option: terminal_type
- Added escape seqences  
    \~., \~^D, \~^Z, \~n, \~t, \~i, \~d, \~\~, \~!, \~$, \~?

### Fixed
- Fixed bug about Ctrl+Arrow keys
- Fixed bug about Ctrl+4..=Ctrl+7 in Linux
- Fixed typo


## [2.3.2] - 2020-10-24
### Fixed
- Fixed a bug that multibyte characters are not displayed properly
- Fixed a bug related to log file overwrite confirmation


## [2.3.1] - 2020-10-24
### Fixed
- Fixed a bug that some keys were not working properly
- When the enter key is pressed,'CR' is sent by default, and 'CRLF' is sent as an option


## [2.3.0] - 2020-06-08
### Changed
- Changed to ignore whitespace except comment lines
- Changed a comment when connecting

### Added
- Supported Debug mode
- Supported TCP connection without telnet
- Supported a space character(s) between <host> and <TCP port>
- Supported a negotiate when resizing the window in telnet
- Supported a specify login user
- Supported enable timestamp by default
- Supported auto save log
- Supported specify log format
- Supported specify log destination

### Fixed
- Fixed a bug about priority of `instead_cr`
- Tweak the regex in the config file


## [2.2.0] - 2020-05-29
### Changed
- Changed to record the time when the enter key is pressed in the timestamp
- Changed default value: `READ_BUFFER_SIZE` 16 => 1024

### Fixed
- Fixed Telnet not working


## [2.1.0] - 2020-05-28
### Added
- Supported arrow keys
- Supported for hostname in telnet
- Color syntax: "#RRGGBB"
- Set default buffer size
- Set default TCP connect timeout
- Variable timestamp format


## [2.0.0] - 2020-05-27
### Added
- Initial release

