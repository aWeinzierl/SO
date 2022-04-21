#![feature(async_closure)]
//#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

mod main_view_model;
mod main_window;

use main_window::MainWindow;

use windows::{
    core::{implement, Result},
    ApplicationModel::{Activation::LaunchActivatedEventArgs, Package},
    Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
    UI::Xaml::{
        Application, ApplicationInitializationCallback, IApplicationOverrides,
        IApplicationOverrides_Impl,
    },
};

use tokio::runtime::Handle;

#[implement(IApplicationOverrides)]
struct App {
    handle: Handle,
}

impl App {
    pub fn new(handle: Handle) -> App {
        App { handle }
    }
}

impl IApplicationOverrides_Impl for App {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        self.handle.block_on(async {
            let mainWindow = MainWindow::new()?;
            mainWindow.Activate()
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?;
        if let Err(result) = Package::Current() {
            MessageBoxW(
                HWND::default(),
                "This sample must be registered (via register.ps1) and launched from Start.",
                "Error",
                MB_ICONSTOP | MB_OK,
            );
            return Err(result);
        }
    }
    let handle = Handle::current();
    Application::Start(ApplicationInitializationCallback::new(move |_| {
        Application::compose(App::new(handle.clone()))?;
        Ok(())
    }))
}
