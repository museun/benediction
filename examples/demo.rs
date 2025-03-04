use anathema::{
    component::{Children, Component, State},
    default_widgets::Canvas,
    geometry::Size,
    prelude::{Backend as _, Context, Document, ToSourceKind, TuiBackend},
    runtime::Runtime,
    state::{Color, List, Value},
    widgets::Style,
};

fn pixel_to_style(p: benediction::Pixel) -> (char, Style) {
    let mut style = Style::new();
    match p.fg {
        benediction::Color::Default => {}
        benediction::Color::Transparent => {}
        benediction::Color::Rgb([r, g, b]) => style.set_fg(Color::Rgb(r, g, b)),
    }
    match p.bg {
        benediction::Color::Default => {}
        benediction::Color::Transparent => {}
        benediction::Color::Rgb([r, g, b]) => style.set_bg(Color::Rgb(r, g, b)),
    }
    (p.ch, style)
}

#[derive(State)]
struct MainState {
    #[state_ignore]
    time: benediction::Time,

    #[state_ignore]
    blobs: benediction::Blobs,

    #[state_ignore]
    plasma: benediction::Plasma,

    #[state_ignore]
    fire: benediction::Fire,

    selected: Value<String>,
    list: Value<List<String>>,
}

struct DemoView;
impl DemoView {
    const VIEWS: &[&str] = &[
        "vwave",
        "hwave",
        "pulse",
        "spiral",
        "checkerboard",
        "blobs",
        "plasma",
        "fire",
    ];
}

impl Component for DemoView {
    type State = MainState;
    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        _: Children<'_, '_>,
        _: Context<'_, '_, Self::State>,
    ) {
        if matches!(key.state, anathema::component::KeyState::Press) {
            match key.code {
                anathema::component::KeyCode::Char('k') => {
                    let p = {
                        let t = state.selected.to_ref();
                        Self::VIEWS.iter().position(|&c| c == &*t).unwrap_or(0)
                    };
                    let n = (p + 1) % Self::VIEWS.len();
                    state.selected.set(Self::VIEWS[n].to_string());
                }
                anathema::component::KeyCode::Char('j') => {
                    let p = {
                        let t = state.selected.to_ref();
                        Self::VIEWS.iter().position(|&c| c == &*t).unwrap_or(0)
                    };
                    let n = p.checked_sub(1).unwrap_or(Self::VIEWS.len() - 1);
                    state.selected.set(Self::VIEWS[n].to_string());
                }
                _ => {}
            }
        }
    }

    fn resize(
        &mut self,
        state: &mut Self::State,
        mut elements: Children<'_, '_>,
        _: Context<'_, '_, Self::State>,
    ) {
        elements.elements().by_tag("canvas").first(move |el, _| {
            let width = el.size().width as benediction::Scalar;
            let height = el.size().height as benediction::Scalar;
            state.plasma.update(width, height);
            state.fire.update(width, height);
            state.blobs.update(width, height);
        });
    }

    fn tick(
        &mut self,
        state: &mut Self::State,
        mut elements: Children<'_, '_>,
        _: Context<'_, '_, Self::State>,
        dt: std::time::Duration,
    ) {
        state.time.update(dt.as_secs_f32());

        let func = Self::VIEWS
            .iter()
            .position(|&c| c == &**state.selected.to_ref())
            .unwrap_or(0);

        let dt = state.time.normalize();

        elements.elements().by_tag("canvas").first(move |el, _| {
            let width = el.size().width as benediction::Scalar;
            let height = el.size().height as benediction::Scalar;
            let canvas = el.to::<Canvas>();

            let draw = move |x, y, p| {
                let (ch, style) = pixel_to_style(p);
                canvas.put(ch, style, (x as u16, y as u16));
            };
            match func {
                0 => benediction::vertical_wave(dt, width, height, draw),
                1 => benediction::horizontal_wave(dt, width, height, draw),
                2 => benediction::pulse(dt, width, height, draw),
                3 => benediction::spiral(dt, width, height, draw),
                4 => benediction::checkerboard(dt, width, height, draw),
                5 => state.blobs.render(dt, width, height, draw),
                6 => state.plasma.render(dt, width, height, draw),
                7 => state.fire.render(dt, width, height, draw),
                _ => return,
            }
        });
    }
}

fn main() {
    let doc = Document::new(String::from("@index"));

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let Size { width, height } = backend.size();

    let mut runtime = Runtime::builder(doc, &backend);
    runtime
        .component(
            "index",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples",
                "/demo.aml"
            ))
            .to_template(),
            DemoView {},
            MainState {
                time: benediction::Time::new(1.0),
                blobs: benediction::Blobs::new(
                    width as benediction::Scalar,
                    height as benediction::Scalar - 1,
                ),
                plasma: benediction::Plasma::new(
                    width as benediction::Scalar,
                    height as benediction::Scalar - 1,
                ),
                fire: benediction::Fire::new(
                    width as benediction::Scalar,
                    height as benediction::Scalar - 1,
                ),
                selected: Value::new(String::from("vwave")),
                list: Value::new(List::from_iter(
                    DemoView::VIEWS.iter().map(|c| c.to_string()),
                )),
            },
        )
        .unwrap();

    runtime.finish(|rt| rt.run(&mut backend)).unwrap();
}
