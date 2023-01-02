use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    fs,
    io::{stdout, Error, Write},
    time::Duration,
};

pub mod sand;

fn main() -> Result<(), Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    // let contents = <String as std::str::FromStr>::from_str(
    //     "498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9",
    // )
    // .unwrap();
    let mut world = sand::parse_input(&contents);
    world.display_to_term(&mut stdout)?;

    stdout.write(b"\n")?;
    stdout.write(b"\n")?;
    stdout.write(b"\n")?;

    let mut poll_duration = Duration::from_millis(50);
    loop {
        if poll(poll_duration)? {
            let event = read()?;
            match event {
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('c'),
                    ..
                }) => break,
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Char('+'),
                    ..
                }) => {
                    let duration = poll_duration.as_millis() as u64;
                    if duration > 10 {
                        poll_duration = Duration::from_millis(duration - 10);
                    } else {
                        poll_duration = Duration::from_millis(1);
                    }
                }
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Char('-'),
                    ..
                }) => {
                    poll_duration = Duration::from_millis(poll_duration.as_millis() as u64 + 10);
                }
                _ => (),
            }
        } else {
            // match world.move_step() {
            match world.move_step_part2() {
                Ok(_) => drop(world.display_to_term(&mut stdout)),
                Err(_) => {
                    break;
                }
            }
        }
    }

    // restore terminal
    stdout.execute(cursor::Show)?;
    disable_raw_mode()?;

    Ok(())
}
