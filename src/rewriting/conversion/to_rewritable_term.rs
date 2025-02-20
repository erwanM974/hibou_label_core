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



use simple_term_rewriter::core::conversion::to_rewritable_term::FromDomainSpecificTermToRewritableTerm;

use crate::{core::syntax::interaction::Interaction, rewriting::lang::HibouRewritableLangOperator};



impl FromDomainSpecificTermToRewritableTerm<HibouRewritableLangOperator> for Interaction {

    fn get_operator_at_root(&self) -> HibouRewritableLangOperator {
        match self {
            Interaction::Strict(_,_) => {
                HibouRewritableLangOperator::Strict
            },
            Interaction::Alt(_,_) => {
                HibouRewritableLangOperator::Alt
            },
            Interaction::CoReg(cr,_,_) => {
                HibouRewritableLangOperator::CoReg(cr.clone())
            },
            Interaction::Loop(lk,_) => {
                HibouRewritableLangOperator::Loop(lk.clone())
            },
            Interaction::And(_,_) => {
                HibouRewritableLangOperator::And
            }
            Interaction::Empty => {
                HibouRewritableLangOperator::Empty
            },
            Interaction::Emission(emission_action) => {
                HibouRewritableLangOperator::Emission(emission_action.clone())
            },
            Interaction::Reception(reception_action) => {
                HibouRewritableLangOperator::Reception(reception_action.clone())
            },
        }
    }

    fn get_subterms<'a>(&'a self) -> Vec<&'a Self> {
        match self {
            Interaction::Strict(i1, i2) => {
                vec![&*i1,&*i2]
            },
            Interaction::Alt(i1, i2) => {
                vec![&*i1,&*i2]
            },
            Interaction::CoReg(_,i1, i2) => {
                vec![&*i1,&*i2]
            },
            Interaction::Loop(_,i1) => {
                vec![&*i1]
            },
            Interaction::And(i1, i2) => {
                vec![&*i1,&*i2]
            }
            Interaction::Empty => {
                vec![]
            },
            Interaction::Emission(_) => {
                vec![]
            },
            Interaction::Reception(_) => {
                vec![]
            },
        }
    }

}















