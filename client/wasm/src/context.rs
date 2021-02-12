static mut INSTANCE: Option<Context> = None;

pub struct Context {
    pub grpc_host: String,
}

impl Context {
    pub fn get() -> &'static Self {
        // XXX not threadsafe, but we don't use threads on wasm environment.
        unsafe {
            match &INSTANCE {
                Some(x) => x,
                None => {
                    let instance = Context::new();
                    INSTANCE = Some(instance);

                    INSTANCE.as_ref().unwrap()
                }
            }
        }
    }

    fn new() -> Self {
        let grpc_host = "http://localhost:8000"; // TODO retrieve it from build environment.

        Self {
            grpc_host: grpc_host.to_owned(),
        }
    }
}
