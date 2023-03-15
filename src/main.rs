use std::rc::Rc;

use deno_core::error::AnyError;

// High Level overview:
// 1. Create a new JSRuntime instance (use file-system loader)
// 2. Load a module into the runtime
// 3. Evaluate the module
// 4. Repeat until resolved
//
// Those are also a high overview of the whole life-cycle that a JavaScript code will go through

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    // Resolve the javascript path module
    // It will return an 'Url' instance
    let main_module = deno_core::resolve_path(file_path)?;

    let runtime_options = deno_core::RuntimeOptions {
        // Use file-system based module loader
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
    };

    // Create new JSRuntime instance with 'runtime_options' as its options
    // the js_runtime uses a file-system based module loader.
    let mut js_runtime = deno_core::JsRuntime::new(runtime_options);

    // Load the main runtime module and all of its dependencies asynchronously
    // the main module takes 'main_module' as its specifier
    // and no code being passed directly here
    let module_id = js_runtime.load_main_module(&main_module, None).await?;

    // Evaluate already enstantiated ESModule
    let result = js_runtime.mod_evaluate(module_id);

    // Runs event loop untul the runtime ('js_runtime') is resolved
    // without waiting for the inspector
    js_runtime.run_event_loop(false).await?;

    // Return the evaluation result
    result.await?

}

fn main() {
    // Create a single threaded 'tokio' runtime with custom config values
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    if let Err(error) = runtime.block_on(run_js("./test.js")) {
        eprintln!("error: {}", error);
    }
}
