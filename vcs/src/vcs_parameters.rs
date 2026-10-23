
use crate::vcs_console_type::ConsoleType;

pub struct VcsParameters {
    pub console_type: ConsoleType,
}

impl VcsParameters {

    pub fn new() -> VcsParameters {
        Self {
            console_type: ConsoleType::NTSC,
        }
    }

}

