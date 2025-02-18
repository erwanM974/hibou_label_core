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

use crate::rewriting::lang::HibouLangOperators;






pub struct HibouEmptyInteractionSimplifier {}



impl GenericBinaryOperatorSimplifier<HibouLangOperators> for HibouEmptyInteractionSimplifier {
    fn is_binary(&self, op : &HibouLangOperators) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouLangOperators,
        left : &LanguageTerm<HibouLangOperators>,
        right : &LanguageTerm<HibouLangOperators>,
    ) -> Option<LanguageTerm<HibouLangOperators>> {
        if left.operator == HibouLangOperators::Empty {
            match top_operator {
                HibouLangOperators::Strict => {
                    return Some(
                        right.clone()
                    );
                },
                HibouLangOperators::CoReg(_) => {
                    return Some(
                        right.clone()
                    );
                },
                _ => {}
            }
        }
        // ***
        if right.operator == HibouLangOperators::Empty {
            match top_operator {
                HibouLangOperators::Strict => {
                    return Some(
                        left.clone()
                    );
                },
                HibouLangOperators::CoReg(_) => {
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




impl GenericUnaryOperatorSimplifier<HibouLangOperators> for HibouEmptyInteractionSimplifier {
    fn is_unary(&self, op : &HibouLangOperators) -> bool {
        op.arity() == 1
    }

    fn try_simplify_under_unary_operator(
        &self,
        top_operator : &HibouLangOperators,
        term_underneath : &LanguageTerm<HibouLangOperators>
    ) -> Option<LanguageTerm<HibouLangOperators>> {
        match (top_operator,&term_underneath.operator) {
            (HibouLangOperators::Loop(_),HibouLangOperators::Empty) => {
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
