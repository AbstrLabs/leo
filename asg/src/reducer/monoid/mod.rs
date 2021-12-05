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

mod bool_and;
pub use bool_and::*;

mod set_append;
pub use set_append::*;

mod vec_append;
pub use vec_append::*;

mod tuple;
pub use tuple::*;

mod forgetful;
pub use forgetful::*;

/// Types that are `magma`s, a basic kind of algebraic structure.
/// See https://en.wikipedia.org/wiki/Magma_(algebra)
pub trait Magma: Default {
    /// Combine an elements of a magma into `self`.
    fn merge(self, other: Self) -> Self;

    /// Combine multiple elements of a magma into `self`, from left to right.
    fn merge_all(self, others: impl Iterator<Item = Self>) -> Self {
        let mut current = self;
        for item in others {
            current = current.merge(item);
        }
        current
    }

    /// Optionally combine an element of a magma into `self`.
    fn merge_option(self, other: Option<Self>) -> Self {
        match other {
            None => self,
            Some(other) => self.merge(other),
        }
    }
}
