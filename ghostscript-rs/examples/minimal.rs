extern crate ghostscript;
use ghostscript as gs;
use gs::builder::{GhostscriptBuilder, BuilderResult};

fn main() {
    let mut builder = GhostscriptBuilder::new();

    builder.with_init_params(&[
        "-sDEVICE=pnggray",
        "-dNOPAUSE",
        "-dSAFER",
        "-sOutputFile=output.png",
        "-r100",
        "-dTextAlphaBits=4",
        "-dGraphicsAlphaBits=4",
        "--",
        "../input.pdf"
    ]);

    // We don't need the parameter to build(), and so pass a unit,
    // but it can be any user data to associate with the new
    // Ghostscript interpreter instance, and it can also implement
    // some useful Ghostscript callback traits.
    match builder.build(()) {
        BuilderResult::Running(instance) => {
            // This is where we could get a running instance for further interpreter work.
            // But our init params should have made the interpreter immediately quit
            // after rendering the file.
            eprintln!("Unexpected ghostscript instance: {:?}", instance);
            // Our user data can be extracted back, destroying the interpreter instance.
            eprintln!("I'm just a unit: {:?}", instance.into_inner());
            unreachable!("The instance should have quit immediately after initialization.");
        }
        BuilderResult::Quit(user_data) => {
            // Interpreter ran and quit. Execution successfully completed.
            // Our user data is returned back, inside a Box.
            println!("I'm just a unit: {:?}", *user_data);
        },
        BuilderResult::Failed(e) => {
            // Interpreter failed to build or run. The user_data is returned to us still.
            eprintln!("I'm just a unit: {:?}", e.user_data);
            panic!("Error building instance: {:?}", e.kind_and_code());
        },
    }

    // The builder can be reused to keep building new instances.
    // All the settings and parameters are preserved.
    // The following repeats the same rendering as above, but the has_quit() shorthand is used
    // to convert BuilderResult into Result in a similar way to the above match.
    builder.build(()).has_quit().expect("Interpreter ran and quit successfully");
}
