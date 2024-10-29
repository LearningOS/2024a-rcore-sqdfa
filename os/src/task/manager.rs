//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
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

    /// Take a process out of the ready queue using stride 
    pub fn stride(&mut self) -> Option<Arc<TaskControlBlock>> {
        if self.ready_queue.is_empty() {
            return None;
        }

        // 查找 pass 最小的任务
        let mut min_index: Option<usize> = None;
        let mut min_pass = usize::MAX;

        for (i, tcb) in self.ready_queue.iter().enumerate() {
            let  inner = tcb.inner_exclusive_access();
            let current_pass = inner.stride;
            if current_pass < min_pass {
                min_pass = current_pass;
                min_index = Some(i);
            }
        }

        if let Some(index) = min_index {
            let tcb = &self.ready_queue[index];
            let mut inner = tcb.inner_exclusive_access();
            // 更新 pass 值
            let new_pass = inner.stride+(inner.priority as usize);
            inner.stride=new_pass;
           drop(inner);
            // 将任务移出队列
            self.ready_queue.remove(index)
        } else {
            None
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

/// stride
pub fn stride_task()-> Option<Arc<TaskControlBlock>>{
    TASK_MANAGER.exclusive_access().stride()
}