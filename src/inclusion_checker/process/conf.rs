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


use graph_process_manager_core::process::config::AbstractProcessConfiguration;

use super::context::InteractionInclusionCheckingContextAndParameterization;
use super::filtration::InteractionInclusionCheckingFiltrationResult;
use super::handler::InteractionInclusionCheckingHandler;
use super::node::InteractionInclusionCheckingNode;
use super::priorities::InteractionInclusionCheckingPriorities;
use super::state::InteractionInclusionCheckingGlobalState;
use super::step::InteractionInclusionCheckingStepKind;



pub struct InteractionInclusionCheckingConfig {}

impl AbstractProcessConfiguration for InteractionInclusionCheckingConfig {
    type ContextAndParameterization = InteractionInclusionCheckingContextAndParameterization;
    // ***
    type AlgorithmOperationHandler = InteractionInclusionCheckingHandler;
    // ***
    type DomainSpecificNode = InteractionInclusionCheckingNode;
    type DomainSpecificStep = InteractionInclusionCheckingStepKind;
    type Priorities = InteractionInclusionCheckingPriorities;
    // ***
    type MutablePersistentState = InteractionInclusionCheckingGlobalState;
    // ***
    type FiltrationResult = InteractionInclusionCheckingFiltrationResult;
}
