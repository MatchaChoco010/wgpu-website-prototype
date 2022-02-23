use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn use_mounted_once(callback: impl FnOnce() + 'static) {
    use_effect_with_deps(
        move |_| {
            callback();
            || ()
        },
        (),
    );
}

pub enum UseAsyncOnceState<T: 'static> {
    Pending,
    Ready(T),
}
pub struct UseAsyncOnceHandle<T: 'static> {
    inner: UseStateHandle<UseAsyncOnceState<T>>,
}
impl<T: 'static> UseAsyncOnceHandle<T> {
    pub fn state(&self) -> &UseAsyncOnceState<T> {
        &(*self.inner)
    }
}
impl<T: 'static> Clone for UseAsyncOnceHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<T: 'static> PartialEq for UseAsyncOnceHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&*self.inner, &*other.inner) {
            (UseAsyncOnceState::Pending, UseAsyncOnceState::Pending) => true,
            (UseAsyncOnceState::Pending, UseAsyncOnceState::Ready(_)) => false,
            (UseAsyncOnceState::Ready(_), UseAsyncOnceState::Pending) => false,
            (UseAsyncOnceState::Ready(_), UseAsyncOnceState::Ready(_)) => true,
        }
    }
}
pub fn use_async_once<F, T>(future: impl FnOnce() -> F) -> UseAsyncOnceHandle<T>
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    let inner = use_state(|| UseAsyncOnceState::Pending);
    use_mounted_once({
        let inner = inner.clone();
        let future = future();
        move || {
            spawn_local(async move {
                inner.set(UseAsyncOnceState::Ready(future.await));
            })
        }
    });
    UseAsyncOnceHandle { inner }
}
