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



use simple_term_rewriter::builtin_trs::rules::simpl_binary::GenericBinaryOperatorSimplifier;
use simple_term_rewriter::builtin_trs::rules::simpl_unary::GenericUnaryOperatorSimplifier;
use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;
use simple_term_rewriter::core::term::LanguageTerm;

use crate::core::syntax::interaction::LoopKind;
use crate::core::syntax::lang_traits::involve::involves::InvolvesLifelines;
use crate::core::syntax::interaction::Interaction;
use crate::rewriting::lang::HibouRewritableLangOperator;




pub struct HibouCoregionMinimizer {}



// loopCr(cr,i1) -> loopCr( cr ∩  Θ(i1), i1)
impl GenericUnaryOperatorSimplifier<HibouRewritableLangOperator> for HibouCoregionMinimizer {
    fn is_unary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 1
    }

    fn try_simplify_under_unary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        term_underneath : &LanguageTerm<HibouRewritableLangOperator>
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if let HibouRewritableLangOperator::Loop(LoopKind::Coreg(cr)) = top_operator {
            // ***
            let involved_in_i1 = {
                let i1 = Interaction::from_rewritable_term(
                    term_underneath
                );
                i1.involved_lifelines()
            };
            // ***
            let mut new_cr = vec![];
            for lf in cr {
                if involved_in_i1.contains(lf) {
                    new_cr.push(*lf);
                }
            }
            // ***
            if new_cr.len() < cr.len() {
                return Some(
                    LanguageTerm::new(
                        HibouRewritableLangOperator::Loop(LoopKind::Coreg(new_cr)), 
                        vec![term_underneath.clone()]
                    )
                );
            }
        } 
        None 
    }
}





// coreg(cr,i1,i2) -> coreg(cr ∩  Θ(i1) ∩  Θ(i2), i1,i2)
impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouCoregionMinimizer {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {
        if let HibouRewritableLangOperator::CoReg(cr) = top_operator {
            // ***
            let involved_in_i1 = {
                let i1 = Interaction::from_rewritable_term(
                    left
                );
                i1.involved_lifelines()
            };
            // ***
            let involved_in_i2 = {
                let i2 = Interaction::from_rewritable_term(
                    right
                );
                i2.involved_lifelines()
            };
            // ***
            let mut new_cr = vec![];
            for lf in cr {
                if involved_in_i1.contains(lf) && involved_in_i2.contains(lf) {
                    new_cr.push(*lf);
                }
            }
            // ***
            if new_cr.len() < cr.len() {
                return Some(
                    LanguageTerm::new(
                        HibouRewritableLangOperator::CoReg(new_cr), 
                        vec![
                            left.clone(),
                            right.clone()
                            ]
                    )
                );
            }
        }
        None 
    }

}


