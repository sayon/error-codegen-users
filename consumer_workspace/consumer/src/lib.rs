use zksync_error::error::{definitions::Sequencer, domains::Core};



fn main() {
    let _err = Core::Sequencer(Sequencer::SomeCoreError { path: "oh no".into(), payload: 99 });

}
