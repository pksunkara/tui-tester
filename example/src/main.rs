use crossterm::{
    cursor::{Hide, RestorePosition, SavePosition, Show},
    event::{read, Event, KeyCode},
    style::{PrintStyledContent, Stylize},
    terminal::enable_raw_mode,
    ExecutableCommand,
};
use std::io::stdout;
use tui_tester::Terminal;

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = stdout();

    enable_raw_mode()?;

    stdout
        .execute(PrintStyledContent("? ".yellow()))?
        .execute(PrintStyledContent("Like this? ".bold()))?
        .execute(PrintStyledContent("(y/n) › ".black()))?
        .execute(SavePosition)?
        .execute(PrintStyledContent("yes".cyan()))?
        .execute(Hide)?;

    loop {
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('y') {
                    stdout
                        .execute(RestorePosition)?
                        .execute(PrintStyledContent("yes".cyan()))?;
                } else if event.code == KeyCode::Char('n') {
                    stdout
                        .execute(RestorePosition)?
                        .execute(PrintStyledContent("no ".cyan()))?;
                } else if event.code == KeyCode::Enter {
                    break;
                }
            }
            _ => (),
        }
    }

    stdout.execute(Show)?;

    Ok(())
}
