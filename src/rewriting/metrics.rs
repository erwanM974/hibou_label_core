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

use std::hash::Hash;
use std::collections::HashSet;

use maplit::hashset;
use simple_term_rewriter::metrics::TermSymbolMetric;

use crate::core::syntax::interaction::LoopKind;

use super::lang::HibouRewritableLangOperator;



#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub enum InteractionTermSymbolMetrics {
    Lifeline(usize),
    Message(usize),
    Empty,
    Emission,
    Reception,
    Strict,
    Seq,
    CoregNonSeq,
    LoopStrict,
    LoopWeak,
    AnyLoop,
    Alt,
    And
}

impl std::fmt::Display for InteractionTermSymbolMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionTermSymbolMetrics::Lifeline(id) => {
                write!(f,"lf_id_{}", id)
            },
            InteractionTermSymbolMetrics::Message(id) => {
                write!(f,"ms_id_{}", id)
            },
            InteractionTermSymbolMetrics::Empty => {
                write!(f,"Empty")
            },
            InteractionTermSymbolMetrics::Emission => {
                write!(f,"Emission")
            },
            InteractionTermSymbolMetrics::Reception => {
                write!(f,"Reception")
            },
            InteractionTermSymbolMetrics::Strict => {
                write!(f,"Strict")
            },
            InteractionTermSymbolMetrics::Seq => {
                write!(f,"Seq")
            },
            InteractionTermSymbolMetrics::CoregNonSeq => {
                write!(f,"CoregNonSeq")
            },
            InteractionTermSymbolMetrics::LoopStrict => {
                write!(f,"LoopStrict")
            },
            InteractionTermSymbolMetrics::LoopWeak => {
                write!(f,"LoopWeak")
            },
            InteractionTermSymbolMetrics::AnyLoop => {
                write!(f,"AnyLoop")
            },
            InteractionTermSymbolMetrics::Alt => {
                write!(f,"Alt")
            },
            InteractionTermSymbolMetrics::And => {
                write!(f,"And")
            },
        }
    }
}



impl TermSymbolMetric<HibouRewritableLangOperator> for InteractionTermSymbolMetrics {
    fn measure_nested_depth(&self) -> bool {
        match self {
            InteractionTermSymbolMetrics::AnyLoop => {
                true
            },
            _ => {
                false
            }
        }
    }

    fn from_operator_symbol(op : &HibouRewritableLangOperator) -> HashSet<Self> {
        match op {
            HibouRewritableLangOperator::Emission(emission_action) => {
                hashset!{
                    InteractionTermSymbolMetrics::Lifeline(emission_action.orig_lf_id),
                    InteractionTermSymbolMetrics::Message(emission_action.ms_id),
                    InteractionTermSymbolMetrics::Emission,
                }
            },
            HibouRewritableLangOperator::Reception(reception_action) => {
                hashset!{
                    InteractionTermSymbolMetrics::Lifeline(reception_action.targ_lf_id),
                    InteractionTermSymbolMetrics::Message(reception_action.ms_id),
                    InteractionTermSymbolMetrics::Reception,
                }
            },
            HibouRewritableLangOperator::Empty => {
                hashset!{
                    InteractionTermSymbolMetrics::Empty,
                }
            },
            HibouRewritableLangOperator::Strict => {
                hashset!{
                    InteractionTermSymbolMetrics::Strict,
                }
            },
            HibouRewritableLangOperator::Alt => {
                hashset!{
                    InteractionTermSymbolMetrics::Alt,
                }
            },
            HibouRewritableLangOperator::CoReg(cr) => {
                if cr.is_empty() {
                    hashset!{
                        InteractionTermSymbolMetrics::Seq,
                    }
                } else {
                    hashset!{
                        InteractionTermSymbolMetrics::CoregNonSeq,
                    }
                }
            },
            HibouRewritableLangOperator::Loop(loop_kind) => {
                match loop_kind {
                    LoopKind::SStrictSeq => {
                        hashset!{
                            InteractionTermSymbolMetrics::LoopStrict,
                            InteractionTermSymbolMetrics::AnyLoop,
                        }
                    },
                    LoopKind::HHeadFirstWS => {
                        hashset!{
                            InteractionTermSymbolMetrics::AnyLoop,
                        }
                    },
                    LoopKind::Coreg(_) => {
                        hashset!{
                            InteractionTermSymbolMetrics::LoopWeak,
                            InteractionTermSymbolMetrics::AnyLoop,
                        }
                    },
                }
            },
            HibouRewritableLangOperator::And => {
                hashset!{
                    InteractionTermSymbolMetrics::And,
                }
            },
        }
    }
}








