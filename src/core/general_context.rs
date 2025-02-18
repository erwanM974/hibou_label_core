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


use std::iter::FromIterator;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GeneralContext {
    lf_names : Vec<String>,
    ms_names : Vec<String>,
    gt_names : Vec<String>
}



impl GeneralContext {

    pub fn new(
        lf_names : Vec<String>,
        ms_names : Vec<String>,
        gt_names : Vec<String>) -> GeneralContext {
        GeneralContext {
            lf_names,
            ms_names,
            gt_names
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_id(&self, lf_name : &str) -> Option<usize> {
        self.lf_names.iter().position(|r| r == lf_name)
    }

    pub fn get_ms_id(&self, ms_name : &str) -> Option<usize> {
        self.ms_names.iter().position(|n| n == ms_name)
    }

    pub fn get_gt_id(&self, gt_name : &str) -> Option<usize> {
        self.gt_names.iter().position(|n| n == gt_name)
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_num(&self) -> usize {
        self.lf_names.len()
    }

    pub fn get_ms_num(&self) -> usize {
        self.ms_names.len()
    }

    pub fn get_gt_num(&self) -> usize {
        self.gt_names.len()
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_all_lfs_ids(&self) -> Vec<usize> {
        Vec::from_iter( 0..self.get_lf_num() )
    }

    pub fn get_lf_names(&self) -> &Vec<String> {
        &self.lf_names
    }

    pub fn get_ms_names(&self) -> &Vec<String> {
        &self.ms_names
    }

    pub fn get_gt_names(&self) -> &Vec<String> {
        &self.gt_names
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_name(&self, lf_id : usize) -> Option<&String> {
        self.lf_names.get(lf_id)
    }

    pub fn get_ms_name(&self, ms_id : usize) -> Option<&String> {
        self.ms_names.get(ms_id)
    }

    pub fn get_gt_name(&self, gt_id : usize) -> Option<&String> {
        self.gt_names.get(gt_id)
    }

}
