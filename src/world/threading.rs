use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc, Mutex,
};

type Runnable<T> = Box<dyn Fn() -> T + Send>;

pub struct Task<T: Send> {
    runnable: Runnable<T>,
    sender_res: Arc<Mutex<Sender<T>>>,
}

impl<T: Send> Task<T> {
    pub fn new(runnable: Runnable<T>, sender_res: Arc<Mutex<Sender<T>>>) -> Task<T> {
        Task {
            runnable,
            sender_res,
        }
    }
}

pub struct Worker<T: Send> {
    sender_in: Mutex<Sender<Task<T>>>,
    is_running: Arc<AtomicBool>,
}
impl<T: Send + 'static> Worker<T> {
    fn send(&self, task: Task<T>) {
        if let Err(e) = self.sender_in.lock().unwrap().send(task) {
            println!("{}", e);
        }
    }
}

pub struct ThreadPool<T: Send> {
    workers: Arc<Vec<Worker<T>>>,
    tasks: Arc<Mutex<Vec<Task<T>>>>,
}
impl<T: Send> ThreadPool<T> {
    pub fn find_free_worker(&self) -> Option<&Worker<T>> {
        self.workers.iter().find(|w| {
            w.is_running
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
        })
    }
}
