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



use simple_term_rewriter::{builtin_trs::rules::modulo_associative_flattened_transfo::ModuloAssociativeFlattenedChecker, core::term::LanguageTerm};

use crate::rewriting::lang::HibouRewritableLangOperator;


pub struct HibouKleeneTightener {}


impl ModuloAssociativeFlattenedChecker<HibouRewritableLangOperator> for HibouKleeneTightener {
    fn is_an_associative_binary_operator_we_may_consider(
        &self, 
        op : &HibouRewritableLangOperator
    ) -> bool {
        op == &HibouRewritableLangOperator::Alt
    }

    fn if_required_is_a_parent_unary_operator_we_may_consider(
        &self,
        op : &HibouRewritableLangOperator
    ) -> Option<bool> {
        if let HibouRewritableLangOperator::Loop(_) = op {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn transform_flattened_sub_terms(
        &self, 
        _considered_ac_op : &HibouRewritableLangOperator, 
        considered_parent_op : Option<&HibouRewritableLangOperator>,
        flattened_subterms : Vec<&LanguageTerm<HibouRewritableLangOperator>>
    ) -> Option<(Option<HibouRewritableLangOperator>,Vec<LanguageTerm<HibouRewritableLangOperator>>)> {
        if let HibouRewritableLangOperator::Loop(lk_outer) = considered_parent_op.unwrap() {
            // ***
            let mut made_at_least_one_simplification = false;
            // ***
            let mut new_flattened_subterms : Vec<LanguageTerm<HibouRewritableLangOperator>> = vec![];
            for sub_term in &flattened_subterms {
                if let HibouRewritableLangOperator::Loop(lk_inner) = &sub_term.operator {
                    if let Some(true) = lk_outer.is_more_permissive(lk_inner) {
                        let sub_sub_term = sub_term.sub_terms.first().unwrap();
                        new_flattened_subterms.push(sub_sub_term.clone());
                        made_at_least_one_simplification = true;
                    } else {
                        new_flattened_subterms.push((*sub_term).clone());
                    }
                } else {
                    new_flattened_subterms.push((*sub_term).clone());
                }
            }
            // ***
            return if made_at_least_one_simplification {
                Some(
                    (
                        Some(HibouRewritableLangOperator::Loop(lk_outer.clone())),
                        new_flattened_subterms
                    )
                )
            } else {
                None 
            };
        }
        panic!("should never be reached")
    }
}








