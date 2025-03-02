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

use common_sequence_diagram_io::internal_representation::{CommonIoInteractionInterface, InteractionOperatorRepresentation};

use crate::core::syntax::interaction::LoopKind;




#[derive(Debug,Clone)]
pub struct HibouLangCioII {}


#[derive(Debug,Clone)]
pub enum HibouLeafPattern {
    EMPTY,
    BROADCAST(HibouBroadcastLeafPattern)
}

#[derive(Debug,Clone, PartialEq, Eq)]
pub enum HibouBroadcastOrigin {
    ENV,
    LF(usize),
    GT(usize)
}

impl HibouBroadcastOrigin {
    pub fn is_lifeline(&self) -> bool {
        match self {
            HibouBroadcastOrigin::LF(_) => {true},
            _ => {false}
        }
    }
    pub fn is_environment(&self) -> bool {
        match self {
            HibouBroadcastOrigin::ENV => {true},
            _ => {false}
        }
    }
}


#[derive(Debug,Clone)]
pub struct HibouBroadcastLeafPattern {
    pub origin : HibouBroadcastOrigin,
    pub msg_id : usize,
    // we use a Vec instead of HashSet so that it is deterministic (we always have the same order when iterating)
    pub lf_targets : Vec<usize>,
    pub gt_targets : Vec<usize>
}

impl HibouBroadcastLeafPattern {
    pub fn new(origin : HibouBroadcastOrigin, msg_id: usize, lf_targets : Vec<usize>, gt_targets : Vec<usize>) -> Self {
        Self { origin, msg_id, lf_targets, gt_targets }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum HibouOperators {
    Strict,
    Alt,
    Coreg(Vec<usize>),
    Loop(LoopKind),
    And
}

impl InteractionOperatorRepresentation for HibouOperators {

    fn arity(&self) -> usize {
        match &self {
            HibouOperators::Strict => 2,
            HibouOperators::Alt => 2,
            HibouOperators::Coreg(_) => 2,
            HibouOperators::Loop(_) => 1,
            HibouOperators::And => 2,
        }
    }

    fn is_associative(&self) -> bool {
        match &self {
            HibouOperators::Strict => true,
            HibouOperators::Alt => true,
            HibouOperators::Coreg(_) => true,
            HibouOperators::Loop(_) => false,
            HibouOperators::And => false,
        }
    }
}


impl CommonIoInteractionInterface for HibouLangCioII {
    type InteractionLeafPatternType = HibouLeafPattern;
    type InteractionOperatorType = HibouOperators;
}