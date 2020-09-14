//lib.rs

use std::cell::Cell;

#[derive(Clone)]
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
    
    //sets the 'finish' field to the simulated completion time
    pub fn calc_fin(&self, time: u32) {
        self.finish.set(self.duration + time);
    }
    
    //sets the 'finish' field back to 0
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

pub struct Processor {
    pub queues: Vec<Vec<Job>>, 
}

impl Processor {
    pub fn new(num: usize) -> Processor {
        let q: Vec<Vec<Job>> = Vec::with_capacity(num);
        Processor {
            queues: q
        }
    }
    /*
    pub fn enqueue(job: Job) {
        
    }
    */
}


//finds the processor queue with the fewest items in it
/*fn find_shortest_queue(/**/) {
    
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
    #[test]
    #[should_panic]
    fn prep_jobs_fail_test() {
        let test_fail_input = String::from("1 a");
        let jobs = prep_jobs(test_fail_input);
    }
    
    #[test]
    fn new_proc_test() {
        let test_p = Processor::new(3);
        assert_eq!(test_p.queues.capacity(), 3);
    }
    
/*
    #[test]
    fn find_shortest_queue_test() {
        
    }
*/
}
