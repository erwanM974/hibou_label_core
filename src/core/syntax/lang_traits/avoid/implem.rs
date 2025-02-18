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

use crate::core::syntax::interaction::Interaction;

use super::avoids::AvoidsLifelines;




impl AvoidsLifelines for Interaction {
    fn avoids_all_of(&self, lf_ids: &BTreeSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                true
            },
            &Interaction::Emission(ref em_act) => {
                !lf_ids.contains(&em_act.orig_lf_id)
            },
            &Interaction::Reception(ref rc_act) => {
                !lf_ids.contains(&rc_act.targ_lf_id)
            },
            &Interaction::Strict(ref i1, ref i2) => {
                i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids)
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids)
            },
            /*&Interaction::Sync(_, ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },*/
            &Interaction::Alt(ref i1, ref i2) => {
                i1.avoids_all_of(lf_ids) || i2.avoids_all_of(lf_ids)
            },
            &Interaction::Loop(_, _) => {
                true
            },
            &Interaction::And(_,_) => {
                panic!("non-conform interaction")
            }
        }
    }
}