use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::Paragraph,
};

struct App {
    counter: i64,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render_counter(app: &App) -> String {
    format!("Counter: {}", app.counter)
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(Paragraph::new(render_counter(&app)), f.size());
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // application state
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    while !app.should_quit {
        // application render
        t.draw(|f| {
            ui(&app, f);
        })?;

        // application update
        update(&mut app)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

// Start simple
// then think about how to generalize this more
struct LemonSuperJuice {
    lemon_peel_g: f32,
    citric_acid_g: f32,
    water_ml: f32,
}

impl LemonSuperJuice {
    // Volume in ml
    fn new_from_total_volume(final_volume: f32) -> LemonSuperJuice {
        // OK, how much extra volume does dissolving citric acid in water add?
        // OR should we just do this by weight and the result will be a little off?
        // No, sounds fun figuring out the result.
        // BUT - for now, just assume adding 1g citric acid adds 1 ml volume?
        // Also - no idea how much the lemon peel will add in the end. Experiment?
        // But, for now - assume it doesn't add anything? The volume of oil is small.
        // (but there's of course some solids that filter through...)
        let citric_acid_g = final_volume / 16.6;
        Self {
            lemon_peel_g: citric_acid_g,
            citric_acid_g: citric_acid_g,
            water_ml: final_volume - citric_acid_g,
        }
    }

    // Weight in grams
    fn new_from_lemon_peel_weight(lemon_peel_weight: f32) -> LemonSuperJuice {
        // Recipe: equal weight lemon and citric acid. 16.66 * the weight of the lemon peel in
        // water.
        Self {
            lemon_peel_g: lemon_peel_weight,
            citric_acid_g: lemon_peel_weight,
            water_ml: lemon_peel_weight * 16.66,
        }
    }

    // TODO: Formatting function?
    // Let the counter inc grams of lemon peel, then write formatted "recipe"
}

mod lemon_super_juice_tests {
    use super::*;

    #[test]
    fn test_from_total_volume() {
        let one_litre_juice = LemonSuperJuice::new_from_total_volume(1000.0);
        // Amount of citric acid && lemon peel: 1000 / 16.6 = 60.24096
        // FIXME: OK, need to be able ot do less precise comparisons
        //        how do we do this in rust?
        assert_eq!(one_litre_juice.water_ml, 939.75903);
        assert_eq!(one_litre_juice.lemon_peel_g, 60.240963);
        assert_eq!(one_litre_juice.citric_acid_g, 60.240963);
    }
    // XXX: Need to return a result? What if we provide a negative volume etc?

    #[test]
    fn test_from_lemon_peel_weight() {
        let juice = LemonSuperJuice::new_from_lemon_peel_weight(10.0);
        assert_eq!(juice.lemon_peel_g, 10.0);
        // OK, now how do I do floating point asserts? Is there a "within X", or do I
        // have to do that myself?
        assert_eq!(juice.citric_acid_g, 10.0);
        assert_eq!(juice.water_ml, 166.6);
    }
}
