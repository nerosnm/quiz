//  app.rs
//  quiz
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-16.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

use yew::prelude::*;
use yew_router::{router::Router, Switch};

use crate::component::{Index, TeamsList};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/teams"]
    Teams,
    #[to = "/"]
    Index,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html! { <Index /> },
                        AppRoute::Teams => html! { <TeamsList /> },
                    }
                })
            />
        }
    }
}
