//  component/index.rs
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

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Index
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{"Index!"}</p>
        }
    }
}
