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

use graph_process_manager_core::{process::{filter::GenericFiltersManager, logger::AbstractProcessLogger, manager::GenericProcessManager}, queue::{priorities::GenericProcessPriorities, strategy::QueueSearchStrategy}};
use simple_term_rewriter::{core::conversion::{from_rewritable_term::FromRewritableTermToDomainSpecificTerm, to_rewritable_term::FromDomainSpecificTermToRewritableTerm}, process::{conf::RewriteConfig, context::{RewritingProcessContextAndParameterization, RewritingProcessPhase}, node::RewriteNodeKind, priorities::RewritePriorities}};

use crate::core::syntax::interaction::Interaction;

use super::{lang::HibouRewritableLangOperator, rules::high_level_hibou_rewrite_rules::HighLevelHibouRewriteRules};


pub fn canonize_interaction(
    int : &Interaction,
    loggers : Vec<Box< dyn AbstractProcessLogger<RewriteConfig<HibouRewritableLangOperator>>>>,
    keep_only_one : bool
) -> Interaction {
    let int_as_term = int.to_rewritable_term();

    let phase1 = RewritingProcessPhase::<HibouRewritableLangOperator>::new(
        vec![
            HighLevelHibouRewriteRules::FlushRight.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderAlt.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderCoregBasic.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::CoregionMinimizationBasic.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::CoregionMinimizationKleene.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::EpsilonFixpoint.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::EpsilonNeutral.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneNesting.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneTighteningModuloAC.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneRolling.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::StrictnessRelaxationBinary.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::StrictnessRelaxationUnary.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::BasicAltDeduplication.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::SequencingCompatibility.get_low_level_rewrite_rule(),
            // ***
            HighLevelHibouRewriteRules::DeFactorizeLeft.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::DeFactorizeRight.get_low_level_rewrite_rule(),
        ],
        keep_only_one
    );

    let phase2 = RewritingProcessPhase::new(
        vec![
            HighLevelHibouRewriteRules::FlushRight.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderAlt.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderCoregBasic.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::CoregionMinimizationBasic.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::CoregionMinimizationKleene.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::EpsilonFixpoint.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::EpsilonNeutral.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneNesting.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneTighteningModuloAC.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneRolling.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::StrictnessRelaxationBinary.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::StrictnessRelaxationUnary.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::BasicAltDeduplication.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::SequencingCompatibility.get_low_level_rewrite_rule(),
            // ***
            HighLevelHibouRewriteRules::FactorizeRight.get_low_level_rewrite_rule(),
        ],
        keep_only_one
    );

    let phase3 = RewritingProcessPhase::new(
        vec![
            HighLevelHibouRewriteRules::FlushRight.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderAlt.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderCoregBasic.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::CoregionMinimizationBasic.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::CoregionMinimizationKleene.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::EpsilonFixpoint.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::EpsilonNeutral.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneNesting.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneTighteningModuloAC.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::KleeneRolling.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::StrictnessRelaxationBinary.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::StrictnessRelaxationUnary.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::BasicAltDeduplication.get_low_level_rewrite_rule(),
            HighLevelHibouRewriteRules::SequencingCompatibility.get_low_level_rewrite_rule(),
            // ***
            HighLevelHibouRewriteRules::FactorizeLeft.get_low_level_rewrite_rule(),
        ],
        keep_only_one
    );

    let context_and_param = RewritingProcessContextAndParameterization::new(vec![phase1,phase2,phase3]);

    // ***

    let mut manager : GenericProcessManager<RewriteConfig<HibouRewritableLangOperator>> = GenericProcessManager::new(
        context_and_param,
        QueueSearchStrategy::DFS,
        GenericProcessPriorities::new(RewritePriorities::default(),false),
        GenericFiltersManager::new(
            vec![], 
            vec![], 
            vec![]
        ),
        loggers,
        true
    );

    manager.start_process(RewriteNodeKind::new(
        int_as_term,
        0)
    );

    let x = manager.global_state.irreducible_terms_per_phase.get(&2).unwrap();
    let result = x.first().unwrap();

    Interaction::from_rewritable_term(result)
}












