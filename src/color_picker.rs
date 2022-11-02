use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum RGB {
    Red,
    Green,
    Blue,
}

pub enum CMYK {
    Cyan,
    Magenta,
    Yellow,
    Key,
}

pub enum Msg {
    ColorStrChanged(String),
    ColorRgbChanged((f32, RGB)),
    ColorCmykChanged((f32, CMYK)),
}

pub struct ColorPicker {
    rgb_string: String,
    rgb_value: (f32, f32, f32),
    cmyk_value: (f32, f32, f32, f32),
}

impl Component for ColorPicker {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            rgb_string: "#ffffff".to_string(),
            rgb_value: (1.0, 1.0, 1.0),
            cmyk_value: (0.0, 0.0, 0.0, 0.0),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_color_str_change = {
            ctx.link().callback(|event: InputEvent| {
                let input = extract_input_element(event);

                Msg::ColorStrChanged(input.value())
            })
        };

        let on_rgb_change = |(event, rgb_id): (InputEvent, RGB)| {
            let input = extract_input_element(event);
            let val = input.value_as_number() as f32;

            Msg::ColorRgbChanged((val, rgb_id))
        };

        let on_cmyk_change = |(event, cmyk_id): (InputEvent, CMYK)| {
            let input = extract_input_element(event);
            let val = input.value_as_number() as f32;

            Msg::ColorCmykChanged((val, cmyk_id))
        };

        let on_red_change = ctx
            .link()
            .callback(move |event: InputEvent| on_rgb_change((event, RGB::Red)));

        let on_green_change = ctx
            .link()
            .callback(move |event: InputEvent| on_rgb_change((event, RGB::Green)));

        let on_blue_change = ctx
            .link()
            .callback(move |event: InputEvent| on_rgb_change((event, RGB::Blue)));

        let on_cyan_change = ctx
            .link()
            .callback(move |event: InputEvent| on_cmyk_change((event, CMYK::Cyan)));

        let on_yellow_change = ctx
            .link()
            .callback(move |event: InputEvent| on_cmyk_change((event, CMYK::Yellow)));

        let on_magenta_change = ctx
            .link()
            .callback(move |event: InputEvent| on_cmyk_change((event, CMYK::Magenta)));

        let on_key_change = ctx
            .link()
            .callback(move |event: InputEvent| on_cmyk_change((event, CMYK::Key)));

        html! {
            <div class="container">
                <div class="form-group">
                    <label for="rgb_string">{"RGB color: "} {self.rgb_string.clone()}</label>
                    <input id="rgb_string" type="color" value={self.rgb_string.clone()} oninput={on_color_str_change} />
                </div>
                <div class="row">
                    <div class="col">
                        <div>
                            <label>{"R: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                value={self.rgb_value.0.to_string()}
                                oninput={on_red_change} />
                            <span>{((self.rgb_value.0 * 255.0) as u8).to_string()}</span>
                        </div>
                        <div>
                            <label>{"G: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                value={self.rgb_value.1.to_string()}
                                oninput={on_green_change} />
                            <span>{((self.rgb_value.1 * 255.0) as u8).to_string()}</span>
                        </div>
                        <div>
                            <label>{"B: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                value={self.rgb_value.2.to_string()}
                                oninput={on_blue_change} />
                            <span>{((self.rgb_value.2 * 255.0) as u8).to_string()}</span>
                        </div>
                    </div>
                    <div class="col">
                        <div>
                            <label>{"C: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                oninput={on_cyan_change}
                                value={self.cmyk_value.0.to_string()} />
                            <span>{format!("{:.2}%", self.cmyk_value.0 * 100.0)}</span>
                        </div>
                        <div>
                            <label>{"M: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                oninput={on_magenta_change}
                                value={self.cmyk_value.1.to_string()} />
                            <span>{format!("{:.2}%", self.cmyk_value.1 * 100.0)}</span>
                        </div>
                        <div>
                            <label>{"Y: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                oninput={on_yellow_change}
                                value={self.cmyk_value.2.to_string()} />
                            <span>{format!("{:.2}%", self.cmyk_value.2 * 100.0)}</span>
                        </div>
                        <div>
                            <label>{"K: "}</label>
                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.004"
                                oninput={on_key_change}
                                value={self.cmyk_value.3.to_string()} />
                            <span>{format!("{:.2}%", self.cmyk_value.3 * 100.0)}</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ColorStrChanged(color) => {
                self.rgb_string = color;
                let r: f32 = u8::from_str_radix(&self.rgb_string[1..3], 16)
                    .expect("Couldn't parse red value.")
                    .into();
                let g: f32 = u8::from_str_radix(&self.rgb_string[3..5], 16)
                    .expect("Couldn't parse green value.")
                    .into();
                let b: f32 = u8::from_str_radix(&self.rgb_string[5..7], 16)
                    .expect("Couldn't parse blue value.")
                    .into();

                self.rgb_value = (r / 255.0, g / 255.0, b / 255.0);
                self.cmyk_value = rgb_to_cmyk(self.rgb_value);

                true
            }
            Msg::ColorRgbChanged((value, id)) => {
                let val_str = format!("{:02x}", (value * 255.0) as u8);
                match id {
                    RGB::Red => {
                        self.rgb_value.0 = value;
                        self.rgb_string.replace_range(1..3, &val_str);
                    }
                    RGB::Green => {
                        self.rgb_value.1 = value;
                        self.rgb_string.replace_range(3..5, &val_str);
                    }
                    RGB::Blue => {
                        self.rgb_value.2 = value;
                        self.rgb_string.replace_range(5..7, &val_str);
                    }
                }

                self.cmyk_value = rgb_to_cmyk(self.rgb_value);

                true
            }
            Msg::ColorCmykChanged((value, id)) => {
                match id {
                    CMYK::Cyan => {
                        self.cmyk_value.0 = value;
                    }
                    CMYK::Magenta => {
                        self.cmyk_value.1 = value;
                    }
                    CMYK::Yellow => {
                        self.cmyk_value.2 = value;
                    }
                    CMYK::Key => {
                        self.cmyk_value.3 = value;
                    }
                }

                self.rgb_value = cmyk_to_rgb(self.cmyk_value);
                self.rgb_string = rgb_to_str(self.rgb_value);

                true
            }
        }
    }
}

fn rgb_to_cmyk((r, g, b): (f32, f32, f32)) -> (f32, f32, f32, f32) {
    let k = 1.0 - r.max(g).max(b);
    if k >= 1.0 {
        return (1.0, 1.0, 1.0, 1.0);
    }

    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);

    (c, m, y, k)
}

fn rgb_to_str((r, g, b): (f32, f32, f32)) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

fn cmyk_to_rgb((c, m, y, k): (f32, f32, f32, f32)) -> (f32, f32, f32) {
    let r = (1.0 - c) * (1.0 - k);
    let g = (1.0 - m) * (1.0 - k);
    let b = (1.0 - y) * (1.0 - k);

    (r, g, b)
}

fn extract_input_element(event: InputEvent) -> HtmlInputElement {
    event
        .target()
        .expect("No color input found.")
        .dyn_into::<HtmlInputElement>()
        .expect("Couldn't cast color input into HtmlInputElement")
}
