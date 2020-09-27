# queue_sim

This program takes a list of jobs from a designated input file (each job is presented on its own line in the format of "[arrival] [duration]"), and simulates the processing of the jobs to calculate wait time data (longest wait time, sum of all wait times, and average wait time).
The simulation is run on 1 to n-1 processors (where n is the total number of jobs) to help figure out how many queues are needed to keep the wait times at an acceptable level.
