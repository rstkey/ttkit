use std::thread;
use std::time::{Duration, Instant};
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

const COL: usize = 4;

fn print_banner(term: &Term) {
    let (_, height) = term.term_size().unwrap();
    for row in 0..height {
        let _ = term.print(row, 0, format!("{} ", row).as_str());
    }
    let _ = term.print(0, COL, "> (q)uit, (r)estart");
    let _ = term.present();
}

fn print_event(term: &Term, ev: Event, now: &Instant) {
    let elapsed = now.elapsed();
    let (_, height) = term.term_size().unwrap();
    let _ = term.print(1, COL, format!("{:?}", ev).as_str());
    let _ = term.print(
        height - 1,
        COL,
        format!(
            "time elapsed since program start: {}s + {}ms",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        )
        .as_str(),
    );
    let _ = term.present();
}

fn main() {
    let term = Term::with_height(TermHeight::Fixed(10));
    let now = Instant::now();

    print_banner(&term);

    let th = thread::spawn(move || {
        while let Ok(ev) = term.poll_event() {
            if let Event::Key(Key::Char('q')) = ev {
                break;
            }

            if let Event::Key(Key::Char('r')) = ev {
                term.pause();
                println!("restart in 2 seconds");
                thread::sleep(Duration::from_secs(2));
                term.restart();
                term.clear();
            }

            print_banner(&term);
            print_event(&term, ev, &now);
        }
    });
    let _ = th.join();
}