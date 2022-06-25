use crate::gui::messages::Files;
use crate::gui::widgets::tab_bar::Tab;
use crate::ipfs_client::api::files::ListDirsRequest;
use crate::ipfs_client::models::{FileEntry, FilesList};
use crate::{gui::messages::Message, gui::IpfsRef};

use iced::pure::widget::{Button, Column, Container, Text};
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

fn request_files_list(client: IpfsRef) -> Command<Message> {
    let route = ListDirsRequest::new();
    let request = client.make_request(route);
    Command::perform(request, |result| match result {
        Ok(data) => Message::Files(Files::ListReceived(data)),
        Err(_) => Message::Files(Files::FailedToFetch),
    })
}

impl<'a> Tab<'a, Message> for HomeTab {
    fn title(&self) -> String {
        "Home".to_string()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        println!("Received message");
        match event {
            Message::Files(Files::ListReceived(files)) => self.files = Some(files),
            Message::Files(Files::FileClicked(file)) => self.selected_file = Some(file),
            Message::Files(Files::FailedToFetch) => {
                println!("Failed to fetch files");
                return request_files_list(self.ipfs_client.clone());
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> iced::pure::Element<'a, Message> {
        let files: Container<Message> = match &self.files {
            Some(files) => {
                let mut col: Column<Message> = Column::new();
                let files_iter = files.entries.iter();
                for entry in files_iter {
                    col = col
                        .push(Button::new(Text::new(&entry.name)).on_press(Message::Files(
                            Files::FileClicked(entry.to_owned().clone()),
                        )));
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
