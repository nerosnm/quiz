//  agent/teams.rs
//  quiz
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-17.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

use yew::{prelude::*, services::fetch::FetchTask};

use crate::{agent::Requests, error::Error, types::Team};

#[derive(Default, Debug)]
pub struct Teams {
    requests: Requests,
}

impl Teams {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get all teams
    pub fn all(&mut self, callback: Callback<Result<Vec<Team>, Error>>) -> FetchTask {
        self.requests.get::<Vec<Team>>("/teams".into(), callback)
    }
}
