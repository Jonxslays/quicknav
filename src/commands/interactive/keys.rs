use anyhow::Result;

use std::io;
use std::io::Write;

use termion::screen::ToMainScreen;
use termion::event::Key;
use termion::input::TermRead;

use super::*;

pub fn map_keys(mut ctx: Context, stdin: io::Stdin) -> Result<i32> {
    base_shell(&mut ctx, "Welcome to the Quicknav interactive shell!")?;
    welcome_page(&mut ctx)?;

    // Iterate over key presses and act accordingly
    for c in stdin.keys().map(|c| c.unwrap()) {
        match c {
            Key::Ctrl('c') => break,
            Key::Backspace => ctx.backspace()?,
            Key::Delete => ctx.del()?,
            Key::Up => {}
            Key::Down => {}
            Key::Left => ctx.left()?,
            Key::Right => ctx.right()?,
            Key::Char('\n') => {
                let content: String = ctx.content.iter().collect();

                ctx.new_line()?;
                ctx.write_line(Line::Str(format!("Content: {}", content)))?; // just testing this here
                ctx.new_line()?;
                ctx.purge();
            }
            Key::Char('0') => ctx.write_char('0')?,
            Key::Char('1') => ctx.write_char('1')?,
            Key::Char('2') => ctx.write_char('2')?,
            Key::Char('3') => ctx.write_char('3')?,
            Key::Char('4') => ctx.write_char('4')?,
            Key::Char('5') => ctx.write_char('5')?,
            Key::Char('6') => ctx.write_char('6')?,
            Key::Char('7') => ctx.write_char('7')?,
            Key::Char('8') => ctx.write_char('8')?,
            Key::Char('9') => ctx.write_char('9')?,
            Key::Char('a') => ctx.write_char('a')?,
            Key::Char('b') => ctx.write_char('b')?,
            Key::Char('c') => ctx.write_char('c')?,
            Key::Char('d') => ctx.write_char('d')?,
            Key::Char('e') => ctx.write_char('e')?,
            Key::Char('f') => ctx.write_char('f')?,
            Key::Char('g') => ctx.write_char('g')?,
            Key::Char('h') => ctx.write_char('h')?,
            Key::Char('i') => ctx.write_char('i')?,
            Key::Char('j') => ctx.write_char('j')?,
            Key::Char('k') => ctx.write_char('k')?,
            Key::Char('l') => ctx.write_char('l')?,
            Key::Char('m') => ctx.write_char('m')?,
            Key::Char('n') => ctx.write_char('n')?,
            Key::Char('o') => ctx.write_char('o')?,
            Key::Char('p') => ctx.write_char('p')?,
            Key::Char('q') => ctx.write_char('q')?,
            Key::Char('r') => ctx.write_char('r')?,
            Key::Char('s') => ctx.write_char('s')?,
            Key::Char('t') => ctx.write_char('t')?,
            Key::Char('u') => ctx.write_char('u')?,
            Key::Char('v') => ctx.write_char('v')?,
            Key::Char('w') => ctx.write_char('w')?,
            Key::Char('x') => ctx.write_char('x')?,
            Key::Char('y') => ctx.write_char('y')?,
            Key::Char('z') => ctx.write_char('z')?,
            Key::Char('A') => ctx.write_char('A')?,
            Key::Char('B') => ctx.write_char('B')?,
            Key::Char('C') => ctx.write_char('C')?,
            Key::Char('D') => ctx.write_char('D')?,
            Key::Char('E') => ctx.write_char('E')?,
            Key::Char('F') => ctx.write_char('F')?,
            Key::Char('G') => ctx.write_char('G')?,
            Key::Char('H') => ctx.write_char('H')?,
            Key::Char('I') => ctx.write_char('I')?,
            Key::Char('J') => ctx.write_char('J')?,
            Key::Char('K') => ctx.write_char('K')?,
            Key::Char('L') => ctx.write_char('L')?,
            Key::Char('M') => ctx.write_char('M')?,
            Key::Char('N') => ctx.write_char('N')?,
            Key::Char('O') => ctx.write_char('O')?,
            Key::Char('P') => ctx.write_char('P')?,
            Key::Char('Q') => ctx.write_char('Q')?,
            Key::Char('R') => ctx.write_char('R')?,
            Key::Char('S') => ctx.write_char('S')?,
            Key::Char('T') => ctx.write_char('T')?,
            Key::Char('U') => ctx.write_char('U')?,
            Key::Char('V') => ctx.write_char('V')?,
            Key::Char('W') => ctx.write_char('W')?,
            Key::Char('X') => ctx.write_char('X')?,
            Key::Char('Y') => ctx.write_char('Y')?,
            Key::Char('Z') => ctx.write_char('Z')?,
            Key::Char('!') => ctx.write_char('!')?,
            Key::Char('"') => ctx.write_char('"')?,
            Key::Char('#') => ctx.write_char('#')?,
            Key::Char('$') => ctx.write_char('$')?,
            Key::Char('%') => ctx.write_char('%')?,
            Key::Char('&') => ctx.write_char('&')?,
            Key::Char('(') => ctx.write_char('(')?,
            Key::Char(')') => ctx.write_char(')')?,
            Key::Char('*') => ctx.write_char('*')?,
            Key::Char('+') => ctx.write_char('+')?,
            Key::Char(',') => ctx.write_char(',')?,
            Key::Char('-') => ctx.write_char('-')?,
            Key::Char('.') => ctx.write_char('.')?,
            Key::Char('/') => ctx.write_char('/')?,
            Key::Char(':') => ctx.write_char(':')?,
            Key::Char(';') => ctx.write_char(';')?,
            Key::Char('<') => ctx.write_char('<')?,
            Key::Char('=') => ctx.write_char('=')?,
            Key::Char('>') => ctx.write_char('>')?,
            Key::Char('?') => ctx.write_char('?')?,
            Key::Char('@') => ctx.write_char('@')?,
            Key::Char('[') => ctx.write_char('[')?,
            Key::Char(']') => ctx.write_char(']')?,
            Key::Char('^') => ctx.write_char('^')?,
            Key::Char('_') => ctx.write_char('_')?,
            Key::Char('`') => ctx.write_char('`')?,
            Key::Char('{') => ctx.write_char('{')?,
            Key::Char('|') => ctx.write_char('|')?,
            Key::Char('}') => ctx.write_char('}')?,
            Key::Char('~') => ctx.write_char('~')?,
            Key::Char(' ') => ctx.write_char(' ')?,
            Key::Char('\'') => ctx.write_char('\'')?,
            Key::Char('\\') => ctx.write_char('\\')?,
            Key::Char('\t') => ctx.write_char('\t')?,
            Key::Char('\r') => ctx.write_char('\r')?,
            _ => write!(ctx.tty, "bad {:?}", c)?,
        }

        ctx.flush()?;
    }

    // Exit the interactive shell
    write!(ctx.tty, "{}", ToMainScreen)?;
    ctx.flush()?;

    Ok(0)
}