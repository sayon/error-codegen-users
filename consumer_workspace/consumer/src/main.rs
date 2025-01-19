use zksync_error::{documentation::Documented, error::{definitions::{Sequencer, Zksolc}, domains::{Compiler, Core, ZksyncError}, ICustomError as _}};

fn main() {
   let err1 = Sequencer::SomeCoreError { path: "oh no".into(), payload: 99 };

   let err2 = Zksolc::FileNotFound { path: "Nono".into(), file_index: 4 };

   let documentation =  err1.get_documentation();
   eprintln!("{documentation:#?}");

   let documentation =  err2.get_documentation();
   eprintln!("{documentation:#?}");

}
