// gui
extern crate tui;
extern crate termion;

pub use std::io::{self, Read, Write};
pub use std::sync::mpsc::{channel, Sender, Receiver};
pub use std::thread;
pub use std::time;

pub use tui::Terminal;
pub use tui::backend::{Backend, TermionBackend};
pub use tui::buffer::Buffer;
pub use tui::widgets::{border, Block, Paragraph, Widget, List, SelectableList, Tabs, BarChart};
pub use tui::layout::{Group, Rect, Size, Direction};
pub use tui::style::{Color, Modifier, Style};
pub use tui::widgets::canvas::{Canvas, Points, Line, Map, MapResolution};

pub use termion::input::TermRead;
pub use termion::event::{Event, Key};
pub use termion::screen::AlternateScreen;

type Lon = f64;
type Lat = f64;


#[derive(Clone)]
pub struct University {
    name: &'static str,
    location: (Lon, Lat),
}

pub struct Stat {
    activity: Vec<u64>,
    _days: Vec<String>,
}

pub struct IU5Years {
    innopolis: University,
    friends: Vec<University>,
    stat: Stat,
    size: Rect,
}


enum AppEvent {
    Input(Key),
}


impl Stat {
    pub fn new(month: u8, day: u8, activity: &[u64]) -> Self {
        let mut _m = month;
        let mut d = day;
        let mut days: Vec<String> = vec![];
        for _ in 0..activity.len() {
            days.push(format!("{}", d));
            d += 1;
            if d > 30 {
                d = 1;
                _m += 1;
            }
        }
        Stat {
            activity: activity.iter().map(|x| *x).collect(),
            _days: days,
        }
    }
    pub fn data<'a>(&'a self) -> Vec<(&'a str, u64)> {
        self._days.iter().map(|x| x.as_str())
            .zip(self.activity.iter().map(|x| *x))
            .collect()
    }
}

fn main() {
    let activity: &[u64] = &[
        20,
        39,
        2,
        55,
        119,
        66,
        41,
        79,
        11,
        7,
        83,
        27,
        82,
        9,
        31,
        41,
        10,
        17,
        31,
        36,
        34,
        150,
        35,
        25,
        36,
        22,
        34,
        68,
        18,
        34,
        18];

    let stat = Stat::new(11, 10, &activity);


    let mut app = IU5Years {
        innopolis: University {
            name: "Innopolis University",
            location: (55.753449, 48.743411)
        },
        friends: vec![
            University {
                name: "香港科技大學",
                location: (22.352493, 113.8475088),
            },
            University {
                name: "北京理工大学自动化学院",
                location: (39.964431, 116.310319),
            },
            University {
                name: "University of Bonn",
                location: (50.7267715, 7.0865227),
            },
            University {
                name: "EURECOM",
                location: (43.614386, 7.071125)
            },
            University {
                name: "University of Innsbruck",
                location: (47.2633542, 11.3838006),
            },
            University {
                name: "Сколковский Институт Науки и Технологий",
                location: (55.6990459, 37.3595933)
            },
            University {
                name: "Politecnico di Milano",
                location: (45.478415, 9.227545),
            },
            University {
                name: "서울대학교",
                location: (37.460137, 126.951862),
            },
            University {
                name: "Universität Innsbruck",
                location: (47.263485, 11.383801),
            },
            University {
                name: "Университет ИТМО",
                location: (59.957363, 30.308169),
            },
            University {
                name: "Orta Doğu Teknik Üniversitesi",
                location: (39.895773, 32.777986),
            },
            University {
                name: "Carnegie Mellon University",
                location: (40.4428081, -79.9430128)
            },
            University {
                name: "Institiúid Teicneolaíochta Bhaile Atha Cliath",
                location: (53.3385296, -6.2665987)
            },
            University {
                name: "भारतीय प्रौद्योगिकी संस्थान",
                location: (12.9952179, 80.2380096)
            },
            University {
                name: "新加坡国立大学",
                location: (1.2966426, 103.7763939)
            },
            University {
                name: "Universidade Federal de Minas Gerais",
                location: (-19.8690878, -43.9663841)
            },
        ],
        stat,
        size: Rect::default(),
    };

    // Terminal
    let _screen = AlternateScreen::from(io::stdout());
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    // Channels
    let (tx, rx) = channel();
    let input_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(AppEvent::Input(evt)).unwrap();
            if evt == Key::Char('q') {
                break;
            }
        }
    });

    // First draw call
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let evt = rx.recv().unwrap();
        match evt {
            AppEvent::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            }
        }

        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}


