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


use simple_term_rewriter::core::conversion::from_rewritable_term::FromRewritableTermToDomainSpecificTerm;

use crate::core::syntax::interaction::Interaction;
use crate::rewriting::lang::HibouRewritableLangOperator;



impl FromRewritableTermToDomainSpecificTerm<HibouRewritableLangOperator> for Interaction {

     fn instantiate_term_under_operator(
        operator : &HibouRewritableLangOperator, 
        sub_terms : &mut Vec<Self>
    ) -> Self {
        match operator {
            HibouRewritableLangOperator::Emission(emission_action) => {
                Interaction::Emission(emission_action.clone())
            },
            HibouRewritableLangOperator::Reception(reception_action) => {
                Interaction::Reception(reception_action.clone())
            },
            HibouRewritableLangOperator::Empty => {
                Interaction::Empty
            },
            HibouRewritableLangOperator::Strict => {
                let i2 = sub_terms.pop().unwrap();
                let i1 = sub_terms.pop().unwrap();
                Interaction::Strict(Box::new(i1),Box::new(i2))
            },
            HibouRewritableLangOperator::Alt => {
                let i2 = sub_terms.pop().unwrap();
                let i1 = sub_terms.pop().unwrap();
                Interaction::Alt(Box::new(i1),Box::new(i2))
            },
            HibouRewritableLangOperator::CoReg(cr) => {
                let i2 = sub_terms.pop().unwrap();
                let i1 = sub_terms.pop().unwrap();
                Interaction::CoReg(cr.clone(),Box::new(i1),Box::new(i2))
            },
            HibouRewritableLangOperator::Loop(loop_kind) => {
                let i1 = sub_terms.pop().unwrap();
                Interaction::Loop(loop_kind.clone(),Box::new(i1))
            },
            HibouRewritableLangOperator::And => {
                let i2 = sub_terms.pop().unwrap();
                let i1 = sub_terms.pop().unwrap();
                Interaction::And(Box::new(i1),Box::new(i2))
            },
        }
    }

}








