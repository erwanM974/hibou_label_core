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
use crate::core::general_context::GeneralContext;
use crate::core::semantics::trace_action::{TraceAction, TraceActionKind};
use image::Rgb;
use image_colored_text::text::line::ColoredTextLine;
use image_colored_text::text::paragraph::{ColoredTextParagraph, MultiLineTextAlignment};


use crate::commons::hibou_color_palette::{HCP_BLACK, HCP_STANDARD_BLUE, HCP_STANDARD_PURPLE, HC_GRAMMAR_SYMBOL, HC_LIFELINE, HC_MESSAGE};
use crate::commons::util::new_image_with_colored_text;
use crate::commons::{DRAWING_GRAPHIC_FONT, SCALE};

use crate::inclusion_checker::process::conf::InteractionInclusionCheckingConfig;
use crate::inclusion_checker::process::context::InteractionInclusionCheckingContextAndParameterization;
use crate::inclusion_checker::process::filtration::InteractionInclusionCheckingFiltrationResult;
use crate::inclusion_checker::process::node::InteractionInclusionCheckingNode;
use crate::inclusion_checker::process::step::InteractionInclusionCheckingStepKind;



pub struct HibouInclusionCheckingAllTheRestDrawer {
    pub font : FontRef<'static>,
    pub gen_ctx : GeneralContext
}

impl HibouInclusionCheckingAllTheRestDrawer {
    pub fn new(gen_ctx : GeneralContext) -> Self {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        Self {font, gen_ctx}
    }
}

pub fn diagram_repr_trace_action(action : &TraceAction, gen_ctx : &GeneralContext) -> Vec<(String,Rgb<u8>)> {
    let mut to_print = Vec::new();
    // ***
    {
        let lf_name = gen_ctx.get_lf_name(action.lf_id).unwrap();
        to_print.push( (lf_name.to_string(),Rgb(HC_LIFELINE)) );
    }
    // ***
    match &action.act_kind {
        &TraceActionKind::Reception => {
            to_print.push( ("?".to_string(),Rgb(HC_GRAMMAR_SYMBOL)) );
        },
        &TraceActionKind::Emission => {
            to_print.push( ("!".to_string(),Rgb(HC_GRAMMAR_SYMBOL)) );
        }
    }
    // ***
    {
        let ms_name = gen_ctx.get_ms_name(action.ms_id).unwrap();
        to_print.push( (ms_name.to_string(),Rgb(HC_MESSAGE)) );
    }
    // ***
    return to_print;
}

impl CustomAllTheRestDrawerForGraphvizLogger<InteractionInclusionCheckingConfig> for HibouInclusionCheckingAllTheRestDrawer {

    fn get_step_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        step : &InteractionInclusionCheckingStepKind,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        let line = match step {
            InteractionInclusionCheckingStepKind::ExecuteAction(frt_elt,_) => {
                let act = frt_elt.target_actions.iter().next().unwrap();
                let mut line = diagram_repr_trace_action(act,&self.gen_ctx);
                line.push( (format!("@"), Rgb(HCP_STANDARD_PURPLE)) );
                line.push( (format!("{:}", frt_elt.position), Rgb(HCP_BLACK)) );
                ColoredTextLine::new(line)
            },
            InteractionInclusionCheckingStepKind::Normalize(_,_) => {
                ColoredTextLine::new(
                    vec![
                        ("normalize".to_string(), Rgb(HCP_STANDARD_BLUE))
                    ]
                )
            },
            InteractionInclusionCheckingStepKind::ContextSimplification(_,_) => {
                ColoredTextLine::new(
                    vec![
                        ("context_simplification".to_string(), Rgb(HCP_STANDARD_BLUE))
                    ]
                )
            },
        };
        let para = ColoredTextParagraph::new(
            vec!(line),
            MultiLineTextAlignment::Center,
            None,
            None
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
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _step : &InteractionInclusionCheckingStepKind,
    ) -> GraphvizColor {
        GraphvizColor::black
    }
    
    fn get_filter_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        filtration_result: &InteractionInclusionCheckingFiltrationResult,
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
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _filtration_result: &InteractionInclusionCheckingFiltrationResult,
    ) -> graphviz_dot_builder::colors::GraphvizColor {
        GraphvizColor::red
    }
    
    fn get_node_phase_id(
        &self,
        _context_and_param: &InteractionInclusionCheckingContextAndParameterization,
        _new_node: &InteractionInclusionCheckingNode
    ) -> Option<usize> {
        None 
    }
    
    fn get_phase_color(&self, _phase_id : usize) -> graphviz_dot_builder::colors::GraphvizColor {
        GraphvizColor::black
    }

}



