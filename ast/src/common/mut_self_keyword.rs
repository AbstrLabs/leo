// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Identifier, Node};
use leo_errors::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// The `&self` keyword can view and modify struct values inside of a struct function.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct RefSelfKeyword {
    pub identifier: Identifier,
}

impl fmt::Display for RefSelfKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&self")
    }
}

impl Node for RefSelfKeyword {
    fn span(&self) -> &Span {
        &self.identifier.span
    }

    fn set_span(&mut self, span: Span) {
        self.identifier.span = span;
    }
}
