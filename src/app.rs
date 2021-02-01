const RACY_WEB_VERSION: &str = env!("RACY_WEB_VERSION");

use console_log::console_log;
use racy::Options;
use std::borrow::Borrow;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::File;
use yew::services::reader::{FileData, ReaderTask};
use yew::services::ReaderService;
use yew::{html, html::ChangeData, Component, ComponentLink, Html, InputData, ShouldRender};

enum MimeType {
    PNG,
    JPEG,
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            MimeType::PNG => "image/png",
            MimeType::JPEG => "image/jpg",
        };
        write!(f, "{}", s)
    }
}

pub struct Model {
    link: ComponentLink<Self>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    stl: Rc<Option<Vec<u8>>>,
    image_url: String,
    error: String,
    // render options
    options: Options,
}

pub enum Msg {
    FileSelection(Vec<File>),
    FileLoaded(FileData),
    Render,
    UpdateWidth(String),
    UpdateHeight(String),
    UpdateFromX(String),
    UpdateFromY(String),
    UpdateFromZ(String),
    UpdateToX(String),
    UpdateToY(String),
    UpdateToZ(String),
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            reader: ReaderService::new(),
            tasks: vec![],
            stl: Rc::new(None),
            image_url: String::new(),
            error: String::new(),
            options: Options::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateFromX(s) => {
                match s.parse::<f32>() {
                    Ok(from) => {
                        self.error = String::new();
                        self.options.from.x = from;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                true
            }
            Msg::UpdateFromY(s) => {
                match s.parse::<f32>() {
                    Ok(from) => {
                        self.error = String::new();
                        self.options.from.y = from;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                true
            }
            Msg::UpdateFromZ(s) => {
                match s.parse::<f32>() {
                    Ok(from) => {
                        self.error = String::new();
                        self.options.from.z = from;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                true
            }
            Msg::UpdateToX(s) => {
                match s.parse::<f32>() {
                    Ok(from) => {
                        self.error = String::new();
                        self.options.to.x = from;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                true
            }
            Msg::UpdateToY(s) => {
                match s.parse::<f32>() {
                    Ok(from) => {
                        self.error = String::new();
                        self.options.to.y = from;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                true
            }
            Msg::UpdateToZ(s) => {
                match s.parse::<f32>() {
                    Ok(from) => {
                        self.error = String::new();
                        self.options.to.z = from;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                true
            }

            Msg::UpdateWidth(s) => {
                let width = s.parse::<usize>().unwrap();
                self.options.width_pixels = width;
                true
            }
            Msg::UpdateHeight(s) => {
                let height = s.parse::<usize>().unwrap();
                self.options.height_pixels = height;
                true
            }
            Msg::FileSelection(files) => {
                for file in files {
                    let callback = self.link.callback(Msg::FileLoaded);
                    let task = self.reader.read_file(file, callback).unwrap();
                    self.tasks.push(task);
                }

                true
            }

            Msg::FileLoaded(file) => {
                console_log!("finished loading image: {}", file.name);

                self.stl = Rc::new(Some(file.content));

                true
            }

            Msg::Reset => {
                self.options = Options::default();
                true
            }

            Msg::Render => {
                if let Some(bytes) = self.stl.borrow() {
                    let window = web_sys::window().unwrap().window();
                    let performance = window.performance().unwrap();

                    let start = performance.now();
                    let mut cursor = std::io::Cursor::new(bytes);
                    let stl = nom_stl::parse_stl(&mut cursor).unwrap();
                    let rendered = racy::render(&stl, &self.options);

                    match rendered {
                        Ok(rendered_bytes) => {
                            self.error = String::new();

                            let end = performance.now();
                            let runtime = end - start;
                            console_log!("rendered in ", runtime, "ms");

                            let start = performance.now();

                            let image_url =
                                bytes_to_object_url(&rendered_bytes, MimeType::JPEG.to_string());

                            self.image_url = image_url;

                            let end = performance.now();
                            let runtime = end - start;
                            console_log!("to object url in ", runtime, "ms")
                        }
                        Err(e) => self.error = e,
                    }

                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div>{ RACY_WEB_VERSION }</div>
                <div class="row">
                    <div class="column">
                        <div>{"width"}</div>
                        <input
                        type="text"
                        name="image-width"
                        value={self.options.width_pixels}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateWidth(e.value))/>

                         <div>{"height"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.height_pixels}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateHeight(e.value))/>

                         <div>{"from x"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.from.x}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateFromX(e.value))/>

                         <div>{"from y"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.from.y}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateFromY(e.value))/>

                         <div>{"from z"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.from.z}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateFromZ(e.value))/>

                         <div>{"to x"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.to.x}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateToX(e.value))/>

                         <div>{"to y"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.to.y}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateToY(e.value))/>

                         <div>{"to z"}</div>
                        <input
                        type="text"
                        name="image-height"
                        value={self.options.to.z}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateToZ(e.value))/>

                        <input type="file" id="input" onchange=self.link.callback(move |v: ChangeData| {
                            let mut res = vec![];

                            if let ChangeData::Files(files) = v {
                                if let Some(file) = files.get(0) {
                                    res.push(file);
                                }
                            }

                            Msg::FileSelection(res)
                        }) />

                        {
                            if !self.error.is_empty() {
                                html! {
                                    <div>{ &self.error }</div>
                                }
                            } else {
                                html! {}
                            }
                        }


                        <div class="row">
                            <div class="column">
                            <button onclick=self.link.callback(|_| Self::Message::Render)>
                                { "Render" }
                            </button>
                            </div>

                            <div class="column">
                            <button onclick=self.link.callback(|_| Self::Message::Reset)>
                                { "Reset" }
                            </button>
                            </div>
                        </div>
                    </div>


                    <div class="column">
                        {
                            if !self.image_url.is_empty() {
                                html! {
                                    <div style="display: inline;">
                                        <img style="display: inline;" src={format!("{}", self.image_url)} alt={"meh"} />
                                    </div>
                                }
                            }  else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}

/// The types we use in this app are:
/// image/png, image/svg+xml, and application/zip
#[wasm_bindgen(module = "/static/js/utils.js")]
extern "C" {
    fn bytes_to_object_url(
        bytes: &[u8],
        #[wasm_bindgen(js_name = mimeType)] mime_type: String,
    ) -> String;
}
