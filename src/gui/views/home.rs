use crate::gui::messages::Files;
use crate::gui::widgets::tab_bar::Tab;
use crate::gui::Context;
use crate::ipfs_client::api::files::{ListDirsRequest, RemoveFilesRequest};
use crate::ipfs_client::models::{FileEntry, FilesList};
use crate::utils;
use crate::{gui::messages::Message, gui::IpfsRef};

use iced::pure::widget::{Button, Column, Container, Row, Text};
use iced::pure::Element;
use iced::{Command, Length, Subscription};

pub struct HomeTab {
    ipfs_client: IpfsRef,
    files: Option<FilesList>,
    selected_file: Option<FileEntry>,
}

impl HomeTab {
    pub fn new(ipfs_client: IpfsRef) -> (Self, Command<Message>) {
        let page = HomeTab {
            ipfs_client,
            files: None,
            selected_file: None,
        };
        let cmd = request_files_list(page.ipfs_client.clone());
        (page, cmd)
    }
}

impl<'a> Tab<'a, Message> for HomeTab {
    fn title(&self) -> String {
        "Home".to_string()
    }

    fn subscription(&self, _: &Context) -> Subscription<Message> {
        Subscription::none()
    }

    fn update(&mut self, event: Message, ctx: &Context) -> Command<Message> {
        match event {
            Message::Files(Files::ListReceived(files)) => self.files = Some(files),
            Message::Files(Files::FileClicked(file)) => self.selected_file = Some(file),
            Message::Files(Files::FailedToFetch) => {
                if ctx.is_connected {
                    println!("Failed to fetch files");
                }
                return request_files_list(self.ipfs_client.clone());
            }
            Message::Files(Files::CloseFile) => self.selected_file = None,
            Message::Files(Files::FileUploaded) => {
                return request_files_list(self.ipfs_client.clone())
            }
            Message::Files(Files::Remove(path)) => {
                return remove_file(self.ipfs_client.clone(), path)
            }
            Message::Files(Files::RemovedSuccessfully) => {
                self.selected_file = None;
                return request_files_list(self.ipfs_client.clone());
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self, _: &Context) -> Element<Message> {
        if let Some(file) = &self.selected_file {
            return display_file(file);
        }
        let files: Container<Message> = match &self.files {
            Some(files) => Container::new(display_files_grid(files)),
            None => Container::new(Text::new("No files have been found!")),
        };
        Container::new(files)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn display_files_grid(list: &FilesList) -> Column<Message> {
    let mut col = Column::new();
    let entries = &list.entries;
    let mut row: Row<Message> = Row::new();
    for (i, file) in entries.iter().enumerate() {
        row = row
            .push(
                Button::new(
                    Text::new(file.get_short_filename(13))
                        .width(Length::Shrink)
                        .height(Length::Fill)
                        .size(20)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .on_press(Message::Files(Files::FileClicked(file.to_owned().clone())))
                .height(Length::Units(40))
                .width(Length::Units(100)),
            )
            .width(Length::Shrink)
            .height(Length::Shrink)
            .spacing(10)
            .padding(10);
        if i > 0 && (i+1) % 5 == 0 {
            col = col.push(row);
            let r: Element<Message> = Row::new().into();
            r.as_widget().width();
            row = Row::new();
        }
    }
    col = col.push(row);
    col.spacing(5)
        .align_items(iced::Alignment::Start)
        .width(Length::Fill)
        .height(Length::Shrink)
}

fn display_file(file: &FileEntry) -> Element<Message> {
    let path = format!("/{}", &file.name);
    let size = utils::shorten_file_size(file.size);
    let col = Column::new()
        .push(Text::new(format!("name: {}", file.name)))
        .push(Text::new(format!("size: {:.1}{}", size.0, size.1)))
        .push(Text::new(format!("hash: {}", file.hash)))
        .push(Button::new(Text::new("Remove file")).on_press(Message::Files(Files::Remove(path))))
        .push(Button::new(Text::new("Close file")).on_press(Message::Files(Files::CloseFile)))
        .align_items(iced::Alignment::Center)
        .spacing(10);
    Container::new(col)
        .center_x()
        .center_y()
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn request_files_list(client: IpfsRef) -> Command<Message> {
    let route = ListDirsRequest::new().long_listed();
    let request = client.make_request(route, None);
    Command::perform(request, |result| match result {
        Ok(data) => Message::Files(Files::ListReceived(data)),
        Err(_) => Message::Files(Files::FailedToFetch),
    })
}

fn remove_file(client: IpfsRef, path: String) -> Command<Message> {
    let route = RemoveFilesRequest::new(path);
    let request = client.make_request(route, None);
    Command::perform(request, |result| match result {
        Ok(_) => Message::Files(Files::RemovedSuccessfully),
        Err(_) => Message::Files(Files::FailedRemovingFile),
    })
}
