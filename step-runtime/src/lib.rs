mod join_handle;
mod runtime;
mod task;
mod timer_future;

pub use join_handle::JoinHandle;
pub use runtime::Runtime;

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::FutureExt;
    use futures::select;
    use instant::Duration;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_should_run_async_block_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let runtime = Runtime::new();
        runtime.spawn({
            let flag = Rc::clone(&flag);
            async move {
                *flag.borrow_mut() = true;
            }
        });
        runtime.step(Duration::from_secs_f32(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
    }

    #[test]
    fn it_should_run_nested_async_block_and_await_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let runtime = Runtime::new();
        runtime.spawn({
            let flag = Rc::clone(&flag);
            async move {
                let result = async { 1 + async { 2 + 3 }.await }.await;
                assert_eq!(result, 6);
                *flag.borrow_mut() = true;
            }
        });
        runtime.step(Duration::from_secs_f32(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
    }

    #[test]
    fn it_should_run_multiple_await_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let runtime = Runtime::new();
        runtime.spawn({
            let flag = Rc::clone(&flag);
            async move {
                let result = async { 1 + 2 }.await;
                assert_eq!(result, 3);
                let result = async { 3 + 4 }.await;
                assert_eq!(result, 7);
                *flag.borrow_mut() = true;
            }
        });
        runtime.step(Duration::from_secs_f32(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
    }

    #[test]
    fn it_should_run_nested_spawn_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let runtime = Runtime::new();
        runtime.spawn({
            let flag = Rc::clone(&flag);
            let runtime = runtime.clone();
            async move {
                let handle = runtime.spawn(async { 1 + 2 });
                let result = handle.await;
                assert_eq!(result, 3);
                *flag.borrow_mut() = true;
            }
        });
        runtime.step(Duration::from_secs_f32(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
    }

    #[test]
    fn delay_future_should_wake_after_delay_time() {
        let flag = Rc::new(RefCell::new(false));
        let runtime = Runtime::new();
        runtime.spawn({
            let flag = Rc::clone(&flag);
            let runtime = runtime.clone();
            async move {
                runtime.delay(Duration::from_millis(32)).await;
                *flag.borrow_mut() = true;
            }
        });

        runtime.step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        runtime.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        runtime.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
    }

    #[test]
    fn multi_delay_future_should_wake_after_delay_time() {
        let flag = Rc::new(RefCell::new(false));
        let runtime = Runtime::new();
        runtime.spawn({
            let flag = Rc::clone(&flag);
            let runtime = runtime.clone();
            async move {
                runtime.delay(Duration::from_millis(16)).await;
                runtime.delay(Duration::from_millis(16)).await;
                select! {
                    () = runtime.delay(Duration::from_millis(16)).fuse() => (),
                    () = runtime.delay(Duration::from_millis(32)).fuse() => (),
                }
                *flag.borrow_mut() = true;
            }
        });

        runtime.step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        runtime.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        runtime.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        runtime.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
    }
}
