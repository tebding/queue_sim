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
        let mut q: Vec<Vec<Job>> = Vec::with_capacity(num);
        while q.len() < q.capacity() {
            let v: Vec<Job> = Vec::new();
            q.push(v);
        }
        Processor {
            queues: q
        }
    }
    
    //takes the input job and inserts it into the shortest queue
    pub fn enqueue(&mut self, job: Job) {
        let target = self.find_shortest_q();
        if self.queues[target].len() == 0 {
            job.wait.set(0);
            job.finish.set(job.arrival + job.duration);
        }
        self.queues[target].push(job);
    }
    
    //finds the processor queue with the fewest items in it
    fn find_shortest_q(&self) -> usize {
        let mut shortest = self.queues[0].len();
        let mut index: usize = 0;
        for i in 1..self.queues.len() {
            if self.queues[i].len() < shortest {
                shortest = self.queues[i].len();
                index = i;
            }
        }
        return index;
    }
    
    //removes+returns the front item from the selected queue and sets
      //finish time for removed item and wait time for next item, if any
    pub fn dequeue(&mut self, index: usize, time: u32) -> Job {
        self.queues[index][0].finish.set(time);
        if self.queues[index].len() > 1 {
            self.queues[index][1].wait
                .set(time - self.queues[index][1].arrival);
        }
        return self.queues[index].remove(0);
    } 
    
    //searches the queues and returns the indices of all jobs that are finished
    pub fn find_finished(&self, time: &u32) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        println!("time={}", time);
        for i in 0..self.queues.len() {
            print!("at i={}  ", i);
            println!("queues[i][0].finish.get() = {}", self.queues[i][0].finish.get());
            if self.queues[i][0].finish.get() == *time {
                println!("finished found");
                res.push(i as u32); //adds index to output Vec
            }
        }
        return res;
    }

}



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

    #[test]
    fn enqueue_test() {
        let mut proc = Processor::new(3);
        let j1 = Job::new(1, 1);
        let j2 = Job::new(2, 2);
        let j3 = Job::new(3, 3);
        let j4 = Job::new(4, 4);

        proc.enqueue(j1);
        assert_eq!(proc.queues[0][0].arrival, 1);
        assert_eq!(proc.queues[0][0].duration, 1);
        assert_eq!(proc.queues[0][0].wait.get(), 0);
        assert_eq!(proc.queues[0][0].finish.get(), 2);
        proc.enqueue(j2);
        assert_eq!(proc.queues[1][0].arrival, 2);
        assert_eq!(proc.queues[1][0].duration, 2);
        assert_eq!(proc.queues[1][0].wait.get(), 0);
        assert_eq!(proc.queues[1][0].finish.get(), 4);
        proc.enqueue(j3);
        assert_eq!(proc.queues[2][0].arrival, 3);
        assert_eq!(proc.queues[2][0].duration, 3);
        assert_eq!(proc.queues[2][0].wait.get(), 0);
        assert_eq!(proc.queues[2][0].finish.get(), 6);
        proc.enqueue(j4);
        assert_eq!(proc.queues[0][1].arrival, 4);
        assert_eq!(proc.queues[0][1].duration, 4);
        assert_eq!(proc.queues[0][1].wait.get(), 0);
        assert_eq!(proc.queues[0][1].finish.get(), 0);
    }
    
    #[test]
    fn find_shortest_q_test() {
        let mut proc = Processor::new(3);
        let j1 = Job::new(1, 1);
        let j2 = Job::new(2, 2);
        let j3 = Job::new(3, 3);
        let j4 = Job::new(4, 4);

        assert_eq!(0, proc.find_shortest_q());
        proc.queues[0].push(j1);
        assert_eq!(1, proc.find_shortest_q());
        proc.queues[1].push(j2);
        assert_eq!(2, proc.find_shortest_q());
        proc.queues[2].push(j3);
        assert_eq!(0, proc.find_shortest_q());
        proc.queues[1].push(j4);
        assert_eq!(0, proc.find_shortest_q());
    }
    
    #[test]
    fn dequeue_test() {
        let mut proc = Processor::new(2);
        let j1 = Job::new(1, 3);
        let j2 = Job::new(2, 3);
        let j3 = Job::new(3, 3);
        proc.queues[0].push(j1);
        proc.queues[1].push(j2);
        proc.queues[0].push(j3);
        
        let r1 = proc.dequeue(0, 4);
        assert_eq!(r1.arrival, 1);
        assert_eq!(r1.duration, 3);
        assert_eq!(r1.finish.get(), 4);
        assert_eq!(r1.wait.get(), 0);
        
        let r2 = proc.dequeue(1, 5);
        assert_eq!(r2.arrival, 2);
        assert_eq!(r2.duration, 3);
        assert_eq!(r2.finish.get(), 5);
        assert_eq!(r2.wait.get(), 0);
        
        let r3 = proc.dequeue(0, 6);
        assert_eq!(r3.arrival, 3);
        assert_eq!(r3.duration, 3);
        assert_eq!(r3.finish.get(), 6);
        assert_eq!(r3.wait.get(), 1);
    }
    
    #[test]
    fn find_finished_test() {
        let mut proc = Processor::new(3);
        let j1 = Job::new(1, 3);
        j1.wait.set(0);
        j1.finish.set(4);
        let j2 = Job::new(1, 3);
        j2.wait.set(0);
        j2.finish.set(4);
        let j3 = Job::new(2, 3);
        j3.wait.set(0);
        j3.finish.set(5);
        let j4 = Job::new(3, 3);
        proc.queues[0].push(j1);
        proc.queues[1].push(j2);
        proc.queues[2].push(j3);
        proc.queues[0].push(j4);
        
        let r1 = proc.find_finished(&3);
        assert_eq!(r1.len(), 0);
        let r2 = proc.find_finished(&4);
        assert_eq!(r2.len(), 2);
        assert_eq!(r2[0], 0);
        assert_eq!(r2[1], 1);
        //confirm 2nd element's data hasn't been set yet
        assert_eq!(proc.queues[0][1].wait.get(), 0);
        assert_eq!(proc.queues[0][1].finish.get(), 0);
        proc.queues[0].remove(0);
        proc.queues[0][0].wait.set(1);
        proc.queues[0][0].finish.set(6);
        let r3 = proc.find_finished(&5);
        assert_eq!(r3.len(), 1);
        assert_eq!(r3[0], 2);
        let r4 = proc.find_finished(&6);
        assert_eq!(r4.len(), 1);
        assert_eq!(r4[0], 0);
        let r5 = proc.find_finished(&7);
        assert_eq!(r5.len(), 0);
    }
    
}
