//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::config::BIG_STRIDE;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;

#[allow(unused)]
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

pub struct StrideTaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple Stride scheduler
impl StrideTaskManager {
    /// Create an empty StrideTaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add task to ready queue
    pub fn push(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a task out of the ready queue
    pub fn get(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut index = 0;
        let mut stride = usize::MAX;
        for (idx, task) in self.ready_queue.iter().enumerate() {
            if (task.get_stride() - stride) as isize >= 0 {
                continue;
            } else {
                stride = task.get_stride();
                index = idx;
            }
        }
        self.ready_queue.remove(index)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<StrideTaskManager> =
        unsafe { UPSafeCell::new(StrideTaskManager::new()) };
}

/// Add task to ready queue
pub fn push_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().push(task);
}

/// Take a task out of the ready queue and plus the pass to stride
pub fn get_task() -> Option<Arc<TaskControlBlock>> {
    if let Some(task) = TASK_MANAGER.exclusive_access().get() {
        let pass = BIG_STRIDE / task.get_priority();
        task.add_stride(pass);
        return Some(task);
    }
    None
}
