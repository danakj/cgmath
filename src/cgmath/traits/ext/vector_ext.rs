// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::{zero, one};

use traits::alg::Field;
use traits::alg::VectorSpace;
use traits::alg::Array;

pub trait VectorExt
<
    S: Field,
    Slice
>
:   VectorSpace<S>
+   Array<S, Slice>
{
    #[inline] fn add_s(&self, s: S) -> Self { self.map(|x| x.add(&s)) }
    #[inline] fn sub_s(&self, s: S) -> Self { self.map(|x| x.sub(&s)) }
    #[inline] fn mul_s(&self, s: S) -> Self { self.map(|x| x.mul(&s)) }
    #[inline] fn div_s(&self, s: S) -> Self { self.map(|x| x.div(&s)) }
    #[inline] fn rem_s(&self, s: S) -> Self { self.map(|x| x.rem(&s)) }

    #[inline] fn add_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.add(b) ) }
    #[inline] fn sub_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.sub(b) ) }
    #[inline] fn mul_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.mul(b) ) }
    #[inline] fn div_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.div(b) ) }
    #[inline] fn rem_v(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.rem(b) ) }

    #[inline] fn neg_self(&mut self)         { for x in self.mut_iter() { *x = x.neg()   } }
    #[inline] fn add_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.add(&s) } }
    #[inline] fn sub_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.sub(&s) } }
    #[inline] fn mul_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.mul(&s) } }
    #[inline] fn div_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.div(&s) } }
    #[inline] fn rem_self_s(&mut self, s: S) { for x in self.mut_iter() { *x = x.rem(&s) } }

    #[inline] fn add_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.add(b) } }
    #[inline] fn sub_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.sub(b) } }
    #[inline] fn mul_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.mul(b) } }
    #[inline] fn div_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.div(b) } }
    #[inline] fn rem_self_v(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.rem(b) } }

    #[inline] fn comp_add(&self) -> S { self.iter().fold(zero::<S>(), |a, b| a.add(b)) }
    #[inline] fn comp_mul(&self) -> S { self.iter().fold(one::<S>(),  |a, b| a.mul(b)) }
}
