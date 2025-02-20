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



use simple_term_rewriter::{builtin_trs::rules::simpl_unary::GenericUnaryOperatorSimplifier, core::term::LanguageTerm};

use crate::rewriting::lang::HibouRewritableLangOperator;






pub struct HibouKleeneNestingSimplifier {}





impl GenericUnaryOperatorSimplifier<HibouRewritableLangOperator> for HibouKleeneNestingSimplifier {
    fn is_unary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 1
    }

    fn try_simplify_under_unary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        term_underneath : &LanguageTerm<HibouRewritableLangOperator>
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        match (top_operator,&term_underneath.operator) {
            (HibouRewritableLangOperator::Loop(lk2),HibouRewritableLangOperator::Loop(lk1)) => {
                if let Some(lk) = lk2.get_most_permissive(lk1) {
                    Some(
                        LanguageTerm::new(
                            HibouRewritableLangOperator::Loop(lk),
                            term_underneath.sub_terms.clone()
                        )
                    )
                } else {
                    None
                }
            },
            (_,_) => {
                None 
            }
        } 
    }
}