use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::future::Future;
use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::task::ArcWake;
use instant::Duration;
use uuid::Uuid;

use crate::join_handle::JoinHandle;
use crate::task::{joinable, Task};
use crate::timer_future::{Timeout, TimerFuture};

pub(crate) struct RuntimeState {
    running_tasks: RefCell<VecDeque<Task>>,
    wait_tasks: RefCell<HashMap<Uuid, Task>>,
    waker_sender: Sender<Uuid>,
    waker_receiver: Receiver<Uuid>,

    pub(crate) current_time: RefCell<instant::Instant>,
    pub(crate) timer_waker_heap: RefCell<BinaryHeap<Timeout>>,
}
impl RuntimeState {
    fn new() -> Self {
        let (waker_sender, waker_receiver) = channel();
        Self {
            running_tasks: Default::default(),
            wait_tasks: Default::default(),
            waker_sender,
            waker_receiver,
            current_time: RefCell::new(instant::Instant::now()),
            timer_waker_heap: RefCell::new(BinaryHeap::new()),
        }
    }
}

#[derive(Clone)]
pub struct Runtime {
    state: Rc<RuntimeState>,
}
impl Runtime {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RuntimeState::new()),
        }
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let (task, handle) = joinable(future);
        self.state.running_tasks.borrow_mut().push_back(task);
        handle
    }

    pub fn step(&self, delta_time: Duration) {
        // Update time
        self.state.current_time.borrow_mut().add_assign(delta_time);

        // Check timeout timer future
        {
            let mut heap = self.state.timer_waker_heap.borrow_mut();
            while let Some(timeout) = heap.pop() {
                if timeout.instant >= *self.state.current_time.borrow() {
                    heap.push(timeout);
                    break;
                }
                timeout.waker.wake();
            }
        }

        // poll loop
        'current_frame: loop {
            for id in self.state.waker_receiver.try_iter() {
                if let Some(task) = self.state.wait_tasks.borrow_mut().remove(&id) {
                    self.state.running_tasks.borrow_mut().push_back(task);
                }
            }

            let task = self.state.running_tasks.borrow_mut().pop_front();
            match task {
                None => break 'current_frame,
                Some(mut task) => {
                    let unpark = Box::new({
                        let id = task.id;
                        let sender = Mutex::new(self.state.waker_sender.clone());
                        move || {
                            if let Ok(s) = sender.lock() {
                                s.send(id).unwrap();
                            }
                        }
                    });
                    let waker = TaskWaker::waker(unpark);
                    let mut cx = Context::from_waker(&waker);

                    match task.poll(&mut cx) {
                        Poll::Ready(_) => {
                            // Send a notification to handle
                            if let Some(handle_waker) = task.handle_waker.borrow_mut().take() {
                                handle_waker.wake_by_ref();
                            }
                        }
                        Poll::Pending => {
                            // park the task
                            self.state.wait_tasks.borrow_mut().insert(task.id, task);
                        }
                    }
                }
            }
        }
    }

    pub fn delay(&self, delay: Duration) -> TimerFuture {
        TimerFuture::new(&self.state, delay)
    }
}

struct TaskWaker {
    unpark: Box<dyn Fn() -> () + Send + Sync>,
}
impl TaskWaker {
    fn waker(unpark: Box<dyn Fn() -> () + Send + Sync>) -> Waker {
        futures::task::waker(Arc::new(TaskWaker { unpark }))
    }
}
impl ArcWake for TaskWaker {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        (arc_self.unpark)();
    }
}
