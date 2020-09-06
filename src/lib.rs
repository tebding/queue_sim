//lib.rs

//note: regular Vec<> would work just as well, but VecDeque is a bit neater
use std::{cell::Cell, collections::VecDeque};

pub struct Job {
    pub arrival: u32,
    pub duration: u32,
    pub finish: /*Cell<u32>*/u32,
    pub wait: /*Cell<u32>*/u32,
}

impl Job {
    pub fn new(/**/) -> Job {
        
    }

    pub fn calc_fin(/**/) {
        
    }

    pub fn reset_fin(/**/) {
        
    }
}

pub fn prep_jobs(/**/) -> {
    
}

pub fn dequeue(/**/) {
    
}

pub fn enqueue(/**/) {
    
}

pub fn find_queue(/**/) {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_job_test() {
        
    }

    #[test]
    fn calc_fin_test() {
        
    }

    #[test]
    fn reset_fin_test() {
        
    }

    #[test]
    fn prep_jobs_test() {
        
    }

    #[test]
    fn dequeue_test() {
        
    }

    #[test]
    fn enqueue_test() {
        
    }

    #[test]
    fn find_queue_test() {
        
    }

}
