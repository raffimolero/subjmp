use std::{
    collections::BTreeMap,
    io::{self, stdout},
    ops::ControlFlow,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct Runtime<T> {
    generation: u8,
    pending: BTreeMap<(Instant, u8), T>,
}

impl<T> Runtime<T> {
    fn new() -> Self {
        Self {
            generation: 0,
            pending: BTreeMap::new(),
        }
    }

    pub fn schedule_at(&mut self, time: Instant, event: T) {
        self.generation = self.generation.wrapping_add(1);
        self.pending.insert((time, self.generation), event);
    }

    pub fn schedule_secs(&mut self, secs: f32, event: T) {
        self.schedule_at(Instant::now() + Duration::from_secs_f32(secs), event);
    }
}

pub enum RuntimeEvent<T> {
    Scheduled(T),
    Input(Event),
}

pub trait App: Sized {
    type Event;

    fn start(&mut self, runtime: &mut Runtime<Self::Event>) {}

    fn event(
        &mut self,
        runtime: &mut Runtime<Self::Event>,
        event: RuntimeEvent<Self::Event>,
    ) -> io::Result<ControlFlow<()>>;

    fn run(&mut self) -> io::Result<()> {
        let mut runtime = Runtime::new();
        self.start(&mut runtime);
        let mut f = stdout();
        execute!(f, Clear(ClearType::All), MoveTo(0, 0))?;
        enable_raw_mode()?;
        'runtime: loop {
            let Some(((next_tick, _generation), scheduled_event)) = runtime.pending.pop_first()
            else {
                // no pending game events, wait for input
                let event = read()?;
                match self.event(&mut runtime, RuntimeEvent::Input(event))? {
                    ControlFlow::Continue(()) => continue,
                    ControlFlow::Break(()) => break 'runtime,
                }
            };

            while poll(next_tick - Instant::now())? {
                // input event received
                let event = read()?;
                match self.event(&mut runtime, RuntimeEvent::Input(event))? {
                    ControlFlow::Continue(()) => continue,
                    ControlFlow::Break(()) => break 'runtime,
                }
            }
            // game event has arrived
            match self.event(&mut runtime, RuntimeEvent::Scheduled(scheduled_event))? {
                ControlFlow::Continue(()) => continue,
                ControlFlow::Break(()) => break 'runtime,
            }
        }
        execute!(f, Clear(ClearType::All), MoveTo(0, 0))?;
        disable_raw_mode()?;
        Ok(())
    }
}
