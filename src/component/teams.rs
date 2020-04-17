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

use yew::prelude::*;

pub struct Teams;

impl Component for Teams {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Teams
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <ul>
                <li>{ "Team 1" }</li>
                <li>{ "Team 2" }</li>
                <li>{ "Team 3" }</li>
            </ul>
        }
    }
}
