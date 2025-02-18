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




use std::cmp::Ordering;

use crate::core::syntax::action::{EmissionAction, ReceptionAction};





impl Ord for EmissionAction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ms_id < other.ms_id {
            return Ordering::Less;
        }
        if self.ms_id > other.ms_id {
            return Ordering::Greater;
        }
        // ***
        if self.orig_lf_id < other.orig_lf_id {
            return Ordering::Less;
        }
        if self.orig_lf_id > other.orig_lf_id {
            return Ordering::Greater;
        }
        // ***
        return Ordering::Equal;
    }
}



impl Ord for ReceptionAction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ms_id < other.ms_id {
            return Ordering::Less;
        }
        if self.ms_id > other.ms_id {
            return Ordering::Greater;
        }
        // ***
        if self.targ_lf_id < other.targ_lf_id {
            return Ordering::Less;
        }
        if self.targ_lf_id > other.targ_lf_id {
            return Ordering::Greater;
        }
        // ***
        return Ordering::Equal;
    }
}






