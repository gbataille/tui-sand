use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{
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

    // let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let contents = <String as std::str::FromStr>::from_str(
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    )
    .unwrap();
    let mut world = sand::parse_input(&contents);
    world.display_to_term(&mut stdout)?;

    stdout.write(b"\n")?;
    stdout.write(b"\n")?;
    stdout.write(b"\n")?;

    loop {
        if poll(Duration::from_millis(50))? {
            let ctrlc = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
            match read()? {
                Event::Key(ctrlc) => break,
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