fn draw(t: &mut Terminal<TermionBackend>, app: &IU5Years) {
    let inno = app.innopolis.clone();

    let area = t.size().unwrap();
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(13), Size::Min(1), Size::Fixed(10)])
        .render(t, &area, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Fixed(44), Size::Min(0)])
                .render(t, &chunks[0], |t, chunks| {
                    Paragraph::default()
                        .text(&logo())
                        .block(Block::default().title("Innopolis University").borders(border::ALL))
                        .style(Style::default().fg(Color::Cyan))
                        .render(t, &chunks[0]);

                    BarChart::default()
                        .block(Block::default()
                            .borders(border::ALL)
                            .title("Telegram activity: 10th of Nov. - 11th of Dec."))
                        .data(&app.stat.data())
                        .bar_width(2)
                        .bar_gap(1)
                        .value_style(
                            Style::default()
                                .bg(Color::Yellow)
                                .fg(Color::Yellow)
                        )
                        .label_style(Style::default().fg(Color::Green))
                        .style(Style::default().fg(Color::Yellow))
                        .render(t, &chunks[1]);
                });
            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(65), Size::Percent(35)])
                .render(t, &chunks[1], |t, chunks| {
                    Canvas::default()
                        .block(Block::default().title("World").borders(border::ALL))
                        .paint(|ctx| {
                            ctx.draw(&Map {
                                color: Color::White,
                                resolution: MapResolution::Low,
                            });
                            ctx.layer();

                            ctx.draw(&Points { coords: &[inno.location], color: Color::Green });

                            for uni in &app.friends {
                                ctx.draw(&Line {
                                    x1: inno.location.1,
                                    y1: inno.location.0,
                                    x2: uni.location.1,
                                    y2: uni.location.0,
                                    color: Color::Green,
                                });
                            }
                        })
                        .x_bounds([-180.0, 180.0])
                        .y_bounds([-90.0, 90.0])
                        .render(t, &chunks[0]);

                    let style = Style::default();
                    List::default()
                        .block(Block::default().title("Friends").borders(border::ALL))
                        .items(app.friends
                                  .iter()
                                  .map(|x| x.name)
                                  .chain(::std::iter::once("..."))
                                  .map(|x| (x, &style))
                                  .collect::<Vec<_>>().as_slice())
                        .render(t, &chunks[1]);
                });

            Paragraph::default()
                .block(Block::default().title("Uptime").borders(border::ALL))
                .text(&uptime())
                .raw(false)
                .style(Style::default().fg(Color::DarkGray).modifier(Modifier::Bold))
                .wrap(false)
                .render(t, &chunks[2]);
        });

    t.flush().unwrap();
}

fn logo() -> String {
    let logo: &'static str = include_str!("university.txt");

    logo.replace('X', "{fg=yellow X}")
        .replace("|-|", "|{fg=green -}|")
        .replace(':', "{fg=green :}")
        .replace("INNOPOLIS", "{fg=white I}{fg=green NN}{fg=white OPOLIS}")
        .replace("UNIVERSITY", "{fg=white UNIVERSITY}")
}


fn uptime() -> String {
    let up: &'static str = include_str!("uptime.txt");

    up.replace('#', "{fg=magenta #}")
}