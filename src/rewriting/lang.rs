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


use crate::core::syntax::action::{EmissionAction, ReceptionAction};
use crate::core::syntax::interaction::LoopKind;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum HibouLangOperators {
    Emission(EmissionAction),
    Reception(ReceptionAction),
    Empty,
    Strict,
    Alt,
    CoReg(Vec<usize>),
    Loop(LoopKind),
    And
}

impl HibouLangOperators {

    pub fn left_and_right_distributes_over(&self, other_op : &Self) -> bool {
        match other_op {
            HibouLangOperators::Alt => {
                match self {
                    HibouLangOperators::Strict   => {true},
                    HibouLangOperators::CoReg(_) => {true},
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
            HibouLangOperators::Emission(_) => {0}
            HibouLangOperators::Reception(_) => {0}
            HibouLangOperators::Empty => {0}
            HibouLangOperators::Strict => {2}
            HibouLangOperators::Alt => {2}
            HibouLangOperators::CoReg(_) => {2}
            HibouLangOperators::Loop(_) => {1}
            HibouLangOperators::And => {2}
        }
    }

    pub fn is_binary_associative(&self) -> bool {
        match self {
            HibouLangOperators::Strict => {true},
            HibouLangOperators::Alt => {true},
            HibouLangOperators::CoReg(_) => {true},
            _ => {false}
        }
    }

}