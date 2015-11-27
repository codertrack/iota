#![cfg(not(test))]

extern crate libc;
extern crate rustc_serialize;
extern crate rustbox;
extern crate docopt;
extern crate iota;

use std::sync::{Arc, Mutex};
use std::thread;
use std::char;
use std::io::stdin;
use docopt::Docopt;
use iota::{
    Editor, Input, UIBuffer, CharStyle, CharColor,
    EditorEvent, Key
};
use rustbox::{InitOptions, RustBox, InputMode};
use rustbox::{Style, Color};

static USAGE: &'static str = "
Usage: iota [<filename>] [options]
       iota --help

Options:
    --vi           Start Iota with vi-like modes
    -h, --help     Show this message.
";


#[derive(RustcDecodable, Debug)]
struct Args {
    arg_filename: Option<String>,
    flag_vi: bool,
    flag_help: bool,
}

fn is_atty(fileno: libc::c_int) -> bool {
    // FIXME: find a way to do this without unsafe
    //        std::io doesn't allow for this, currently
    unsafe { libc::isatty(fileno) != 0 }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let stdin_is_atty = is_atty(libc::STDIN_FILENO);
    let stderr_is_atty = is_atty(libc::STDERR_FILENO);

    // editor source - either a filename or stdin
    let source = if stdin_is_atty {
        Input::Filename(args.arg_filename)
    } else {
        Input::Stdin(stdin())
    };


    // initialise rustbox
    let rb = match RustBox::init(InitOptions{
        buffer_stderr: stderr_is_atty,
        input_mode: InputMode::Esc,
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    // initialise the editor mode
    // let mode: Box<Mode> = if args.flag_vi {
    //     Box::new(NormalMode::new())
    // } else {
    //      Box::new(StandardMode::new())
    // };

    let height = rb.height();
    let width = rb.width();

    // start the editor
    // editor.start();
    let mut editor = Editor::new(source, width, height);

    while editor.running {
        editor.draw();
        // editor.view.draw()
        // editor.get_uibuf()

        {
            // view contents
            let content = editor.get_content();
            draw_everything(content, &rb);
        }


        {
            let cursor_pos = editor.get_cursor_pos().unwrap();
            rb.set_cursor(cursor_pos.0, cursor_pos.1);
        }
        // cursor position

        rb.present();

        // let event = match rb.poll_event(true).unwrap() {
        //     rustbox::Event::KeyEventRaw(_, key, ch) => {
        //         let k = match key {
        //             0 => char::from_u32(ch).map(|c| Key::Char(c)),
        //             a => Key::from_special_code(a),
        //         };
        //         EditorEvent::KeyEvent(k)
        //     }
        //     rustbox::Event::ResizeEvent(width, height) => {
        //         EditorEvent::Resize(width as usize, height as usize)
        //     }
        //     _ => EditorEvent::UnSupported
        // };
        //
        // editor.handle_raw_event(event);
    }
}

fn draw_everything(content: &mut UIBuffer, rb: &RustBox) {
    let stop = content.get_height();
    let start = 0;

    let rows = &mut content.rows[start..stop];
    for row in rows.iter_mut() {
        for cell in row.iter_mut().filter(|cell| cell.dirty) {
            // rb.draw_char(cell.x, cell.y, cell.ch, cell.fg, cell.bg, CharStyle::Normal);

            let bg = get_color(cell.bg);
            let fg = get_color(cell.fg);
            let style = get_style(CharStyle::Normal);

            rb.print_char(cell.x, cell.y, style, fg, bg, cell.ch);

            cell.dirty = false;
        }
    }
}

fn get_color(c: CharColor) -> Color {
    match c {
        CharColor::Default => Color::Default,
        CharColor::Blue    => Color::Blue,
        CharColor::Black   => Color::Black,
    }
}

/// Translate a CharStyle to rustbox::Style
fn get_style(s: CharStyle) -> Style {
    match s {
        CharStyle::Normal => Style::empty(),
    }
}
