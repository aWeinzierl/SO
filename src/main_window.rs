use crate::main_view_model::{BehaviorSubject, MainViewModel};
use async_broadcast::Sender;
use futures::{ready, StreamExt};
use futures::future::ready;
use tokio_stream::wrappers::WatchStream;
use windows::{
    core::*,
    Storage::{
        Pickers::{FileOpenPicker, PickerLocationId},
        StorageFile,
    },
    System::VirtualKey,
    UI::{
        Core::{CoreDispatcher, CoreDispatcherPriority, DispatchedHandler},
        Xaml::Controls::Primitives::*,
        Xaml::Controls::*,
        Xaml::*,
    },
};

pub struct MainWindow {
    window: Window,
    model: MainViewModel,
}

impl MainWindow {
    pub fn new() -> Result<MainWindow> {
        let window = Window::Current()?;
        let dispatcher = window.Dispatcher()?;

        let (select_executable_sender, select_executable_receiver) = async_broadcast::broadcast(1);
        let (select_key_sender, select_key_receiver) = async_broadcast::broadcast(1);
        let (send_key_sender, send_key_receiver) = async_broadcast::broadcast(1);
        let model = MainViewModel::new(
            select_executable_receiver.deactivate(),
            select_key_receiver.deactivate(),
            send_key_receiver.deactivate(),
        );

        let selected_executable = model.selected_executable.clone();
        tokio::spawn(model.select_executable_clicked.activate_cloned().for_each(move |v| {
            let selected_executable = selected_executable.clone();
            async move {
                let selected_executable = selected_executable.clone();
                // Option 1: Get file from FileOpenPicker here
                match (*selected_executable).send(None) {
                    Ok(()) => { dbg!("Some"); }
                    Err(e) => { dbg!("SendError"); }
                };
            }
        }));


        let stack_panel = StackPanel::new()?;
        let sp_children = stack_panel.cast::<Panel>()?.Children()?;
        let heading = Self::generate_title()?;
        sp_children.Append(heading)?;

        let executable_label = TextBlock::new()?;
        executable_label
            .SetText("Select file")?;
        sp_children.Append(executable_label)?;
        let select_executable = Self::generate_executable_selection(
            dispatcher.clone(),
            select_executable_sender,
            &model.selected_executable,
        )?;
        sp_children.Append(select_executable)?;

        let select_key_label = TextBlock::new()?;
        select_key_label.SetText("Select key")?;
        sp_children.Append(select_key_label)?;
        let select_key =
            Self::generate_key_selection(dispatcher, select_key_sender, &model.selected_key)?;
        sp_children.Append(select_key)?;

        // listbox

        let send_key = Button::new()?;
        send_key
            .cast::<ContentControl>()?
            .SetContent(IInspectable::try_from("Act")?)?;
        send_key
            .cast::<FrameworkElement>()?
            .SetHorizontalAlignment(HorizontalAlignment::Center)?;
        send_key
            .cast::<ButtonBase>()?
            .Click(RoutedEventHandler::new(move |_, _| -> Result<()> {
                send_key_sender.try_broadcast(());
                Ok(())
            }))?;
        sp_children.Append(send_key)?;

        let border = Border::new()?;
        border.SetPadding(ThicknessHelper::FromUniformLength(15.0)?)?;
        border.SetChild(stack_panel)?;

        window.SetContent(border)?;

        Ok(MainWindow { model, window })
    }

