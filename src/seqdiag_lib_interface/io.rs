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


use std::fs;
use std::path::Path;
use common_sequence_diagram_io::conversion::lang_to_repr::FromInteractionTermToInternalRepresentation;
use common_sequence_diagram_io::conversion::repr_to_lang::FromInternalRepresentationToInteractionTerm;
use common_sequence_diagram_io::from_text::parse::parse_interaction;
use common_sequence_diagram_io::to_image::interface::draw_interaction_as_sequence_diagram;
use common_sequence_diagram_io::to_text::print::print_interaction;
use crate::rewriting::draw_as_term::draw_interaction_as_term_tree_on_file;
use crate::seqdiag_lib_interface::internal_representation::HibouLangCioII;
use crate::core::general_context::GeneralContext;
use crate::seqdiag_lib_interface::to_image::drawing_context::HibouDrawingContext;
use crate::core::syntax::interaction::Interaction;




pub fn parse_interaction_from_text(
    raw_str_input : &str,
    ctx : &GeneralContext
) -> Result<Interaction,String> {
    match parse_interaction::
    <HibouLangCioII,GeneralContext>(
        raw_str_input,
        ctx
    ) {
        Ok(internal_repr) => {
            let interaction = Interaction::from_io_repr(&internal_repr);
            Ok(interaction)
        },
        Err(e2) => {
            Err(e2.to_string())
        }
    }
}



pub fn read_interaction_from_text_on_file(
    file_path : &Path,
    ctx : &GeneralContext
) -> Result<Interaction,String> {
    match fs::read_to_string(file_path) {
        Ok(data) => {
            parse_interaction_from_text(&data,ctx)
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub fn write_interaction_as_text_on_file(
    file_path : &Path,
    ctx : &GeneralContext,
    int : &Interaction, 
    merge_patterns : bool
) {
    let as_txt = print_interaction::<HibouLangCioII,GeneralContext>(
        &int.to_io_repr(merge_patterns),
        ctx
    );
    let _ = fs::write(file_path, as_txt);
}



pub enum InteractionDrawingKind {
    AsSequenceDiagram,
    AsTermTree
}


pub fn draw_interaction_on_file(
    file_path : &Path,
    ctx : &GeneralContext,
    int : &Interaction,
    draw_kind : &InteractionDrawingKind
) {
    match draw_kind {
        InteractionDrawingKind::AsSequenceDiagram => {
            let draw_ctx = HibouDrawingContext::new(ctx.clone());
            draw_interaction_as_sequence_diagram::<HibouLangCioII,usize,HibouDrawingContext,HibouDrawingContext>(
                &int.to_io_repr(true),
                &draw_ctx,
                &draw_ctx,
                file_path
            );
        },
        InteractionDrawingKind::AsTermTree => {
            draw_interaction_as_term_tree_on_file(
                file_path,
                ctx,
                int
            );
        }
    }
}















