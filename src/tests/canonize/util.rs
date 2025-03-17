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


use crate::{core::general_context::GeneralContext, interfaces::HibouGraphvizLoggerParam, rewriting::canonize::canonize_interaction, seqdiag_lib_interface::io::parse_interaction_from_text};



pub fn tool_test_canonize(
        gen_ctx : GeneralContext,
        input_text : &str, 
        expected_interaction : Option<&str>,
        expected_canonic : Option<&str>,
        svg_logger : Option<&'static str>,
        keep_only_one : bool,
        simplified_text : &str
    ) {
    // we parse the input text and verifify that the obtained interaction is indeed the expected one
    let int = parse_interaction_from_text(
        input_text,
        &gen_ctx
    ).unwrap();
    if let Some(exp_int) = expected_interaction {
        let got_int : String = format!("{:?}",int).chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(exp_int, got_int);
    }

    // we canonize the interaction and verify that it is indeed the expected one
    let gv_log_prm = HibouGraphvizLoggerParam::SeqDiagAndTermTree;
    let graphviz_param : Option<(&GeneralContext,&str,&HibouGraphvizLoggerParam)> = match svg_logger {
        Some(logger_output) => {
            Some((&gen_ctx,logger_output,&gv_log_prm))
        },
        None => {
            None 
        }
    };
    eprintln!("starting canonization");
    let canonized = canonize_interaction(
        &int,
        graphviz_param,
        keep_only_one,
        true
    );
    eprintln!("canonized input");
    if let Some(exp_can) = expected_canonic {
        let got_can : String = format!("{:?}",canonized).chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(exp_can, got_can);
    }

    // finally we parse and canonize the simplified textual version and see if we have the same thing
    let simplified_canonic = {
        canonize_interaction(
            &parse_interaction_from_text(
                simplified_text,
                &gen_ctx
            ).unwrap(),
            None,
            true,
            true
        )
    };
    assert_eq!(canonized,simplified_canonic);
}




pub fn get_gen_ctx() -> GeneralContext {
    GeneralContext::new(
        vec![
            "l1".to_string(),
            "l2".to_string(),
            "l3".to_string(),
        ], 
        vec![
            "m1".to_string(),
            "m2".to_string(),
            "m3".to_string(),
        ],
        vec![]
    )
}

