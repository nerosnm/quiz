//  error.rs
//  quiz
//
//  Created by Søren Mortensen <soren@neros.dev> on 2020-04-17.
//  Copyright (c) 2020 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
//  http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
//  http://opensource.org/licenses/MIT>, at your option. This file may not be
//  copied, modified, or distributed except according to those terms.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// HTTP 401 (Unauthorized) error.
    #[error("unauthorized")]
    Unauthorized,

    /// HTTP 403 (Forbidden) error.
    #[error("forbidden")]
    Forbidden,

    /// HTTP 404 (Not Found) error.
    #[error("not found")]
    NotFound,

    /// HTTP 422 (Unprocessable Entity) error.
    #[error("unprocessable entity")]
    UnprocessableEntity,

    /// HTTP 500 (Internal Server Error) error.
    #[error("internal server error")]
    InternalServerError,

    /// Serde deserialize error.
    #[error("deserialize error")]
    DeserializeError,

    /// Generic HTTP request error.
    #[error("HTTP request error")]
    RequestError,
}
