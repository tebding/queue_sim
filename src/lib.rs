//lib.rs

//note: regular Vec<> would work just as well, but Vec is a bit neater
use std::cell::Cell;

pub struct Job {
    pub arrival: u32,
    pub duration: u32,
    pub finish: Cell<u32>,
    pub wait: Cell<u32>,
}

impl Job {
    pub fn new(arrival: u32, duration: u32) -> Job {
        let finish = Cell::new(0);
        let wait = Cell::new(0);
        Job {
            arrival,
            duration,
            finish,
            wait,
       }
    }
    
    pub fn calc_fin(&self, time: u32) {
        self.finish.set(self.duration + time);
    }
    
    pub fn reset_fin(&self) {
        self.finish.set(0);
    }
    
}
//handles reading and parsing of input file, returning a queue of jobs
pub fn prep_jobs(joblist: String) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    for line in joblist.lines() {
        let mut nums = line.split_whitespace();
        let a = nums.next().expect("invalid input")
            .parse::<u32>().expect("invalid input");
        let d = nums.next().expect("invalid input")
            .parse::<u32>().expect("invalid input");
        let new_job = Job::new(a, d);
        jobs.push(new_job);
    }
    return jobs;
}
/*
pub fn dequeue(/**/) {
    
}

pub fn enqueue(/**/) {
    
}

pub fn find_queue(/**/) {
    
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_job_test() {
        let test_job = Job::new(4, 1);
        assert_eq!(test_job.arrival, 4);
        assert_eq!(test_job.duration, 1);
        assert_eq!(test_job.finish.get(), 0);
        assert_eq!(test_job.wait.get(), 0);
    }

    #[test]
    fn calc_fin_test() {
        let test_job = Job::new(4, 1);
        test_job.calc_fin(5);
        assert_eq!(test_job.finish.get(), 6);
    }

    #[test]
    fn reset_fin_test() {
        let test_job = Job::new(4, 1);
        test_job.calc_fin(5);
        assert_eq!(test_job.finish.get(), 6);
        test_job.reset_fin();
        assert_eq!(test_job.finish.get(), 0);
    }

    #[test]
    fn prep_jobs_test() {
        let test_input = String::from("1 2\n3 1");
        let jobs = prep_jobs(test_input);
        assert_eq!(jobs[0].arrival, 1);
        assert_eq!(jobs[0].duration, 2);
        assert_eq!(jobs[0].finish.get(), 0);
        assert_eq!(jobs[0].wait.get(), 0);
        assert_eq!(jobs[1].arrival, 3);
        assert_eq!(jobs[1].duration, 1);
        assert_eq!(jobs[1].finish.get(), 0);
        assert_eq!(jobs[1].wait.get(), 0);
    }
/*
    #[test]
    fn dequeue_test() {
        
    }

    #[test]
    fn enqueue_test() {
        
    }

    #[test]
    fn find_queue_test() {
        
    }
*/
}
