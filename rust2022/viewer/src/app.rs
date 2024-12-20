extern crate aoc_14;
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
};
use gloo::timers::callback::Interval;

use image::{ImageBuffer, Rgb, RgbImage};

use std::collections::HashSet;

use aoc_14::{get_grid, get_grid_repr,  step_limited, Coords};
use yew::prelude::*;

pub struct App {
    repr: String,
    map: HashSet<Coords>,
    sand: HashSet<Coords>,
    interval: Option<Interval>,
}

pub enum Msg {
    Step(usize),
    Reset,
    Tick,
    StartInterval,
    Cancel,
}

impl App{
    fn step(&mut self, num_steps: usize) -> bool{
        let mut done = false;
        for _ in 0..num_steps {
            if step_limited(&mut self.map, &mut self.sand) {
                done = true;
                break;
            }
        }
        self.repr = get_grid_img(&self.map, &self.sand);
        done
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let steps: Vec<(usize, Callback<MouseEvent>)> = (0..4)
            .map(|x| 10_usize.pow(x))
            .map(|x| (x, ctx.link().callback(move |_| Msg::Step(x))))
            .collect();

        html! {
            <main>
                <div>
                    <img style="height:500px;image-rendering: pixelated;" src={self.repr.clone()}/>
                </div>
                <div>
                <button onclick={ctx.link().callback(|_|  Msg::Reset)}>{"Reset"}</button>
                <button onclick={ctx.link().callback(|_|  Msg::StartInterval)}>{"Start"}</button>
                <button onclick={ctx.link().callback(|_|  Msg::Cancel)}>{"Cancel"}</button>
                    {steps.iter()
                        .map(|(x, callback)| html!{
                            <button onclick={callback}>{format!("Step {}", x)}</button>}
                            ).collect::<Html>()
                    }
                </div>
            </main>
        }
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Step(x) => {
                log::info!("update");
                // self.image = img::new_image(self.image_size);
                self.step(x);

                true
            },
            Msg::Reset =>{
                (self.map, self.sand) = get_grid();
                self.repr = get_grid_img(&self.map, &self.sand);
                true
            },
            Msg::StartInterval=>{
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(10, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);
                true


            },
            Msg::Tick =>{self.step(1); true},
            Msg::Cancel =>{
                self.interval = None;
                true},
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("create");
        let (map, sand) = get_grid();
        Self {
            repr: get_grid_img(&map, &sand),
            map,
            sand,
            interval: None,
        }
    }
}
    fn get_grid_img(map: &HashSet<Coords>, sand: &HashSet<Coords>) -> String {
        let grid = get_grid_repr(map, sand);
        let vals: Vec<[u8; 3]> = vec![
            [8, 135, 209],   // Water  0
            [201, 185, 84],  // Sand   1
            [163, 161, 151], // Stone  2
        ];
        let image_height: u32 = grid.len().try_into().unwrap();
        let image_width: u32 = grid[0].len().try_into().unwrap();
        log::info!("width: {}, height: {}", image_width, image_height);
        let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
        for y in 0..image_height {
            for x in 0..image_width {
                match grid[y as usize][x as usize] {
                    '.' => img.put_pixel(x, y, Rgb(vals[0])),
                    's' => img.put_pixel(x, y, Rgb(vals[1])),
                    '#' => img.put_pixel(x, y, Rgb(vals[2])),
                    _ => img.put_pixel(x, y, Rgb([0, 0, 0])),
                }
            }
        }
        let mut cursor = Cursor::new(Vec::new());

        match img.write_to(&mut cursor, image::ImageFormat::Png) {
            Ok(_) => println!("this is ok"),
            Err(_) => println!("ERROR"),
        };
        let mut out = Vec::new();
        cursor.seek(SeekFrom::Start(0)).unwrap();
        cursor.read_to_end(&mut out).unwrap();
        format!("data:image/png;base64,{}", base64::encode(&out))
    }
