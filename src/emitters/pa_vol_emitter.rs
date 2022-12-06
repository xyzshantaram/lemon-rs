use std::{cell::RefCell, ops::Deref, rc::Rc};

use libpulse_binding::{
    context::{Context, FlagSet},
    mainloop::standard::{IterateResult, Mainloop},
    proplist::Proplist,
};

use crate::emitter::{Alignment, Emitter};
struct VolumeState {
    ml: Rc<RefCell<Mainloop>>,
}

impl VolumeState {
    fn new() -> Self {
        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().expect("Error creating PulseAudio event loop."),
        ));
        let mut proplist = Proplist::new().unwrap();
        proplist
            .set_str(
                libpulse_binding::proplist::properties::APPLICATION_NAME,
                "lime-rs",
            )
            .expect("Setting Pulse client name failed.");

        let context = Rc::new(RefCell::new(
            Context::new_with_proplist(mainloop.borrow_mut().deref(), "LimeContext", &proplist)
                .expect("Failed to create new context"),
        ));
        context
            .borrow_mut()
            .connect(None, FlagSet::NOFLAGS, None)
            .expect("Pulse context failed to connect.");

        loop {
            match mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    panic!("Iteration failed while connecting to Pulse");
                }
                IterateResult::Success(_) => {}
            }
            match context.borrow().get_state() {
                libpulse_binding::context::State::Ready => {
                    break;
                }
                libpulse_binding::context::State::Failed
                | libpulse_binding::context::State::Terminated => {
                    panic!("Context state failed/terminated, quitting...");
                }
                _ => {}
            }
        }

        VolumeState { ml: mainloop }
    }
}
pub struct PaVolumeEmitter(pub Emitter, VolumeState);

impl PaVolumeEmitter {
    pub fn new(millis: i64, icon: String, alignment: Alignment) -> Self {
        let mut emitter = Emitter::new(millis, icon, alignment);
        let mut state = VolumeState::new();

        emitter.set_emitter(|alignment, fg_color, _, icon| todo!());
        PaVolumeEmitter(emitter, state)
    }
}