    fn generate_key_selection(
        dispatcher: CoreDispatcher,
        select_key_click: Sender<()>,
        selected_key: &BehaviorSubject<Option<VirtualKey>>,
    ) -> Result<StackPanel> {
        let selected_key_box = TextBox::new()?;
        let select_key_button = Button::new()?;
        let select_key = StackPanel::new()?;
        selected_key_box.SetIsReadOnly(true)?;
        selected_key_box.SetText("1")?;
        let selected_key_box_r = AgileReference::new(&selected_key_box)?;
        tokio::spawn(
            WatchStream::new(selected_key.subscribe()).for_each(move |key| {
                let selected_key_box_r = selected_key_box_r.clone();
                dispatcher
                    .RunAsync(
                        CoreDispatcherPriority::default(),
                        DispatchedHandler::new(move || {
                            match &key {
                                None => selected_key_box_r.resolve()?.SetText(None)?,
                                Some(key) => selected_key_box_r
                                    .resolve()?
                                    .SetText(format!("{:?}", key))?,
                            };
                            Ok(())
                        }),
                    )
                    .unwrap();
                ready(())
            }),
        );
        select_key_button
            .cast::<ContentControl>()?
            .SetContent(IInspectable::try_from("Select key")?)?;
        select_key_button
            .cast::<ButtonBase>()?
            .Click(RoutedEventHandler::new(move |_, _| -> Result<()> {
                select_key_click.try_broadcast(());
                Ok(())
            }))?;
        select_key.SetPadding(Thickness {
            Left: 0.0,
            Top: 0.0,
            Right: 0.0,
            Bottom: 7.0,
        })?;
        select_key.SetOrientation(Orientation::Horizontal)?;
        let select_key_children = select_key.cast::<Panel>()?.Children()?;
        select_key_children.Append(selected_key_box)?;
        select_key_children.Append(select_key_button)?;
        Ok(select_key)
    }

    fn generate_executable_selection(
        dispatcher: CoreDispatcher,
        select_executable_click: Sender<()>,
        selected_executable: &BehaviorSubject<Option<AgileReference<StorageFile>>>,
    ) -> Result<StackPanel> {
        let selected_executable_box = TextBox::new()?;
        let select_executable_button = Button::new()?;
        let select_executable = StackPanel::new()?;
        let select_executable_children = select_executable.cast::<Panel>()?.Children()?;
        selected_executable_box.SetIsReadOnly(true)?;
        let selected_executable_box_r = AgileReference::new(&selected_executable_box)?;
        tokio::spawn(
            WatchStream::new(selected_executable.subscribe()).for_each(move |file| {
                let selected_executable_box_r = selected_executable_box_r.clone();
                dispatcher
                    .RunAsync(
                        CoreDispatcherPriority::default(),
                        DispatchedHandler::new(move || {
                            match &file {
                                None => selected_executable_box_r.resolve()?.SetText(None)?,
                                Some(file) => selected_executable_box_r
                                    .resolve()?
                                    .SetText(file.resolve()?.Path()?)?,
                            };
                            Ok(())
                        }),
                    )
                    .unwrap();
                ready(())
            }),
        );
        selected_executable_box
            .cast::<FrameworkElement>()?
            .SetVerticalAlignment(VerticalAlignment::Center)?;
        select_executable_button
            .cast::<ContentControl>()?
            .SetContent(IInspectable::try_from("Select file")?)?;
        select_executable_button
            .cast::<ButtonBase>()?
            .Click(RoutedEventHandler::new(move |_, _| -> Result<()> {
                // Option 2: Get file from FileOpenPicker here
                select_executable_click.try_broadcast(());
                Ok(())
            }))?;
        select_executable.SetOrientation(Orientation::Horizontal)?;
        select_executable.SetPadding(Thickness {
            Left: 0.0,
            Top: 0.0,
            Right: 0.0,
            Bottom: 7.0,
        })?;
        select_executable_children.Append(selected_executable_box)?;
        select_executable_children.Append(select_executable_button)?;
        Ok(select_executable)
    }

    fn generate_title() -> Result<TextBlock> {
        let heading = TextBlock::new()?;
        heading.SetText("ABC")?;
        heading.SetFontSize(24.0)?;
        heading.SetPadding(Thickness {
            Left: 0.0,
            Top: 0.0,
            Right: 0.0,
            Bottom: 11.0,
        })?;
        Ok(heading)
    }

    pub fn Activate(&self) -> Result<()> {
        self.window.Activate()
    }
}

