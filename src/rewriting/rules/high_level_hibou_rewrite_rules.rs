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


use simple_term_rewriter::builtin_trs::builtin_transfo::{BuiltinRewriteTransformation, BuiltinRewriteTransformationKind};
use simple_term_rewriter::core::rule::RewriteRule;

use crate::rewriting::lang::HibouRewritableLangOperator;

use crate::rewriting::rules::transfos_canonize::associativity_checker::HibouAssociativityChecker;
use crate::rewriting::rules::transfos_canonize::commutative_checker_and_orderer::HibouCommutativeCheckerAndOrderer;
use crate::rewriting::rules::transfos_canonize::distributivity_checker::HibouDistributivityChecker;

use super::transfos_canonize::basic_alt_deduplication::HibouAltDeduplicator;
use super::transfos_canonize::coregion_minimization::HibouCoregionMinimizer;
use super::transfos_canonize::kleene_tightening::HibouKleeneTightener;
use super::transfos_canonize::sequencing_compatibility::HibouSequencingCompatibilizer;
use super::transfos_canonize::strictness_relaxation::HibouStrictnessRelaxer;
use super::transfos_canonize::empty_interaction_simplifier::HibouEmptyInteractionSimplifier;
use super::transfos_canonize::kleene_nesting::HibouKleeneNestingSimplifier;
use super::transfos_canonize::kleene_rolling::HibouKleeneRoller;




pub enum HighLevelHibouRewriteRules {

    FlushRight,

    ReorderSubInteractionsUnderAlt,
    ReorderSubInteractionsUnderCoregBasic,

    // TODO: this requires further work : modulo coreg op AC: 
    // make a graph of dependencies between sub-terms under the same coreg op
    // then reorder according t
    //CoregionReorderAndMinimizeModuloAC, 

    CoregionMinimizationBasic,
    CoregionMinimizationKleene,

    FactorizeLeft,
    FactorizeRight,
    DeFactorizeLeft,
    DeFactorizeRight,

    EpsilonFixpoint,
    EpsilonNeutral,

    KleeneNesting,
    KleeneTighteningModuloAC,
    KleeneRolling,

    StrictnessRelaxationBinary,
    StrictnessRelaxationUnary,

    SequencingCompatibility,


    BasicAltDeduplication,
    //SummandInclusionModuloAC
}


impl HighLevelHibouRewriteRules {

    pub fn get_low_level_rewrite_rule(&self) -> Box<dyn RewriteRule<HibouRewritableLangOperator>> {
        match self {
            HighLevelHibouRewriteRules::FlushRight => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::AssociativeFlushRight(Box::new(HibouAssociativityChecker{})),
                    desc : "FlushRight".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderAlt => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::ReorderOperandsIfCommutative(Box::new(HibouCommutativeCheckerAndOrderer{consider_alt:true,consider_coreg:false})),
                    desc : "ReorderSubInteractionsUnderAlt".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::ReorderSubInteractionsUnderCoregBasic => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::ReorderOperandsIfCommutative(Box::new(HibouCommutativeCheckerAndOrderer{consider_alt:false,consider_coreg:true})),
                    desc : "ReorderSubInteractionsUnderCoregBasic".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },

            HighLevelHibouRewriteRules::CoregionMinimizationBasic => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(Box::new(HibouCoregionMinimizer{})),
                    desc : "CoregionMinimizationBasic".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::CoregionMinimizationKleene => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderUnary(Box::new(HibouCoregionMinimizer{})),
                    desc : "CoregionMinimizationKleene".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },

            HighLevelHibouRewriteRules::EpsilonFixpoint => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderUnary(Box::new(HibouEmptyInteractionSimplifier{})),
                    desc : "EpsilonFixpoint".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::EpsilonNeutral => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(Box::new(HibouEmptyInteractionSimplifier{})),
                    desc : "EpsilonNeutral".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },

            HighLevelHibouRewriteRules::FactorizeLeft => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::FactorizeLeftDistributiveModuloAC(Box::new(HibouDistributivityChecker{})),
                    desc : "FactorizeLeft".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::FactorizeRight => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::FactorizeRightDistributiveModuloAC(Box::new(HibouDistributivityChecker{})),
                    desc : "FactorizeRight".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::DeFactorizeLeft => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::DeFactorizeLeftDistributive(Box::new(HibouDistributivityChecker{})),
                    desc : "DeFactorizeLeft".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::DeFactorizeRight => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::DeFactorizeRightDistributive(Box::new(HibouDistributivityChecker{})),
                    desc : "DeFactorizeRight".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },


            HighLevelHibouRewriteRules::KleeneNesting => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderUnary(Box::new(HibouKleeneNestingSimplifier{})),
                    desc : "KleeneNesting".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::KleeneTighteningModuloAC => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::ModuloAssociativeFlattenedTransfo(Box::new(HibouKleeneTightener{})),
                    desc : "KleeneTighteningModuloAC".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::KleeneRolling => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(Box::new(HibouKleeneRoller{})),
                    desc : "KleeneRolling".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },

            HighLevelHibouRewriteRules::StrictnessRelaxationBinary => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(Box::new(HibouStrictnessRelaxer{})),
                    desc : "StrictnessRelaxationBinary".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
            HighLevelHibouRewriteRules::StrictnessRelaxationUnary => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderUnary(Box::new(HibouStrictnessRelaxer{})),
                    desc : "StrictnessRelaxationUnary".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },

            HighLevelHibouRewriteRules::SequencingCompatibility => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::GenericSimplifyUnderBinary(Box::new(HibouSequencingCompatibilizer{})),
                    desc : "SequencingCompatibility".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },

            HighLevelHibouRewriteRules::BasicAltDeduplication => {
                Box::new(BuiltinRewriteTransformation{
                    kind : BuiltinRewriteTransformationKind::ModuloAssociativeFlattenedTransfo(Box::new(HibouAltDeduplicator{})),
                    desc : "BasicAltDeduplication".to_owned()
                }) as Box<dyn RewriteRule<HibouRewritableLangOperator>>
            },
        }
    }

}



