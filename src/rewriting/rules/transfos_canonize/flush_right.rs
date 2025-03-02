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




use simple_term_rewriter::builtin_trs::rules::flush::AssociativityChecker;

use crate::rewriting::lang::HibouRewritableLangOperator;



pub struct HibouAssocCheckerToFlushStrictRight {}


impl AssociativityChecker<HibouRewritableLangOperator> for HibouAssocCheckerToFlushStrictRight {
    fn is_binary_associative(&self, op : &HibouRewritableLangOperator) -> bool {
        op == &HibouRewritableLangOperator::Strict
    }
}



pub struct HibouAssocCheckerToFlushAltCoregRight {}


impl AssociativityChecker<HibouRewritableLangOperator> for HibouAssocCheckerToFlushAltCoregRight {
    fn is_binary_associative(&self, op : &HibouRewritableLangOperator) -> bool {
        match op {
            HibouRewritableLangOperator::Alt => {
                true
            },
            HibouRewritableLangOperator::CoReg(_) => {
                true
            },
            _ => {false}
        }
    }
}

