#[macro_export]
macro_rules! gl_call {
    ($fun:expr) => {{
        $fun;
        let gl_log_call = || {
            loop
            {
                let error = gl::GetError();
                if error == gl::NO_ERROR
                {
                    break;
                }
                println!("[OpenGL Error] ({:x})", error);
                return false;
            }
            return true;
        };
        if !gl_log_call()
        {
            println!("{} yielded an error in {}, line: {}", stringify!($fun), file!(), line!());
            std::process::exit(1);
        }
    }};
}
