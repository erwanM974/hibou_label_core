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

use super::util::{get_gen_ctx, tool_test_canonize};








#[test]
pub fn test_undisturbed_alt_factorize() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
seq(
        l1 -- m1 -> l2,
        alt(
                l2 -- m2 -> l1,
                0
        )
)
    "#;

    let simplified_text = r#"
seq(
        l1 -- m1 -> l2,
        alt(
                l2 -- m2 -> l1,
                0
        )
)
    "#;

    let expected_int : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    Alt(
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        ),
        Empty
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    Alt(
        Empty,
        Strict(
            Emission(EmissionAction{orig_lf_id:1,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:0})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        Some(&expected_int),
        Some(&expected_canonized),
        None,
        true,
        &simplified_text
    )
}















#[test]
pub fn test_alt_left_factorize() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
alt(
    seq(
        l1 -- m1 -> l2,
        l1 -- m2 -> l2
    ),
    seq(
        l1 -- m1 -> l2,
        l1 -- m3 -> l2
    )
)
    "#;

    let simplified_text = r#"
seq(
        l1 -- m1 -> l2,
        alt(
            l1 -- m2 -> l2,
            l1 -- m3 -> l2
        )
)
    "#;

    let expected_int : String = r#"
Alt(
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
        )
    ),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:1})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    ),
    Alt(
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:1})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        Some(&expected_int),
        Some(&expected_canonized),
        None,
        true,
        &simplified_text
    )
}
















#[test]
pub fn test_alt_right_factorize() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
alt(
    seq(
        l1 -- m2 -> l2,
        l1 -- m1 -> l2
    ),
    seq(
        l1 -- m3 -> l2,
        l1 -- m1 -> l2
    )
)
    "#;

    let simplified_text = r#"
seq(
        alt(
            l1 -- m2 -> l2,
            l1 -- m3 -> l2
        ),
        l1 -- m1 -> l2
)
    "#;

    let expected_int : String = r#"
Alt(
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        )
    ),
    CoReg([],
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_canonized : String = r#"
CoReg([],
    Alt(
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:1,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:1,targ_lf_id:1})
        ),
        Strict(
            Emission(EmissionAction{orig_lf_id:0,ms_id:2,target_gates:[]}),
            Reception(ReceptionAction{origin_gate:None,ms_id:2,targ_lf_id:1})
        )
    ),
    Strict(
        Emission(EmissionAction{orig_lf_id:0,ms_id:0,target_gates:[]}),
        Reception(ReceptionAction{origin_gate:None,ms_id:0,targ_lf_id:1})
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_canonize(
        gen_ctx,
        input_text,
        Some(&expected_int),
        Some(&expected_canonized),
        None,
        true,
        &simplified_text
    )
}









#[test]
pub fn test_alt_factorize_both_possible() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
alt(
    seq(
        l1 -- m1 -> l2,
        l1 -- m2 -> l2
    ),
    seq(
        l1 -- m1 -> l2,
        l1 -- m3 -> l2
    ),
    seq(
        l1 -- m2 -> l2,
        l1 -- m3 -> l2
    )
)
    "#;

    let simplified_text = r#"
alt(
    seq(
        l1 -- m1 -> l2,
        alt(
            l1 -- m2 -> l2,
            l1 -- m3 -> l2
        )
    ),
    seq(
        l1 -- m2 -> l2,
        l1 -- m3 -> l2
    )
)
    "#;

    tool_test_canonize(
        gen_ctx,
        input_text,
        None,
        None,
        None,
        true,
        &simplified_text
    )
}




#[test]
pub fn test_alt_factorize_both_possible_from_other_possiblity() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
alt(
    seq(
        l1 -- m1 -> l2,
        l1 -- m2 -> l2
    ),
    seq(
        alt(
            l1 -- m1 -> l2,
            l1 -- m2 -> l2
        ),
        l1 -- m3 -> l2
    )
)
    "#;

    let simplified_text = r#"
alt(
    seq(
        l1 -- m1 -> l2,
        alt(
            l1 -- m2 -> l2,
            l1 -- m3 -> l2
        )
    ),
    seq(
        l1 -- m2 -> l2,
        l1 -- m3 -> l2
    )
)
    "#;

    tool_test_canonize(
        gen_ctx,
        input_text,
        None,
        None,
        None,
        true,
        &simplified_text
    )
}






#[test]
pub fn test_factorize_and_sequencing_undisturbed() {

    let gen_ctx = get_gen_ctx();
    let input_text = r#"
seq(
    l1 -- m1 -> l2,
    alt(
        l1 -- m2 -> l2,
        l1 -- m3 -> l2
    ),
    alt(
        l2 -- m1 -> l1,
        l2 -- m2 -> l1,
        l2 -- m3 -> l1
    )
)
    "#;

    tool_test_canonize(
        gen_ctx,
        input_text,
        None,
        None,
        None,
        true,
        input_text
    )
}

