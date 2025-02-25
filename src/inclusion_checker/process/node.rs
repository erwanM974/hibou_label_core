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




use std::collections::BTreeSet;

use graph_process_manager_core::process::config::AbstractNodeKind;
use crate::core::syntax::interaction::Interaction;




#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InteractionInclusionCheckingNode {
    pub included_candidate : Interaction,
    pub loop_depth : u32,
    pub including_candidates : BTreeSet<Interaction>
}

impl InteractionInclusionCheckingNode {
    pub fn new(
        included_candidate : Interaction,
        loop_depth : u32,
        including_candidates : BTreeSet<Interaction>
    ) -> Self {
        Self { included_candidate, loop_depth, including_candidates }
    }
}


impl AbstractNodeKind for InteractionInclusionCheckingNode {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        if self.included_candidate == memoized_node.included_candidate && 
        self.including_candidates == memoized_node.including_candidates {
            if self.loop_depth >= memoized_node.loop_depth {
                // means that we might have already explored more from the memoized node
                return true;
            }
        }
        false
    }
}

