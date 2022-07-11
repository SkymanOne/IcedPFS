use std::path::PathBuf;

use iced::pure::widget::{Button, Column, Container, Text};
use iced::{Command, Length, Subscription};
use rfd::{AsyncFileDialog, FileHandle};

use crate::gui::messages::Files;
use crate::gui::{messages::Message, widgets::tab_bar::Tab};
use crate::gui::{Context, IpfsRef};
use crate::ipfs_client::api::files::WriteRequest;

pub struct UploadTab {
    ipfs_client: IpfsRef,
    path: Option<PathBuf>,
}

impl UploadTab {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        UploadTab {
            ipfs_client,
            path: None,
        }
    }
}

impl<'a> Tab<'a, Message> for UploadTab {
    fn title(&self) -> String {
        "Upload".to_string()
    }

    fn subscription(&self, _: &Context) -> Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message, ctx: &Context) -> Command<Message> {
        if ctx.is_connected() {
            if let Message::Files(file_msg) = event {
                match file_msg {
                    Files::SelectFile => return select_file(),
                    Files::FileSelected(path) => self.path = path,
                    Files::UploadClicked => {
                        if let Some(path) = self.path.clone() {
                            return upload_file(self.ipfs_client.clone(), path);
                        }
                    }
                    Files::FileUploaded => self.path = None,
                    Files::UploadFailed => panic!("Files upload failed"),
                    _ => {}
                }
            }
        }
        Command::none()
    }

    fn view(&self, ctx: &Context) -> iced::pure::Element<Message> {
        let mut col: Column<Message> = Column::new().align_items(iced::Alignment::Center);
        col = col.push(Text::new("Upload file to IPFS: "));
        if ctx.is_connected() {
            if let Some(path) = self.path.clone() {
                col = col.push(Text::new(format!(
                    "file: <{}>",
                    path.as_path().to_str().unwrap()
                )));
                col = col.push(
                    Button::new(Text::new("Upload")).on_press(Message::Files(Files::UploadClicked)),
                );
            } else {
                col = col.push(Text::new("files: <No file selected>"));
                col = col.push(
                    Button::new(Text::new("Select file..."))
                        .on_press(Message::Files(Files::SelectFile)),
                );
            }
        } else {
            col = col.push(Text::new("Your are not connected to the network!"));
        }
        Container::new(col)
            .padding(20)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn upload_file(client: IpfsRef, path: PathBuf) -> Command<Message> {
    let filename = path.file_name().unwrap().to_str().unwrap_or("default");
    let destination = format!("/{}", filename);
    let route = WriteRequest::new(destination).create();
    let request = client.make_request(route, Some(path));
    Command::perform(request, |result| match result {
        Ok(_) => Message::Files(Files::FileUploaded),
        Err(err) => {
            println!("{:?}", err);
            Message::Files(Files::UploadFailed)
        }
    })
}

fn select_file() -> Command<Message> {
    Command::perform(
        AsyncFileDialog::new().pick_file(),
        |result: Option<FileHandle>| {
            if let Some(handle) = result {
                Message::Files(Files::FileSelected(Some(handle.path().to_owned())))
            } else {
                Message::Files(Files::FileSelected(None))
            }
        },
    )
}
