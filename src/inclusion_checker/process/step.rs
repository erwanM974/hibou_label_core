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

use crate::core::semantics::frontier::FrontierElement;
use crate::core::syntax::interaction::Interaction;




pub enum InteractionInclusionCheckingStepKind {
    // execute an action both on the included candidate and the set of including candidates
    ExecuteAction(
        FrontierElement, // the frontier element to execute on the included candidate
        BTreeSet<Interaction> // the next set of including candidates
    ),
    // normalize both the included candidate and all the including candidates
    Normalize(Interaction,BTreeSet<Interaction>),
    // if the included interaction and all including interactions share the same context
    // we remove it ang only keep the different sub-interactions
    ContextSimplification(Interaction,BTreeSet<Interaction>)
}


