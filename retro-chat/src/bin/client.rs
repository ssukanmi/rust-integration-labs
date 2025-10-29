use anyhow::Result;
use chrono::Local;
use clap::Parser;
use cursive::{
    Cursive,
    align::HAlign,
    event::Key,
    theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme},
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
};
use retro_chat::args::Args;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let Args { name, server } = Args::parse();

    let mut siv = cursive::default();
    siv.set_theme(create_retro_theme());

    // header
    let header = TextView::new(format!(
        r#"╔═ RETRO CHAT ═╗ User: {} ╔═ {} ═╗"#,
        name,
        Local::now().format("%H:%M:%S")
    ))
    .style(Color::Light(BaseColor::Green))
    .h_align(HAlign::Center);

    // message
    let messages = TextView::new("")
        .with_name("messages")
        .min_height(20)
        .scrollable();

    let messages = ScrollView::new(messages)
        .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
        .min_width(60)
        .full_width();

    // Creating an input area for typing messages
    let input = EditView::new()
        .on_submit(move |s, text| send_message(s, text.to_string()))
        .with_name("input")
        .min_width(50)
        .max_height(3)
        .full_width();

    // help
    let help_text = TextView::new("ESC:quit | Enter:send | Commands: /help, /clear, /quit")
        .style(Color::Dark(BaseColor::White));

    // assemble layers
    let layout = LinearLayout::vertical()
        .child(Panel::new(header)) // Header
        .child(
            Dialog::around(messages)
                .title("Messages")
                .title_position(HAlign::Center)
                .full_width(),
        )
        .child(
            Dialog::around(input)
                .title("Message")
                .title_position(HAlign::Center)
                .full_width(),
        )
        .child(Panel::new(help_text).full_width()); // Help

    let centered_layout = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(layout)
        .child(DummyView.full_width());

    // callbacks
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('/', |s| {
        s.call_on_name("input", |view: &mut EditView| {
            view.set_content("/");
        });
    });

    // connect to server
    let stream = TcpStream::connect(server).await?;
    let (_reader, mut writer) = stream.into_split();

    writer.write_all(format!("{}\n", name).as_bytes()).await?;

    siv.add_fullscreen_layer(centered_layout);

    siv.run();

    Ok(())
}

fn send_message(siv: &mut Cursive, msg: String) {
    if msg.is_empty() {
        // Ignore empty messages
        return;
    }

    // Handle specific commands
    match msg.as_str() {
        "/help" => {
            siv.call_on_name("messages", |view: &mut TextView| {
                view.append("\n=== Commands ===\n/help - Show this help\n/clear - Clear messages\n/quit - Exit chat\n\n");
            });
            siv.call_on_name("input", |view: &mut EditView| {
                view.set_content("");
            });
            return;
        }
        "/clear" => {
            siv.call_on_name("messages", |view: &mut TextView| {
                view.set_content("");
            });
            siv.call_on_name("input", |view: &mut EditView| {
                view.set_content("");
            });
            return;
        }
        "/quit" => {
            siv.quit();
            return;
        }
        _ => {}
    }

    siv.call_on_name("input", |view: &mut EditView| {
        view.set_content("");
    });
}

fn create_retro_theme() -> Theme {
    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Rgb(0, 0, 20);
    palette[PaletteColor::View] = Color::Rgb(0, 0, 20);
    palette[PaletteColor::Primary] = Color::Rgb(0, 255, 0);
    palette[PaletteColor::TitlePrimary] = Color::Rgb(0, 255, 128);
    palette[PaletteColor::Secondary] = Color::Rgb(255, 191, 0);
    palette[PaletteColor::Highlight] = Color::Rgb(0, 255, 255);
    palette[PaletteColor::HighlightInactive] = Color::Rgb(0, 128, 128);
    palette[PaletteColor::Shadow] = Color::Rgb(0, 0, 40);

    Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette,
    }
}
