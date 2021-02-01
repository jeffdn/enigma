// src/ui/generic.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use anyhow::Result;

pub trait UiAgent {
    fn start(self) -> Result<ApplicationExitReason>;
}

pub enum ApplicationExitReason {
    UserExit,
    Reload(usize),
}
