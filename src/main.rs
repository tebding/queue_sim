//main.rs

use std::{env, fs};
use queue_sim::*;

fn main() {
    
    //collect and check valid num of CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Error: invalid number of arguments");
    }
    
    //check input file can be opened
    let infile = fs::read_to_string(&args[1])
        .expect("unable to open specified file");
    
    //create list of jobs. errors handled in prep_jobs
    //former maintains order between iterations; latter is used during them
    let job_list = prep_jobs(infile);
    let mut jobs_list: Vec<Job>;
    
    //initialize data computation variables
    let mut wait_times: Vec<u32> = Vec::with_capacity(job_list.len());
    let mut avg_wait: f32;
    let mut max_wait: u32;
    let mut sum_wait: u32;
    
    let mut time: u32;
    
    for num_procs in 1..job_list.len() {
        let mut processor = Processor::new(num_procs as usize);
        
        //reset data from previous iteration
        time = 0;
        wait_times.clear();
        avg_wait = 0.0;
        max_wait = 0;
        sum_wait = 0;
        jobs_list = job_list.clone();
        //runs the simulation for a given number of queues
        while wait_times.len() < wait_times.capacity() {
            //first check for finished jobs to dequeue
            let finished = processor.find_finished(&time);
            if finished.len() > 0 {
                for i in 0..finished.len() {
                    //add wait times to list
                    wait_times.push(processor.queues[finished[i] as usize][0].wait.get());
                    //remove the now-finished job
                    processor.dequeue(finished[i] as usize, &time);
                }
            }
            
            //then check for items ready to enqueue
            while jobs_list.is_empty() == false && jobs_list[0].arrival == time {
                processor.enqueue(jobs_list.remove(0));
            }
            time += 1;
        }
        
        
        //iterate through wait_times to extract relevant data
        for i in 0..wait_times.len() {
            sum_wait += wait_times[i];
            avg_wait += wait_times[i] as f32;
            if wait_times[i] > max_wait {
                max_wait = wait_times[i];
            }
        }
        avg_wait = avg_wait / wait_times.len() as f32;
        
        //print iteration stats
        if num_procs == 1 { println!("\nfor 1 processor:"); }
        else { println!("\nfor {} processors:", num_procs); }
        println!("sum time waited: {}\nlongest wait time: {}\n\
                 average wait time: {}", sum_wait, max_wait, avg_wait);
        if max_wait == 0 {
            println!("MAX WAIT FOR {} PROCESSORS WAS 0. FURTHER ITERATIONS REDUNDANT", num_procs);
            break;
        }
        
    }
}
