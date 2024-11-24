use std::fs;
use csv::ReaderBuilder;
use iced::{Alignment, Element, Length};
use iced::widget::{Column, Container, Text, Row, Checkbox, Scrollable, Button};
use rfd::{FileDialog, MessageDialog};
use serde::Deserialize;

fn main() -> iced::Result {
    iced::application("Listmatica", Listmatica::update, Listmatica::view).run()
}

#[derive(Debug, Deserialize)]
struct LitematicaEntry {
    Item: String,
    Total: u32,
    Missing: u32,
    Available: u32
}

#[derive(Debug)]
struct Material {
    checked: bool,
    name: String,
    count: u32
}

#[derive(Default)]
struct Listmatica {
    materials: Vec<Material>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleCheck(usize),
    LoadLitematic
}

fn format_item_count(item_count: u32) -> String {
    let boxes;
    let stacks;
    let items;

    if item_count >= 1728 {
        boxes = item_count / 1728;
        stacks = (item_count - (boxes * 1728)) / 64;
        items = item_count - (boxes * 1728) - (stacks * 64);
        format!("{boxes} boxes, {stacks} stacks, {items} items")
    } else if item_count >= 64 {
        stacks = item_count / 64;
        items = item_count % 64;
        format!("{stacks} stacks, {items} items")
    } else {
        format!("{item_count} items")
    }
}

impl Listmatica {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleCheck(index) => {
                if let Some(material) = self.materials.get_mut(index) {
                    material.checked = !material.checked;
                }
            }
            Message::LoadLitematic => {
                if let Some(path) = FileDialog::new().add_filter("CSV Material List", &["csv"]).pick_file() {
                    if let Ok(contents) =  fs::read_to_string(path) {
                        let mut reader = ReaderBuilder::new()
                            .has_headers(true)
                            .from_reader(contents.as_bytes());

                        let mut materials = Vec::new();

                        for result in reader.deserialize() {
                            match result {
                                Ok(item) => {
                                    let item: LitematicaEntry = item;

                                    materials.push(Material {
                                        checked: false,
                                        name: item.Item,
                                        count: item.Total
                                    });
                                }
                                Err(_err) => {
                                    MessageDialog::new()
                                        .set_title("Error")
                                        .set_description("Failed parse an entry")
                                        .set_buttons(rfd::MessageButtons::Ok)
                                        .show();
                                }
                            }
                        }

                        self.materials = materials;
                    } else {
                        MessageDialog::new()
                            .set_title("Error")
                            .set_description("Failed to read file")
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                    }
                } else {
                    MessageDialog::new()
                        .set_title("Error")
                        .set_description("Please select a file")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let scrollable_content = self.materials.iter().enumerate().fold(
            Column::new().spacing(10),
            |column, (index, material)| {
                column.push(
                    Row::new()
                        .spacing(20)
                        .align_y(Alignment::Center)
                        .push(
                            Checkbox::new("", material.checked).on_toggle(move |_| Message::ToggleCheck(index)).width(40)
                        )
                        .push(Text::new(material.name.clone()).width(Length::Fill))
                        .push(Text::new(format_item_count(material.count)).width(Length::Fill))
                )
            },
        );

        let scrollable = Scrollable::new(scrollable_content);

        let load_button = Button::new("Load Material CSV").on_press(Message::LoadLitematic);

        let layout = Column::new()
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Row::new()
                    .spacing(10)
                    .align_y(Alignment::End)
                    .push(load_button)
            )
            .push(scrollable);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
