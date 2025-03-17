/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/


use simple_term_rewriter::core::terms::term::RewritableLanguageOperatorSymbol;

use crate::core::syntax::action::{EmissionAction, ReceptionAction};
use crate::core::syntax::interaction::LoopKind;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum HibouRewritableLangOperator {
    Emission(EmissionAction),
    Reception(ReceptionAction),
    Empty,
    Strict,
    Alt,
    CoReg(Vec<usize>),
    Loop(LoopKind),
    And
}

impl RewritableLanguageOperatorSymbol for HibouRewritableLangOperator {}

impl HibouRewritableLangOperator {

    pub fn left_and_right_distributes_over(&self, other_op : &Self) -> bool {
        match other_op {
            HibouRewritableLangOperator::Alt => {
                match self {
                    HibouRewritableLangOperator::Strict   => {true},
                    HibouRewritableLangOperator::CoReg(_) => {true},
                    _ => {false}
                }
            },
            _ => {
                false 
            }
        }

    }


    pub fn arity(&self) -> usize {
        match self {
            HibouRewritableLangOperator::Emission(_) => {0}
            HibouRewritableLangOperator::Reception(_) => {0}
            HibouRewritableLangOperator::Empty => {0}
            HibouRewritableLangOperator::Strict => {2}
            HibouRewritableLangOperator::Alt => {2}
            HibouRewritableLangOperator::CoReg(_) => {2}
            HibouRewritableLangOperator::Loop(_) => {1}
            HibouRewritableLangOperator::And => {2}
        }
    }

    pub fn is_binary_associative(&self) -> bool {
        match self {
            HibouRewritableLangOperator::Strict => {true},
            HibouRewritableLangOperator::Alt => {true},
            HibouRewritableLangOperator::CoReg(_) => {true},
            _ => {false}
        }
    }

}