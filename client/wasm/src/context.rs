static mut INSTANCE: Option<Context> = None;

pub struct Context {
    pub grpc_host: String,
}

impl Context {
    pub fn get() -> &'static Self {
        return unsafe { INSTANCE.as_ref() }.unwrap();
    }

    pub fn init(grpc_host: &str) {
        let instance = Self {
            grpc_host: grpc_host.to_owned(),
        };

        unsafe {
            INSTANCE = Some(instance);
        }
    }
}
