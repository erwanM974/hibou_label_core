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


use std::path::{Path, PathBuf};
use graph_process_manager_loggers::graphviz::drawers::node_drawer::CustomNodeDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::item::BuiltinGraphvizLoggerItemStyle;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};

use simple_term_rewriter::draw_term::{draw_term_tree_with_graphviz, TermDrawingContext};
use simple_term_rewriter::process::conf::RewriteConfig;
use simple_term_rewriter::process::context::RewritingProcessContextAndParameterization;
use simple_term_rewriter::process::node::RewriteNodeKind;

use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::LoopKind;
use crate::rewriting::lang::HibouRewritableLangOperator;



pub struct HibouRewritingNodeAsTermDrawer {
    pub gen_ctx : GeneralContext
}

impl TermDrawingContext<HibouRewritableLangOperator> for HibouRewritingNodeAsTermDrawer {
    fn get_operator_representation_as_graphviz_node_style(
        &self, 
        operator : &HibouRewritableLangOperator
    ) -> GraphvizNodeStyle {
        let label = match operator {
            HibouRewritableLangOperator::Emission(emission_action) => {
                format!(
                    "{}!{}", 
                    self.gen_ctx.get_lf_name(emission_action.orig_lf_id).unwrap(), 
                    self.gen_ctx.get_ms_name(emission_action.ms_id).unwrap(), 
                )
            },
            HibouRewritableLangOperator::Reception(reception_action) => {
                format!(
                    "{}?{}", 
                    self.gen_ctx.get_lf_name(reception_action.targ_lf_id).unwrap(), 
                    self.gen_ctx.get_ms_name(reception_action.ms_id).unwrap(), 
                )
            },
            HibouRewritableLangOperator::Empty => {
                "∅".to_owned()
            },
            HibouRewritableLangOperator::Strict => {
                "strict".to_owned()
            },
            HibouRewritableLangOperator::Alt => {
                "alt".to_owned()
            },
            HibouRewritableLangOperator::CoReg(cr) => {
                let cr_lfs : Vec<String> = cr.iter().map(|lf_id| self.gen_ctx.get_lf_name(*lf_id).unwrap().clone()).collect();
                format!(
                    "coreg({})",
                    cr_lfs.join(",")
                )
            },
            HibouRewritableLangOperator::Loop(lk) => {
                match lk {
                    LoopKind::HHeadFirstWS => {
                        "loopH".to_owned()
                    },
                    LoopKind::SStrictSeq => {
                        "loopS".to_owned()
                    },
                    LoopKind::Coreg(cr) => {
                        let cr_lfs : Vec<String> = cr.iter().map(|lf_id| self.gen_ctx.get_lf_name(*lf_id).unwrap().clone()).collect();
                        format!(
                            "loopC({})",
                            cr_lfs.join(",")
                        )
                    },
                }
            },
            HibouRewritableLangOperator::And => {
                "and".to_owned()
            },
        };
        vec![
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
            GraphvizNodeStyleItem::Label(label)
        ]
    }
}

impl CustomNodeDrawerForGraphvizLogger<RewriteConfig<HibouRewritableLangOperator>> for HibouRewritingNodeAsTermDrawer {

    fn get_node_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param : &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        node : &RewriteNodeKind<HibouRewritableLangOperator>,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        // ***
        let temp_file_name = "temp.dot";
        let temp_path : PathBuf = [&temp_file_name].iter().collect();
        // ***
        draw_term_tree_with_graphviz::<HibouRewritableLangOperator,HibouRewritingNodeAsTermDrawer>(
            self,
            &node.term,
            &temp_path.as_path(),
            full_path
        );
        BuiltinGraphvizLoggerItemStyle::CustomImage
    }

}




