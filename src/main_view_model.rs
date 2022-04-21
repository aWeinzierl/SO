use async_broadcast::InactiveReceiver;
use std::sync::Arc;
use futures::StreamExt;
use tokio::sync::watch::{channel, Sender};
use windows::{
    core::{Result, AgileReference},
    Storage::{Pickers::{PickerLocationId, FileOpenPicker}, StorageFile},
    System::VirtualKey
};

pub type BehaviorSubject<T> = Arc<Sender<T>>;
pub type Subject<T> = InactiveReceiver<T>;

#[must_use]
pub struct MainViewModel {
    pub selected_executable: BehaviorSubject<Option<AgileReference<StorageFile>>>,
    pub selected_key: BehaviorSubject<Option<VirtualKey>>,
    pub select_executable_clicked: Subject<()>,
    pub select_key_clicked: Subject<()>,
    pub send_key_clicked: Subject<()>,
}

impl MainViewModel {
    pub fn new(
        select_executable_clicked: InactiveReceiver<()>,
        select_key_clicked: InactiveReceiver<()>,
        send_key_clicked: InactiveReceiver<()>,
    ) -> MainViewModel {
        MainViewModel {
            selected_executable: Arc::new(channel(None).0),
            selected_key: Arc::new(channel(None).0),
            select_executable_clicked,
            select_key_clicked,
            send_key_clicked,
        }
    }
}
