use std::collections::VecDeque;

fn print_separator() {
    println!("---------------------------------------");
}

#[derive(Debug, PartialEq)]
enum State {
    Execution,
    Watiting,
    Executable
}

impl Default for State {
    fn default() -> State {
        State::Watiting
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Task {
    name: &'static str,
    arrival_time: u32,
    processing_time: u32,
    finish_time: u32,
    priority: u32,
    state: State,
}

#[derive(Debug, PartialEq)]
pub struct TaskResult {
    name: &'static str,
    finish_time: u32,
    turnaround_time: u32,
}

impl TaskResult {
    pub fn new(name: &'static str, finish_time: u32, turnaround_time: u32) -> TaskResult {
        TaskResult {
            name,
            finish_time,
            turnaround_time
        }
    }
}

impl Task {
    pub fn new(name: &'static str, arrival_time: u32, processing_time: u32, priority: u32) -> Task {
        Task {
            name,
            arrival_time,
            processing_time,
            finish_time: 0,
            priority,
            state: State::Watiting
        }
    }
}

// Algorithm 
pub enum Algorithm {
    ArrivalOrder,
    ProcessingTimeOrder,
    RoundRobin(u32, bool)
}

type QueueList = Vec<(VecDeque<Task>, Algorithm)>;
//type QueueID = usize;

fn print_queue_list(query_list: &QueueList) {
    use std::io::{stdout, Write};
    for (i, queue) in query_list.iter().enumerate() {
        print!("Task Queue {}: ", i);
        for task in queue.0.iter() {
            print!("< {}", task.name);
        }
        print!("\n");
    }
    stdout().flush().unwrap();
}

fn fetch_new_tasks(task_list: &mut Vec<Task>, time: u32) -> Option<Vec<Task>> {
    if task_list.is_empty() {
        return None;
    }
    let mut new_tasks = Vec::new();
    loop {
        if let Some(task) = task_list.last() {
            if task.arrival_time <= time {
                let mut task = task_list.pop().unwrap();
                task.state = State::Executable;
                new_tasks.push(task);
                continue;
            }
        }
        break;
    }
    if new_tasks.len() > 0 { Some(new_tasks) } else { None }
}

// Finished Some(Task, true)
// NotFinished Some(Task, false)
// Queue is empty None
fn dispatch(task_queue: &mut VecDeque<Task>, time: u32) -> Option<(Task, bool)> {
    if let Some(mut task) = task_queue.pop_front() {
        if task.state == State::Execution {
            println!("    Task {} was executed.", task.name);
        } else {
            println!("    Task {} was dispatched and executed.", task.name);
            task.state = State::Execution;
        }
        task.processing_time -= 1;
        if task.processing_time <= 0 {
            println!("    Task {} was finished.", task.name);
            task.finish_time = time + 1;
            return Some((task, true));
        } else {
            return Some((task, false));
        }
    }
    None
}

fn add_task(query_list: &mut QueueList, task: Task) {
    let (queue, algorithm) = &mut query_list[task.priority as usize];
    queue.push_back(task);
    match algorithm {
        Algorithm::ArrivalOrder => (),
        _ => todo!("Not Implement"),
    }
}

fn add_result(result_list: &mut Vec<TaskResult>, time: u32, task: Task) {
    let finish_time = time + 1;
    let turnaround_time = finish_time - task.arrival_time;
    result_list.push(TaskResult::new(task.name, finish_time, turnaround_time));
}

pub fn dispatch_task(query_list: &mut QueueList, time: u32, result_list: &mut Vec<TaskResult>) {
    //for i in 0..query_list.len() {
    //    let (queue, algorithm) = &mut query_list[i];
    //    match algorithm {
    //        Algorithm::ArrivalOrder => {
    //            match dispatch(queue, time) {
    //                Some((task, true)) => result_list.push(task),
    //                Some((task, false)) => queue.push_front(task),
    //                None => (),
    //            }
    //        },
    //        _ => todo!("Not Implement")
    //    }

    //}

    for (queue, algorithm) in query_list.iter_mut() {
        match algorithm {
           Algorithm::ArrivalOrder => {
                match dispatch(queue, time) {
                    Some((task, true)) => add_result(result_list, time, task),
                    Some((task, false)) => queue.push_front(task),
                    None => (),
                }
            },
            _ => todo!("Not Implement")
        }
    }
}

//validation
fn validation(task_list: &Vec<Task>, query_list: &QueueList) {
    let max_priority = query_list.len() as u32;
    for task in task_list.iter() {
        if task.priority >= max_priority {
            panic!("Validation failed")
        }
    }
}

pub fn create_queue_list(algorithm_list: Vec<Algorithm>) -> QueueList {
    let mut queue_list = Vec::with_capacity(algorithm_list.len());
    for algorithm in algorithm_list {
        queue_list.push((VecDeque::new(), algorithm));
    }
    queue_list
}

pub fn run_simulator(mut query_list: QueueList, mut task_list: Vec<Task>) -> Vec<TaskResult> {
    validation(&task_list, &query_list);

    task_list.sort_by(|a, b| b.arrival_time.cmp(&a.arrival_time));

    let mut result_list = Vec::new();
    let mut time = 0;
    let task_list_len = task_list.len();

    while task_list_len > result_list.len() {
        println!("Time {}-{}", time, time + 1);
        if let Some(tasks) = fetch_new_tasks(&mut task_list, time) {
            for task in tasks {
                println!("    Task {} arrived on Queue {}.", task.name, task.priority);
                add_task(&mut query_list, task);
            }
        }

        dispatch_task(&mut query_list, time, &mut result_list);
        print_queue_list(&query_list);
        print_separator();
        time += 1;
    }

    result_list
}
