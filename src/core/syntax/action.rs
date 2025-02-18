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



use std::fmt::Debug;




#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd)]
pub struct EmissionAction {
    pub orig_lf_id : usize,
    pub ms_id : usize,
    pub target_gates : Vec<usize>
}

impl EmissionAction {
    pub fn new(orig_lf_id : usize,
               ms_id : usize,
               target_gates : Vec<usize>) -> EmissionAction {
        EmissionAction{orig_lf_id,ms_id,target_gates}
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd)]
pub struct ReceptionAction {
    pub origin_gate : Option<usize>,
    pub ms_id : usize,
    pub targ_lf_id : usize
}

impl ReceptionAction {
    pub fn new(
        origin_gate : Option<usize>,
        ms_id : usize,
        targ_lf_id : usize) -> ReceptionAction {
        ReceptionAction{origin_gate,ms_id,targ_lf_id}
    }
}

