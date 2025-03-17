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

use std::path::Path;
use ab_glyph::FontRef;
use graph_process_manager_loggers::graphviz::drawers::all_the_rest_drawer::CustomAllTheRestDrawerForGraphvizLogger;
use graph_process_manager_loggers::graphviz::item::{BuiltinGraphvizLoggerDefaultGvItemStyle, BuiltinGraphvizLoggerItemStyle};
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::node::style::GvNodeShape;


use simple_term_rewriter::rewriting_process::conf::RewriteConfig;
use simple_term_rewriter::rewriting_process::context::RewritingProcessContextAndParameterization;
use simple_term_rewriter::rewriting_process::filtration::RewritingFiltrationResult;
use simple_term_rewriter::rewriting_process::loggers::glog::all_the_rest_drawer_utils::get_step_node_inner_style_as_image_paragraph;
use simple_term_rewriter::rewriting_process::node::RewriteNodeKind;
use simple_term_rewriter::rewriting_process::step::RewriteStepKind;

use crate::commons::hibou_color_palette::{HCP_BLACK, HCP_STANDARD_RED};
use crate::commons::util::new_image_with_colored_text;
use crate::commons::{DRAWING_GRAPHIC_FONT, SCALE};
use crate::rewriting::lang::HibouRewritableLangOperator;



pub struct HibouRewritingAllTheRestDrawer {
    pub font : FontRef<'static>,
}

impl HibouRewritingAllTheRestDrawer {
    pub fn new() -> Self {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        Self {font}
    }
}

impl CustomAllTheRestDrawerForGraphvizLogger<RewriteConfig<HibouRewritableLangOperator>> for HibouRewritingAllTheRestDrawer {

    fn get_step_node_inner_style_and_draw_if_needed(
        &self,
        context_and_param: &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        step : &RewriteStepKind<HibouRewritableLangOperator>,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        let para = get_step_node_inner_style_as_image_paragraph(
            context_and_param,
            step,
            HCP_BLACK,
            HCP_STANDARD_RED
        );
        new_image_with_colored_text(
            full_path,
            &para,
            &self.font,
            SCALE
        );
        BuiltinGraphvizLoggerItemStyle::CustomImage
    }
    
    fn get_step_edge_color(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        _step : &RewriteStepKind<HibouRewritableLangOperator>,
    ) -> GraphvizColor {
        GraphvizColor::black
    }
    
    fn get_filter_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        filtration_result: &RewritingFiltrationResult,
        _image_file_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        BuiltinGraphvizLoggerItemStyle::Default(
            BuiltinGraphvizLoggerDefaultGvItemStyle::new(
                GvNodeShape::Rectangle,
                filtration_result.to_string(), 
                18, 
                None,
                GraphvizColor::red, 
                GraphvizColor::red, 
                GraphvizColor::wheat
            )
        )
    }
    
    fn get_filter_edge_color(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        _filtration_result: &<RewriteConfig<HibouRewritableLangOperator> as graph_process_manager_core::process::config::AbstractProcessConfiguration>::FiltrationResult,
    ) -> graphviz_dot_builder::colors::GraphvizColor {
        GraphvizColor::red
    }
    
    fn get_node_phase_id(
        &self,
        _context_and_param: &RewritingProcessContextAndParameterization<HibouRewritableLangOperator>,
        new_node: &RewriteNodeKind<HibouRewritableLangOperator>
    ) -> Option<usize> {
        Some(new_node.concrete_rewrite_phase_index)
    }
    
    fn get_phase_color(&self, phase_id : usize) -> graphviz_dot_builder::colors::GraphvizColor {
        [
            GraphvizColor::lightskyblue,
            GraphvizColor::lightgoldenrod1,
            GraphvizColor::seagreen1,
            GraphvizColor::lightsalmon
        ].get(phase_id % 4).unwrap().clone()
    }

}



