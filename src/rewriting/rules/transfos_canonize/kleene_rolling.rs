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





use simple_term_rewriter::{builtin_trs::rules::simpl_binary::GenericBinaryOperatorSimplifier, core::term::LanguageTerm};

use crate::core::syntax::interaction::LoopKind;
use crate::rewriting::lang::HibouLangOperators;




pub struct HibouKleeneRoller {}








impl GenericBinaryOperatorSimplifier<HibouLangOperators> for HibouKleeneRoller {
    fn is_binary(&self, op : &HibouLangOperators) -> bool {
        op.arity() == 2
    }

    fn try_simplify_under_binary_operator(
        &self,
        top_operator : &HibouLangOperators,
        left : &LanguageTerm<HibouLangOperators>,
        right : &LanguageTerm<HibouLangOperators>,
    ) -> Option<LanguageTerm<HibouLangOperators>> {

        if top_operator == &HibouLangOperators::Alt && left.operator == HibouLangOperators::Empty {
            match &right.operator {
                HibouLangOperators::Strict => {
                    // interaction of the form alt(0,strict(x,y))
                    let x = right.sub_terms.first().unwrap();
                    let y = right.sub_terms.get(1).unwrap();
                    if y.operator == HibouLangOperators::Loop(LoopKind::SStrictSeq) {
                        // interaction of the form alt(0,strict(x,loopS(z)))
                        let z = y.sub_terms.first().unwrap();
                        if x == z {
                            // interaction of the form alt(0,strict(x,loopS(x)))
                            return Some(y.clone());
                        }
                    }
                },
                HibouLangOperators::CoReg(cr) => {
                    // interaction of the form alt(0,coreg(cr,x,y))
                    let x = right.sub_terms.first().unwrap();
                    let y = right.sub_terms.get(1).unwrap();
                    let loop_op = HibouLangOperators::Loop(LoopKind::Coreg(cr.clone()));
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
