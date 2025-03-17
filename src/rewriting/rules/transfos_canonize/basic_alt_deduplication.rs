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

use simple_term_rewriter::core::terms::term::LanguageTerm;
use simple_term_rewriter::builtin_trs::rules::modulo_associative_flattened_transfo::ModuloAssociativeGenericFlattenedChecker; 

use crate::rewriting::lang::HibouRewritableLangOperator;


pub struct HibouAltDeduplicator {}


impl ModuloAssociativeGenericFlattenedChecker<HibouRewritableLangOperator> for HibouAltDeduplicator {
    fn is_an_associative_binary_operator_we_may_consider(
        &self, 
        op : &HibouRewritableLangOperator
    ) -> bool {
        op == &HibouRewritableLangOperator::Alt
    }

    fn requires_a_specific_parent_operator(
        &self,
    ) -> Option<Box<dyn Fn(&HibouRewritableLangOperator) -> bool>> {
        None
    }

    fn transform_flattened_sub_terms(
        &self, 
        _considered_ac_op : &HibouRewritableLangOperator, 
        _considered_parent_op : Option<&HibouRewritableLangOperator>,
        flattened_subterms : Vec<&LanguageTerm<HibouRewritableLangOperator>>
    ) -> Option<Vec<LanguageTerm<HibouRewritableLangOperator>>> {
        let mut new_flattened_subterms : Vec<LanguageTerm<HibouRewritableLangOperator>> = vec![];
        for sub_term in &flattened_subterms {
            if !new_flattened_subterms.contains(*sub_term) {
                new_flattened_subterms.push((*sub_term).clone())
            }
        }
        if new_flattened_subterms.len() < flattened_subterms.len() {
            Some(new_flattened_subterms)
        } else {
            None 
        }
    }
}







