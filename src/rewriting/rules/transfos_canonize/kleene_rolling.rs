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
use simple_term_rewriter::core::terms::term::LanguageTerm;
use crate::core::syntax::interaction::LoopKind;
use crate::rewriting::lang::HibouRewritableLangOperator;




pub struct HibouKleeneRoller {}








impl GenericBinaryOperatorSimplifier<HibouRewritableLangOperator> for HibouKleeneRoller {
    fn is_binary(&self, op : &HibouRewritableLangOperator) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouRewritableLangOperator,
        left : &LanguageTerm<HibouRewritableLangOperator>,
        right : &LanguageTerm<HibouRewritableLangOperator>,
    ) -> Option<LanguageTerm<HibouRewritableLangOperator>> {

        if top_operator == &HibouRewritableLangOperator::Alt && left.operator == HibouRewritableLangOperator::Empty {
            match &right.operator {
                HibouRewritableLangOperator::Strict => {
                    // interaction of the form alt(0,strict(x,y))
                    let x = right.sub_terms.first().unwrap();
                    let y = right.sub_terms.get(1).unwrap();
                    if y.operator == HibouRewritableLangOperator::Loop(LoopKind::SStrictSeq) {
                        // interaction of the form alt(0,strict(x,loopS(z)))
                        let z = y.sub_terms.first().unwrap();
                        if x == z {
                            // interaction of the form alt(0,strict(x,loopS(x)))
                            return Some(y.clone());
                        }
                    }
                },
                HibouRewritableLangOperator::CoReg(cr) => {
                    // interaction of the form alt(0,coreg(cr,x,y))
                    let x = right.sub_terms.first().unwrap();
                    let y = right.sub_terms.get(1).unwrap();
                    let loop_op = HibouRewritableLangOperator::Loop(LoopKind::Coreg(cr.clone()));
                    if y.operator == loop_op {
                        // interaction of the form alt(0,coreg(cr,x,loopCr(cr,z)))
                        let z = y.sub_terms.first().unwrap();
                        if x == z {
                            // interaction of the form alt(0,strict(x,loopCr(cr,x)))
                            return Some(y.clone());
                        }
                    }
                },
                _ => {}
            }
        }
        None 

    }

}
