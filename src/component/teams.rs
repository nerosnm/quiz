//  component/teams.rs
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

use crate::{agent::Teams, error::Error, types::Team};

pub struct TeamsList {
    teams: Teams,
    teams_list: Option<Vec<Team>>,
    response: Callback<Result<Vec<Team>, Error>>,
    task: Option<FetchTask>,
}

#[derive(Debug)]
pub enum Msg {
    Response(Result<Vec<Team>, Error>),
}

impl Component for TeamsList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            teams: Teams::new(),
            teams_list: None,
            response: link.callback(Msg::Response),
            task: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.task = Some(self.teams.all(self.response.clone()));

        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(teams_list)) => {
                self.teams_list = Some(teams_list);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
        }

        true
    }

    fn view(&self) -> Html {
        if let Some(teams_list) = &self.teams_list {
            html! {
                <ul>
                    {
                        for teams_list.iter().map(|team| html! {
                            <li>{ &team.name }</li>
                        })
                    }
                </ul>
            }
        } else {
            html! {}
        }
    }
}
