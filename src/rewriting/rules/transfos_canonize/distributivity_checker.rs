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





use simple_term_rewriter::builtin_trs::rules::factorize::DistributivityChecker;

use crate::rewriting::lang::HibouLangOperators;



pub struct HibouDistributivityChecker {}


impl DistributivityChecker<HibouLangOperators> for HibouDistributivityChecker {
    fn is_binary(&self, op : &HibouLangOperators) -> bool {
        op.arity() == 2
    }

    fn is_left_distributive_over(&self, op1 : &HibouLangOperators, op2 : &HibouLangOperators) -> bool {
        op1.left_and_right_distributes_over(op2)
    }

    fn is_right_distributive_over(&self, op1 : &HibouLangOperators, op2 : &HibouLangOperators) -> bool {
        op1.left_and_right_distributes_over(op2)
    }
}



