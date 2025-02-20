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



use simple_term_rewriter::{builtin_trs::rules::{simpl_binary::GenericBinaryOperatorSimplifier, simpl_unary::GenericUnaryOperatorSimplifier}, core::term::LanguageTerm};

use crate::rewriting::lang::HibouRewritableLangOperator;






pub struct HibouEmptyInteractionSimplifier {}



impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouEmptyInteractionSimplifier {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if left.operator == HibouRewritableLangOperator::Empty {
            match top_operator {
                HibouRewritableLangOperator::Strict => {
                    return Some(
                        right.clone()
                    );
                },
                HibouRewritableLangOperator::CoReg(_) => {
                    return Some(
                        right.clone()
                    );
                },
                _ => {}
            }
        }
        // ***
        if right.operator == HibouRewritableLangOperator::Empty {
            match top_operator {
                HibouRewritableLangOperator::Strict => {
                    return Some(
                        left.clone()
                    );
                },
                HibouRewritableLangOperator::CoReg(_) => {
                    return Some(
                        left.clone()
                    );
                },
                _ => {}
            }
        }
        // ***
        None 
    }
}




impl GenericUnaryOperatorSimplifier<HibouRewritableLangOperator> for HibouEmptyInteractionSimplifier {
    fn is_unary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 1
    }

    fn try_simplify_under_unary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        term_underneath : &LanguageTerm<HibouRewritableLangOperator>
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        match (top_operator,&term_underneath.operator) {
            (HibouRewritableLangOperator::Loop(_),HibouRewritableLangOperator::Empty) => {
                Some(
                    term_underneath.clone()
                )
            },
            (_,_) => {
                None 
            }
        } 
    }
}
