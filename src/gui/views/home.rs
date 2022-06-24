use crate::ipfs_client::api::files::ListDirsRequest;
use crate::ipfs_client::models::{FilesList, FileEntry};
use crate::{gui::messages::Message, gui::IpfsRef};

use iced::pure::widget::{Button, Column, Container, Text};
use iced::{Command, Length};

use super::tab::TabMessage;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    FilesReceived(FilesList),
    FileClicked(FileEntry)
}

pub struct HomeTab {
    ipfs_client: IpfsRef,
    files: Option<FilesList>,
    selected_file: Option<FileEntry>
}

impl HomeTab {
    pub fn new(ipfs_client: IpfsRef) -> (Self, Command<Message>) {
        let page = HomeTab {
            ipfs_client,
            files: None,
            selected_file: None
        };
        let cmd = request_files_list(page.ipfs_client.clone());
        (page, cmd)
    }

    pub fn update(&mut self, event: HomeMessage) -> Command<Message> {
        match event {
            HomeMessage::FilesReceived(files) => self.files = Some(files),
            HomeMessage::FileClicked(file) => self.selected_file = Some(file)
        }
        Command::none()
    }

    pub fn view(&self) -> iced::pure::Element<HomeMessage> {
        let files: Container<HomeMessage> = match &self.files {
            Some(files) => {
                let mut col: Column<HomeMessage> = Column::new();
                let files_iter = files.entries.iter();
                for entry in files_iter {
                    col = col.push(Button::new(Text::new(&entry.name)).on_press(HomeMessage::FileClicked(entry.to_owned().clone())));
                }
                Container::new(col.align_items(iced::Alignment::Center).spacing(5))
            }
            None => Container::new(Text::new("No files have been found!")),
        };
        Container::new(files)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn request_files_list(client: IpfsRef) -> Command<Message> {
    let route = ListDirsRequest::new();
    let request = client.make_request(route);
    Command::perform(request, |result| match result {
        Ok(data) => Message::Tabs(TabMessage::Home(HomeMessage::FilesReceived(data))),
        Err(_) => Message::Disconnected,
    })
}
