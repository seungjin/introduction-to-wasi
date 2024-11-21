#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod cli {
        #[allow(dead_code, clippy::all)]
        pub mod stdout {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stdout() -> OutputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stdout@0.2.2")]
                    extern "C" {
                        #[link_name = "get-stdout"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stderr {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stderr() -> OutputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stderr@0.2.2")]
                    extern "C" {
                        #[link_name = "get-stderr"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdin {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stdin() -> InputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stdin@0.2.2")]
                    extern "C" {
                        #[link_name = "get-stdin"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::InputStream::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod environment {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the POSIX-style environment variables.
            ///
            /// Each environment variable is provided as a pair of string variable names
            /// and string value.
            ///
            /// Morally, these are a value import, but until value imports are available
            /// in the component model, this import function should return the same
            /// values each time it is called.
            pub fn get_environment() -> _rt::Vec<(_rt::String, _rt::String)> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.2")]
                    extern "C" {
                        #[link_name = "get-environment"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base9 = l1;
                    let len9 = l2;
                    let mut result9 = _rt::Vec::with_capacity(len9);
                    for i in 0..len9 {
                        let base = base9.add(i * 16);
                        let e9 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            let l6 = *base.add(8).cast::<*mut u8>();
                            let l7 = *base.add(12).cast::<usize>();
                            let len8 = l7;
                            let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                            (_rt::string_lift(bytes5), _rt::string_lift(bytes8))
                        };
                        result9.push(e9);
                    }
                    _rt::cabi_dealloc(base9, len9 * 16, 4);
                    result9
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the POSIX-style arguments to the program.
            pub fn get_arguments() -> _rt::Vec<_rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.2")]
                    extern "C" {
                        #[link_name = "get-arguments"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base6 = l1;
                    let len6 = l2;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 8);
                        let e6 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            _rt::string_lift(bytes5)
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 8, 4);
                    result6
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Return a path that programs should use as their initial current working
            /// directory, interpreting `.` as shorthand for this.
            pub fn initial_cwd() -> Option<_rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.2")]
                    extern "C" {
                        #[link_name = "initial-cwd"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<*mut u8>();
                                let l3 = *ptr0.add(8).cast::<usize>();
                                let len4 = l3;
                                let bytes4 = _rt::Vec::from_raw_parts(
                                    l2.cast(),
                                    len4,
                                    len4,
                                );
                                _rt::string_lift(bytes4)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod exit {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            /// Exit the current instance and any linked instances.
            pub fn exit(status: Result<(), ()>) {
                unsafe {
                    let result0 = match status {
                        Ok(_) => 0i32,
                        Err(_) => 1i32,
                    };
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/exit@0.2.2")]
                    extern "C" {
                        #[link_name = "exit"]
                        fn wit_import(_: i32);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        unreachable!()
                    }
                    wit_import(result0);
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_input {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// The input side of a terminal.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TerminalInput {
                handle: _rt::Resource<TerminalInput>,
            }
            impl TerminalInput {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TerminalInput {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:cli/terminal-input@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]terminal-input"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_output {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// The output side of a terminal.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TerminalOutput {
                handle: _rt::Resource<TerminalOutput>,
            }
            impl TerminalOutput {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TerminalOutput {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:cli/terminal-output@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]terminal-output"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_stdin {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type TerminalInput = super::super::super::wasi::cli::terminal_input::TerminalInput;
            #[allow(unused_unsafe, clippy::all)]
            /// If stdin is connected to a terminal, return a `terminal-input` handle
            /// allowing further interaction with it.
            pub fn get_terminal_stdin() -> Option<TerminalInput> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/terminal-stdin@0.2.2")]
                    extern "C" {
                        #[link_name = "get-terminal-stdin"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::cli::terminal_input::TerminalInput::from_handle(
                                    l2 as u32,
                                )
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_stdout {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type TerminalOutput = super::super::super::wasi::cli::terminal_output::TerminalOutput;
            #[allow(unused_unsafe, clippy::all)]
            /// If stdout is connected to a terminal, return a `terminal-output` handle
            /// allowing further interaction with it.
            pub fn get_terminal_stdout() -> Option<TerminalOutput> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/terminal-stdout@0.2.2")]
                    extern "C" {
                        #[link_name = "get-terminal-stdout"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::cli::terminal_output::TerminalOutput::from_handle(
                                    l2 as u32,
                                )
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_stderr {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type TerminalOutput = super::super::super::wasi::cli::terminal_output::TerminalOutput;
            #[allow(unused_unsafe, clippy::all)]
            /// If stderr is connected to a terminal, return a `terminal-output` handle
            /// allowing further interaction with it.
            pub fn get_terminal_stderr() -> Option<TerminalOutput> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/terminal-stderr@0.2.2")]
                    extern "C" {
                        #[link_name = "get-terminal-stderr"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::cli::terminal_output::TerminalOutput::from_handle(
                                    l2 as u32,
                                )
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod clocks {
        #[allow(dead_code, clippy::all)]
        pub mod monotonic_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An instant in time, in nanoseconds. An instant is relative to an
            /// unspecified initial value, and can only be compared to instances from
            /// the same monotonic-clock.
            pub type Instant = u64;
            /// A duration of time, in nanoseconds.
            pub type Duration = u64;
            #[allow(unused_unsafe, clippy::all)]
            /// Read the current value of the clock.
            ///
            /// The clock is monotonic, therefore calling this function repeatedly will
            /// produce a sequence of non-decreasing values.
            pub fn now() -> Instant {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.2")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Query the resolution of the clock. Returns the duration of time
            /// corresponding to a clock tick.
            pub fn resolution() -> Duration {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.2")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Create a `pollable` which will resolve once the specified instant
            /// has occurred.
            pub fn subscribe_instant(when: Instant) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.2")]
                    extern "C" {
                        #[link_name = "subscribe-instant"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(
                        ret as u32,
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Create a `pollable` that will resolve after the specified duration has
            /// elapsed from the time this function is invoked.
            pub fn subscribe_duration(when: Duration) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.2")]
                    extern "C" {
                        #[link_name = "subscribe-duration"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod wall_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            /// A time and date in seconds plus nanoseconds.
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Datetime {
                pub seconds: u64,
                pub nanoseconds: u32,
            }
            impl ::core::fmt::Debug for Datetime {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Datetime")
                        .field("seconds", &self.seconds)
                        .field("nanoseconds", &self.nanoseconds)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Read the current value of the clock.
            ///
            /// This clock is not monotonic, therefore calling this function repeatedly
            /// will not necessarily produce a sequence of non-decreasing values.
            ///
            /// The returned timestamps represent the number of seconds since
            /// 1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch],
            /// also known as [Unix Time].
            ///
            /// The nanoseconds field of the output is always less than 1000000000.
            ///
            /// [POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
            /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time
            pub fn now() -> Datetime {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/wall-clock@0.2.2")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i32>();
                    Datetime {
                        seconds: l1 as u64,
                        nanoseconds: l2 as u32,
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Query the resolution of the clock.
            ///
            /// The nanoseconds field of the output is always less than 1000000000.
            pub fn resolution() -> Datetime {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/wall-clock@0.2.2")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i32>();
                    Datetime {
                        seconds: l1 as u64,
                        nanoseconds: l2 as u32,
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod filesystem {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Error = super::super::super::wasi::io::streams::Error;
            pub type Datetime = super::super::super::wasi::clocks::wall_clock::Datetime;
            /// File size or length of a region within a file.
            pub type Filesize = u64;
            /// The type of a filesystem object referenced by a descriptor.
            ///
            /// Note: This was called `filetype` in earlier versions of WASI.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum DescriptorType {
                /// The type of the descriptor or file is unknown or is different from
                /// any of the other types specified.
                Unknown,
                /// The descriptor refers to a block device inode.
                BlockDevice,
                /// The descriptor refers to a character device inode.
                CharacterDevice,
                /// The descriptor refers to a directory inode.
                Directory,
                /// The descriptor refers to a named pipe.
                Fifo,
                /// The file refers to a symbolic link inode.
                SymbolicLink,
                /// The descriptor refers to a regular file inode.
                RegularFile,
                /// The descriptor refers to a socket.
                Socket,
            }
            impl ::core::fmt::Debug for DescriptorType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        DescriptorType::Unknown => {
                            f.debug_tuple("DescriptorType::Unknown").finish()
                        }
                        DescriptorType::BlockDevice => {
                            f.debug_tuple("DescriptorType::BlockDevice").finish()
                        }
                        DescriptorType::CharacterDevice => {
                            f.debug_tuple("DescriptorType::CharacterDevice").finish()
                        }
                        DescriptorType::Directory => {
                            f.debug_tuple("DescriptorType::Directory").finish()
                        }
                        DescriptorType::Fifo => {
                            f.debug_tuple("DescriptorType::Fifo").finish()
                        }
                        DescriptorType::SymbolicLink => {
                            f.debug_tuple("DescriptorType::SymbolicLink").finish()
                        }
                        DescriptorType::RegularFile => {
                            f.debug_tuple("DescriptorType::RegularFile").finish()
                        }
                        DescriptorType::Socket => {
                            f.debug_tuple("DescriptorType::Socket").finish()
                        }
                    }
                }
            }
            impl DescriptorType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> DescriptorType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => DescriptorType::Unknown,
                        1 => DescriptorType::BlockDevice,
                        2 => DescriptorType::CharacterDevice,
                        3 => DescriptorType::Directory,
                        4 => DescriptorType::Fifo,
                        5 => DescriptorType::SymbolicLink,
                        6 => DescriptorType::RegularFile,
                        7 => DescriptorType::Socket,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[doc = " Descriptor flags."] #[doc = ""] #[doc =
                " Note: This was called `fdflags` in earlier versions of WASI."]
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct DescriptorFlags : u8 { #[doc = " Read mode: Data can be read."]
                const READ = 1 << 0; #[doc = " Write mode: Data can be written to."]
                const WRITE = 1 << 1; #[doc =
                " Request that writes be performed according to synchronized I/O file"]
                #[doc =
                " integrity completion. The data stored in the file and the file's"]
                #[doc =
                " metadata are synchronized. This is similar to `O_SYNC` in POSIX."]
                #[doc = ""] #[doc =
                " The precise semantics of this operation have not yet been defined for"]
                #[doc =
                " WASI. At this time, it should be interpreted as a request, and not a"]
                #[doc = " requirement."] const FILE_INTEGRITY_SYNC = 1 << 2; #[doc =
                " Request that writes be performed according to synchronized I/O data"]
                #[doc = " integrity completion. Only the data stored in the file is"]
                #[doc = " synchronized. This is similar to `O_DSYNC` in POSIX."] #[doc =
                ""] #[doc =
                " The precise semantics of this operation have not yet been defined for"]
                #[doc =
                " WASI. At this time, it should be interpreted as a request, and not a"]
                #[doc = " requirement."] const DATA_INTEGRITY_SYNC = 1 << 3; #[doc =
                " Requests that reads be performed at the same level of integrity"] #[doc
                = " requested for writes. This is similar to `O_RSYNC` in POSIX."] #[doc
                = ""] #[doc =
                " The precise semantics of this operation have not yet been defined for"]
                #[doc =
                " WASI. At this time, it should be interpreted as a request, and not a"]
                #[doc = " requirement."] const REQUESTED_WRITE_SYNC = 1 << 4; #[doc =
                " Mutating directories mode: Directory contents may be mutated."] #[doc =
                ""] #[doc =
                " When this flag is unset on a descriptor, operations using the"] #[doc =
                " descriptor which would create, rename, delete, modify the data or"]
                #[doc =
                " metadata of filesystem objects, or obtain another handle which"] #[doc
                =
                " would permit any of those, shall fail with `error-code::read-only` if"]
                #[doc = " they would otherwise succeed."] #[doc = ""] #[doc =
                " This may only be set on directories."] const MUTATE_DIRECTORY = 1 << 5;
                }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[doc = " Flags determining the method of how paths are resolved."]
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct PathFlags : u8 { #[doc =
                " As long as the resolved path corresponds to a symbolic link, it is"]
                #[doc = " expanded."] const SYMLINK_FOLLOW = 1 << 0; }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[doc = " Open flags used by `open-at`."] #[derive(PartialEq, Eq,
                PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub struct OpenFlags : u8 {
                #[doc =
                " Create file if it does not exist, similar to `O_CREAT` in POSIX."]
                const CREATE = 1 << 0; #[doc =
                " Fail if not a directory, similar to `O_DIRECTORY` in POSIX."] const
                DIRECTORY = 1 << 1; #[doc =
                " Fail if file already exists, similar to `O_EXCL` in POSIX."] const
                EXCLUSIVE = 1 << 2; #[doc =
                " Truncate file to size 0, similar to `O_TRUNC` in POSIX."] const
                TRUNCATE = 1 << 3; }
            }
            /// Number of hard links to an inode.
            pub type LinkCount = u64;
            /// File attributes.
            ///
            /// Note: This was called `filestat` in earlier versions of WASI.
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct DescriptorStat {
                /// File type.
                pub type_: DescriptorType,
                /// Number of hard links to the file.
                pub link_count: LinkCount,
                /// For regular files, the file size in bytes. For symbolic links, the
                /// length in bytes of the pathname contained in the symbolic link.
                pub size: Filesize,
                /// Last data access timestamp.
                ///
                /// If the `option` is none, the platform doesn't maintain an access
                /// timestamp for this file.
                pub data_access_timestamp: Option<Datetime>,
                /// Last data modification timestamp.
                ///
                /// If the `option` is none, the platform doesn't maintain a
                /// modification timestamp for this file.
                pub data_modification_timestamp: Option<Datetime>,
                /// Last file status-change timestamp.
                ///
                /// If the `option` is none, the platform doesn't maintain a
                /// status-change timestamp for this file.
                pub status_change_timestamp: Option<Datetime>,
            }
            impl ::core::fmt::Debug for DescriptorStat {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DescriptorStat")
                        .field("type", &self.type_)
                        .field("link-count", &self.link_count)
                        .field("size", &self.size)
                        .field("data-access-timestamp", &self.data_access_timestamp)
                        .field(
                            "data-modification-timestamp",
                            &self.data_modification_timestamp,
                        )
                        .field("status-change-timestamp", &self.status_change_timestamp)
                        .finish()
                }
            }
            /// When setting a timestamp, this gives the value to set it to.
            #[derive(Clone, Copy)]
            pub enum NewTimestamp {
                /// Leave the timestamp set to its previous value.
                NoChange,
                /// Set the timestamp to the current time of the system clock associated
                /// with the filesystem.
                Now,
                /// Set the timestamp to the given value.
                Timestamp(Datetime),
            }
            impl ::core::fmt::Debug for NewTimestamp {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        NewTimestamp::NoChange => {
                            f.debug_tuple("NewTimestamp::NoChange").finish()
                        }
                        NewTimestamp::Now => f.debug_tuple("NewTimestamp::Now").finish(),
                        NewTimestamp::Timestamp(e) => {
                            f.debug_tuple("NewTimestamp::Timestamp").field(e).finish()
                        }
                    }
                }
            }
            /// A directory entry.
            #[derive(Clone)]
            pub struct DirectoryEntry {
                /// The type of the file referred to by this directory entry.
                pub type_: DescriptorType,
                /// The name of the object.
                pub name: _rt::String,
            }
            impl ::core::fmt::Debug for DirectoryEntry {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DirectoryEntry")
                        .field("type", &self.type_)
                        .field("name", &self.name)
                        .finish()
                }
            }
            /// Error codes returned by functions, similar to `errno` in POSIX.
            /// Not all of these error codes are returned by the functions provided by this
            /// API; some are used in higher-level library layers, and others are provided
            /// merely for alignment with POSIX.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ErrorCode {
                /// Permission denied, similar to `EACCES` in POSIX.
                Access,
                /// Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.
                WouldBlock,
                /// Connection already in progress, similar to `EALREADY` in POSIX.
                Already,
                /// Bad descriptor, similar to `EBADF` in POSIX.
                BadDescriptor,
                /// Device or resource busy, similar to `EBUSY` in POSIX.
                Busy,
                /// Resource deadlock would occur, similar to `EDEADLK` in POSIX.
                Deadlock,
                /// Storage quota exceeded, similar to `EDQUOT` in POSIX.
                Quota,
                /// File exists, similar to `EEXIST` in POSIX.
                Exist,
                /// File too large, similar to `EFBIG` in POSIX.
                FileTooLarge,
                /// Illegal byte sequence, similar to `EILSEQ` in POSIX.
                IllegalByteSequence,
                /// Operation in progress, similar to `EINPROGRESS` in POSIX.
                InProgress,
                /// Interrupted function, similar to `EINTR` in POSIX.
                Interrupted,
                /// Invalid argument, similar to `EINVAL` in POSIX.
                Invalid,
                /// I/O error, similar to `EIO` in POSIX.
                Io,
                /// Is a directory, similar to `EISDIR` in POSIX.
                IsDirectory,
                /// Too many levels of symbolic links, similar to `ELOOP` in POSIX.
                Loop,
                /// Too many links, similar to `EMLINK` in POSIX.
                TooManyLinks,
                /// Message too large, similar to `EMSGSIZE` in POSIX.
                MessageSize,
                /// Filename too long, similar to `ENAMETOOLONG` in POSIX.
                NameTooLong,
                /// No such device, similar to `ENODEV` in POSIX.
                NoDevice,
                /// No such file or directory, similar to `ENOENT` in POSIX.
                NoEntry,
                /// No locks available, similar to `ENOLCK` in POSIX.
                NoLock,
                /// Not enough space, similar to `ENOMEM` in POSIX.
                InsufficientMemory,
                /// No space left on device, similar to `ENOSPC` in POSIX.
                InsufficientSpace,
                /// Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.
                NotDirectory,
                /// Directory not empty, similar to `ENOTEMPTY` in POSIX.
                NotEmpty,
                /// State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.
                NotRecoverable,
                /// Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.
                Unsupported,
                /// Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.
                NoTty,
                /// No such device or address, similar to `ENXIO` in POSIX.
                NoSuchDevice,
                /// Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.
                Overflow,
                /// Operation not permitted, similar to `EPERM` in POSIX.
                NotPermitted,
                /// Broken pipe, similar to `EPIPE` in POSIX.
                Pipe,
                /// Read-only file system, similar to `EROFS` in POSIX.
                ReadOnly,
                /// Invalid seek, similar to `ESPIPE` in POSIX.
                InvalidSeek,
                /// Text file busy, similar to `ETXTBSY` in POSIX.
                TextFileBusy,
                /// Cross-device link, similar to `EXDEV` in POSIX.
                CrossDevice,
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => "access",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::Already => "already",
                        ErrorCode::BadDescriptor => "bad-descriptor",
                        ErrorCode::Busy => "busy",
                        ErrorCode::Deadlock => "deadlock",
                        ErrorCode::Quota => "quota",
                        ErrorCode::Exist => "exist",
                        ErrorCode::FileTooLarge => "file-too-large",
                        ErrorCode::IllegalByteSequence => "illegal-byte-sequence",
                        ErrorCode::InProgress => "in-progress",
                        ErrorCode::Interrupted => "interrupted",
                        ErrorCode::Invalid => "invalid",
                        ErrorCode::Io => "io",
                        ErrorCode::IsDirectory => "is-directory",
                        ErrorCode::Loop => "loop",
                        ErrorCode::TooManyLinks => "too-many-links",
                        ErrorCode::MessageSize => "message-size",
                        ErrorCode::NameTooLong => "name-too-long",
                        ErrorCode::NoDevice => "no-device",
                        ErrorCode::NoEntry => "no-entry",
                        ErrorCode::NoLock => "no-lock",
                        ErrorCode::InsufficientMemory => "insufficient-memory",
                        ErrorCode::InsufficientSpace => "insufficient-space",
                        ErrorCode::NotDirectory => "not-directory",
                        ErrorCode::NotEmpty => "not-empty",
                        ErrorCode::NotRecoverable => "not-recoverable",
                        ErrorCode::Unsupported => "unsupported",
                        ErrorCode::NoTty => "no-tty",
                        ErrorCode::NoSuchDevice => "no-such-device",
                        ErrorCode::Overflow => "overflow",
                        ErrorCode::NotPermitted => "not-permitted",
                        ErrorCode::Pipe => "pipe",
                        ErrorCode::ReadOnly => "read-only",
                        ErrorCode::InvalidSeek => "invalid-seek",
                        ErrorCode::TextFileBusy => "text-file-busy",
                        ErrorCode::CrossDevice => "cross-device",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => {
                            "Permission denied, similar to `EACCES` in POSIX."
                        }
                        ErrorCode::WouldBlock => {
                            "Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX."
                        }
                        ErrorCode::Already => {
                            "Connection already in progress, similar to `EALREADY` in POSIX."
                        }
                        ErrorCode::BadDescriptor => {
                            "Bad descriptor, similar to `EBADF` in POSIX."
                        }
                        ErrorCode::Busy => {
                            "Device or resource busy, similar to `EBUSY` in POSIX."
                        }
                        ErrorCode::Deadlock => {
                            "Resource deadlock would occur, similar to `EDEADLK` in POSIX."
                        }
                        ErrorCode::Quota => {
                            "Storage quota exceeded, similar to `EDQUOT` in POSIX."
                        }
                        ErrorCode::Exist => "File exists, similar to `EEXIST` in POSIX.",
                        ErrorCode::FileTooLarge => {
                            "File too large, similar to `EFBIG` in POSIX."
                        }
                        ErrorCode::IllegalByteSequence => {
                            "Illegal byte sequence, similar to `EILSEQ` in POSIX."
                        }
                        ErrorCode::InProgress => {
                            "Operation in progress, similar to `EINPROGRESS` in POSIX."
                        }
                        ErrorCode::Interrupted => {
                            "Interrupted function, similar to `EINTR` in POSIX."
                        }
                        ErrorCode::Invalid => {
                            "Invalid argument, similar to `EINVAL` in POSIX."
                        }
                        ErrorCode::Io => "I/O error, similar to `EIO` in POSIX.",
                        ErrorCode::IsDirectory => {
                            "Is a directory, similar to `EISDIR` in POSIX."
                        }
                        ErrorCode::Loop => {
                            "Too many levels of symbolic links, similar to `ELOOP` in POSIX."
                        }
                        ErrorCode::TooManyLinks => {
                            "Too many links, similar to `EMLINK` in POSIX."
                        }
                        ErrorCode::MessageSize => {
                            "Message too large, similar to `EMSGSIZE` in POSIX."
                        }
                        ErrorCode::NameTooLong => {
                            "Filename too long, similar to `ENAMETOOLONG` in POSIX."
                        }
                        ErrorCode::NoDevice => {
                            "No such device, similar to `ENODEV` in POSIX."
                        }
                        ErrorCode::NoEntry => {
                            "No such file or directory, similar to `ENOENT` in POSIX."
                        }
                        ErrorCode::NoLock => {
                            "No locks available, similar to `ENOLCK` in POSIX."
                        }
                        ErrorCode::InsufficientMemory => {
                            "Not enough space, similar to `ENOMEM` in POSIX."
                        }
                        ErrorCode::InsufficientSpace => {
                            "No space left on device, similar to `ENOSPC` in POSIX."
                        }
                        ErrorCode::NotDirectory => {
                            "Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX."
                        }
                        ErrorCode::NotEmpty => {
                            "Directory not empty, similar to `ENOTEMPTY` in POSIX."
                        }
                        ErrorCode::NotRecoverable => {
                            "State not recoverable, similar to `ENOTRECOVERABLE` in POSIX."
                        }
                        ErrorCode::Unsupported => {
                            "Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX."
                        }
                        ErrorCode::NoTty => {
                            "Inappropriate I/O control operation, similar to `ENOTTY` in POSIX."
                        }
                        ErrorCode::NoSuchDevice => {
                            "No such device or address, similar to `ENXIO` in POSIX."
                        }
                        ErrorCode::Overflow => {
                            "Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX."
                        }
                        ErrorCode::NotPermitted => {
                            "Operation not permitted, similar to `EPERM` in POSIX."
                        }
                        ErrorCode::Pipe => "Broken pipe, similar to `EPIPE` in POSIX.",
                        ErrorCode::ReadOnly => {
                            "Read-only file system, similar to `EROFS` in POSIX."
                        }
                        ErrorCode::InvalidSeek => {
                            "Invalid seek, similar to `ESPIPE` in POSIX."
                        }
                        ErrorCode::TextFileBusy => {
                            "Text file busy, similar to `ETXTBSY` in POSIX."
                        }
                        ErrorCode::CrossDevice => {
                            "Cross-device link, similar to `EXDEV` in POSIX."
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), * self as i32)
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Access,
                        1 => ErrorCode::WouldBlock,
                        2 => ErrorCode::Already,
                        3 => ErrorCode::BadDescriptor,
                        4 => ErrorCode::Busy,
                        5 => ErrorCode::Deadlock,
                        6 => ErrorCode::Quota,
                        7 => ErrorCode::Exist,
                        8 => ErrorCode::FileTooLarge,
                        9 => ErrorCode::IllegalByteSequence,
                        10 => ErrorCode::InProgress,
                        11 => ErrorCode::Interrupted,
                        12 => ErrorCode::Invalid,
                        13 => ErrorCode::Io,
                        14 => ErrorCode::IsDirectory,
                        15 => ErrorCode::Loop,
                        16 => ErrorCode::TooManyLinks,
                        17 => ErrorCode::MessageSize,
                        18 => ErrorCode::NameTooLong,
                        19 => ErrorCode::NoDevice,
                        20 => ErrorCode::NoEntry,
                        21 => ErrorCode::NoLock,
                        22 => ErrorCode::InsufficientMemory,
                        23 => ErrorCode::InsufficientSpace,
                        24 => ErrorCode::NotDirectory,
                        25 => ErrorCode::NotEmpty,
                        26 => ErrorCode::NotRecoverable,
                        27 => ErrorCode::Unsupported,
                        28 => ErrorCode::NoTty,
                        29 => ErrorCode::NoSuchDevice,
                        30 => ErrorCode::Overflow,
                        31 => ErrorCode::NotPermitted,
                        32 => ErrorCode::Pipe,
                        33 => ErrorCode::ReadOnly,
                        34 => ErrorCode::InvalidSeek,
                        35 => ErrorCode::TextFileBusy,
                        36 => ErrorCode::CrossDevice,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            /// File or memory access pattern advisory information.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum Advice {
                /// The application has no advice to give on its behavior with respect
                /// to the specified data.
                Normal,
                /// The application expects to access the specified data sequentially
                /// from lower offsets to higher offsets.
                Sequential,
                /// The application expects to access the specified data in a random
                /// order.
                Random,
                /// The application expects to access the specified data in the near
                /// future.
                WillNeed,
                /// The application expects that it will not access the specified data
                /// in the near future.
                DontNeed,
                /// The application expects to access the specified data once and then
                /// not reuse it thereafter.
                NoReuse,
            }
            impl ::core::fmt::Debug for Advice {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Advice::Normal => f.debug_tuple("Advice::Normal").finish(),
                        Advice::Sequential => {
                            f.debug_tuple("Advice::Sequential").finish()
                        }
                        Advice::Random => f.debug_tuple("Advice::Random").finish(),
                        Advice::WillNeed => f.debug_tuple("Advice::WillNeed").finish(),
                        Advice::DontNeed => f.debug_tuple("Advice::DontNeed").finish(),
                        Advice::NoReuse => f.debug_tuple("Advice::NoReuse").finish(),
                    }
                }
            }
            impl Advice {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Advice {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Advice::Normal,
                        1 => Advice::Sequential,
                        2 => Advice::Random,
                        3 => Advice::WillNeed,
                        4 => Advice::DontNeed,
                        5 => Advice::NoReuse,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            /// A 128-bit hash value, split into parts because wasm doesn't have a
            /// 128-bit integer type.
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct MetadataHashValue {
                /// 64 bits of a 128-bit hash value.
                pub lower: u64,
                /// Another 64 bits of a 128-bit hash value.
                pub upper: u64,
            }
            impl ::core::fmt::Debug for MetadataHashValue {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("MetadataHashValue")
                        .field("lower", &self.lower)
                        .field("upper", &self.upper)
                        .finish()
                }
            }
            /// A descriptor is a reference to a filesystem object, which may be a file,
            /// directory, named pipe, special file, or other object on which filesystem
            /// calls may be made.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Descriptor {
                handle: _rt::Resource<Descriptor>,
            }
            impl Descriptor {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Descriptor {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]descriptor"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// A stream of directory entries.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct DirectoryEntryStream {
                handle: _rt::Resource<DirectoryEntryStream>,
            }
            impl DirectoryEntryStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for DirectoryEntryStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]directory-entry-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a stream for reading from a file, if available.
                ///
                /// May fail with an error-code describing why the file cannot be read.
                ///
                /// Multiple read, write, and append streams may be active on the same open
                /// file and they do not interfere with each other.
                ///
                /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.
                pub fn read_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<InputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read-via-stream"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(offset), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::InputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a stream for writing to a file, if available.
                ///
                /// May fail with an error-code describing why the file cannot be written.
                ///
                /// Note: This allows using `write-stream`, which is similar to `write` in
                /// POSIX.
                pub fn write_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<OutputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.write-via-stream"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(offset), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a stream for appending to a file, if available.
                ///
                /// May fail with an error-code describing why the file cannot be appended.
                ///
                /// Note: This allows using `write-stream`, which is similar to `write` with
                /// `O_APPEND` in in POSIX.
                pub fn append_via_stream(&self) -> Result<OutputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.append-via-stream"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Provide file advisory information on a descriptor.
                ///
                /// This is similar to `posix_fadvise` in POSIX.
                pub fn advise(
                    &self,
                    offset: Filesize,
                    length: Filesize,
                    advice: Advice,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.advise"]
                            fn wit_import(_: i32, _: i64, _: i64, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(offset),
                            _rt::as_i64(length),
                            advice.clone() as i32,
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Synchronize the data of a file to disk.
                ///
                /// This function succeeds with no effect if the file descriptor is not
                /// opened for writing.
                ///
                /// Note: This is similar to `fdatasync` in POSIX.
                pub fn sync_data(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.sync-data"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Get flags associated with a descriptor.
                ///
                /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.
                ///
                /// Note: This returns the value that was the `fs_flags` value returned
                /// from `fdstat_get` in earlier versions of WASI.
                pub fn get_flags(&self) -> Result<DescriptorFlags, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.get-flags"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    DescriptorFlags::empty()
                                        | DescriptorFlags::from_bits_retain(((l2 as u8) << 0) as _)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the dynamic type of a descriptor.
                ///
                /// Note: This returns the same value as the `type` field of the `fd-stat`
                /// returned by `stat`, `stat-at` and similar.
                ///
                /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided
                /// by `fstat` in POSIX.
                ///
                /// Note: This returns the value that was the `fs_filetype` value returned
                /// from `fdstat_get` in earlier versions of WASI.
                pub fn get_type(&self) -> Result<DescriptorType, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.get-type"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    DescriptorType::_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Adjust the size of an open file. If this increases the file's size, the
                /// extra bytes are filled with zeros.
                ///
                /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
                pub fn set_size(&self, size: Filesize) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(size), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Adjust the timestamps of an open file or directory.
                ///
                /// Note: This is similar to `futimens` in POSIX.
                ///
                /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
                pub fn set_times(
                    &self,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let (result1_0, result1_1, result1_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds0,
                                    nanoseconds: nanoseconds0,
                                } = e;
                                (2i32, _rt::as_i64(seconds0), _rt::as_i32(nanoseconds0))
                            }
                        };
                        let (result3_0, result3_1, result3_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (2i32, _rt::as_i64(seconds2), _rt::as_i32(nanoseconds2))
                            }
                        };
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-times"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                            result3_0,
                            result3_1,
                            result3_2,
                            ptr4,
                        );
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        match l5 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr4.add(1).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from a descriptor, without using and updating the descriptor's offset.
                ///
                /// This function returns a list of bytes containing the data that was
                /// read, along with a bool which, when true, indicates that the end of the
                /// file was reached. The returned list will contain up to `length` bytes; it
                /// may return fewer than requested, if the end of the file is reached or
                /// if the I/O operation is interrupted.
                ///
                /// In the future, this may change to return a `stream<u8, error-code>`.
                ///
                /// Note: This is similar to `pread` in POSIX.
                pub fn read(
                    &self,
                    length: Filesize,
                    offset: Filesize,
                ) -> Result<(_rt::Vec<u8>, bool), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read"]
                            fn wit_import(_: i32, _: i64, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(length),
                            _rt::as_i64(offset),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let l5 = i32::from(*ptr0.add(12).cast::<u8>());
                                    (
                                        _rt::Vec::from_raw_parts(l2.cast(), len4, len4),
                                        _rt::bool_lift(l5 as u8),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Write to a descriptor, without using and updating the descriptor's offset.
                ///
                /// It is valid to write past the end of a file; the file is extended to the
                /// extent of the write, with bytes between the previous end and the start of
                /// the write set to zero.
                ///
                /// In the future, this may change to take a `stream<u8, error-code>`.
                ///
                /// Note: This is similar to `pwrite` in POSIX.
                pub fn write(
                    &self,
                    buffer: &[u8],
                    offset: Filesize,
                ) -> Result<Filesize, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = buffer;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.write"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i64,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            _rt::as_i64(offset),
                            ptr1,
                        );
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(8).cast::<i64>();
                                    l3 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr1.add(8).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Read directory entries from a directory.
                ///
                /// On filesystems where directories contain entries referring to themselves
                /// and their parents, often named `.` and `..` respectively, these entries
                /// are omitted.
                ///
                /// This always returns a new stream which starts at the beginning of the
                /// directory. Multiple streams may be active on the same directory, and they
                /// do not interfere with each other.
                pub fn read_directory(&self) -> Result<DirectoryEntryStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read-directory"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    DirectoryEntryStream::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Synchronize the data and metadata of a file to disk.
                ///
                /// This function succeeds with no effect if the file descriptor is not
                /// opened for writing.
                ///
                /// Note: This is similar to `fsync` in POSIX.
                pub fn sync(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.sync"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a directory.
                ///
                /// Note: This is similar to `mkdirat` in POSIX.
                pub fn create_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.create-directory-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the attributes of an open file or directory.
                ///
                /// Note: This is similar to `fstat` in POSIX, except that it does not return
                /// device and inode information. For testing whether two descriptors refer to
                /// the same underlying filesystem object, use `is-same-object`. To obtain
                /// additional data that can be used do determine whether a file has been
                /// modified, use `metadata-hash`.
                ///
                /// Note: This was called `fd_filestat_get` in earlier versions of WASI.
                pub fn stat(&self) -> Result<DescriptorStat, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 104]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 104],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.stat"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let l3 = *ptr0.add(16).cast::<i64>();
                                    let l4 = *ptr0.add(24).cast::<i64>();
                                    let l5 = i32::from(*ptr0.add(32).cast::<u8>());
                                    let l8 = i32::from(*ptr0.add(56).cast::<u8>());
                                    let l11 = i32::from(*ptr0.add(80).cast::<u8>());
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l2 as u8),
                                        link_count: l3 as u64,
                                        size: l4 as u64,
                                        data_access_timestamp: match l5 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l6 = *ptr0.add(40).cast::<i64>();
                                                    let l7 = *ptr0.add(48).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l6 as u64,
                                                        nanoseconds: l7 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l8 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l9 = *ptr0.add(64).cast::<i64>();
                                                    let l10 = *ptr0.add(72).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l9 as u64,
                                                        nanoseconds: l10 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l11 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l12 = *ptr0.add(88).cast::<i64>();
                                                    let l13 = *ptr0.add(96).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l12 as u64,
                                                        nanoseconds: l13 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l14 = i32::from(*ptr0.add(8).cast::<u8>());
                                    ErrorCode::_lift(l14 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the attributes of a file or directory.
                ///
                /// Note: This is similar to `fstatat` in POSIX, except that it does not
                /// return device and inode information. See the `stat` description for a
                /// discussion of alternatives.
                ///
                /// Note: This was called `path_filestat_get` in earlier versions of WASI.
                pub fn stat_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<DescriptorStat, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 104]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 104],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.stat-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(8).cast::<u8>());
                                    let l5 = *ptr2.add(16).cast::<i64>();
                                    let l6 = *ptr2.add(24).cast::<i64>();
                                    let l7 = i32::from(*ptr2.add(32).cast::<u8>());
                                    let l10 = i32::from(*ptr2.add(56).cast::<u8>());
                                    let l13 = i32::from(*ptr2.add(80).cast::<u8>());
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l4 as u8),
                                        link_count: l5 as u64,
                                        size: l6 as u64,
                                        data_access_timestamp: match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr2.add(40).cast::<i64>();
                                                    let l9 = *ptr2.add(48).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l8 as u64,
                                                        nanoseconds: l9 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l10 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l11 = *ptr2.add(64).cast::<i64>();
                                                    let l12 = *ptr2.add(72).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l11 as u64,
                                                        nanoseconds: l12 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l13 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l14 = *ptr2.add(88).cast::<i64>();
                                                    let l15 = *ptr2.add(96).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l14 as u64,
                                                        nanoseconds: l15 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l16 = i32::from(*ptr2.add(8).cast::<u8>());
                                    ErrorCode::_lift(l16 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Adjust the timestamps of a file or directory.
                ///
                /// Note: This is similar to `utimensat` in POSIX.
                ///
                /// Note: This was called `path_filestat_set_times` in earlier versions of
                /// WASI.
                pub fn set_times_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let (result3_0, result3_1, result3_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (2i32, _rt::as_i64(seconds2), _rt::as_i32(nanoseconds2))
                            }
                        };
                        let (result5_0, result5_1, result5_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds4,
                                    nanoseconds: nanoseconds4,
                                } = e;
                                (2i32, _rt::as_i64(seconds4), _rt::as_i32(nanoseconds4))
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-times-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            result3_0,
                            result3_1,
                            result3_2,
                            result5_0,
                            result5_1,
                            result5_2,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    ErrorCode::_lift(l8 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a hard link.
                ///
                /// Note: This is similar to `linkat` in POSIX.
                pub fn link_at(
                    &self,
                    old_path_flags: PathFlags,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let flags0 = old_path_flags;
                        let vec1 = old_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let vec2 = new_path;
                        let ptr2 = vec2.as_ptr().cast::<u8>();
                        let len2 = vec2.len();
                        let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.link-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            (new_descriptor).handle() as i32,
                            ptr2.cast_mut(),
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*ptr3.add(0).cast::<u8>());
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr3.add(1).cast::<u8>());
                                    ErrorCode::_lift(l5 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Open a file or directory.
                ///
                /// If `flags` contains `descriptor-flags::mutate-directory`, and the base
                /// descriptor doesn't have `descriptor-flags::mutate-directory` set,
                /// `open-at` fails with `error-code::read-only`.
                ///
                /// If `flags` contains `write` or `mutate-directory`, or `open-flags`
                /// contains `truncate` or `create`, and the base descriptor doesn't have
                /// `descriptor-flags::mutate-directory` set, `open-at` fails with
                /// `error-code::read-only`.
                ///
                /// Note: This is similar to `openat` in POSIX.
                pub fn open_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    open_flags: OpenFlags,
                    flags: DescriptorFlags,
                ) -> Result<Descriptor, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let flags2 = open_flags;
                        let flags3 = flags;
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.open-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            (flags2.bits() >> 0) as i32,
                            (flags3.bits() >> 0) as i32,
                            ptr4,
                        );
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        match l5 {
                            0 => {
                                let e = {
                                    let l6 = *ptr4.add(4).cast::<i32>();
                                    Descriptor::from_handle(l6 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr4.add(4).cast::<u8>());
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Read the contents of a symbolic link.
                ///
                /// If the contents contain an absolute or rooted path in the underlying
                /// filesystem, this function fails with `error-code::not-permitted`.
                ///
                /// Note: This is similar to `readlinkat` in POSIX.
                pub fn readlink_at(&self, path: &str) -> Result<_rt::String, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.readlink-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<*mut u8>();
                                    let l4 = *ptr1.add(8).cast::<usize>();
                                    let len5 = l4;
                                    let bytes5 = _rt::Vec::from_raw_parts(
                                        l3.cast(),
                                        len5,
                                        len5,
                                    );
                                    _rt::string_lift(bytes5)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr1.add(4).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Remove a directory.
                ///
                /// Return `error-code::not-empty` if the directory is not empty.
                ///
                /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
                pub fn remove_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.remove-directory-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Rename a filesystem object.
                ///
                /// Note: This is similar to `renameat` in POSIX.
                pub fn rename_at(
                    &self,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.rename-at"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            (new_descriptor).handle() as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a symbolic link (also known as a "symlink").
                ///
                /// If `old-path` starts with `/`, the function fails with
                /// `error-code::not-permitted`.
                ///
                /// Note: This is similar to `symlinkat` in POSIX.
                pub fn symlink_at(
                    &self,
                    old_path: &str,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.symlink-at"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Unlink a filesystem object that is not a directory.
                ///
                /// Return `error-code::is-directory` if the path refers to a directory.
                /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
                pub fn unlink_file_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.unlink-file-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Test whether two descriptors refer to the same filesystem object.
                ///
                /// In POSIX, this corresponds to testing whether the two descriptors have the
                /// same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers.
                /// wasi-filesystem does not expose device and inode numbers, so this function
                /// may be used instead.
                pub fn is_same_object(&self, other: &Descriptor) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.is-same-object"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            (other).handle() as i32,
                        );
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a hash of the metadata associated with a filesystem object referred
                /// to by a descriptor.
                ///
                /// This returns a hash of the last-modification timestamp and file size, and
                /// may also include the inode number, device number, birth timestamp, and
                /// other metadata fields that may change when the file is modified or
                /// replaced. It may also include a secret value chosen by the
                /// implementation and not otherwise exposed.
                ///
                /// Implementations are encourated to provide the following properties:
                ///
                /// - If the file is not modified or replaced, the computed hash value should
                /// usually not change.
                /// - If the object is modified or replaced, the computed hash value should
                /// usually change.
                /// - The inputs to the hash should not be easily computable from the
                /// computed hash.
                ///
                /// However, none of these is required.
                pub fn metadata_hash(&self) -> Result<MetadataHashValue, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 24],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.metadata-hash"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    let l3 = *ptr0.add(16).cast::<i64>();
                                    MetadataHashValue {
                                        lower: l2 as u64,
                                        upper: l3 as u64,
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                /// Return a hash of the metadata associated with a filesystem object referred
                /// to by a directory descriptor and a relative path.
                ///
                /// This performs the same hash computation as `metadata-hash`.
                pub fn metadata_hash_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<MetadataHashValue, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 24],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]descriptor.metadata-hash-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = *ptr2.add(8).cast::<i64>();
                                    let l5 = *ptr2.add(16).cast::<i64>();
                                    MetadataHashValue {
                                        lower: l4 as u64,
                                        upper: l5 as u64,
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr2.add(8).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl DirectoryEntryStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read a single directory entry from a `directory-entry-stream`.
                pub fn read_directory_entry(
                    &self,
                ) -> Result<Option<DirectoryEntry>, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 20],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]directory-entry-stream.read-directory-entry"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                                let l4 = *ptr0.add(12).cast::<*mut u8>();
                                                let l5 = *ptr0.add(16).cast::<usize>();
                                                let len6 = l5;
                                                let bytes6 = _rt::Vec::from_raw_parts(
                                                    l4.cast(),
                                                    len6,
                                                    len6,
                                                );
                                                DirectoryEntry {
                                                    type_: DescriptorType::_lift(l3 as u8),
                                                    name: _rt::string_lift(bytes6),
                                                }
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Attempts to extract a filesystem-related `error-code` from the stream
            /// `error` provided.
            ///
            /// Stream operations which return `stream-error::last-operation-failed`
            /// have a payload with more information about the operation that failed.
            /// This payload can be passed through to this function to see if there's
            /// filesystem-related information about the error to return.
            ///
            /// Note that this function is fallible because not all stream-related
            /// errors are filesystem-related errors.
            pub fn filesystem_error_code(err: &Error) -> Option<ErrorCode> {
                unsafe {
                    #[repr(align(1))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:filesystem/types@0.2.2")]
                    extern "C" {
                        #[link_name = "filesystem-error-code"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((err).handle() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                ErrorCode::_lift(l2 as u8)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod preopens {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Descriptor = super::super::super::wasi::filesystem::types::Descriptor;
            #[allow(unused_unsafe, clippy::all)]
            /// Return the set of preopened directories, and their path.
            pub fn get_directories() -> _rt::Vec<(Descriptor, _rt::String)> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:filesystem/preopens@0.2.2")]
                    extern "C" {
                        #[link_name = "get-directories"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base7 = l1;
                    let len7 = l2;
                    let mut result7 = _rt::Vec::with_capacity(len7);
                    for i in 0..len7 {
                        let base = base7.add(i * 12);
                        let e7 = {
                            let l3 = *base.add(0).cast::<i32>();
                            let l4 = *base.add(4).cast::<*mut u8>();
                            let l5 = *base.add(8).cast::<usize>();
                            let len6 = l5;
                            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
                            (
                                super::super::super::wasi::filesystem::types::Descriptor::from_handle(
                                    l3 as u32,
                                ),
                                _rt::string_lift(bytes6),
                            )
                        };
                        result7.push(e7);
                    }
                    _rt::cabi_dealloc(base7, len7 * 12, 4);
                    result7
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod http {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type IoError = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// This type corresponds to HTTP standard Methods.
            #[derive(Clone)]
            pub enum Method {
                Get,
                Head,
                Post,
                Put,
                Delete,
                Connect,
                Options,
                Trace,
                Patch,
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Method {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Method::Get => f.debug_tuple("Method::Get").finish(),
                        Method::Head => f.debug_tuple("Method::Head").finish(),
                        Method::Post => f.debug_tuple("Method::Post").finish(),
                        Method::Put => f.debug_tuple("Method::Put").finish(),
                        Method::Delete => f.debug_tuple("Method::Delete").finish(),
                        Method::Connect => f.debug_tuple("Method::Connect").finish(),
                        Method::Options => f.debug_tuple("Method::Options").finish(),
                        Method::Trace => f.debug_tuple("Method::Trace").finish(),
                        Method::Patch => f.debug_tuple("Method::Patch").finish(),
                        Method::Other(e) => {
                            f.debug_tuple("Method::Other").field(e).finish()
                        }
                    }
                }
            }
            /// This type corresponds to HTTP standard Related Schemes.
            #[derive(Clone)]
            pub enum Scheme {
                Http,
                Https,
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Scheme {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Scheme::Http => f.debug_tuple("Scheme::Http").finish(),
                        Scheme::Https => f.debug_tuple("Scheme::Https").finish(),
                        Scheme::Other(e) => {
                            f.debug_tuple("Scheme::Other").field(e).finish()
                        }
                    }
                }
            }
            /// Defines the case payload type for `DNS-error` above:
            #[derive(Clone)]
            pub struct DnsErrorPayload {
                pub rcode: Option<_rt::String>,
                pub info_code: Option<u16>,
            }
            impl ::core::fmt::Debug for DnsErrorPayload {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DnsErrorPayload")
                        .field("rcode", &self.rcode)
                        .field("info-code", &self.info_code)
                        .finish()
                }
            }
            /// Defines the case payload type for `TLS-alert-received` above:
            #[derive(Clone)]
            pub struct TlsAlertReceivedPayload {
                pub alert_id: Option<u8>,
                pub alert_message: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for TlsAlertReceivedPayload {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("TlsAlertReceivedPayload")
                        .field("alert-id", &self.alert_id)
                        .field("alert-message", &self.alert_message)
                        .finish()
                }
            }
            /// Defines the case payload type for `HTTP-response-{header,trailer}-size` above:
            #[derive(Clone)]
            pub struct FieldSizePayload {
                pub field_name: Option<_rt::String>,
                pub field_size: Option<u32>,
            }
            impl ::core::fmt::Debug for FieldSizePayload {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("FieldSizePayload")
                        .field("field-name", &self.field_name)
                        .field("field-size", &self.field_size)
                        .finish()
                }
            }
            /// These cases are inspired by the IANA HTTP Proxy Error Types:
            /// https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types
            #[derive(Clone)]
            pub enum ErrorCode {
                DnsTimeout,
                DnsError(DnsErrorPayload),
                DestinationNotFound,
                DestinationUnavailable,
                DestinationIpProhibited,
                DestinationIpUnroutable,
                ConnectionRefused,
                ConnectionTerminated,
                ConnectionTimeout,
                ConnectionReadTimeout,
                ConnectionWriteTimeout,
                ConnectionLimitReached,
                TlsProtocolError,
                TlsCertificateError,
                TlsAlertReceived(TlsAlertReceivedPayload),
                HttpRequestDenied,
                HttpRequestLengthRequired,
                HttpRequestBodySize(Option<u64>),
                HttpRequestMethodInvalid,
                HttpRequestUriInvalid,
                HttpRequestUriTooLong,
                HttpRequestHeaderSectionSize(Option<u32>),
                HttpRequestHeaderSize(Option<FieldSizePayload>),
                HttpRequestTrailerSectionSize(Option<u32>),
                HttpRequestTrailerSize(FieldSizePayload),
                HttpResponseIncomplete,
                HttpResponseHeaderSectionSize(Option<u32>),
                HttpResponseHeaderSize(FieldSizePayload),
                HttpResponseBodySize(Option<u64>),
                HttpResponseTrailerSectionSize(Option<u32>),
                HttpResponseTrailerSize(FieldSizePayload),
                HttpResponseTransferCoding(Option<_rt::String>),
                HttpResponseContentCoding(Option<_rt::String>),
                HttpResponseTimeout,
                HttpUpgradeFailed,
                HttpProtocolError,
                LoopDetected,
                ConfigurationError,
                /// This is a catch-all error for anything that doesn't fit cleanly into a
                /// more specific case. It also includes an optional string for an
                /// unstructured description of the error. Users should not depend on the
                /// string for diagnosing errors, as it's not required to be consistent
                /// between implementations.
                InternalError(Option<_rt::String>),
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        ErrorCode::DnsTimeout => {
                            f.debug_tuple("ErrorCode::DnsTimeout").finish()
                        }
                        ErrorCode::DnsError(e) => {
                            f.debug_tuple("ErrorCode::DnsError").field(e).finish()
                        }
                        ErrorCode::DestinationNotFound => {
                            f.debug_tuple("ErrorCode::DestinationNotFound").finish()
                        }
                        ErrorCode::DestinationUnavailable => {
                            f.debug_tuple("ErrorCode::DestinationUnavailable").finish()
                        }
                        ErrorCode::DestinationIpProhibited => {
                            f.debug_tuple("ErrorCode::DestinationIpProhibited").finish()
                        }
                        ErrorCode::DestinationIpUnroutable => {
                            f.debug_tuple("ErrorCode::DestinationIpUnroutable").finish()
                        }
                        ErrorCode::ConnectionRefused => {
                            f.debug_tuple("ErrorCode::ConnectionRefused").finish()
                        }
                        ErrorCode::ConnectionTerminated => {
                            f.debug_tuple("ErrorCode::ConnectionTerminated").finish()
                        }
                        ErrorCode::ConnectionTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionTimeout").finish()
                        }
                        ErrorCode::ConnectionReadTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionReadTimeout").finish()
                        }
                        ErrorCode::ConnectionWriteTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionWriteTimeout").finish()
                        }
                        ErrorCode::ConnectionLimitReached => {
                            f.debug_tuple("ErrorCode::ConnectionLimitReached").finish()
                        }
                        ErrorCode::TlsProtocolError => {
                            f.debug_tuple("ErrorCode::TlsProtocolError").finish()
                        }
                        ErrorCode::TlsCertificateError => {
                            f.debug_tuple("ErrorCode::TlsCertificateError").finish()
                        }
                        ErrorCode::TlsAlertReceived(e) => {
                            f.debug_tuple("ErrorCode::TlsAlertReceived")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestDenied => {
                            f.debug_tuple("ErrorCode::HttpRequestDenied").finish()
                        }
                        ErrorCode::HttpRequestLengthRequired => {
                            f.debug_tuple("ErrorCode::HttpRequestLengthRequired")
                                .finish()
                        }
                        ErrorCode::HttpRequestBodySize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestBodySize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestMethodInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestMethodInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestUriInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriTooLong => {
                            f.debug_tuple("ErrorCode::HttpRequestUriTooLong").finish()
                        }
                        ErrorCode::HttpRequestHeaderSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestHeaderSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestHeaderSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestHeaderSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestTrailerSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestTrailerSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestTrailerSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestTrailerSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseIncomplete => {
                            f.debug_tuple("ErrorCode::HttpResponseIncomplete").finish()
                        }
                        ErrorCode::HttpResponseHeaderSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseHeaderSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseHeaderSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseHeaderSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseBodySize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseBodySize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTrailerSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTrailerSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTrailerSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTrailerSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTransferCoding(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTransferCoding")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseContentCoding(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseContentCoding")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTimeout => {
                            f.debug_tuple("ErrorCode::HttpResponseTimeout").finish()
                        }
                        ErrorCode::HttpUpgradeFailed => {
                            f.debug_tuple("ErrorCode::HttpUpgradeFailed").finish()
                        }
                        ErrorCode::HttpProtocolError => {
                            f.debug_tuple("ErrorCode::HttpProtocolError").finish()
                        }
                        ErrorCode::LoopDetected => {
                            f.debug_tuple("ErrorCode::LoopDetected").finish()
                        }
                        ErrorCode::ConfigurationError => {
                            f.debug_tuple("ErrorCode::ConfigurationError").finish()
                        }
                        ErrorCode::InternalError(e) => {
                            f.debug_tuple("ErrorCode::InternalError").field(e).finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for ErrorCode {}
            /// This type enumerates the different kinds of errors that may occur when
            /// setting or appending to a `fields` resource.
            #[derive(Clone, Copy)]
            pub enum HeaderError {
                /// This error indicates that a `field-name` or `field-value` was
                /// syntactically invalid when used with an operation that sets headers in a
                /// `fields`.
                InvalidSyntax,
                /// This error indicates that a forbidden `field-name` was used when trying
                /// to set a header in a `fields`.
                Forbidden,
                /// This error indicates that the operation on the `fields` was not
                /// permitted because the fields are immutable.
                Immutable,
            }
            impl ::core::fmt::Debug for HeaderError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        HeaderError::InvalidSyntax => {
                            f.debug_tuple("HeaderError::InvalidSyntax").finish()
                        }
                        HeaderError::Forbidden => {
                            f.debug_tuple("HeaderError::Forbidden").finish()
                        }
                        HeaderError::Immutable => {
                            f.debug_tuple("HeaderError::Immutable").finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for HeaderError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for HeaderError {}
            /// Field keys are always strings.
            ///
            /// Field keys should always be treated as case insensitive by the `fields`
            /// resource for the purposes of equality checking.
            ///
            /// # Deprecation
            ///
            /// This type has been deprecated in favor of the `field-name` type.
            pub type FieldKey = _rt::String;
            /// Field names are always strings.
            ///
            /// Field names should always be treated as case insensitive by the `fields`
            /// resource for the purposes of equality checking.
            pub type FieldName = FieldKey;
            /// Field values should always be ASCII strings. However, in
            /// reality, HTTP implementations often have to interpret malformed values,
            /// so they are provided as a list of bytes.
            pub type FieldValue = _rt::Vec<u8>;
            /// This following block defines the `fields` resource which corresponds to
            /// HTTP standard Fields. Fields are a common representation used for both
            /// Headers and Trailers.
            ///
            /// A `fields` may be mutable or immutable. A `fields` created using the
            /// constructor, `from-list`, or `clone` will be mutable, but a `fields`
            /// resource given by other means (including, but not limited to,
            /// `incoming-request.headers`, `outgoing-request.headers`) might be be
            /// immutable. In an immutable fields, the `set`, `append`, and `delete`
            /// operations will fail with `header-error.immutable`.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Fields {
                handle: _rt::Resource<Fields>,
            }
            impl Fields {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Fields {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]fields"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Headers is an alias for Fields.
            pub type Headers = Fields;
            /// Trailers is an alias for Fields.
            pub type Trailers = Fields;
            /// Represents an incoming HTTP Request.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingRequest {
                handle: _rt::Resource<IncomingRequest>,
            }
            impl IncomingRequest {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingRequest {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-request"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents an outgoing HTTP Request.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingRequest {
                handle: _rt::Resource<OutgoingRequest>,
            }
            impl OutgoingRequest {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingRequest {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-request"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Parameters for making an HTTP Request. Each of these parameters is
            /// currently an optional timeout applicable to the transport layer of the
            /// HTTP protocol.
            ///
            /// These timeouts are separate from any the user may use to bound a
            /// blocking call to `wasi:io/poll.poll`.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct RequestOptions {
                handle: _rt::Resource<RequestOptions>,
            }
            impl RequestOptions {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for RequestOptions {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]request-options"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents the ability to send an HTTP Response.
            ///
            /// This resource is used by the `wasi:http/incoming-handler` interface to
            /// allow a Response to be sent corresponding to the Request provided as the
            /// other argument to `incoming-handler.handle`.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ResponseOutparam {
                handle: _rt::Resource<ResponseOutparam>,
            }
            impl ResponseOutparam {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ResponseOutparam {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]response-outparam"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// This type corresponds to the HTTP standard Status Code.
            pub type StatusCode = u16;
            /// Represents an incoming HTTP Response.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingResponse {
                handle: _rt::Resource<IncomingResponse>,
            }
            impl IncomingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents an incoming HTTP Request or Response's Body.
            ///
            /// A body has both its contents - a stream of bytes - and a (possibly
            /// empty) set of trailers, indicating that the full contents of the
            /// body have been received. This resource represents the contents as
            /// an `input-stream` and the delivery of trailers as a `future-trailers`,
            /// and ensures that the user of this interface may only be consuming either
            /// the body contents or waiting on trailers at any given time.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingBody {
                handle: _rt::Resource<IncomingBody>,
            }
            impl IncomingBody {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingBody {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-body"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents a future which may eventually return trailers, or an error.
            ///
            /// In the case that the incoming HTTP Request or Response did not have any
            /// trailers, this future will resolve to the empty set of trailers once the
            /// complete Request or Response body has been received.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct FutureTrailers {
                handle: _rt::Resource<FutureTrailers>,
            }
            impl FutureTrailers {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for FutureTrailers {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]future-trailers"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents an outgoing HTTP Response.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingResponse {
                handle: _rt::Resource<OutgoingResponse>,
            }
            impl OutgoingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents an outgoing HTTP Request or Response's Body.
            ///
            /// A body has both its contents - a stream of bytes - and a (possibly
            /// empty) set of trailers, inducating the full contents of the body
            /// have been sent. This resource represents the contents as an
            /// `output-stream` child resource, and the completion of the body (with
            /// optional trailers) with a static function that consumes the
            /// `outgoing-body` resource, and ensures that the user of this interface
            /// may not write to the body contents after the body has been finished.
            ///
            /// If the user code drops this resource, as opposed to calling the static
            /// method `finish`, the implementation should treat the body as incomplete,
            /// and that an error has occurred. The implementation should propagate this
            /// error to the HTTP protocol by whatever means it has available,
            /// including: corrupting the body on the wire, aborting the associated
            /// Request, or sending a late status code for the Response.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingBody {
                handle: _rt::Resource<OutgoingBody>,
            }
            impl OutgoingBody {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingBody {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-body"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Represents a future which may eventually return an incoming HTTP
            /// Response, or an error.
            ///
            /// This resource is returned by the `wasi:http/outgoing-handler` interface to
            /// provide the HTTP Response corresponding to the sent Request.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct FutureIncomingResponse {
                handle: _rt::Resource<FutureIncomingResponse>,
            }
            impl FutureIncomingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for FutureIncomingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]future-incoming-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Attempts to extract a http-related `error` from the wasi:io `error`
            /// provided.
            ///
            /// Stream operations which return
            /// `wasi:io/stream/stream-error::last-operation-failed` have a payload of
            /// type `wasi:io/error/error` with more information about the operation
            /// that failed. This payload can be passed through to this function to see
            /// if there's http-related information about the error to return.
            ///
            /// Note that this function is fallible because not all io-errors are
            /// http-related errors.
            pub fn http_error_code(err: &IoError) -> Option<ErrorCode> {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                    extern "C" {
                        #[link_name = "http-error-code"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((err).handle() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                let v64 = match l2 {
                                    0 => ErrorCode::DnsTimeout,
                                    1 => {
                                        let e64 = {
                                            let l3 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l7 = i32::from(*ptr0.add(28).cast::<u8>());
                                            DnsErrorPayload {
                                                rcode: match l3 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l4 = *ptr0.add(20).cast::<*mut u8>();
                                                            let l5 = *ptr0.add(24).cast::<usize>();
                                                            let len6 = l5;
                                                            let bytes6 = _rt::Vec::from_raw_parts(
                                                                l4.cast(),
                                                                len6,
                                                                len6,
                                                            );
                                                            _rt::string_lift(bytes6)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                info_code: match l7 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l8 = i32::from(*ptr0.add(30).cast::<u16>());
                                                            l8 as u16
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::DnsError(e64)
                                    }
                                    2 => ErrorCode::DestinationNotFound,
                                    3 => ErrorCode::DestinationUnavailable,
                                    4 => ErrorCode::DestinationIpProhibited,
                                    5 => ErrorCode::DestinationIpUnroutable,
                                    6 => ErrorCode::ConnectionRefused,
                                    7 => ErrorCode::ConnectionTerminated,
                                    8 => ErrorCode::ConnectionTimeout,
                                    9 => ErrorCode::ConnectionReadTimeout,
                                    10 => ErrorCode::ConnectionWriteTimeout,
                                    11 => ErrorCode::ConnectionLimitReached,
                                    12 => ErrorCode::TlsProtocolError,
                                    13 => ErrorCode::TlsCertificateError,
                                    14 => {
                                        let e64 = {
                                            let l9 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l11 = i32::from(*ptr0.add(20).cast::<u8>());
                                            TlsAlertReceivedPayload {
                                                alert_id: match l9 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l10 = i32::from(*ptr0.add(17).cast::<u8>());
                                                            l10 as u8
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                alert_message: match l11 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l12 = *ptr0.add(24).cast::<*mut u8>();
                                                            let l13 = *ptr0.add(28).cast::<usize>();
                                                            let len14 = l13;
                                                            let bytes14 = _rt::Vec::from_raw_parts(
                                                                l12.cast(),
                                                                len14,
                                                                len14,
                                                            );
                                                            _rt::string_lift(bytes14)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::TlsAlertReceived(e64)
                                    }
                                    15 => ErrorCode::HttpRequestDenied,
                                    16 => ErrorCode::HttpRequestLengthRequired,
                                    17 => {
                                        let e64 = {
                                            let l15 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l15 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l16 = *ptr0.add(24).cast::<i64>();
                                                        l16 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestBodySize(e64)
                                    }
                                    18 => ErrorCode::HttpRequestMethodInvalid,
                                    19 => ErrorCode::HttpRequestUriInvalid,
                                    20 => ErrorCode::HttpRequestUriTooLong,
                                    21 => {
                                        let e64 = {
                                            let l17 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l17 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l18 = *ptr0.add(20).cast::<i32>();
                                                        l18 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestHeaderSectionSize(e64)
                                    }
                                    22 => {
                                        let e64 = {
                                            let l19 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l19 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l20 = i32::from(*ptr0.add(20).cast::<u8>());
                                                        let l24 = i32::from(*ptr0.add(32).cast::<u8>());
                                                        FieldSizePayload {
                                                            field_name: match l20 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l21 = *ptr0.add(24).cast::<*mut u8>();
                                                                        let l22 = *ptr0.add(28).cast::<usize>();
                                                                        let len23 = l22;
                                                                        let bytes23 = _rt::Vec::from_raw_parts(
                                                                            l21.cast(),
                                                                            len23,
                                                                            len23,
                                                                        );
                                                                        _rt::string_lift(bytes23)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            field_size: match l24 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l25 = *ptr0.add(36).cast::<i32>();
                                                                        l25 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestHeaderSize(e64)
                                    }
                                    23 => {
                                        let e64 = {
                                            let l26 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l26 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l27 = *ptr0.add(20).cast::<i32>();
                                                        l27 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestTrailerSectionSize(e64)
                                    }
                                    24 => {
                                        let e64 = {
                                            let l28 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l32 = i32::from(*ptr0.add(28).cast::<u8>());
                                            FieldSizePayload {
                                                field_name: match l28 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l29 = *ptr0.add(20).cast::<*mut u8>();
                                                            let l30 = *ptr0.add(24).cast::<usize>();
                                                            let len31 = l30;
                                                            let bytes31 = _rt::Vec::from_raw_parts(
                                                                l29.cast(),
                                                                len31,
                                                                len31,
                                                            );
                                                            _rt::string_lift(bytes31)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l32 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l33 = *ptr0.add(32).cast::<i32>();
                                                            l33 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::HttpRequestTrailerSize(e64)
                                    }
                                    25 => ErrorCode::HttpResponseIncomplete,
                                    26 => {
                                        let e64 = {
                                            let l34 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l34 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l35 = *ptr0.add(20).cast::<i32>();
                                                        l35 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseHeaderSectionSize(e64)
                                    }
                                    27 => {
                                        let e64 = {
                                            let l36 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l40 = i32::from(*ptr0.add(28).cast::<u8>());
                                            FieldSizePayload {
                                                field_name: match l36 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l37 = *ptr0.add(20).cast::<*mut u8>();
                                                            let l38 = *ptr0.add(24).cast::<usize>();
                                                            let len39 = l38;
                                                            let bytes39 = _rt::Vec::from_raw_parts(
                                                                l37.cast(),
                                                                len39,
                                                                len39,
                                                            );
                                                            _rt::string_lift(bytes39)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l40 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l41 = *ptr0.add(32).cast::<i32>();
                                                            l41 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::HttpResponseHeaderSize(e64)
                                    }
                                    28 => {
                                        let e64 = {
                                            let l42 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l42 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l43 = *ptr0.add(24).cast::<i64>();
                                                        l43 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseBodySize(e64)
                                    }
                                    29 => {
                                        let e64 = {
                                            let l44 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l44 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l45 = *ptr0.add(20).cast::<i32>();
                                                        l45 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseTrailerSectionSize(e64)
                                    }
                                    30 => {
                                        let e64 = {
                                            let l46 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l50 = i32::from(*ptr0.add(28).cast::<u8>());
                                            FieldSizePayload {
                                                field_name: match l46 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l47 = *ptr0.add(20).cast::<*mut u8>();
                                                            let l48 = *ptr0.add(24).cast::<usize>();
                                                            let len49 = l48;
                                                            let bytes49 = _rt::Vec::from_raw_parts(
                                                                l47.cast(),
                                                                len49,
                                                                len49,
                                                            );
                                                            _rt::string_lift(bytes49)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l50 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l51 = *ptr0.add(32).cast::<i32>();
                                                            l51 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::HttpResponseTrailerSize(e64)
                                    }
                                    31 => {
                                        let e64 = {
                                            let l52 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l52 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l53 = *ptr0.add(20).cast::<*mut u8>();
                                                        let l54 = *ptr0.add(24).cast::<usize>();
                                                        let len55 = l54;
                                                        let bytes55 = _rt::Vec::from_raw_parts(
                                                            l53.cast(),
                                                            len55,
                                                            len55,
                                                        );
                                                        _rt::string_lift(bytes55)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseTransferCoding(e64)
                                    }
                                    32 => {
                                        let e64 = {
                                            let l56 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l56 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l57 = *ptr0.add(20).cast::<*mut u8>();
                                                        let l58 = *ptr0.add(24).cast::<usize>();
                                                        let len59 = l58;
                                                        let bytes59 = _rt::Vec::from_raw_parts(
                                                            l57.cast(),
                                                            len59,
                                                            len59,
                                                        );
                                                        _rt::string_lift(bytes59)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseContentCoding(e64)
                                    }
                                    33 => ErrorCode::HttpResponseTimeout,
                                    34 => ErrorCode::HttpUpgradeFailed,
                                    35 => ErrorCode::HttpProtocolError,
                                    36 => ErrorCode::LoopDetected,
                                    37 => ErrorCode::ConfigurationError,
                                    n => {
                                        debug_assert_eq!(n, 38, "invalid enum discriminant");
                                        let e64 = {
                                            let l60 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l60 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l61 = *ptr0.add(20).cast::<*mut u8>();
                                                        let l62 = *ptr0.add(24).cast::<usize>();
                                                        let len63 = l62;
                                                        let bytes63 = _rt::Vec::from_raw_parts(
                                                            l61.cast(),
                                                            len63,
                                                            len63,
                                                        );
                                                        _rt::string_lift(bytes63)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::InternalError(e64)
                                    }
                                };
                                v64
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Construct an empty HTTP Fields.
                ///
                /// The resulting `fields` is mutable.
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[constructor]fields"]
                            fn wit_import() -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Construct an HTTP Fields.
                ///
                /// The resulting `fields` is mutable.
                ///
                /// The list represents each name-value pair in the Fields. Names
                /// which have multiple values are represented by multiple entries in this
                /// list with the same name.
                ///
                /// The tuple is a pair of the field name, represented as a string, and
                /// Value, represented as a list of bytes.
                ///
                /// An error result will be returned if any `field-name` or `field-value` is
                /// syntactically invalid, or if a field is forbidden.
                pub fn from_list(
                    entries: &[(FieldName, FieldValue)],
                ) -> Result<Fields, HeaderError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let vec3 = entries;
                        let len3 = vec3.len();
                        let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec3.len() * 16,
                            4,
                        );
                        let result3 = if layout3.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout3);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec3.into_iter().enumerate() {
                            let base = result3.add(i * 16);
                            {
                                let (t0_0, t0_1) = e;
                                let vec1 = t0_0;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                let vec2 = t0_1;
                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                let len2 = vec2.len();
                                *base.add(12).cast::<usize>() = len2;
                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                            }
                        }
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[static]fields.from-list"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(result3, len3, ptr4);
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        if layout3.size() != 0 {
                            _rt::alloc::dealloc(result3.cast(), layout3);
                        }
                        match l5 {
                            0 => {
                                let e = {
                                    let l6 = *ptr4.add(4).cast::<i32>();
                                    Fields::from_handle(l6 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr4.add(4).cast::<u8>());
                                    let v8 = match l7 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v8
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Get all of the values corresponding to a name. If the name is not present
                /// in this `fields` or is syntactically invalid, an empty list is returned.
                /// However, if the name is present but empty, this is represented by a list
                /// with one or more empty field-values present.
                pub fn get(&self, name: &FieldName) -> _rt::Vec<FieldValue> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.get"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = *ptr1.add(0).cast::<*mut u8>();
                        let l3 = *ptr1.add(4).cast::<usize>();
                        let base7 = l2;
                        let len7 = l3;
                        let mut result7 = _rt::Vec::with_capacity(len7);
                        for i in 0..len7 {
                            let base = base7.add(i * 8);
                            let e7 = {
                                let l4 = *base.add(0).cast::<*mut u8>();
                                let l5 = *base.add(4).cast::<usize>();
                                let len6 = l5;
                                _rt::Vec::from_raw_parts(l4.cast(), len6, len6)
                            };
                            result7.push(e7);
                        }
                        _rt::cabi_dealloc(base7, len7 * 8, 4);
                        result7
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns `true` when the name is present in this `fields`. If the name is
                /// syntactically invalid, `false` is returned.
                pub fn has(&self, name: &FieldName) -> bool {
                    unsafe {
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.has"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                        );
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Set all of the values for a name. Clears any existing values for that
                /// name, if they have been set.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                ///
                /// Fails with `header-error.invalid-syntax` if the `field-name` or any of
                /// the `field-value`s are syntactically invalid.
                pub fn set(
                    &self,
                    name: &FieldName,
                    value: &[FieldValue],
                ) -> Result<(), HeaderError> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec2 = value;
                        let len2 = vec2.len();
                        let layout2 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec2.len() * 8,
                            4,
                        );
                        let result2 = if layout2.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout2).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout2);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec2.into_iter().enumerate() {
                            let base = result2.add(i * 8);
                            {
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                            }
                        }
                        let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.set"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result2,
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*ptr3.add(0).cast::<u8>());
                        if layout2.size() != 0 {
                            _rt::alloc::dealloc(result2.cast(), layout2);
                        }
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr3.add(1).cast::<u8>());
                                    let v6 = match l5 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v6
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Delete all values for a name. Does nothing if no values for the name
                /// exist.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                ///
                /// Fails with `header-error.invalid-syntax` if the `field-name` is
                /// syntactically invalid.
                pub fn delete(&self, name: &FieldName) -> Result<(), HeaderError> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.delete"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Append a value for a name. Does not change or delete any existing
                /// values for that name.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                ///
                /// Fails with `header-error.invalid-syntax` if the `field-name` or
                /// `field-value` are syntactically invalid.
                pub fn append(
                    &self,
                    name: &FieldName,
                    value: &FieldValue,
                ) -> Result<(), HeaderError> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = value;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.append"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    let v5 = match l4 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Retrieve the full set of names and values in the Fields. Like the
                /// constructor, the list represents each name-value pair.
                ///
                /// The outer list represents each name-value pair in the Fields. Names
                /// which have multiple values are represented by multiple entries in this
                /// list with the same name.
                ///
                /// The names and values are always returned in the original casing and in
                /// the order in which they will be serialized for transport.
                pub fn entries(&self) -> _rt::Vec<(FieldName, FieldValue)> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.entries"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base9 = l1;
                        let len9 = l2;
                        let mut result9 = _rt::Vec::with_capacity(len9);
                        for i in 0..len9 {
                            let base = base9.add(i * 16);
                            let e9 = {
                                let l3 = *base.add(0).cast::<*mut u8>();
                                let l4 = *base.add(4).cast::<usize>();
                                let len5 = l4;
                                let bytes5 = _rt::Vec::from_raw_parts(
                                    l3.cast(),
                                    len5,
                                    len5,
                                );
                                let l6 = *base.add(8).cast::<*mut u8>();
                                let l7 = *base.add(12).cast::<usize>();
                                let len8 = l7;
                                (
                                    _rt::string_lift(bytes5),
                                    _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                                )
                            };
                            result9.push(e9);
                        }
                        _rt::cabi_dealloc(base9, len9 * 16, 4);
                        result9
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                /// Make a deep copy of the Fields. Equivalent in behavior to calling the
                /// `fields` constructor on the return value of `entries`. The resulting
                /// `fields` is mutable.
                pub fn clone(&self) -> Fields {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]fields.clone"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the method of the incoming request.
                pub fn method(&self) -> Method {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.method"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        let v5 = match l1 {
                            0 => Method::Get,
                            1 => Method::Head,
                            2 => Method::Post,
                            3 => Method::Put,
                            4 => Method::Delete,
                            5 => Method::Connect,
                            6 => Method::Options,
                            7 => Method::Trace,
                            8 => Method::Patch,
                            n => {
                                debug_assert_eq!(n, 9, "invalid enum discriminant");
                                let e5 = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(
                                        l2.cast(),
                                        len4,
                                        len4,
                                    );
                                    _rt::string_lift(bytes4)
                                };
                                Method::Other(e5)
                            }
                        };
                        v5
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the path with query parameters from the request, as a string.
                pub fn path_with_query(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.path-with-query"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(
                                        l2.cast(),
                                        len4,
                                        len4,
                                    );
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the protocol scheme from the request.
                pub fn scheme(&self) -> Option<Scheme> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.scheme"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v6 = match l2 {
                                        0 => Scheme::Http,
                                        1 => Scheme::Https,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e6 = {
                                                let l3 = *ptr0.add(8).cast::<*mut u8>();
                                                let l4 = *ptr0.add(12).cast::<usize>();
                                                let len5 = l4;
                                                let bytes5 = _rt::Vec::from_raw_parts(
                                                    l3.cast(),
                                                    len5,
                                                    len5,
                                                );
                                                _rt::string_lift(bytes5)
                                            };
                                            Scheme::Other(e6)
                                        }
                                    };
                                    v6
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the authority of the Request's target URI, if present.
                pub fn authority(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.authority"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(
                                        l2.cast(),
                                        len4,
                                        len4,
                                    );
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the `headers` associated with the request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// The `headers` returned are a child resource: it must be dropped before
                /// the parent `incoming-request` is dropped. Dropping this
                /// `incoming-request` before all children are dropped will trap.
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Gives the `incoming-body` associated with this request. Will only
                /// return success at most once, and subsequent calls will return error.
                pub fn consume(&self) -> Result<IncomingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.consume"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    IncomingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Construct a new `outgoing-request` with a default `method` of `GET`, and
                /// `none` values for `path-with-query`, `scheme`, and `authority`.
                ///
                /// * `headers` is the HTTP Headers for the Request.
                ///
                /// It is possible to construct, or manipulate with the accessor functions
                /// below, an `outgoing-request` with an invalid combination of `scheme`
                /// and `authority`, or `headers` which are not permitted to be sent.
                /// It is the obligation of the `outgoing-handler.handle` implementation
                /// to reject invalid constructions of `outgoing-request`.
                pub fn new(headers: Headers) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[constructor]outgoing-request"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((&headers).take_handle() as i32);
                        OutgoingRequest::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the resource corresponding to the outgoing Body for this
                /// Request.
                ///
                /// Returns success on the first call: the `outgoing-body` resource for
                /// this `outgoing-request` can be retrieved at most once. Subsequent
                /// calls will return error.
                pub fn body(&self) -> Result<OutgoingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.body"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    OutgoingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the Method for the Request.
                pub fn method(&self) -> Method {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.method"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        let v5 = match l1 {
                            0 => Method::Get,
                            1 => Method::Head,
                            2 => Method::Post,
                            3 => Method::Put,
                            4 => Method::Delete,
                            5 => Method::Connect,
                            6 => Method::Options,
                            7 => Method::Trace,
                            8 => Method::Patch,
                            n => {
                                debug_assert_eq!(n, 9, "invalid enum discriminant");
                                let e5 = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(
                                        l2.cast(),
                                        len4,
                                        len4,
                                    );
                                    _rt::string_lift(bytes4)
                                };
                                Method::Other(e5)
                            }
                        };
                        v5
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the Method for the Request. Fails if the string present in a
                /// `method.other` argument is not a syntactically valid method.
                pub fn set_method(&self, method: &Method) -> Result<(), ()> {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match method {
                            Method::Get => (0i32, ::core::ptr::null_mut(), 0usize),
                            Method::Head => (1i32, ::core::ptr::null_mut(), 0usize),
                            Method::Post => (2i32, ::core::ptr::null_mut(), 0usize),
                            Method::Put => (3i32, ::core::ptr::null_mut(), 0usize),
                            Method::Delete => (4i32, ::core::ptr::null_mut(), 0usize),
                            Method::Connect => (5i32, ::core::ptr::null_mut(), 0usize),
                            Method::Options => (6i32, ::core::ptr::null_mut(), 0usize),
                            Method::Trace => (7i32, ::core::ptr::null_mut(), 0usize),
                            Method::Patch => (8i32, ::core::ptr::null_mut(), 0usize),
                            Method::Other(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (9i32, ptr0.cast_mut(), len0)
                            }
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-method"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the combination of the HTTP Path and Query for the Request.
                /// When `none`, this represents an empty Path and empty Query.
                pub fn path_with_query(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.path-with-query"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(
                                        l2.cast(),
                                        len4,
                                        len4,
                                    );
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the combination of the HTTP Path and Query for the Request.
                /// When `none`, this represents an empty Path and empty Query. Fails is the
                /// string given is not a syntactically valid path and query uri component.
                pub fn set_path_with_query(
                    &self,
                    path_with_query: Option<&str>,
                ) -> Result<(), ()> {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match path_with_query {
                            Some(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (1i32, ptr0.cast_mut(), len0)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-path-with-query"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the HTTP Related Scheme for the Request. When `none`, the
                /// implementation may choose an appropriate default scheme.
                pub fn scheme(&self) -> Option<Scheme> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.scheme"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v6 = match l2 {
                                        0 => Scheme::Http,
                                        1 => Scheme::Https,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e6 = {
                                                let l3 = *ptr0.add(8).cast::<*mut u8>();
                                                let l4 = *ptr0.add(12).cast::<usize>();
                                                let len5 = l4;
                                                let bytes5 = _rt::Vec::from_raw_parts(
                                                    l3.cast(),
                                                    len5,
                                                    len5,
                                                );
                                                _rt::string_lift(bytes5)
                                            };
                                            Scheme::Other(e6)
                                        }
                                    };
                                    v6
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the HTTP Related Scheme for the Request. When `none`, the
                /// implementation may choose an appropriate default scheme. Fails if the
                /// string given is not a syntactically valid uri scheme.
                pub fn set_scheme(&self, scheme: Option<&Scheme>) -> Result<(), ()> {
                    unsafe {
                        let (result2_0, result2_1, result2_2, result2_3) = match scheme {
                            Some(e) => {
                                let (result1_0, result1_1, result1_2) = match e {
                                    Scheme::Http => (0i32, ::core::ptr::null_mut(), 0usize),
                                    Scheme::Https => (1i32, ::core::ptr::null_mut(), 0usize),
                                    Scheme::Other(e) => {
                                        let vec0 = e;
                                        let ptr0 = vec0.as_ptr().cast::<u8>();
                                        let len0 = vec0.len();
                                        (2i32, ptr0.cast_mut(), len0)
                                    }
                                };
                                (1i32, result1_0, result1_1, result1_2)
                            }
                            None => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-scheme"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result2_0,
                            result2_1,
                            result2_2,
                            result2_3,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the authority of the Request's target URI. A value of `none` may be used
                /// with Related Schemes which do not require an authority. The HTTP and
                /// HTTPS schemes always require an authority.
                pub fn authority(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.authority"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(
                                        l2.cast(),
                                        len4,
                                        len4,
                                    );
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the authority of the Request's target URI. A value of `none` may be used
                /// with Related Schemes which do not require an authority. The HTTP and
                /// HTTPS schemes always require an authority. Fails if the string given is
                /// not a syntactically valid URI authority.
                pub fn set_authority(&self, authority: Option<&str>) -> Result<(), ()> {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match authority {
                            Some(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (1i32, ptr0.cast_mut(), len0)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-authority"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the headers associated with the Request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `outgoing-request` is dropped, or its ownership is transferred to
                /// another component by e.g. `outgoing-handler.handle`.
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// Construct a default `request-options` value.
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[constructor]request-options"]
                            fn wit_import() -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        RequestOptions::from_handle(ret as u32)
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// The timeout for the initial connect to the HTTP Server.
                pub fn connect_timeout(&self) -> Option<Duration> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]request-options.connect-timeout"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the timeout for the initial connect to the HTTP Server. An error
                /// return value indicates that this timeout is not supported.
                pub fn set_connect_timeout(
                    &self,
                    duration: Option<Duration>,
                ) -> Result<(), ()> {
                    unsafe {
                        let (result0_0, result0_1) = match duration {
                            Some(e) => (1i32, _rt::as_i64(e)),
                            None => (0i32, 0i64),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]request-options.set-connect-timeout"]
                            fn wit_import(_: i32, _: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result0_0,
                            result0_1,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// The timeout for receiving the first byte of the Response body.
                pub fn first_byte_timeout(&self) -> Option<Duration> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]request-options.first-byte-timeout"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the timeout for receiving the first byte of the Response body. An
                /// error return value indicates that this timeout is not supported.
                pub fn set_first_byte_timeout(
                    &self,
                    duration: Option<Duration>,
                ) -> Result<(), ()> {
                    unsafe {
                        let (result0_0, result0_1) = match duration {
                            Some(e) => (1i32, _rt::as_i64(e)),
                            None => (0i32, 0i64),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]request-options.set-first-byte-timeout"]
                            fn wit_import(_: i32, _: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result0_0,
                            result0_1,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// The timeout for receiving subsequent chunks of bytes in the Response
                /// body stream.
                pub fn between_bytes_timeout(&self) -> Option<Duration> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]request-options.between-bytes-timeout"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the timeout for receiving subsequent chunks of bytes in the Response
                /// body stream. An error return value indicates that this timeout is not
                /// supported.
                pub fn set_between_bytes_timeout(
                    &self,
                    duration: Option<Duration>,
                ) -> Result<(), ()> {
                    unsafe {
                        let (result0_0, result0_1) = match duration {
                            Some(e) => (1i32, _rt::as_i64(e)),
                            None => (0i32, 0i64),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]request-options.set-between-bytes-timeout"]
                            fn wit_import(_: i32, _: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result0_0,
                            result0_1,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ResponseOutparam {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the value of the `response-outparam` to either send a response,
                /// or indicate an error.
                ///
                /// This method consumes the `response-outparam` to ensure that it is
                /// called at most once. If it is never called, the implementation
                /// will respond with an error.
                ///
                /// The user may provide an `error` to `response` to allow the
                /// implementation determine how to respond with an HTTP error response.
                pub fn set(
                    param: ResponseOutparam,
                    response: Result<OutgoingResponse, ErrorCode>,
                ) {
                    unsafe {
                        let (
                            result38_0,
                            result38_1,
                            result38_2,
                            result38_3,
                            result38_4,
                            result38_5,
                            result38_6,
                            result38_7,
                        ) = match &response {
                            Ok(e) => {
                                (
                                    0i32,
                                    (e).take_handle() as i32,
                                    0i32,
                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                    ::core::ptr::null_mut(),
                                    ::core::ptr::null_mut(),
                                    0usize,
                                    0i32,
                                )
                            }
                            Err(e) => {
                                let (
                                    result37_0,
                                    result37_1,
                                    result37_2,
                                    result37_3,
                                    result37_4,
                                    result37_5,
                                    result37_6,
                                ) = match e {
                                    ErrorCode::DnsTimeout => {
                                        (
                                            0i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::DnsError(e) => {
                                        let DnsErrorPayload {
                                            rcode: rcode0,
                                            info_code: info_code0,
                                        } = e;
                                        let (result2_0, result2_1, result2_2) = match rcode0 {
                                            Some(e) => {
                                                let vec1 = e;
                                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                                let len1 = vec1.len();
                                                (1i32, ptr1.cast_mut(), len1)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        let (result3_0, result3_1) = match info_code0 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            1i32,
                                            result2_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result2_1);
                                                t
                                            },
                                            result2_2 as *mut u8,
                                            result3_0 as *mut u8,
                                            result3_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::DestinationNotFound => {
                                        (
                                            2i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::DestinationUnavailable => {
                                        (
                                            3i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::DestinationIpProhibited => {
                                        (
                                            4i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::DestinationIpUnroutable => {
                                        (
                                            5i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConnectionRefused => {
                                        (
                                            6i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConnectionTerminated => {
                                        (
                                            7i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConnectionTimeout => {
                                        (
                                            8i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConnectionReadTimeout => {
                                        (
                                            9i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConnectionWriteTimeout => {
                                        (
                                            10i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConnectionLimitReached => {
                                        (
                                            11i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::TlsProtocolError => {
                                        (
                                            12i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::TlsCertificateError => {
                                        (
                                            13i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::TlsAlertReceived(e) => {
                                        let TlsAlertReceivedPayload {
                                            alert_id: alert_id4,
                                            alert_message: alert_message4,
                                        } = e;
                                        let (result5_0, result5_1) = match alert_id4 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        let (result7_0, result7_1, result7_2) = match alert_message4 {
                                            Some(e) => {
                                                let vec6 = e;
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                (1i32, ptr6.cast_mut(), len6)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            14i32,
                                            result5_0,
                                            ::core::mem::MaybeUninit::new(i64::from(result5_1) as u64),
                                            result7_0 as *mut u8,
                                            result7_1,
                                            result7_2,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestDenied => {
                                        (
                                            15i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestLengthRequired => {
                                        (
                                            16i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestBodySize(e) => {
                                        let (result8_0, result8_1) = match e {
                                            Some(e) => (1i32, _rt::as_i64(e)),
                                            None => (0i32, 0i64),
                                        };
                                        (
                                            17i32,
                                            result8_0,
                                            ::core::mem::MaybeUninit::new(result8_1 as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestMethodInvalid => {
                                        (
                                            18i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestUriInvalid => {
                                        (
                                            19i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestUriTooLong => {
                                        (
                                            20i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestHeaderSectionSize(e) => {
                                        let (result9_0, result9_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            21i32,
                                            result9_0,
                                            ::core::mem::MaybeUninit::new(i64::from(result9_1) as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestHeaderSize(e) => {
                                        let (
                                            result14_0,
                                            result14_1,
                                            result14_2,
                                            result14_3,
                                            result14_4,
                                            result14_5,
                                        ) = match e {
                                            Some(e) => {
                                                let FieldSizePayload {
                                                    field_name: field_name10,
                                                    field_size: field_size10,
                                                } = e;
                                                let (result12_0, result12_1, result12_2) = match field_name10 {
                                                    Some(e) => {
                                                        let vec11 = e;
                                                        let ptr11 = vec11.as_ptr().cast::<u8>();
                                                        let len11 = vec11.len();
                                                        (1i32, ptr11.cast_mut(), len11)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                let (result13_0, result13_1) = match field_size10 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    1i32,
                                                    result12_0,
                                                    result12_1,
                                                    result12_2,
                                                    result13_0,
                                                    result13_1,
                                                )
                                            }
                                            None => {
                                                (0i32, 0i32, ::core::ptr::null_mut(), 0usize, 0i32, 0i32)
                                            }
                                        };
                                        (
                                            22i32,
                                            result14_0,
                                            ::core::mem::MaybeUninit::new(i64::from(result14_1) as u64),
                                            result14_2,
                                            result14_3 as *mut u8,
                                            result14_4 as usize,
                                            result14_5,
                                        )
                                    }
                                    ErrorCode::HttpRequestTrailerSectionSize(e) => {
                                        let (result15_0, result15_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            23i32,
                                            result15_0,
                                            ::core::mem::MaybeUninit::new(i64::from(result15_1) as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestTrailerSize(e) => {
                                        let FieldSizePayload {
                                            field_name: field_name16,
                                            field_size: field_size16,
                                        } = e;
                                        let (result18_0, result18_1, result18_2) = match field_name16 {
                                            Some(e) => {
                                                let vec17 = e;
                                                let ptr17 = vec17.as_ptr().cast::<u8>();
                                                let len17 = vec17.len();
                                                (1i32, ptr17.cast_mut(), len17)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        let (result19_0, result19_1) = match field_size16 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            24i32,
                                            result18_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result18_1);
                                                t
                                            },
                                            result18_2 as *mut u8,
                                            result19_0 as *mut u8,
                                            result19_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseIncomplete => {
                                        (
                                            25i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseHeaderSectionSize(e) => {
                                        let (result20_0, result20_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            26i32,
                                            result20_0,
                                            ::core::mem::MaybeUninit::new(i64::from(result20_1) as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseHeaderSize(e) => {
                                        let FieldSizePayload {
                                            field_name: field_name21,
                                            field_size: field_size21,
                                        } = e;
                                        let (result23_0, result23_1, result23_2) = match field_name21 {
                                            Some(e) => {
                                                let vec22 = e;
                                                let ptr22 = vec22.as_ptr().cast::<u8>();
                                                let len22 = vec22.len();
                                                (1i32, ptr22.cast_mut(), len22)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        let (result24_0, result24_1) = match field_size21 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            27i32,
                                            result23_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result23_1);
                                                t
                                            },
                                            result23_2 as *mut u8,
                                            result24_0 as *mut u8,
                                            result24_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseBodySize(e) => {
                                        let (result25_0, result25_1) = match e {
                                            Some(e) => (1i32, _rt::as_i64(e)),
                                            None => (0i32, 0i64),
                                        };
                                        (
                                            28i32,
                                            result25_0,
                                            ::core::mem::MaybeUninit::new(result25_1 as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTrailerSectionSize(e) => {
                                        let (result26_0, result26_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            29i32,
                                            result26_0,
                                            ::core::mem::MaybeUninit::new(i64::from(result26_1) as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTrailerSize(e) => {
                                        let FieldSizePayload {
                                            field_name: field_name27,
                                            field_size: field_size27,
                                        } = e;
                                        let (result29_0, result29_1, result29_2) = match field_name27 {
                                            Some(e) => {
                                                let vec28 = e;
                                                let ptr28 = vec28.as_ptr().cast::<u8>();
                                                let len28 = vec28.len();
                                                (1i32, ptr28.cast_mut(), len28)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        let (result30_0, result30_1) = match field_size27 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            30i32,
                                            result29_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result29_1);
                                                t
                                            },
                                            result29_2 as *mut u8,
                                            result30_0 as *mut u8,
                                            result30_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTransferCoding(e) => {
                                        let (result32_0, result32_1, result32_2) = match e {
                                            Some(e) => {
                                                let vec31 = e;
                                                let ptr31 = vec31.as_ptr().cast::<u8>();
                                                let len31 = vec31.len();
                                                (1i32, ptr31.cast_mut(), len31)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            31i32,
                                            result32_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result32_1);
                                                t
                                            },
                                            result32_2 as *mut u8,
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseContentCoding(e) => {
                                        let (result34_0, result34_1, result34_2) = match e {
                                            Some(e) => {
                                                let vec33 = e;
                                                let ptr33 = vec33.as_ptr().cast::<u8>();
                                                let len33 = vec33.len();
                                                (1i32, ptr33.cast_mut(), len33)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            32i32,
                                            result34_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result34_1);
                                                t
                                            },
                                            result34_2 as *mut u8,
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTimeout => {
                                        (
                                            33i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpUpgradeFailed => {
                                        (
                                            34i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpProtocolError => {
                                        (
                                            35i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::LoopDetected => {
                                        (
                                            36i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::ConfigurationError => {
                                        (
                                            37i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::InternalError(e) => {
                                        let (result36_0, result36_1, result36_2) = match e {
                                            Some(e) => {
                                                let vec35 = e;
                                                let ptr35 = vec35.as_ptr().cast::<u8>();
                                                let len35 = vec35.len();
                                                (1i32, ptr35.cast_mut(), len35)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            38i32,
                                            result36_0,
                                            {
                                                let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result36_1);
                                                t
                                            },
                                            result36_2 as *mut u8,
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                };
                                (
                                    1i32,
                                    result37_0,
                                    result37_1,
                                    result37_2,
                                    result37_3,
                                    result37_4,
                                    result37_5,
                                    result37_6,
                                )
                            }
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[static]response-outparam.set"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: *mut u8,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: ::core::mem::MaybeUninit<u64>,
                            _: *mut u8,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (&param).take_handle() as i32,
                            result38_0,
                            result38_1,
                            result38_2,
                            result38_3,
                            result38_4,
                            result38_5,
                            result38_6,
                            result38_7,
                        );
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the status code from the incoming response.
                pub fn status(&self) -> StatusCode {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.status"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u16
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the headers from the incoming response.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `incoming-response` is dropped.
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the incoming body. May be called at most once. Returns error
                /// if called additional times.
                pub fn consume(&self) -> Result<IncomingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.consume"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    IncomingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingBody {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the contents of the body, as a stream of bytes.
                ///
                /// Returns success on first call: the stream representing the contents
                /// can be retrieved at most once. Subsequent calls will return error.
                ///
                /// The returned `input-stream` resource is a child: it must be dropped
                /// before the parent `incoming-body` is dropped, or consumed by
                /// `incoming-body.finish`.
                ///
                /// This invariant ensures that the implementation can determine whether
                /// the user is consuming the contents of the body, waiting on the
                /// `future-trailers` to be ready, or neither. This allows for network
                /// backpressure is to be applied when the user is consuming the body,
                /// and for that backpressure to not inhibit delivery of the trailers if
                /// the user does not read the entire body.
                pub fn stream(&self) -> Result<InputStream, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-body.stream"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::InputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingBody {
                #[allow(unused_unsafe, clippy::all)]
                /// Takes ownership of `incoming-body`, and returns a `future-trailers`.
                /// This function will trap if the `input-stream` child is still alive.
                pub fn finish(this: IncomingBody) -> FutureTrailers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[static]incoming-body.finish"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((&this).take_handle() as i32);
                        FutureTrailers::from_handle(ret as u32)
                    }
                }
            }
            impl FutureTrailers {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns a pollable which becomes ready when either the trailers have
                /// been received, or an error has occurred. When this pollable is ready,
                /// the `get` method will return `some`.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]future-trailers.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl FutureTrailers {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the contents of the trailers, or an error which occurred,
                /// once the future is ready.
                ///
                /// The outer `option` represents future readiness. Users can wait on this
                /// `option` to become `some` using the `subscribe` method.
                ///
                /// The outer `result` is used to retrieve the trailers or error at most
                /// once. It will be success on the first call in which the outer option
                /// is `some`, and error on subsequent calls.
                ///
                /// The inner `result` represents that either the HTTP Request or Response
                /// body, as well as any trailers, were received successfully, or that an
                /// error occurred receiving them. The optional `trailers` indicates whether
                /// or not trailers were present in the body.
                ///
                /// When some `trailers` are returned by this method, the `trailers`
                /// resource is immutable, and a child. Use of the `set`, `append`, or
                /// `delete` methods will return an error, and the resource must be
                /// dropped before the parent `future-trailers` is dropped.
                pub fn get(
                    &self,
                ) -> Option<Result<Result<Option<Trailers>, ErrorCode>, ()>> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 56]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 56],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]future-trailers.get"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(16).cast::<u8>());
                                                match l3 {
                                                    0 => {
                                                        let e = {
                                                            let l4 = i32::from(*ptr0.add(24).cast::<u8>());
                                                            match l4 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l5 = *ptr0.add(28).cast::<i32>();
                                                                        Fields::from_handle(l5 as u32)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            }
                                                        };
                                                        Ok(e)
                                                    }
                                                    1 => {
                                                        let e = {
                                                            let l6 = i32::from(*ptr0.add(24).cast::<u8>());
                                                            let v68 = match l6 {
                                                                0 => ErrorCode::DnsTimeout,
                                                                1 => {
                                                                    let e68 = {
                                                                        let l7 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l11 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        DnsErrorPayload {
                                                                            rcode: match l7 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l8 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l9 = *ptr0.add(40).cast::<usize>();
                                                                                        let len10 = l9;
                                                                                        let bytes10 = _rt::Vec::from_raw_parts(
                                                                                            l8.cast(),
                                                                                            len10,
                                                                                            len10,
                                                                                        );
                                                                                        _rt::string_lift(bytes10)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            info_code: match l11 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l12 = i32::from(*ptr0.add(46).cast::<u16>());
                                                                                        l12 as u16
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::DnsError(e68)
                                                                }
                                                                2 => ErrorCode::DestinationNotFound,
                                                                3 => ErrorCode::DestinationUnavailable,
                                                                4 => ErrorCode::DestinationIpProhibited,
                                                                5 => ErrorCode::DestinationIpUnroutable,
                                                                6 => ErrorCode::ConnectionRefused,
                                                                7 => ErrorCode::ConnectionTerminated,
                                                                8 => ErrorCode::ConnectionTimeout,
                                                                9 => ErrorCode::ConnectionReadTimeout,
                                                                10 => ErrorCode::ConnectionWriteTimeout,
                                                                11 => ErrorCode::ConnectionLimitReached,
                                                                12 => ErrorCode::TlsProtocolError,
                                                                13 => ErrorCode::TlsCertificateError,
                                                                14 => {
                                                                    let e68 = {
                                                                        let l13 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l15 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                        TlsAlertReceivedPayload {
                                                                            alert_id: match l13 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l14 = i32::from(*ptr0.add(33).cast::<u8>());
                                                                                        l14 as u8
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            alert_message: match l15 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l16 = *ptr0.add(40).cast::<*mut u8>();
                                                                                        let l17 = *ptr0.add(44).cast::<usize>();
                                                                                        let len18 = l17;
                                                                                        let bytes18 = _rt::Vec::from_raw_parts(
                                                                                            l16.cast(),
                                                                                            len18,
                                                                                            len18,
                                                                                        );
                                                                                        _rt::string_lift(bytes18)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::TlsAlertReceived(e68)
                                                                }
                                                                15 => ErrorCode::HttpRequestDenied,
                                                                16 => ErrorCode::HttpRequestLengthRequired,
                                                                17 => {
                                                                    let e68 = {
                                                                        let l19 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l19 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l20 = *ptr0.add(40).cast::<i64>();
                                                                                    l20 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestBodySize(e68)
                                                                }
                                                                18 => ErrorCode::HttpRequestMethodInvalid,
                                                                19 => ErrorCode::HttpRequestUriInvalid,
                                                                20 => ErrorCode::HttpRequestUriTooLong,
                                                                21 => {
                                                                    let e68 = {
                                                                        let l21 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l21 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l22 = *ptr0.add(36).cast::<i32>();
                                                                                    l22 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSectionSize(e68)
                                                                }
                                                                22 => {
                                                                    let e68 = {
                                                                        let l23 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l23 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l24 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                                    let l28 = i32::from(*ptr0.add(48).cast::<u8>());
                                                                                    FieldSizePayload {
                                                                                        field_name: match l24 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l25 = *ptr0.add(40).cast::<*mut u8>();
                                                                                                    let l26 = *ptr0.add(44).cast::<usize>();
                                                                                                    let len27 = l26;
                                                                                                    let bytes27 = _rt::Vec::from_raw_parts(
                                                                                                        l25.cast(),
                                                                                                        len27,
                                                                                                        len27,
                                                                                                    );
                                                                                                    _rt::string_lift(bytes27)
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                        field_size: match l28 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l29 = *ptr0.add(52).cast::<i32>();
                                                                                                    l29 as u32
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                    }
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSize(e68)
                                                                }
                                                                23 => {
                                                                    let e68 = {
                                                                        let l30 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l30 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l31 = *ptr0.add(36).cast::<i32>();
                                                                                    l31 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSectionSize(e68)
                                                                }
                                                                24 => {
                                                                    let e68 = {
                                                                        let l32 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l36 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l32 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l33 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l34 = *ptr0.add(40).cast::<usize>();
                                                                                        let len35 = l34;
                                                                                        let bytes35 = _rt::Vec::from_raw_parts(
                                                                                            l33.cast(),
                                                                                            len35,
                                                                                            len35,
                                                                                        );
                                                                                        _rt::string_lift(bytes35)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l36 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l37 = *ptr0.add(48).cast::<i32>();
                                                                                        l37 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSize(e68)
                                                                }
                                                                25 => ErrorCode::HttpResponseIncomplete,
                                                                26 => {
                                                                    let e68 = {
                                                                        let l38 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l38 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l39 = *ptr0.add(36).cast::<i32>();
                                                                                    l39 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSectionSize(e68)
                                                                }
                                                                27 => {
                                                                    let e68 = {
                                                                        let l40 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l44 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l40 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l41 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l42 = *ptr0.add(40).cast::<usize>();
                                                                                        let len43 = l42;
                                                                                        let bytes43 = _rt::Vec::from_raw_parts(
                                                                                            l41.cast(),
                                                                                            len43,
                                                                                            len43,
                                                                                        );
                                                                                        _rt::string_lift(bytes43)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l44 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l45 = *ptr0.add(48).cast::<i32>();
                                                                                        l45 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSize(e68)
                                                                }
                                                                28 => {
                                                                    let e68 = {
                                                                        let l46 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l46 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l47 = *ptr0.add(40).cast::<i64>();
                                                                                    l47 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseBodySize(e68)
                                                                }
                                                                29 => {
                                                                    let e68 = {
                                                                        let l48 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l48 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l49 = *ptr0.add(36).cast::<i32>();
                                                                                    l49 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSectionSize(e68)
                                                                }
                                                                30 => {
                                                                    let e68 = {
                                                                        let l50 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l54 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l50 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l51 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l52 = *ptr0.add(40).cast::<usize>();
                                                                                        let len53 = l52;
                                                                                        let bytes53 = _rt::Vec::from_raw_parts(
                                                                                            l51.cast(),
                                                                                            len53,
                                                                                            len53,
                                                                                        );
                                                                                        _rt::string_lift(bytes53)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l54 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l55 = *ptr0.add(48).cast::<i32>();
                                                                                        l55 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSize(e68)
                                                                }
                                                                31 => {
                                                                    let e68 = {
                                                                        let l56 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l56 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l57 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l58 = *ptr0.add(40).cast::<usize>();
                                                                                    let len59 = l58;
                                                                                    let bytes59 = _rt::Vec::from_raw_parts(
                                                                                        l57.cast(),
                                                                                        len59,
                                                                                        len59,
                                                                                    );
                                                                                    _rt::string_lift(bytes59)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTransferCoding(e68)
                                                                }
                                                                32 => {
                                                                    let e68 = {
                                                                        let l60 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l60 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l61 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l62 = *ptr0.add(40).cast::<usize>();
                                                                                    let len63 = l62;
                                                                                    let bytes63 = _rt::Vec::from_raw_parts(
                                                                                        l61.cast(),
                                                                                        len63,
                                                                                        len63,
                                                                                    );
                                                                                    _rt::string_lift(bytes63)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseContentCoding(e68)
                                                                }
                                                                33 => ErrorCode::HttpResponseTimeout,
                                                                34 => ErrorCode::HttpUpgradeFailed,
                                                                35 => ErrorCode::HttpProtocolError,
                                                                36 => ErrorCode::LoopDetected,
                                                                37 => ErrorCode::ConfigurationError,
                                                                n => {
                                                                    debug_assert_eq!(n, 38, "invalid enum discriminant");
                                                                    let e68 = {
                                                                        let l64 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l64 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l65 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l66 = *ptr0.add(40).cast::<usize>();
                                                                                    let len67 = l66;
                                                                                    let bytes67 = _rt::Vec::from_raw_parts(
                                                                                        l65.cast(),
                                                                                        len67,
                                                                                        len67,
                                                                                    );
                                                                                    _rt::string_lift(bytes67)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::InternalError(e68)
                                                                }
                                                            };
                                                            v68
                                                        };
                                                        Err(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            Ok(e)
                                        }
                                        1 => {
                                            let e = ();
                                            Err(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Construct an `outgoing-response`, with a default `status-code` of `200`.
                /// If a different `status-code` is needed, it must be set via the
                /// `set-status-code` method.
                ///
                /// * `headers` is the HTTP Headers for the Response.
                pub fn new(headers: Headers) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[constructor]outgoing-response"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((&headers).take_handle() as i32);
                        OutgoingResponse::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the HTTP Status Code for the Response.
                pub fn status_code(&self) -> StatusCode {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.status-code"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u16
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the HTTP Status Code for the Response. Fails if the status-code
                /// given is not a valid http status code.
                pub fn set_status_code(
                    &self,
                    status_code: StatusCode,
                ) -> Result<(), ()> {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.set-status-code"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(status_code),
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the headers associated with the Request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `outgoing-request` is dropped, or its ownership is transferred to
                /// another component by e.g. `outgoing-handler.handle`.
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the resource corresponding to the outgoing Body for this Response.
                ///
                /// Returns success on the first call: the `outgoing-body` resource for
                /// this `outgoing-response` can be retrieved at most once. Subsequent
                /// calls will return error.
                pub fn body(&self) -> Result<OutgoingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.body"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    OutgoingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingBody {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns a stream for writing the body contents.
                ///
                /// The returned `output-stream` is a child resource: it must be dropped
                /// before the parent `outgoing-body` resource is dropped (or finished),
                /// otherwise the `outgoing-body` drop or `finish` will trap.
                ///
                /// Returns success on the first call: the `output-stream` resource for
                /// this `outgoing-body` may be retrieved at most once. Subsequent calls
                /// will return error.
                pub fn write(&self) -> Result<OutputStream, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-body.write"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingBody {
                #[allow(unused_unsafe, clippy::all)]
                /// Finalize an outgoing body, optionally providing trailers. This must be
                /// called to signal that the response is complete. If the `outgoing-body`
                /// is dropped without calling `outgoing-body.finalize`, the implementation
                /// should treat the body as corrupted.
                ///
                /// Fails if the body's `outgoing-request` or `outgoing-response` was
                /// constructed with a Content-Length header, and the contents written
                /// to the body (via `write`) does not match the value given in the
                /// Content-Length.
                pub fn finish(
                    this: OutgoingBody,
                    trailers: Option<Trailers>,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 40],
                        );
                        let (result0_0, result0_1) = match &trailers {
                            Some(e) => (1i32, (e).take_handle() as i32),
                            None => (0i32, 0i32),
                        };
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[static]outgoing-body.finish"]
                            fn wit_import(_: i32, _: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (&this).take_handle() as i32,
                            result0_0,
                            result0_1,
                            ptr1,
                        );
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(8).cast::<u8>());
                                    let v65 = match l3 {
                                        0 => ErrorCode::DnsTimeout,
                                        1 => {
                                            let e65 = {
                                                let l4 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l8 = i32::from(*ptr1.add(28).cast::<u8>());
                                                DnsErrorPayload {
                                                    rcode: match l4 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l5 = *ptr1.add(20).cast::<*mut u8>();
                                                                let l6 = *ptr1.add(24).cast::<usize>();
                                                                let len7 = l6;
                                                                let bytes7 = _rt::Vec::from_raw_parts(
                                                                    l5.cast(),
                                                                    len7,
                                                                    len7,
                                                                );
                                                                _rt::string_lift(bytes7)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    info_code: match l8 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l9 = i32::from(*ptr1.add(30).cast::<u16>());
                                                                l9 as u16
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::DnsError(e65)
                                        }
                                        2 => ErrorCode::DestinationNotFound,
                                        3 => ErrorCode::DestinationUnavailable,
                                        4 => ErrorCode::DestinationIpProhibited,
                                        5 => ErrorCode::DestinationIpUnroutable,
                                        6 => ErrorCode::ConnectionRefused,
                                        7 => ErrorCode::ConnectionTerminated,
                                        8 => ErrorCode::ConnectionTimeout,
                                        9 => ErrorCode::ConnectionReadTimeout,
                                        10 => ErrorCode::ConnectionWriteTimeout,
                                        11 => ErrorCode::ConnectionLimitReached,
                                        12 => ErrorCode::TlsProtocolError,
                                        13 => ErrorCode::TlsCertificateError,
                                        14 => {
                                            let e65 = {
                                                let l10 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l12 = i32::from(*ptr1.add(20).cast::<u8>());
                                                TlsAlertReceivedPayload {
                                                    alert_id: match l10 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l11 = i32::from(*ptr1.add(17).cast::<u8>());
                                                                l11 as u8
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    alert_message: match l12 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l13 = *ptr1.add(24).cast::<*mut u8>();
                                                                let l14 = *ptr1.add(28).cast::<usize>();
                                                                let len15 = l14;
                                                                let bytes15 = _rt::Vec::from_raw_parts(
                                                                    l13.cast(),
                                                                    len15,
                                                                    len15,
                                                                );
                                                                _rt::string_lift(bytes15)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::TlsAlertReceived(e65)
                                        }
                                        15 => ErrorCode::HttpRequestDenied,
                                        16 => ErrorCode::HttpRequestLengthRequired,
                                        17 => {
                                            let e65 = {
                                                let l16 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l16 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l17 = *ptr1.add(24).cast::<i64>();
                                                            l17 as u64
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestBodySize(e65)
                                        }
                                        18 => ErrorCode::HttpRequestMethodInvalid,
                                        19 => ErrorCode::HttpRequestUriInvalid,
                                        20 => ErrorCode::HttpRequestUriTooLong,
                                        21 => {
                                            let e65 = {
                                                let l18 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l18 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l19 = *ptr1.add(20).cast::<i32>();
                                                            l19 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestHeaderSectionSize(e65)
                                        }
                                        22 => {
                                            let e65 = {
                                                let l20 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l20 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l21 = i32::from(*ptr1.add(20).cast::<u8>());
                                                            let l25 = i32::from(*ptr1.add(32).cast::<u8>());
                                                            FieldSizePayload {
                                                                field_name: match l21 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l22 = *ptr1.add(24).cast::<*mut u8>();
                                                                            let l23 = *ptr1.add(28).cast::<usize>();
                                                                            let len24 = l23;
                                                                            let bytes24 = _rt::Vec::from_raw_parts(
                                                                                l22.cast(),
                                                                                len24,
                                                                                len24,
                                                                            );
                                                                            _rt::string_lift(bytes24)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                field_size: match l25 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l26 = *ptr1.add(36).cast::<i32>();
                                                                            l26 as u32
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                            }
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestHeaderSize(e65)
                                        }
                                        23 => {
                                            let e65 = {
                                                let l27 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l27 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l28 = *ptr1.add(20).cast::<i32>();
                                                            l28 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestTrailerSectionSize(e65)
                                        }
                                        24 => {
                                            let e65 = {
                                                let l29 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l33 = i32::from(*ptr1.add(28).cast::<u8>());
                                                FieldSizePayload {
                                                    field_name: match l29 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l30 = *ptr1.add(20).cast::<*mut u8>();
                                                                let l31 = *ptr1.add(24).cast::<usize>();
                                                                let len32 = l31;
                                                                let bytes32 = _rt::Vec::from_raw_parts(
                                                                    l30.cast(),
                                                                    len32,
                                                                    len32,
                                                                );
                                                                _rt::string_lift(bytes32)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    field_size: match l33 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l34 = *ptr1.add(32).cast::<i32>();
                                                                l34 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::HttpRequestTrailerSize(e65)
                                        }
                                        25 => ErrorCode::HttpResponseIncomplete,
                                        26 => {
                                            let e65 = {
                                                let l35 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l35 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l36 = *ptr1.add(20).cast::<i32>();
                                                            l36 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseHeaderSectionSize(e65)
                                        }
                                        27 => {
                                            let e65 = {
                                                let l37 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l41 = i32::from(*ptr1.add(28).cast::<u8>());
                                                FieldSizePayload {
                                                    field_name: match l37 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l38 = *ptr1.add(20).cast::<*mut u8>();
                                                                let l39 = *ptr1.add(24).cast::<usize>();
                                                                let len40 = l39;
                                                                let bytes40 = _rt::Vec::from_raw_parts(
                                                                    l38.cast(),
                                                                    len40,
                                                                    len40,
                                                                );
                                                                _rt::string_lift(bytes40)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    field_size: match l41 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l42 = *ptr1.add(32).cast::<i32>();
                                                                l42 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::HttpResponseHeaderSize(e65)
                                        }
                                        28 => {
                                            let e65 = {
                                                let l43 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l43 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l44 = *ptr1.add(24).cast::<i64>();
                                                            l44 as u64
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseBodySize(e65)
                                        }
                                        29 => {
                                            let e65 = {
                                                let l45 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l45 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l46 = *ptr1.add(20).cast::<i32>();
                                                            l46 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseTrailerSectionSize(e65)
                                        }
                                        30 => {
                                            let e65 = {
                                                let l47 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l51 = i32::from(*ptr1.add(28).cast::<u8>());
                                                FieldSizePayload {
                                                    field_name: match l47 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l48 = *ptr1.add(20).cast::<*mut u8>();
                                                                let l49 = *ptr1.add(24).cast::<usize>();
                                                                let len50 = l49;
                                                                let bytes50 = _rt::Vec::from_raw_parts(
                                                                    l48.cast(),
                                                                    len50,
                                                                    len50,
                                                                );
                                                                _rt::string_lift(bytes50)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    field_size: match l51 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l52 = *ptr1.add(32).cast::<i32>();
                                                                l52 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::HttpResponseTrailerSize(e65)
                                        }
                                        31 => {
                                            let e65 = {
                                                let l53 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l53 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l54 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l55 = *ptr1.add(24).cast::<usize>();
                                                            let len56 = l55;
                                                            let bytes56 = _rt::Vec::from_raw_parts(
                                                                l54.cast(),
                                                                len56,
                                                                len56,
                                                            );
                                                            _rt::string_lift(bytes56)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseTransferCoding(e65)
                                        }
                                        32 => {
                                            let e65 = {
                                                let l57 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l57 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l58 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l59 = *ptr1.add(24).cast::<usize>();
                                                            let len60 = l59;
                                                            let bytes60 = _rt::Vec::from_raw_parts(
                                                                l58.cast(),
                                                                len60,
                                                                len60,
                                                            );
                                                            _rt::string_lift(bytes60)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseContentCoding(e65)
                                        }
                                        33 => ErrorCode::HttpResponseTimeout,
                                        34 => ErrorCode::HttpUpgradeFailed,
                                        35 => ErrorCode::HttpProtocolError,
                                        36 => ErrorCode::LoopDetected,
                                        37 => ErrorCode::ConfigurationError,
                                        n => {
                                            debug_assert_eq!(n, 38, "invalid enum discriminant");
                                            let e65 = {
                                                let l61 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l61 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l62 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l63 = *ptr1.add(24).cast::<usize>();
                                                            let len64 = l63;
                                                            let bytes64 = _rt::Vec::from_raw_parts(
                                                                l62.cast(),
                                                                len64,
                                                                len64,
                                                            );
                                                            _rt::string_lift(bytes64)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::InternalError(e65)
                                        }
                                    };
                                    v65
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl FutureIncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns a pollable which becomes ready when either the Response has
                /// been received, or an error has occurred. When this pollable is ready,
                /// the `get` method will return `some`.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]future-incoming-response.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl FutureIncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the incoming HTTP Response, or an error, once one is ready.
                ///
                /// The outer `option` represents future readiness. Users can wait on this
                /// `option` to become `some` using the `subscribe` method.
                ///
                /// The outer `result` is used to retrieve the response or error at most
                /// once. It will be success on the first call in which the outer option
                /// is `some`, and error on subsequent calls.
                ///
                /// The inner `result` represents that either the incoming HTTP Response
                /// status and headers have received successfully, or that an error
                /// occurred. Errors may also occur while consuming the response body,
                /// but those will be reported by the `incoming-body` and its
                /// `output-stream` child.
                pub fn get(
                    &self,
                ) -> Option<Result<Result<IncomingResponse, ErrorCode>, ()>> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 56]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 56],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]future-incoming-response.get"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(16).cast::<u8>());
                                                match l3 {
                                                    0 => {
                                                        let e = {
                                                            let l4 = *ptr0.add(24).cast::<i32>();
                                                            IncomingResponse::from_handle(l4 as u32)
                                                        };
                                                        Ok(e)
                                                    }
                                                    1 => {
                                                        let e = {
                                                            let l5 = i32::from(*ptr0.add(24).cast::<u8>());
                                                            let v67 = match l5 {
                                                                0 => ErrorCode::DnsTimeout,
                                                                1 => {
                                                                    let e67 = {
                                                                        let l6 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l10 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        DnsErrorPayload {
                                                                            rcode: match l6 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l7 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l8 = *ptr0.add(40).cast::<usize>();
                                                                                        let len9 = l8;
                                                                                        let bytes9 = _rt::Vec::from_raw_parts(
                                                                                            l7.cast(),
                                                                                            len9,
                                                                                            len9,
                                                                                        );
                                                                                        _rt::string_lift(bytes9)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            info_code: match l10 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l11 = i32::from(*ptr0.add(46).cast::<u16>());
                                                                                        l11 as u16
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::DnsError(e67)
                                                                }
                                                                2 => ErrorCode::DestinationNotFound,
                                                                3 => ErrorCode::DestinationUnavailable,
                                                                4 => ErrorCode::DestinationIpProhibited,
                                                                5 => ErrorCode::DestinationIpUnroutable,
                                                                6 => ErrorCode::ConnectionRefused,
                                                                7 => ErrorCode::ConnectionTerminated,
                                                                8 => ErrorCode::ConnectionTimeout,
                                                                9 => ErrorCode::ConnectionReadTimeout,
                                                                10 => ErrorCode::ConnectionWriteTimeout,
                                                                11 => ErrorCode::ConnectionLimitReached,
                                                                12 => ErrorCode::TlsProtocolError,
                                                                13 => ErrorCode::TlsCertificateError,
                                                                14 => {
                                                                    let e67 = {
                                                                        let l12 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l14 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                        TlsAlertReceivedPayload {
                                                                            alert_id: match l12 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l13 = i32::from(*ptr0.add(33).cast::<u8>());
                                                                                        l13 as u8
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            alert_message: match l14 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l15 = *ptr0.add(40).cast::<*mut u8>();
                                                                                        let l16 = *ptr0.add(44).cast::<usize>();
                                                                                        let len17 = l16;
                                                                                        let bytes17 = _rt::Vec::from_raw_parts(
                                                                                            l15.cast(),
                                                                                            len17,
                                                                                            len17,
                                                                                        );
                                                                                        _rt::string_lift(bytes17)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::TlsAlertReceived(e67)
                                                                }
                                                                15 => ErrorCode::HttpRequestDenied,
                                                                16 => ErrorCode::HttpRequestLengthRequired,
                                                                17 => {
                                                                    let e67 = {
                                                                        let l18 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l18 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l19 = *ptr0.add(40).cast::<i64>();
                                                                                    l19 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestBodySize(e67)
                                                                }
                                                                18 => ErrorCode::HttpRequestMethodInvalid,
                                                                19 => ErrorCode::HttpRequestUriInvalid,
                                                                20 => ErrorCode::HttpRequestUriTooLong,
                                                                21 => {
                                                                    let e67 = {
                                                                        let l20 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l20 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l21 = *ptr0.add(36).cast::<i32>();
                                                                                    l21 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSectionSize(e67)
                                                                }
                                                                22 => {
                                                                    let e67 = {
                                                                        let l22 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l22 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l23 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                                    let l27 = i32::from(*ptr0.add(48).cast::<u8>());
                                                                                    FieldSizePayload {
                                                                                        field_name: match l23 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l24 = *ptr0.add(40).cast::<*mut u8>();
                                                                                                    let l25 = *ptr0.add(44).cast::<usize>();
                                                                                                    let len26 = l25;
                                                                                                    let bytes26 = _rt::Vec::from_raw_parts(
                                                                                                        l24.cast(),
                                                                                                        len26,
                                                                                                        len26,
                                                                                                    );
                                                                                                    _rt::string_lift(bytes26)
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                        field_size: match l27 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l28 = *ptr0.add(52).cast::<i32>();
                                                                                                    l28 as u32
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                    }
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSize(e67)
                                                                }
                                                                23 => {
                                                                    let e67 = {
                                                                        let l29 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l29 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l30 = *ptr0.add(36).cast::<i32>();
                                                                                    l30 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSectionSize(e67)
                                                                }
                                                                24 => {
                                                                    let e67 = {
                                                                        let l31 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l35 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l31 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l32 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l33 = *ptr0.add(40).cast::<usize>();
                                                                                        let len34 = l33;
                                                                                        let bytes34 = _rt::Vec::from_raw_parts(
                                                                                            l32.cast(),
                                                                                            len34,
                                                                                            len34,
                                                                                        );
                                                                                        _rt::string_lift(bytes34)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l35 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l36 = *ptr0.add(48).cast::<i32>();
                                                                                        l36 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSize(e67)
                                                                }
                                                                25 => ErrorCode::HttpResponseIncomplete,
                                                                26 => {
                                                                    let e67 = {
                                                                        let l37 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l37 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l38 = *ptr0.add(36).cast::<i32>();
                                                                                    l38 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSectionSize(e67)
                                                                }
                                                                27 => {
                                                                    let e67 = {
                                                                        let l39 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l43 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l39 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l40 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l41 = *ptr0.add(40).cast::<usize>();
                                                                                        let len42 = l41;
                                                                                        let bytes42 = _rt::Vec::from_raw_parts(
                                                                                            l40.cast(),
                                                                                            len42,
                                                                                            len42,
                                                                                        );
                                                                                        _rt::string_lift(bytes42)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l43 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l44 = *ptr0.add(48).cast::<i32>();
                                                                                        l44 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSize(e67)
                                                                }
                                                                28 => {
                                                                    let e67 = {
                                                                        let l45 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l45 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l46 = *ptr0.add(40).cast::<i64>();
                                                                                    l46 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseBodySize(e67)
                                                                }
                                                                29 => {
                                                                    let e67 = {
                                                                        let l47 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l47 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l48 = *ptr0.add(36).cast::<i32>();
                                                                                    l48 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSectionSize(e67)
                                                                }
                                                                30 => {
                                                                    let e67 = {
                                                                        let l49 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l53 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l49 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l50 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l51 = *ptr0.add(40).cast::<usize>();
                                                                                        let len52 = l51;
                                                                                        let bytes52 = _rt::Vec::from_raw_parts(
                                                                                            l50.cast(),
                                                                                            len52,
                                                                                            len52,
                                                                                        );
                                                                                        _rt::string_lift(bytes52)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l53 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l54 = *ptr0.add(48).cast::<i32>();
                                                                                        l54 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSize(e67)
                                                                }
                                                                31 => {
                                                                    let e67 = {
                                                                        let l55 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l55 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l56 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l57 = *ptr0.add(40).cast::<usize>();
                                                                                    let len58 = l57;
                                                                                    let bytes58 = _rt::Vec::from_raw_parts(
                                                                                        l56.cast(),
                                                                                        len58,
                                                                                        len58,
                                                                                    );
                                                                                    _rt::string_lift(bytes58)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTransferCoding(e67)
                                                                }
                                                                32 => {
                                                                    let e67 = {
                                                                        let l59 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l59 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l60 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l61 = *ptr0.add(40).cast::<usize>();
                                                                                    let len62 = l61;
                                                                                    let bytes62 = _rt::Vec::from_raw_parts(
                                                                                        l60.cast(),
                                                                                        len62,
                                                                                        len62,
                                                                                    );
                                                                                    _rt::string_lift(bytes62)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseContentCoding(e67)
                                                                }
                                                                33 => ErrorCode::HttpResponseTimeout,
                                                                34 => ErrorCode::HttpUpgradeFailed,
                                                                35 => ErrorCode::HttpProtocolError,
                                                                36 => ErrorCode::LoopDetected,
                                                                37 => ErrorCode::ConfigurationError,
                                                                n => {
                                                                    debug_assert_eq!(n, 38, "invalid enum discriminant");
                                                                    let e67 = {
                                                                        let l63 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l63 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l64 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l65 = *ptr0.add(40).cast::<usize>();
                                                                                    let len66 = l65;
                                                                                    let bytes66 = _rt::Vec::from_raw_parts(
                                                                                        l64.cast(),
                                                                                        len66,
                                                                                        len66,
                                                                                    );
                                                                                    _rt::string_lift(bytes66)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::InternalError(e67)
                                                                }
                                                            };
                                                            v67
                                                        };
                                                        Err(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            Ok(e)
                                        }
                                        1 => {
                                            let e = ();
                                            Err(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod outgoing_handler {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type OutgoingRequest = super::super::super::wasi::http::types::OutgoingRequest;
            pub type RequestOptions = super::super::super::wasi::http::types::RequestOptions;
            pub type FutureIncomingResponse = super::super::super::wasi::http::types::FutureIncomingResponse;
            pub type ErrorCode = super::super::super::wasi::http::types::ErrorCode;
            #[allow(unused_unsafe, clippy::all)]
            /// This function is invoked with an outgoing HTTP Request, and it returns
            /// a resource `future-incoming-response` which represents an HTTP Response
            /// which may arrive in the future.
            ///
            /// The `options` argument accepts optional parameters for the HTTP
            /// protocol's transport layer.
            ///
            /// This function may return an error if the `outgoing-request` is invalid
            /// or not allowed to be made. Otherwise, protocol errors are reported
            /// through the `future-incoming-response`.
            pub fn handle(
                request: OutgoingRequest,
                options: Option<RequestOptions>,
            ) -> Result<FutureIncomingResponse, ErrorCode> {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
                    let (result0_0, result0_1) = match &options {
                        Some(e) => (1i32, (e).take_handle() as i32),
                        None => (0i32, 0i32),
                    };
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:http/outgoing-handler@0.2.2")]
                    extern "C" {
                        #[link_name = "handle"]
                        fn wit_import(_: i32, _: i32, _: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(
                        (&request).take_handle() as i32,
                        result0_0,
                        result0_1,
                        ptr1,
                    );
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(8).cast::<i32>();
                                super::super::super::wasi::http::types::FutureIncomingResponse::from_handle(
                                    l3 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr1.add(8).cast::<u8>());
                                use super::super::super::wasi::http::types::ErrorCode as V66;
                                let v66 = match l4 {
                                    0 => V66::DnsTimeout,
                                    1 => {
                                        let e66 = {
                                            let l5 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l9 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::DnsErrorPayload {
                                                rcode: match l5 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l6 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l7 = *ptr1.add(24).cast::<usize>();
                                                            let len8 = l7;
                                                            let bytes8 = _rt::Vec::from_raw_parts(
                                                                l6.cast(),
                                                                len8,
                                                                len8,
                                                            );
                                                            _rt::string_lift(bytes8)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                info_code: match l9 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l10 = i32::from(*ptr1.add(30).cast::<u16>());
                                                            l10 as u16
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::DnsError(e66)
                                    }
                                    2 => V66::DestinationNotFound,
                                    3 => V66::DestinationUnavailable,
                                    4 => V66::DestinationIpProhibited,
                                    5 => V66::DestinationIpUnroutable,
                                    6 => V66::ConnectionRefused,
                                    7 => V66::ConnectionTerminated,
                                    8 => V66::ConnectionTimeout,
                                    9 => V66::ConnectionReadTimeout,
                                    10 => V66::ConnectionWriteTimeout,
                                    11 => V66::ConnectionLimitReached,
                                    12 => V66::TlsProtocolError,
                                    13 => V66::TlsCertificateError,
                                    14 => {
                                        let e66 = {
                                            let l11 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l13 = i32::from(*ptr1.add(20).cast::<u8>());
                                            super::super::super::wasi::http::types::TlsAlertReceivedPayload {
                                                alert_id: match l11 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l12 = i32::from(*ptr1.add(17).cast::<u8>());
                                                            l12 as u8
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                alert_message: match l13 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l14 = *ptr1.add(24).cast::<*mut u8>();
                                                            let l15 = *ptr1.add(28).cast::<usize>();
                                                            let len16 = l15;
                                                            let bytes16 = _rt::Vec::from_raw_parts(
                                                                l14.cast(),
                                                                len16,
                                                                len16,
                                                            );
                                                            _rt::string_lift(bytes16)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::TlsAlertReceived(e66)
                                    }
                                    15 => V66::HttpRequestDenied,
                                    16 => V66::HttpRequestLengthRequired,
                                    17 => {
                                        let e66 = {
                                            let l17 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l17 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l18 = *ptr1.add(24).cast::<i64>();
                                                        l18 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestBodySize(e66)
                                    }
                                    18 => V66::HttpRequestMethodInvalid,
                                    19 => V66::HttpRequestUriInvalid,
                                    20 => V66::HttpRequestUriTooLong,
                                    21 => {
                                        let e66 = {
                                            let l19 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l19 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l20 = *ptr1.add(20).cast::<i32>();
                                                        l20 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestHeaderSectionSize(e66)
                                    }
                                    22 => {
                                        let e66 = {
                                            let l21 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l21 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l22 = i32::from(*ptr1.add(20).cast::<u8>());
                                                        let l26 = i32::from(*ptr1.add(32).cast::<u8>());
                                                        super::super::super::wasi::http::types::FieldSizePayload {
                                                            field_name: match l22 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l23 = *ptr1.add(24).cast::<*mut u8>();
                                                                        let l24 = *ptr1.add(28).cast::<usize>();
                                                                        let len25 = l24;
                                                                        let bytes25 = _rt::Vec::from_raw_parts(
                                                                            l23.cast(),
                                                                            len25,
                                                                            len25,
                                                                        );
                                                                        _rt::string_lift(bytes25)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            field_size: match l26 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l27 = *ptr1.add(36).cast::<i32>();
                                                                        l27 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestHeaderSize(e66)
                                    }
                                    23 => {
                                        let e66 = {
                                            let l28 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l28 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l29 = *ptr1.add(20).cast::<i32>();
                                                        l29 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestTrailerSectionSize(e66)
                                    }
                                    24 => {
                                        let e66 = {
                                            let l30 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l34 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::FieldSizePayload {
                                                field_name: match l30 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l31 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l32 = *ptr1.add(24).cast::<usize>();
                                                            let len33 = l32;
                                                            let bytes33 = _rt::Vec::from_raw_parts(
                                                                l31.cast(),
                                                                len33,
                                                                len33,
                                                            );
                                                            _rt::string_lift(bytes33)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l34 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l35 = *ptr1.add(32).cast::<i32>();
                                                            l35 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::HttpRequestTrailerSize(e66)
                                    }
                                    25 => V66::HttpResponseIncomplete,
                                    26 => {
                                        let e66 = {
                                            let l36 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l36 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l37 = *ptr1.add(20).cast::<i32>();
                                                        l37 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseHeaderSectionSize(e66)
                                    }
                                    27 => {
                                        let e66 = {
                                            let l38 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l42 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::FieldSizePayload {
                                                field_name: match l38 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l39 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l40 = *ptr1.add(24).cast::<usize>();
                                                            let len41 = l40;
                                                            let bytes41 = _rt::Vec::from_raw_parts(
                                                                l39.cast(),
                                                                len41,
                                                                len41,
                                                            );
                                                            _rt::string_lift(bytes41)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l42 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l43 = *ptr1.add(32).cast::<i32>();
                                                            l43 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::HttpResponseHeaderSize(e66)
                                    }
                                    28 => {
                                        let e66 = {
                                            let l44 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l44 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l45 = *ptr1.add(24).cast::<i64>();
                                                        l45 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseBodySize(e66)
                                    }
                                    29 => {
                                        let e66 = {
                                            let l46 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l46 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l47 = *ptr1.add(20).cast::<i32>();
                                                        l47 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseTrailerSectionSize(e66)
                                    }
                                    30 => {
                                        let e66 = {
                                            let l48 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l52 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::FieldSizePayload {
                                                field_name: match l48 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l49 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l50 = *ptr1.add(24).cast::<usize>();
                                                            let len51 = l50;
                                                            let bytes51 = _rt::Vec::from_raw_parts(
                                                                l49.cast(),
                                                                len51,
                                                                len51,
                                                            );
                                                            _rt::string_lift(bytes51)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l52 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l53 = *ptr1.add(32).cast::<i32>();
                                                            l53 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::HttpResponseTrailerSize(e66)
                                    }
                                    31 => {
                                        let e66 = {
                                            let l54 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l54 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l55 = *ptr1.add(20).cast::<*mut u8>();
                                                        let l56 = *ptr1.add(24).cast::<usize>();
                                                        let len57 = l56;
                                                        let bytes57 = _rt::Vec::from_raw_parts(
                                                            l55.cast(),
                                                            len57,
                                                            len57,
                                                        );
                                                        _rt::string_lift(bytes57)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseTransferCoding(e66)
                                    }
                                    32 => {
                                        let e66 = {
                                            let l58 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l58 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l59 = *ptr1.add(20).cast::<*mut u8>();
                                                        let l60 = *ptr1.add(24).cast::<usize>();
                                                        let len61 = l60;
                                                        let bytes61 = _rt::Vec::from_raw_parts(
                                                            l59.cast(),
                                                            len61,
                                                            len61,
                                                        );
                                                        _rt::string_lift(bytes61)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseContentCoding(e66)
                                    }
                                    33 => V66::HttpResponseTimeout,
                                    34 => V66::HttpUpgradeFailed,
                                    35 => V66::HttpProtocolError,
                                    36 => V66::LoopDetected,
                                    37 => V66::ConfigurationError,
                                    n => {
                                        debug_assert_eq!(n, 38, "invalid enum discriminant");
                                        let e66 = {
                                            let l62 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l62 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l63 = *ptr1.add(20).cast::<*mut u8>();
                                                        let l64 = *ptr1.add(24).cast::<usize>();
                                                        let len65 = l64;
                                                        let bytes65 = _rt::Vec::from_raw_parts(
                                                            l63.cast(),
                                                            len65,
                                                            len65,
                                                        );
                                                        _rt::string_lift(bytes65)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::InternalError(e66)
                                    }
                                };
                                v66
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod io {
        #[allow(dead_code, clippy::all)]
        pub mod poll {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// `pollable` represents a single I/O event which may be ready, or not.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Pollable {
                handle: _rt::Resource<Pollable>,
            }
            impl Pollable {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Pollable {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/poll@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]pollable"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the readiness of a pollable. This function never blocks.
                ///
                /// Returns `true` when the pollable is ready, and `false` otherwise.
                pub fn ready(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]pollable.ready"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// `block` returns immediately if the pollable is ready, and otherwise
                /// blocks until ready.
                ///
                /// This function is equivalent to calling `poll.poll` on a list
                /// containing only this pollable.
                pub fn block(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]pollable.block"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Poll for completion on a set of pollables.
            ///
            /// This function takes a list of pollables, which identify I/O sources of
            /// interest, and waits until one or more of the events is ready for I/O.
            ///
            /// The result `list<u32>` contains one or more indices of handles in the
            /// argument list that is ready for I/O.
            ///
            /// This function traps if either:
            /// - the list is empty, or:
            /// - the list contains more elements than can be indexed with a `u32` value.
            ///
            /// A timeout can be implemented by adding a pollable from the
            /// wasi-clocks API to the list.
            ///
            /// This function does not return a `result`; polling in itself does not
            /// do any I/O so it doesn't fail. If any of the I/O sources identified by
            /// the pollables has an error, it is indicated by marking the source as
            /// being ready for I/O.
            pub fn poll(in_: &[&Pollable]) -> _rt::Vec<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = in_;
                    let len0 = vec0.len();
                    let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec0.len() * 4,
                        4,
                    );
                    let result0 = if layout0.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).handle() as i32;
                        }
                    }
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:io/poll@0.2.2")]
                    extern "C" {
                        #[link_name = "poll"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result0, len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    if layout0.size() != 0 {
                        _rt::alloc::dealloc(result0.cast(), layout0);
                    }
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod error {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// A resource which represents some error information.
            ///
            /// The only method provided by this resource is `to-debug-string`,
            /// which provides some human-readable information about the error.
            ///
            /// In the `wasi:io` package, this resource is returned through the
            /// `wasi:io/streams/stream-error` type.
            ///
            /// To provide more specific error information, other interfaces may
            /// offer functions to "downcast" this error into more specific types. For example,
            /// errors returned from streams derived from filesystem types can be described using
            /// the filesystem's own error-code type. This is done using the function
            /// `wasi:filesystem/types/filesystem-error-code`, which takes a `borrow<error>`
            /// parameter and returns an `option<wasi:filesystem/types/error-code>`.
            ///
            /// The set of functions which can "downcast" an `error` into a more
            /// concrete type is open.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Error {
                handle: _rt::Resource<Error>,
            }
            impl Error {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Error {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/error@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]error"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Error {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns a string that is suitable to assist humans in debugging
                /// this error.
                ///
                /// WARNING: The returned string should not be consumed mechanically!
                /// It may change across platforms, hosts, or other implementation
                /// details. Parsing this string is a major platform-compatibility
                /// hazard.
                pub fn to_debug_string(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/error@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]error.to-debug-string"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod streams {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Error = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An error for input-stream and output-stream operations.
            pub enum StreamError {
                /// The last operation (a write or flush) failed before completion.
                ///
                /// More information is available in the `error` payload.
                ///
                /// After this, the stream will be closed. All future operations return
                /// `stream-error::closed`.
                LastOperationFailed(Error),
                /// The stream is closed: no more input will be accepted by the
                /// stream. A closed output-stream will return this error on all
                /// future operations.
                Closed,
            }
            impl ::core::fmt::Debug for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        StreamError::LastOperationFailed(e) => {
                            f.debug_tuple("StreamError::LastOperationFailed")
                                .field(e)
                                .finish()
                        }
                        StreamError::Closed => {
                            f.debug_tuple("StreamError::Closed").finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for StreamError {}
            /// An input bytestream.
            ///
            /// `input-stream`s are *non-blocking* to the extent practical on underlying
            /// platforms. I/O operations always return promptly; if fewer bytes are
            /// promptly available than requested, they return the number of bytes promptly
            /// available, which could even be zero. To wait for data to be available,
            /// use the `subscribe` function to obtain a `pollable` which can be polled
            /// for using `wasi:io/poll`.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct InputStream {
                handle: _rt::Resource<InputStream>,
            }
            impl InputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for InputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]input-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// An output bytestream.
            ///
            /// `output-stream`s are *non-blocking* to the extent practical on
            /// underlying platforms. Except where specified otherwise, I/O operations also
            /// always return promptly, after the number of bytes that can be written
            /// promptly, which could even be zero. To wait for the stream to be ready to
            /// accept data, the `subscribe` function to obtain a `pollable` which can be
            /// polled for using `wasi:io/poll`.
            ///
            /// Dropping an `output-stream` while there's still an active write in
            /// progress may result in the data being lost. Before dropping the stream,
            /// be sure to fully flush your writes.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutputStream {
                handle: _rt::Resource<OutputStream>,
            }
            impl OutputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]output-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a non-blocking read from the stream.
                ///
                /// When the source of a `read` is binary data, the bytes from the source
                /// are returned verbatim. When the source of a `read` is known to the
                /// implementation to be text, bytes containing the UTF-8 encoding of the
                /// text are returned.
                ///
                /// This function returns a list of bytes containing the read data,
                /// when successful. The returned list will contain up to `len` bytes;
                /// it may return fewer than requested, but not more. The list is
                /// empty when no bytes are available for reading at this time. The
                /// pollable given by `subscribe` will be ready when more bytes are
                /// available.
                ///
                /// This function fails with a `stream-error` when the operation
                /// encounters an error, giving `last-operation-failed`, or when the
                /// stream is closed, giving `closed`.
                ///
                /// When the caller gives a `len` of 0, it represents a request to
                /// read 0 bytes. If the stream is still open, this call should
                /// succeed and return an empty list, or otherwise fail with `closed`.
                ///
                /// The `len` parameter is a `u64`, which could represent a list of u8 which
                /// is not possible to allocate in wasm32, or not desirable to allocate as
                /// as a return value by the callee. The callee may return a list of bytes
                /// less than `len` in size while more bytes are available for reading.
                pub fn read(&self, len: u64) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]input-stream.read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read bytes from a stream, after blocking until at least one byte can
                /// be read. Except for blocking, behavior is identical to `read`.
                pub fn blocking_read(
                    &self,
                    len: u64,
                ) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Skip bytes from a stream. Returns number of bytes skipped.
                ///
                /// Behaves identical to `read`, except instead of returning a list
                /// of bytes, returns the number of bytes consumed from the stream.
                pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]input-stream.skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Skip bytes from a stream, after blocking until at least one byte
                /// can be skipped. Except for blocking behavior, identical to `skip`.
                pub fn blocking_skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once either the specified stream
                /// has bytes available to read or the other end of the stream has been
                /// closed.
                /// The created `pollable` is a child resource of the `input-stream`.
                /// Implementations may trap if the `input-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]input-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Check readiness for writing. This function never blocks.
                ///
                /// Returns the number of bytes permitted for the next call to `write`,
                /// or an error. Calling `write` with more bytes than this function has
                /// permitted will trap.
                ///
                /// When this function returns 0 bytes, the `subscribe` pollable will
                /// become ready when this function will report at least 1 byte, or an
                /// error.
                pub fn check_write(&self) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.check-write"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write. This function never blocks.
                ///
                /// When the destination of a `write` is binary data, the bytes from
                /// `contents` are written verbatim. When the destination of a `write` is
                /// known to the implementation to be text, the bytes of `contents` are
                /// transcoded from UTF-8 into the encoding of the destination and then
                /// written.
                ///
                /// Precondition: check-write gave permit of Ok(n) and contents has a
                /// length of less than or equal to n. Otherwise, this function will trap.
                ///
                /// returns Err(closed) without writing if the stream has closed since
                /// the last call to check-write provided a permit.
                pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                /// until all of these operations are complete, or an error occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write`, and `flush`, and is implemented with the
                /// following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while !contents.is_empty() {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, contents.len());
                /// let (chunk, rest) = contents.split_at(len);
                /// this.write(chunk  );            // eliding error handling
                /// contents = rest;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                pub fn blocking_write_and_flush(
                    &self,
                    contents: &[u8],
                ) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-and-flush"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Request to flush buffered output. This function never blocks.
                ///
                /// This tells the output-stream that the caller intends any buffered
                /// output to be flushed. the output which is expected to be flushed
                /// is all that has been passed to `write` prior to this call.
                ///
                /// Upon calling this function, the `output-stream` will not accept any
                /// writes (`check-write` will return `ok(0)`) until the flush has
                /// completed. The `subscribe` pollable will become ready when the
                /// flush has completed and the stream can accept more writes.
                pub fn flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Request to flush buffered output, and block until flush completes
                /// and stream is ready for writing again.
                pub fn blocking_flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the output-stream
                /// is ready for more writing, or an error has occurred. When this
                /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                /// error.
                ///
                /// If the stream is closed, this pollable is always ready immediately.
                ///
                /// The created `pollable` is a child resource of the `output-stream`.
                /// Implementations may trap if the `output-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Write zeroes to a stream.
                ///
                /// This should be used precisely like `write` with the exact same
                /// preconditions (must use check-write first), but instead of
                /// passing a list of bytes, you simply pass the number of zero-bytes
                /// that should be written.
                pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write-zeroes"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write of up to 4096 zeroes, and then flush the stream.
                /// Block until all of these operations are complete, or an error
                /// occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                /// the following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while num_zeroes != 0 {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, num_zeroes);
                /// this.write-zeroes(len);         // eliding error handling
                /// num_zeroes -= len;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                pub fn blocking_write_zeroes_and_flush(
                    &self,
                    len: u64,
                ) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-zeroes-and-flush"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from one stream and write to another.
                ///
                /// The behavior of splice is equivalent to:
                /// 1. calling `check-write` on the `output-stream`
                /// 2. calling `read` on the `input-stream` with the smaller of the
                /// `check-write` permitted length and the `len` provided to `splice`
                /// 3. calling `write` on the `output-stream` with that read data.
                ///
                /// Any error reported by the call to `check-write`, `read`, or
                /// `write` ends the splice and reports that error.
                ///
                /// This function returns the number of bytes transferred; it may be less
                /// than `len`.
                pub fn splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from one stream and write to another, with blocking.
                ///
                /// This is similar to `splice`, except that it blocks until the
                /// `output-stream` is ready for writing, and the `input-stream`
                /// is ready for reading, before performing the `splice`.
                pub fn blocking_splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod random {
        #[allow(dead_code, clippy::all)]
        pub mod random {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Return `len` cryptographically-secure random or pseudo-random bytes.
            ///
            /// This function must produce data at least as cryptographically secure and
            /// fast as an adequately seeded cryptographically-secure pseudo-random
            /// number generator (CSPRNG). It must not block, from the perspective of
            /// the calling program, under any circumstances, including on the first
            /// request and on requests for numbers of bytes. The returned data must
            /// always be unpredictable.
            ///
            /// This function must always return fresh data. Deterministic environments
            /// must omit this function, rather than implementing it with deterministic
            /// data.
            pub fn get_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.2")]
                    extern "C" {
                        #[link_name = "get-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Return a cryptographically-secure random or pseudo-random `u64` value.
            ///
            /// This function returns the same type of data as `get-random-bytes`,
            /// represented as a `u64`.
            pub fn get_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.2")]
                    extern "C" {
                        #[link_name = "get-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod insecure {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Return `len` insecure pseudo-random bytes.
            ///
            /// This function is not cryptographically secure. Do not use it for
            /// anything related to security.
            ///
            /// There are no requirements on the values of the returned bytes, however
            /// implementations are encouraged to return evenly distributed values with
            /// a long period.
            pub fn get_insecure_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure@0.2.2")]
                    extern "C" {
                        #[link_name = "get-insecure-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Return an insecure pseudo-random `u64` value.
            ///
            /// This function returns the same type of pseudo-random data as
            /// `get-insecure-random-bytes`, represented as a `u64`.
            pub fn get_insecure_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure@0.2.2")]
                    extern "C" {
                        #[link_name = "get-insecure-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod insecure_seed {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            /// Return a 128-bit value that may contain a pseudo-random value.
            ///
            /// The returned value is not required to be computed from a CSPRNG, and may
            /// even be entirely deterministic. Host implementations are encouraged to
            /// provide pseudo-random values to any program exposed to
            /// attacker-controlled content, to enable DoS protection built into many
            /// languages' hash-map implementations.
            ///
            /// This function is intended to only be called once, by a source language
            /// to initialize Denial Of Service (DoS) protection in its hash-map
            /// implementation.
            ///
            /// # Expected future evolution
            ///
            /// This will likely be changed to a value import, to prevent it from being
            /// called multiple times and potentially used for purposes other than DoS
            /// protection.
            pub fn insecure_seed() -> (u64, u64) {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure-seed@0.2.2")]
                    extern "C" {
                        #[link_name = "insecure-seed"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    (l1 as u64, l2 as u64)
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod sockets {
        #[allow(dead_code, clippy::all)]
        pub mod network {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// An opaque resource that represents access to (a subset of) the network.
            /// This enables context-based security for networking.
            /// There is no need for this to map 1:1 to a physical network interface.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Network {
                handle: _rt::Resource<Network>,
            }
            impl Network {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Network {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/network@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]network"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Error codes.
            ///
            /// In theory, every API can return any error code.
            /// In practice, API's typically only return the errors documented per API
            /// combined with a couple of errors that are always possible:
            /// - `unknown`
            /// - `access-denied`
            /// - `not-supported`
            /// - `out-of-memory`
            /// - `concurrency-conflict`
            ///
            /// See each individual API for what the POSIX equivalents are. They sometimes differ per API.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ErrorCode {
                /// Unknown error
                Unknown,
                /// Access denied.
                ///
                /// POSIX equivalent: EACCES, EPERM
                AccessDenied,
                /// The operation is not supported.
                ///
                /// POSIX equivalent: EOPNOTSUPP
                NotSupported,
                /// One of the arguments is invalid.
                ///
                /// POSIX equivalent: EINVAL
                InvalidArgument,
                /// Not enough memory to complete the operation.
                ///
                /// POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY
                OutOfMemory,
                /// The operation timed out before it could finish completely.
                Timeout,
                /// This operation is incompatible with another asynchronous operation that is already in progress.
                ///
                /// POSIX equivalent: EALREADY
                ConcurrencyConflict,
                /// Trying to finish an asynchronous operation that:
                /// - has not been started yet, or:
                /// - was already finished by a previous `finish-*` call.
                ///
                /// Note: this is scheduled to be removed when `future`s are natively supported.
                NotInProgress,
                /// The operation has been aborted because it could not be completed immediately.
                ///
                /// Note: this is scheduled to be removed when `future`s are natively supported.
                WouldBlock,
                /// The operation is not valid in the socket's current state.
                InvalidState,
                /// A new socket resource could not be created because of a system limit.
                NewSocketLimit,
                /// A bind operation failed because the provided address is not an address that the `network` can bind to.
                AddressNotBindable,
                /// A bind operation failed because the provided address is already in use or because there are no ephemeral ports available.
                AddressInUse,
                /// The remote address is not reachable
                RemoteUnreachable,
                /// The TCP connection was forcefully rejected
                ConnectionRefused,
                /// The TCP connection was reset.
                ConnectionReset,
                /// A TCP connection was aborted.
                ConnectionAborted,
                /// The size of a datagram sent to a UDP socket exceeded the maximum
                /// supported size.
                DatagramTooLarge,
                /// Name does not exist or has no suitable associated IP addresses.
                NameUnresolvable,
                /// A temporary failure in name resolution occurred.
                TemporaryResolverFailure,
                /// A permanent failure in name resolution occurred.
                PermanentResolverFailure,
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Unknown => "unknown",
                        ErrorCode::AccessDenied => "access-denied",
                        ErrorCode::NotSupported => "not-supported",
                        ErrorCode::InvalidArgument => "invalid-argument",
                        ErrorCode::OutOfMemory => "out-of-memory",
                        ErrorCode::Timeout => "timeout",
                        ErrorCode::ConcurrencyConflict => "concurrency-conflict",
                        ErrorCode::NotInProgress => "not-in-progress",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::InvalidState => "invalid-state",
                        ErrorCode::NewSocketLimit => "new-socket-limit",
                        ErrorCode::AddressNotBindable => "address-not-bindable",
                        ErrorCode::AddressInUse => "address-in-use",
                        ErrorCode::RemoteUnreachable => "remote-unreachable",
                        ErrorCode::ConnectionRefused => "connection-refused",
                        ErrorCode::ConnectionReset => "connection-reset",
                        ErrorCode::ConnectionAborted => "connection-aborted",
                        ErrorCode::DatagramTooLarge => "datagram-too-large",
                        ErrorCode::NameUnresolvable => "name-unresolvable",
                        ErrorCode::TemporaryResolverFailure => {
                            "temporary-resolver-failure"
                        }
                        ErrorCode::PermanentResolverFailure => {
                            "permanent-resolver-failure"
                        }
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Unknown => "Unknown error",
                        ErrorCode::AccessDenied => {
                            "Access denied.

            POSIX equivalent: EACCES, EPERM"
                        }
                        ErrorCode::NotSupported => {
                            "The operation is not supported.

            POSIX equivalent: EOPNOTSUPP"
                        }
                        ErrorCode::InvalidArgument => {
                            "One of the arguments is invalid.

            POSIX equivalent: EINVAL"
                        }
                        ErrorCode::OutOfMemory => {
                            "Not enough memory to complete the operation.

            POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY"
                        }
                        ErrorCode::Timeout => {
                            "The operation timed out before it could finish completely."
                        }
                        ErrorCode::ConcurrencyConflict => {
                            "This operation is incompatible with another asynchronous operation that is already in progress.

            POSIX equivalent: EALREADY"
                        }
                        ErrorCode::NotInProgress => {
                            "Trying to finish an asynchronous operation that:
            - has not been started yet, or:
            - was already finished by a previous `finish-*` call.

            Note: this is scheduled to be removed when `future`s are natively supported."
                        }
                        ErrorCode::WouldBlock => {
                            "The operation has been aborted because it could not be completed immediately.

            Note: this is scheduled to be removed when `future`s are natively supported."
                        }
                        ErrorCode::InvalidState => {
                            "The operation is not valid in the socket's current state."
                        }
                        ErrorCode::NewSocketLimit => {
                            "A new socket resource could not be created because of a system limit."
                        }
                        ErrorCode::AddressNotBindable => {
                            "A bind operation failed because the provided address is not an address that the `network` can bind to."
                        }
                        ErrorCode::AddressInUse => {
                            "A bind operation failed because the provided address is already in use or because there are no ephemeral ports available."
                        }
                        ErrorCode::RemoteUnreachable => {
                            "The remote address is not reachable"
                        }
                        ErrorCode::ConnectionRefused => {
                            "The TCP connection was forcefully rejected"
                        }
                        ErrorCode::ConnectionReset => "The TCP connection was reset.",
                        ErrorCode::ConnectionAborted => "A TCP connection was aborted.",
                        ErrorCode::DatagramTooLarge => {
                            "The size of a datagram sent to a UDP socket exceeded the maximum
            supported size."
                        }
                        ErrorCode::NameUnresolvable => {
                            "Name does not exist or has no suitable associated IP addresses."
                        }
                        ErrorCode::TemporaryResolverFailure => {
                            "A temporary failure in name resolution occurred."
                        }
                        ErrorCode::PermanentResolverFailure => {
                            "A permanent failure in name resolution occurred."
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), * self as i32)
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Unknown,
                        1 => ErrorCode::AccessDenied,
                        2 => ErrorCode::NotSupported,
                        3 => ErrorCode::InvalidArgument,
                        4 => ErrorCode::OutOfMemory,
                        5 => ErrorCode::Timeout,
                        6 => ErrorCode::ConcurrencyConflict,
                        7 => ErrorCode::NotInProgress,
                        8 => ErrorCode::WouldBlock,
                        9 => ErrorCode::InvalidState,
                        10 => ErrorCode::NewSocketLimit,
                        11 => ErrorCode::AddressNotBindable,
                        12 => ErrorCode::AddressInUse,
                        13 => ErrorCode::RemoteUnreachable,
                        14 => ErrorCode::ConnectionRefused,
                        15 => ErrorCode::ConnectionReset,
                        16 => ErrorCode::ConnectionAborted,
                        17 => ErrorCode::DatagramTooLarge,
                        18 => ErrorCode::NameUnresolvable,
                        19 => ErrorCode::TemporaryResolverFailure,
                        20 => ErrorCode::PermanentResolverFailure,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum IpAddressFamily {
                /// Similar to `AF_INET` in POSIX.
                Ipv4,
                /// Similar to `AF_INET6` in POSIX.
                Ipv6,
            }
            impl ::core::fmt::Debug for IpAddressFamily {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        IpAddressFamily::Ipv4 => {
                            f.debug_tuple("IpAddressFamily::Ipv4").finish()
                        }
                        IpAddressFamily::Ipv6 => {
                            f.debug_tuple("IpAddressFamily::Ipv6").finish()
                        }
                    }
                }
            }
            impl IpAddressFamily {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> IpAddressFamily {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => IpAddressFamily::Ipv4,
                        1 => IpAddressFamily::Ipv6,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            pub type Ipv4Address = (u8, u8, u8, u8);
            pub type Ipv6Address = (u16, u16, u16, u16, u16, u16, u16, u16);
            #[derive(Clone, Copy)]
            pub enum IpAddress {
                Ipv4(Ipv4Address),
                Ipv6(Ipv6Address),
            }
            impl ::core::fmt::Debug for IpAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        IpAddress::Ipv4(e) => {
                            f.debug_tuple("IpAddress::Ipv4").field(e).finish()
                        }
                        IpAddress::Ipv6(e) => {
                            f.debug_tuple("IpAddress::Ipv6").field(e).finish()
                        }
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Ipv4SocketAddress {
                /// sin_port
                pub port: u16,
                /// sin_addr
                pub address: Ipv4Address,
            }
            impl ::core::fmt::Debug for Ipv4SocketAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Ipv4SocketAddress")
                        .field("port", &self.port)
                        .field("address", &self.address)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Ipv6SocketAddress {
                /// sin6_port
                pub port: u16,
                /// sin6_flowinfo
                pub flow_info: u32,
                /// sin6_addr
                pub address: Ipv6Address,
                /// sin6_scope_id
                pub scope_id: u32,
            }
            impl ::core::fmt::Debug for Ipv6SocketAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Ipv6SocketAddress")
                        .field("port", &self.port)
                        .field("flow-info", &self.flow_info)
                        .field("address", &self.address)
                        .field("scope-id", &self.scope_id)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum IpSocketAddress {
                Ipv4(Ipv4SocketAddress),
                Ipv6(Ipv6SocketAddress),
            }
            impl ::core::fmt::Debug for IpSocketAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        IpSocketAddress::Ipv4(e) => {
                            f.debug_tuple("IpSocketAddress::Ipv4").field(e).finish()
                        }
                        IpSocketAddress::Ipv6(e) => {
                            f.debug_tuple("IpSocketAddress::Ipv6").field(e).finish()
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod instance_network {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            #[allow(unused_unsafe, clippy::all)]
            /// Get a handle to the default network.
            pub fn instance_network() -> Network {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/instance-network@0.2.2")]
                    extern "C" {
                        #[link_name = "instance-network"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::sockets::network::Network::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod udp {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpSocketAddress = super::super::super::wasi::sockets::network::IpSocketAddress;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            /// A received datagram.
            #[derive(Clone)]
            pub struct IncomingDatagram {
                /// The payload.
                ///
                /// Theoretical max size: ~64 KiB. In practice, typically less than 1500 bytes.
                pub data: _rt::Vec<u8>,
                /// The source address.
                ///
                /// This field is guaranteed to match the remote address the stream was initialized with, if any.
                ///
                /// Equivalent to the `src_addr` out parameter of `recvfrom`.
                pub remote_address: IpSocketAddress,
            }
            impl ::core::fmt::Debug for IncomingDatagram {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("IncomingDatagram")
                        .field("data", &self.data)
                        .field("remote-address", &self.remote_address)
                        .finish()
                }
            }
            /// A datagram to be sent out.
            #[derive(Clone)]
            pub struct OutgoingDatagram {
                /// The payload.
                pub data: _rt::Vec<u8>,
                /// The destination address.
                ///
                /// The requirements on this field depend on how the stream was initialized:
                /// - with a remote address: this field must be None or match the stream's remote address exactly.
                /// - without a remote address: this field is required.
                ///
                /// If this value is None, the send operation is equivalent to `send` in POSIX. Otherwise it is equivalent to `sendto`.
                pub remote_address: Option<IpSocketAddress>,
            }
            impl ::core::fmt::Debug for OutgoingDatagram {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("OutgoingDatagram")
                        .field("data", &self.data)
                        .field("remote-address", &self.remote_address)
                        .finish()
                }
            }
            /// A UDP socket handle.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct UdpSocket {
                handle: _rt::Resource<UdpSocket>,
            }
            impl UdpSocket {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for UdpSocket {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]udp-socket"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingDatagramStream {
                handle: _rt::Resource<IncomingDatagramStream>,
            }
            impl IncomingDatagramStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingDatagramStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-datagram-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingDatagramStream {
                handle: _rt::Resource<OutgoingDatagramStream>,
            }
            impl OutgoingDatagramStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingDatagramStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-datagram-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Bind the socket to a specific network on the provided IP address and port.
                ///
                /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which
                /// network interface(s) to bind to.
                /// If the port is zero, the socket will be bound to a random free port.
                ///
                /// # Typical errors
                /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)
                /// - `invalid-state`:             The socket is already bound. (EINVAL)
                /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)
                /// - `address-in-use`:            Address is already in use. (EADDRINUSE)
                /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)
                /// - `not-in-progress`:           A `bind` operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// Unlike in POSIX, in WASI the bind operation is async. This enables
                /// interactive WASI hosts to inject permission prompts. Runtimes that
                /// don't want to make use of this ability can simply call the native
                /// `bind` as part of either `start-bind` or `finish-bind`.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>
                /// - <https://man7.org/linux/man-pages/man2/bind.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>
                pub fn start_bind(
                    &self,
                    network: &Network,
                    local_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match local_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.start-bind"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_bind(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.finish-bind"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Set up inbound & outbound communication channels, optionally to a specific peer.
                ///
                /// This function only changes the local socket configuration and does not generate any network traffic.
                /// On success, the `remote-address` of the socket is updated. The `local-address` may be updated as well,
                /// based on the best network path to `remote-address`.
                ///
                /// When a `remote-address` is provided, the returned streams are limited to communicating with that specific peer:
                /// - `send` can only be used to send to this destination.
                /// - `receive` will only return datagrams sent from the provided `remote-address`.
                ///
                /// This method may be called multiple times on the same socket to change its association, but
                /// only the most recently returned pair of streams will be operational. Implementations may trap if
                /// the streams returned by a previous invocation haven't been dropped yet before calling `stream` again.
                ///
                /// The POSIX equivalent in pseudo-code is:
                /// ```text
                /// if (was previously connected) {
                /// connect(s, AF_UNSPEC)
                /// }
                /// if (remote_address is Some) {
                /// connect(s, remote_address)
                /// }
                /// ```
                ///
                /// Unlike in POSIX, the socket must already be explicitly bound.
                ///
                /// # Typical errors
                /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)
                /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)
                /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)
                /// - `invalid-state`:             The socket is not bound.
                /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)
                /// - `remote-unreachable`:        The remote address is not reachable. (ECONNRESET, ENETRESET, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)
                /// - `connection-refused`:        The connection was refused. (ECONNREFUSED)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>
                /// - <https://man7.org/linux/man-pages/man2/connect.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>
                /// - <https://man.freebsd.org/cgi/man.cgi?connect>
                pub fn stream(
                    &self,
                    remote_address: Option<IpSocketAddress>,
                ) -> Result<
                    (IncomingDatagramStream, OutgoingDatagramStream),
                    ErrorCode,
                > {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let (
                            result6_0,
                            result6_1,
                            result6_2,
                            result6_3,
                            result6_4,
                            result6_5,
                            result6_6,
                            result6_7,
                            result6_8,
                            result6_9,
                            result6_10,
                            result6_11,
                            result6_12,
                        ) = match remote_address {
                            Some(e) => {
                                use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                                let (
                                    result5_0,
                                    result5_1,
                                    result5_2,
                                    result5_3,
                                    result5_4,
                                    result5_5,
                                    result5_6,
                                    result5_7,
                                    result5_8,
                                    result5_9,
                                    result5_10,
                                    result5_11,
                                ) = match e {
                                    V4::Ipv4(e) => {
                                        let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                            port: port0,
                                            address: address0,
                                        } = e;
                                        let (t1_0, t1_1, t1_2, t1_3) = address0;
                                        (
                                            0i32,
                                            _rt::as_i32(port0),
                                            _rt::as_i32(t1_0),
                                            _rt::as_i32(t1_1),
                                            _rt::as_i32(t1_2),
                                            _rt::as_i32(t1_3),
                                            0i32,
                                            0i32,
                                            0i32,
                                            0i32,
                                            0i32,
                                            0i32,
                                        )
                                    }
                                    V4::Ipv6(e) => {
                                        let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                            port: port2,
                                            flow_info: flow_info2,
                                            address: address2,
                                            scope_id: scope_id2,
                                        } = e;
                                        let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                        (
                                            1i32,
                                            _rt::as_i32(port2),
                                            _rt::as_i32(flow_info2),
                                            _rt::as_i32(t3_0),
                                            _rt::as_i32(t3_1),
                                            _rt::as_i32(t3_2),
                                            _rt::as_i32(t3_3),
                                            _rt::as_i32(t3_4),
                                            _rt::as_i32(t3_5),
                                            _rt::as_i32(t3_6),
                                            _rt::as_i32(t3_7),
                                            _rt::as_i32(scope_id2),
                                        )
                                    }
                                };
                                (
                                    1i32,
                                    result5_0,
                                    result5_1,
                                    result5_2,
                                    result5_3,
                                    result5_4,
                                    result5_5,
                                    result5_6,
                                    result5_7,
                                    result5_8,
                                    result5_9,
                                    result5_10,
                                    result5_11,
                                )
                            }
                            None => {
                                (
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                        };
                        let ptr7 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.stream"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            result6_0,
                            result6_1,
                            result6_2,
                            result6_3,
                            result6_4,
                            result6_5,
                            result6_6,
                            result6_7,
                            result6_8,
                            result6_9,
                            result6_10,
                            result6_11,
                            result6_12,
                            ptr7,
                        );
                        let l8 = i32::from(*ptr7.add(0).cast::<u8>());
                        match l8 {
                            0 => {
                                let e = {
                                    let l9 = *ptr7.add(4).cast::<i32>();
                                    let l10 = *ptr7.add(8).cast::<i32>();
                                    (
                                        IncomingDatagramStream::from_handle(l9 as u32),
                                        OutgoingDatagramStream::from_handle(l10 as u32),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l11 = i32::from(*ptr7.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l11 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the current bound address.
                ///
                /// POSIX mentions:
                /// > If the socket has not been bound to a local name, the value
                /// > stored in the object pointed to by `address` is unspecified.
                ///
                /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn't been bound yet.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not bound to any local address.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>
                /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>
                /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>
                pub fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 36],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.local-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the address the socket is currently streaming to.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not streaming to a specific remote address. (ENOTCONN)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>
                /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>
                pub fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 36],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.remote-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Whether this is a IPv4 or IPv6 socket.
                ///
                /// Equivalent to the SO_DOMAIN socket option.
                pub fn address_family(&self) -> IpAddressFamily {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.address-family"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::sockets::network::IpAddressFamily::_lift(
                            ret as u8,
                        )
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.
                pub fn unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.unicast-hop-limit"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    l2 as u8
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_unicast_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.set-unicast-hop-limit"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The kernel buffer space reserved for sends/receives on this socket.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.receive-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_receive_buffer_size(
                    &self,
                    value: u64,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.set-receive-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.send-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.set-send-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the socket is ready for I/O.
                ///
                /// Note: this function is here for WASI 0.2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl IncomingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Receive messages on the socket.
                ///
                /// This function attempts to receive up to `max-results` datagrams on the socket without blocking.
                /// The returned list may contain fewer elements than requested, but never more.
                ///
                /// This function returns successfully with an empty list when either:
                /// - `max-results` is 0, or:
                /// - `max-results` is greater than 0, but no results are immediately available.
                /// This function never returns `error(would-block)`.
                ///
                /// # Typical errors
                /// - `remote-unreachable`: The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)
                /// - `connection-refused`: The connection was refused. (ECONNREFUSED)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html>
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html>
                /// - <https://man7.org/linux/man-pages/man2/recv.2.html>
                /// - <https://man7.org/linux/man-pages/man2/recvmmsg.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom>
                /// - <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms741687(v=vs.85)>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=recv&sektion=2>
                pub fn receive(
                    &self,
                    max_results: u64,
                ) -> Result<_rt::Vec<IncomingDatagram>, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-datagram-stream.receive"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(&max_results),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let base25 = l2;
                                    let len25 = l3;
                                    let mut result25 = _rt::Vec::with_capacity(len25);
                                    for i in 0..len25 {
                                        let base = base25.add(i * 40);
                                        let e25 = {
                                            let l4 = *base.add(0).cast::<*mut u8>();
                                            let l5 = *base.add(4).cast::<usize>();
                                            let len6 = l5;
                                            let l7 = i32::from(*base.add(8).cast::<u8>());
                                            use super::super::super::wasi::sockets::network::IpSocketAddress as V24;
                                            let v24 = match l7 {
                                                0 => {
                                                    let e24 = {
                                                        let l8 = i32::from(*base.add(12).cast::<u16>());
                                                        let l9 = i32::from(*base.add(14).cast::<u8>());
                                                        let l10 = i32::from(*base.add(15).cast::<u8>());
                                                        let l11 = i32::from(*base.add(16).cast::<u8>());
                                                        let l12 = i32::from(*base.add(17).cast::<u8>());
                                                        super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                            port: l8 as u16,
                                                            address: (l9 as u8, l10 as u8, l11 as u8, l12 as u8),
                                                        }
                                                    };
                                                    V24::Ipv4(e24)
                                                }
                                                n => {
                                                    debug_assert_eq!(n, 1, "invalid enum discriminant");
                                                    let e24 = {
                                                        let l13 = i32::from(*base.add(12).cast::<u16>());
                                                        let l14 = *base.add(16).cast::<i32>();
                                                        let l15 = i32::from(*base.add(20).cast::<u16>());
                                                        let l16 = i32::from(*base.add(22).cast::<u16>());
                                                        let l17 = i32::from(*base.add(24).cast::<u16>());
                                                        let l18 = i32::from(*base.add(26).cast::<u16>());
                                                        let l19 = i32::from(*base.add(28).cast::<u16>());
                                                        let l20 = i32::from(*base.add(30).cast::<u16>());
                                                        let l21 = i32::from(*base.add(32).cast::<u16>());
                                                        let l22 = i32::from(*base.add(34).cast::<u16>());
                                                        let l23 = *base.add(36).cast::<i32>();
                                                        super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                            port: l13 as u16,
                                                            flow_info: l14 as u32,
                                                            address: (
                                                                l15 as u16,
                                                                l16 as u16,
                                                                l17 as u16,
                                                                l18 as u16,
                                                                l19 as u16,
                                                                l20 as u16,
                                                                l21 as u16,
                                                                l22 as u16,
                                                            ),
                                                            scope_id: l23 as u32,
                                                        }
                                                    };
                                                    V24::Ipv6(e24)
                                                }
                                            };
                                            IncomingDatagram {
                                                data: _rt::Vec::from_raw_parts(l4.cast(), len6, len6),
                                                remote_address: v24,
                                            }
                                        };
                                        result25.push(e25);
                                    }
                                    _rt::cabi_dealloc(base25, len25 * 40, 4);
                                    result25
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l26 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l26 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the stream is ready to receive again.
                ///
                /// Note: this function is here for WASI 0.2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]incoming-datagram-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutgoingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Check readiness for sending. This function never blocks.
                ///
                /// Returns the number of datagrams permitted for the next call to `send`,
                /// or an error. Calling `send` with more datagrams than this function has
                /// permitted will trap.
                ///
                /// When this function returns ok(0), the `subscribe` pollable will
                /// become ready when this function will report at least ok(1), or an
                /// error.
                ///
                /// Never returns `would-block`.
                pub fn check_send(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-datagram-stream.check-send"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Send messages on the socket.
                ///
                /// This function attempts to send all provided `datagrams` on the socket without blocking and
                /// returns how many messages were actually sent (or queued for sending). This function never
                /// returns `error(would-block)`. If none of the datagrams were able to be sent, `ok(0)` is returned.
                ///
                /// This function semantically behaves the same as iterating the `datagrams` list and sequentially
                /// sending each individual datagram until either the end of the list has been reached or the first error occurred.
                /// If at least one datagram has been sent successfully, this function never returns an error.
                ///
                /// If the input list is empty, the function returns `ok(0)`.
                ///
                /// Each call to `send` must be permitted by a preceding `check-send`. Implementations must trap if
                /// either `check-send` was not called or `datagrams` contains more items than `check-send` permitted.
                ///
                /// # Typical errors
                /// - `invalid-argument`:        The `remote-address` has the wrong address family. (EAFNOSUPPORT)
                /// - `invalid-argument`:        The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)
                /// - `invalid-argument`:        The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)
                /// - `invalid-argument`:        The socket is in "connected" mode and `remote-address` is `some` value that does not match the address passed to `stream`. (EISCONN)
                /// - `invalid-argument`:        The socket is not "connected" and no value for `remote-address` was provided. (EDESTADDRREQ)
                /// - `remote-unreachable`:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)
                /// - `connection-refused`:      The connection was refused. (ECONNREFUSED)
                /// - `datagram-too-large`:      The datagram is too large. (EMSGSIZE)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html>
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html>
                /// - <https://man7.org/linux/man-pages/man2/send.2.html>
                /// - <https://man7.org/linux/man-pages/man2/sendmmsg.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=send&sektion=2>
                pub fn send(
                    &self,
                    datagrams: &[OutgoingDatagram],
                ) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec7 = datagrams;
                        let len7 = vec7.len();
                        let layout7 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec7.len() * 44,
                            4,
                        );
                        let result7 = if layout7.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout7);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec7.into_iter().enumerate() {
                            let base = result7.add(i * 44);
                            {
                                let OutgoingDatagram {
                                    data: data0,
                                    remote_address: remote_address0,
                                } = e;
                                let vec1 = data0;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                match remote_address0 {
                                    Some(e) => {
                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::wasi::sockets::network::IpSocketAddress as V6;
                                        match e {
                                            V6::Ipv4(e) => {
                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: port2,
                                                    address: address2,
                                                } = e;
                                                *base.add(16).cast::<u16>() = (_rt::as_i32(port2)) as u16;
                                                let (t3_0, t3_1, t3_2, t3_3) = address2;
                                                *base.add(18).cast::<u8>() = (_rt::as_i32(t3_0)) as u8;
                                                *base.add(19).cast::<u8>() = (_rt::as_i32(t3_1)) as u8;
                                                *base.add(20).cast::<u8>() = (_rt::as_i32(t3_2)) as u8;
                                                *base.add(21).cast::<u8>() = (_rt::as_i32(t3_3)) as u8;
                                            }
                                            V6::Ipv6(e) => {
                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: port4,
                                                    flow_info: flow_info4,
                                                    address: address4,
                                                    scope_id: scope_id4,
                                                } = e;
                                                *base.add(16).cast::<u16>() = (_rt::as_i32(port4)) as u16;
                                                *base.add(20).cast::<i32>() = _rt::as_i32(flow_info4);
                                                let (t5_0, t5_1, t5_2, t5_3, t5_4, t5_5, t5_6, t5_7) = address4;
                                                *base.add(24).cast::<u16>() = (_rt::as_i32(t5_0)) as u16;
                                                *base.add(26).cast::<u16>() = (_rt::as_i32(t5_1)) as u16;
                                                *base.add(28).cast::<u16>() = (_rt::as_i32(t5_2)) as u16;
                                                *base.add(30).cast::<u16>() = (_rt::as_i32(t5_3)) as u16;
                                                *base.add(32).cast::<u16>() = (_rt::as_i32(t5_4)) as u16;
                                                *base.add(34).cast::<u16>() = (_rt::as_i32(t5_5)) as u16;
                                                *base.add(36).cast::<u16>() = (_rt::as_i32(t5_6)) as u16;
                                                *base.add(38).cast::<u16>() = (_rt::as_i32(t5_7)) as u16;
                                                *base.add(40).cast::<i32>() = _rt::as_i32(scope_id4);
                                            }
                                        }
                                    }
                                    None => {
                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                    }
                                };
                            }
                        }
                        let ptr8 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-datagram-stream.send"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, result7, len7, ptr8);
                        let l9 = i32::from(*ptr8.add(0).cast::<u8>());
                        if layout7.size() != 0 {
                            _rt::alloc::dealloc(result7.cast(), layout7);
                        }
                        match l9 {
                            0 => {
                                let e = {
                                    let l10 = *ptr8.add(8).cast::<i64>();
                                    l10 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l11 = i32::from(*ptr8.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l11 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the stream is ready to send again.
                ///
                /// Note: this function is here for WASI 0.2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]outgoing-datagram-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod udp_create_socket {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            pub type UdpSocket = super::super::super::wasi::sockets::udp::UdpSocket;
            #[allow(unused_unsafe, clippy::all)]
            /// Create a new UDP socket.
            ///
            /// Similar to `socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)` in POSIX.
            /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can't be configured otherwise.
            ///
            /// This function does not require a network capability handle. This is considered to be safe because
            /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind` is called,
            /// the socket is effectively an in-memory configuration object, unable to communicate with the outside world.
            ///
            /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.
            ///
            /// # Typical errors
            /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)
            /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)
            ///
            /// # References:
            /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>
            /// - <https://man7.org/linux/man-pages/man2/socket.2.html>
            /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>
            /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>
            pub fn create_udp_socket(
                address_family: IpAddressFamily,
            ) -> Result<UdpSocket, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/udp-create-socket@0.2.2")]
                    extern "C" {
                        #[link_name = "create-udp-socket"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(address_family.clone() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::sockets::udp::UdpSocket::from_handle(
                                    l2 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l3 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tcp {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpSocketAddress = super::super::super::wasi::sockets::network::IpSocketAddress;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ShutdownType {
                /// Similar to `SHUT_RD` in POSIX.
                Receive,
                /// Similar to `SHUT_WR` in POSIX.
                Send,
                /// Similar to `SHUT_RDWR` in POSIX.
                Both,
            }
            impl ::core::fmt::Debug for ShutdownType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        ShutdownType::Receive => {
                            f.debug_tuple("ShutdownType::Receive").finish()
                        }
                        ShutdownType::Send => {
                            f.debug_tuple("ShutdownType::Send").finish()
                        }
                        ShutdownType::Both => {
                            f.debug_tuple("ShutdownType::Both").finish()
                        }
                    }
                }
            }
            impl ShutdownType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ShutdownType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ShutdownType::Receive,
                        1 => ShutdownType::Send,
                        2 => ShutdownType::Both,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            /// A TCP socket resource.
            ///
            /// The socket can be in one of the following states:
            /// - `unbound`
            /// - `bind-in-progress`
            /// - `bound` (See note below)
            /// - `listen-in-progress`
            /// - `listening`
            /// - `connect-in-progress`
            /// - `connected`
            /// - `closed`
            /// See <https://github.com/WebAssembly/wasi-sockets/blob/main/TcpSocketOperationalSemantics.md>
            /// for more information.
            ///
            /// Note: Except where explicitly mentioned, whenever this documentation uses
            /// the term "bound" without backticks it actually means: in the `bound` state *or higher*.
            /// (i.e. `bound`, `listen-in-progress`, `listening`, `connect-in-progress` or `connected`)
            ///
            /// In addition to the general error codes documented on the
            /// `network::error-code` type, TCP socket methods may always return
            /// `error(invalid-state)` when in the `closed` state.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TcpSocket {
                handle: _rt::Resource<TcpSocket>,
            }
            impl TcpSocket {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TcpSocket {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]tcp-socket"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Bind the socket to a specific network on the provided IP address and port.
                ///
                /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which
                /// network interface(s) to bind to.
                /// If the TCP/UDP port is zero, the socket will be bound to a random free port.
                ///
                /// Bind can be attempted multiple times on the same socket, even with
                /// different arguments on each iteration. But never concurrently and
                /// only as long as the previous bind failed. Once a bind succeeds, the
                /// binding can't be changed anymore.
                ///
                /// # Typical errors
                /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)
                /// - `invalid-argument`:          `local-address` is not a unicast address. (EINVAL)
                /// - `invalid-argument`:          `local-address` is an IPv4-mapped IPv6 address. (EINVAL)
                /// - `invalid-state`:             The socket is already bound. (EINVAL)
                /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)
                /// - `address-in-use`:            Address is already in use. (EADDRINUSE)
                /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)
                /// - `not-in-progress`:           A `bind` operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// When binding to a non-zero port, this bind operation shouldn't be affected by the TIME_WAIT
                /// state of a recently closed socket on the same local address. In practice this means that the SO_REUSEADDR
                /// socket option should be set implicitly on all platforms, except on Windows where this is the default behavior
                /// and SO_REUSEADDR performs something different entirely.
                ///
                /// Unlike in POSIX, in WASI the bind operation is async. This enables
                /// interactive WASI hosts to inject permission prompts. Runtimes that
                /// don't want to make use of this ability can simply call the native
                /// `bind` as part of either `start-bind` or `finish-bind`.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>
                /// - <https://man7.org/linux/man-pages/man2/bind.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>
                pub fn start_bind(
                    &self,
                    network: &Network,
                    local_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match local_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-bind"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_bind(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-bind"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Connect to a remote endpoint.
                ///
                /// On success:
                /// - the socket is transitioned into the `connected` state.
                /// - a pair of streams is returned that can be used to read & write to the connection
                ///
                /// After a failed connection attempt, the socket will be in the `closed`
                /// state and the only valid action left is to `drop` the socket. A single
                /// socket can not be used to connect more than once.
                ///
                /// # Typical errors
                /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)
                /// - `invalid-argument`:          `remote-address` is not a unicast address. (EINVAL, ENETUNREACH on Linux, EAFNOSUPPORT on MacOS)
                /// - `invalid-argument`:          `remote-address` is an IPv4-mapped IPv6 address. (EINVAL, EADDRNOTAVAIL on Illumos)
                /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EADDRNOTAVAIL on Windows)
                /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EADDRNOTAVAIL on Windows)
                /// - `invalid-argument`:          The socket is already attached to a different network. The `network` passed to `connect` must be identical to the one passed to `bind`.
                /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN)
                /// - `invalid-state`:             The socket is already in the `listening` state. (EOPNOTSUPP, EINVAL on Windows)
                /// - `timeout`:                   Connection timed out. (ETIMEDOUT)
                /// - `connection-refused`:        The connection was forcefully rejected. (ECONNREFUSED)
                /// - `connection-reset`:          The connection was reset. (ECONNRESET)
                /// - `connection-aborted`:        The connection was aborted. (ECONNABORTED)
                /// - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)
                /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)
                /// - `not-in-progress`:           A connect operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// The POSIX equivalent of `start-connect` is the regular `connect` syscall.
                /// Because all WASI sockets are non-blocking this is expected to return
                /// EINPROGRESS, which should be translated to `ok()` in WASI.
                ///
                /// The POSIX equivalent of `finish-connect` is a `poll` for event `POLLOUT`
                /// with a timeout of 0 on the socket descriptor. Followed by a check for
                /// the `SO_ERROR` socket option, in case the poll signaled readiness.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>
                /// - <https://man7.org/linux/man-pages/man2/connect.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>
                /// - <https://man.freebsd.org/cgi/man.cgi?connect>
                pub fn start_connect(
                    &self,
                    network: &Network,
                    remote_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match remote_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-connect"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_connect(
                    &self,
                ) -> Result<(InputStream, OutputStream), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-connect"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    let l3 = *ptr0.add(8).cast::<i32>();
                                    (
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l2 as u32,
                                        ),
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l3 as u32,
                                        ),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l4 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Start listening for new connections.
                ///
                /// Transitions the socket into the `listening` state.
                ///
                /// Unlike POSIX, the socket must already be explicitly bound.
                ///
                /// # Typical errors
                /// - `invalid-state`:             The socket is not bound to any local address. (EDESTADDRREQ)
                /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN, EINVAL on BSD)
                /// - `invalid-state`:             The socket is already in the `listening` state.
                /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)
                /// - `not-in-progress`:           A listen operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// Unlike in POSIX, in WASI the listen operation is async. This enables
                /// interactive WASI hosts to inject permission prompts. Runtimes that
                /// don't want to make use of this ability can simply call the native
                /// `listen` as part of either `start-listen` or `finish-listen`.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html>
                /// - <https://man7.org/linux/man-pages/man2/listen.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=listen&sektion=2>
                pub fn start_listen(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-listen"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_listen(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-listen"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Accept a new client socket.
                ///
                /// The returned socket is bound and in the `connected` state. The following properties are inherited from the listener socket:
                /// - `address-family`
                /// - `keep-alive-enabled`
                /// - `keep-alive-idle-time`
                /// - `keep-alive-interval`
                /// - `keep-alive-count`
                /// - `hop-limit`
                /// - `receive-buffer-size`
                /// - `send-buffer-size`
                ///
                /// On success, this function returns the newly accepted client socket along with
                /// a pair of streams that can be used to read & write to the connection.
                ///
                /// # Typical errors
                /// - `invalid-state`:      Socket is not in the `listening` state. (EINVAL)
                /// - `would-block`:        No pending connections at the moment. (EWOULDBLOCK, EAGAIN)
                /// - `connection-aborted`: An incoming connection was pending, but was terminated by the client before this listener could accept it. (ECONNABORTED)
                /// - `new-socket-limit`:   The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html>
                /// - <https://man7.org/linux/man-pages/man2/accept.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=accept&sektion=2>
                pub fn accept(
                    &self,
                ) -> Result<(TcpSocket, InputStream, OutputStream), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.accept"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    let l3 = *ptr0.add(8).cast::<i32>();
                                    let l4 = *ptr0.add(12).cast::<i32>();
                                    (
                                        TcpSocket::from_handle(l2 as u32),
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l3 as u32,
                                        ),
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l4 as u32,
                                        ),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l5 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the bound local address.
                ///
                /// POSIX mentions:
                /// > If the socket has not been bound to a local name, the value
                /// > stored in the object pointed to by `address` is unspecified.
                ///
                /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn't been bound yet.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not bound to any local address.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>
                /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>
                /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>
                pub fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 36],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.local-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the remote address.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not connected to a remote address. (ENOTCONN)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>
                /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>
                pub fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 36],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.remote-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Whether the socket is in the `listening` state.
                ///
                /// Equivalent to the SO_ACCEPTCONN socket option.
                pub fn is_listening(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.is-listening"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Whether this is a IPv4 or IPv6 socket.
                ///
                /// Equivalent to the SO_DOMAIN socket option.
                pub fn address_family(&self) -> IpAddressFamily {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.address-family"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::sockets::network::IpAddressFamily::_lift(
                            ret as u8,
                        )
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Hints the desired listen queue size. Implementations are free to ignore this.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                ///
                /// # Typical errors
                /// - `not-supported`:        (set) The platform does not support changing the backlog size after the initial listen.
                /// - `invalid-argument`:     (set) The provided value was 0.
                /// - `invalid-state`:        (set) The socket is in the `connect-in-progress` or `connected` state.
                pub fn set_listen_backlog_size(
                    &self,
                    value: u64,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-listen-backlog-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Enables or disables keepalive.
                ///
                /// The keepalive behavior can be adjusted using:
                /// - `keep-alive-idle-time`
                /// - `keep-alive-interval`
                /// - `keep-alive-count`
                /// These properties can be configured while `keep-alive-enabled` is false, but only come into effect when `keep-alive-enabled` is true.
                ///
                /// Equivalent to the SO_KEEPALIVE socket option.
                pub fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-enabled"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_enabled(
                    &self,
                    value: bool,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-enabled"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            match &value {
                                true => 1,
                                false => 0,
                            },
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Amount of time the connection has to be idle before TCP starts sending keepalive packets.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-idle-time"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_idle_time(
                    &self,
                    value: Duration,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-idle-time"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The time between keepalive packets.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the TCP_KEEPINTVL socket option.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-interval"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_interval(
                    &self,
                    value: Duration,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-interval"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The maximum amount of keepalive packets TCP should send before aborting the connection.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the TCP_KEEPCNT socket option.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-count"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    l2 as u32
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-count"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.
                pub fn hop_limit(&self) -> Result<u8, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.hop-limit"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    l2 as u8
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-hop-limit"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The kernel buffer space reserved for sends/receives on this socket.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.receive-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_receive_buffer_size(
                    &self,
                    value: u64,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-receive-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.send-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-send-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which can be used to poll for, or block on,
                /// completion of any of the asynchronous operations of this socket.
                ///
                /// When `finish-bind`, `finish-listen`, `finish-connect` or `accept`
                /// return `error(would-block)`, this pollable can be used to wait for
                /// their success or failure, after which the method can be retried.
                ///
                /// The pollable is not limited to the async operation that happens to be
                /// in progress at the time of calling `subscribe` (if any). Theoretically,
                /// `subscribe` only has to be called once per socket and can then be
                /// (re)used for the remainder of the socket's lifetime.
                ///
                /// See <https://github.com/WebAssembly/wasi-sockets/blob/main/TcpSocketOperationalSemantics.md#pollable-readiness>
                /// for more information.
                ///
                /// Note: this function is here for WASI 0.2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Initiate a graceful shutdown.
                ///
                /// - `receive`: The socket is not expecting to receive any data from
                /// the peer. The `input-stream` associated with this socket will be
                /// closed. Any data still in the receive queue at time of calling
                /// this method will be discarded.
                /// - `send`: The socket has no more data to send to the peer. The `output-stream`
                /// associated with this socket will be closed and a FIN packet will be sent.
                /// - `both`: Same effect as `receive` & `send` combined.
                ///
                /// This function is idempotent; shutting down a direction more than once
                /// has no effect and returns `ok`.
                ///
                /// The shutdown function does not close (drop) the socket.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not in the `connected` state. (ENOTCONN)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html>
                /// - <https://man7.org/linux/man-pages/man2/shutdown.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2>
                pub fn shutdown(
                    &self,
                    shutdown_type: ShutdownType,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.shutdown"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            shutdown_type.clone() as i32,
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tcp_create_socket {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            pub type TcpSocket = super::super::super::wasi::sockets::tcp::TcpSocket;
            #[allow(unused_unsafe, clippy::all)]
            /// Create a new TCP socket.
            ///
            /// Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)` in POSIX.
            /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can't be configured otherwise.
            ///
            /// This function does not require a network capability handle. This is considered to be safe because
            /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind`/`connect`
            /// is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.
            ///
            /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.
            ///
            /// # Typical errors
            /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)
            /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)
            ///
            /// # References
            /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>
            /// - <https://man7.org/linux/man-pages/man2/socket.2.html>
            /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>
            /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>
            pub fn create_tcp_socket(
                address_family: IpAddressFamily,
            ) -> Result<TcpSocket, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/tcp-create-socket@0.2.2")]
                    extern "C" {
                        #[link_name = "create-tcp-socket"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(address_family.clone() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::sockets::tcp::TcpSocket::from_handle(
                                    l2 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l3 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod ip_name_lookup {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddress = super::super::super::wasi::sockets::network::IpAddress;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ResolveAddressStream {
                handle: _rt::Resource<ResolveAddressStream>,
            }
            impl ResolveAddressStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ResolveAddressStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.2")]
                        extern "C" {
                            #[link_name = "[resource-drop]resolve-address-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Resolve an internet host name to a list of IP addresses.
            ///
            /// Unicode domain names are automatically converted to ASCII using IDNA encoding.
            /// If the input is an IP address string, the address is parsed and returned
            /// as-is without making any external requests.
            ///
            /// See the wasi-socket proposal README.md for a comparison with getaddrinfo.
            ///
            /// This function never blocks. It either immediately fails or immediately
            /// returns successfully with a `resolve-address-stream` that can be used
            /// to (asynchronously) fetch the results.
            ///
            /// # Typical errors
            /// - `invalid-argument`: `name` is a syntactically invalid domain name or IP address.
            ///
            /// # References:
            /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html>
            /// - <https://man7.org/linux/man-pages/man3/getaddrinfo.3.html>
            /// - <https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo>
            /// - <https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&sektion=3>
            pub fn resolve_addresses(
                network: &Network,
                name: &str,
            ) -> Result<ResolveAddressStream, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = name;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.2")]
                    extern "C" {
                        #[link_name = "resolve-addresses"]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((network).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<i32>();
                                ResolveAddressStream::from_handle(l3 as u32)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l4 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            impl ResolveAddressStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the next address from the resolver.
                ///
                /// This function should be called multiple times. On each call, it will
                /// return the next address in connection order preference. If all
                /// addresses have been exhausted, this function returns `none`.
                ///
                /// This function never returns IPv4-mapped IPv6 addresses.
                ///
                /// # Typical errors
                /// - `name-unresolvable`:          Name does not exist or has no suitable associated IP addresses. (EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY)
                /// - `temporary-resolver-failure`: A temporary failure in name resolution occurred. (EAI_AGAIN)
                /// - `permanent-resolver-failure`: A permanent failure in name resolution occurred. (EAI_FAIL)
                /// - `would-block`:                A result is not available yet. (EWOULDBLOCK, EAGAIN)
                pub fn resolve_next_address(
                    &self,
                ) -> Result<Option<IpAddress>, ErrorCode> {
                    unsafe {
                        #[repr(align(2))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 22]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 22],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]resolve-address-stream.resolve-next-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(2).cast::<u8>());
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                                use super::super::super::wasi::sockets::network::IpAddress as V16;
                                                let v16 = match l3 {
                                                    0 => {
                                                        let e16 = {
                                                            let l4 = i32::from(*ptr0.add(6).cast::<u8>());
                                                            let l5 = i32::from(*ptr0.add(7).cast::<u8>());
                                                            let l6 = i32::from(*ptr0.add(8).cast::<u8>());
                                                            let l7 = i32::from(*ptr0.add(9).cast::<u8>());
                                                            (l4 as u8, l5 as u8, l6 as u8, l7 as u8)
                                                        };
                                                        V16::Ipv4(e16)
                                                    }
                                                    n => {
                                                        debug_assert_eq!(n, 1, "invalid enum discriminant");
                                                        let e16 = {
                                                            let l8 = i32::from(*ptr0.add(6).cast::<u16>());
                                                            let l9 = i32::from(*ptr0.add(8).cast::<u16>());
                                                            let l10 = i32::from(*ptr0.add(10).cast::<u16>());
                                                            let l11 = i32::from(*ptr0.add(12).cast::<u16>());
                                                            let l12 = i32::from(*ptr0.add(14).cast::<u16>());
                                                            let l13 = i32::from(*ptr0.add(16).cast::<u16>());
                                                            let l14 = i32::from(*ptr0.add(18).cast::<u16>());
                                                            let l15 = i32::from(*ptr0.add(20).cast::<u16>());
                                                            (
                                                                l8 as u16,
                                                                l9 as u16,
                                                                l10 as u16,
                                                                l11 as u16,
                                                                l12 as u16,
                                                                l13 as u16,
                                                                l14 as u16,
                                                                l15 as u16,
                                                            )
                                                        };
                                                        V16::Ipv6(e16)
                                                    }
                                                };
                                                v16
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l17 = i32::from(*ptr0.add(2).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l17 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ResolveAddressStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the stream is ready for I/O.
                ///
                /// Note: this function is here for WASI 0.2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.2")]
                        extern "C" {
                            #[link_name = "[method]resolve-address-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod wasi {
        #[allow(dead_code)]
        pub mod cli {
            #[allow(dead_code, clippy::all)]
            pub mod run {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::run();
                    let result1 = match result0 {
                        Ok(_) => 0i32,
                        Err(_) => 1i32,
                    };
                    result1
                }
                pub trait Guest {
                    /// Run the program.
                    fn run() -> Result<(), ()>;
                }
                #[doc(hidden)]
                macro_rules! __export_wasi_cli_run_0_2_2_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "wasi:cli/run@0.2.2#run"] unsafe
                        extern "C" fn export_run() -> i32 { $($path_to_types)*::
                        _export_run_cabi::<$ty > () } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wasi_cli_run_0_2_2_cabi;
            }
        }
        #[allow(dead_code)]
        pub mod http {
            #[allow(dead_code, clippy::all)]
            pub mod incoming_handler {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type IncomingRequest = super::super::super::super::wasi::http::types::IncomingRequest;
                pub type ResponseOutparam = super::super::super::super::wasi::http::types::ResponseOutparam;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_handle_cabi<T: Guest>(arg0: i32, arg1: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::handle(
                        super::super::super::super::wasi::http::types::IncomingRequest::from_handle(
                            arg0 as u32,
                        ),
                        super::super::super::super::wasi::http::types::ResponseOutparam::from_handle(
                            arg1 as u32,
                        ),
                    );
                }
                pub trait Guest {
                    /// This function is invoked with an incoming HTTP Request, and a resource
                    /// `response-outparam` which provides the capability to reply with an HTTP
                    /// Response. The response is sent by calling the `response-outparam.set`
                    /// method, which allows execution to continue after the response has been
                    /// sent. This enables both streaming to the response body, and performing other
                    /// work.
                    ///
                    /// The implementor of this function must write a response to the
                    /// `response-outparam` before returning, or else the caller will respond
                    /// with an error on its behalf.
                    fn handle(request: IncomingRequest, response_out: ResponseOutparam);
                }
                #[doc(hidden)]
                macro_rules! __export_wasi_http_incoming_handler_0_2_2_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "wasi:http/incoming-handler@0.2.2#handle"] unsafe extern "C" fn
                        export_handle(arg0 : i32, arg1 : i32,) { $($path_to_types)*::
                        _export_handle_cabi::<$ty > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wasi_http_incoming_handler_0_2_2_cabi;
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::alloc;
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub use alloc_crate::string::String;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_http_request_witbindgen_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::wasi::http::incoming_handler::__export_wasi_http_incoming_handler_0_2_2_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::wasi::http::incoming_handler);
        $($path_to_types_root)*::
        exports::wasi::cli::run::__export_wasi_cli_run_0_2_2_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::wasi::cli::run);
    };
}
#[doc(inline)]
pub(crate) use __export_http_request_witbindgen_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:seungjin:http-request-witbindigen:http-request-witbindgen:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 15903] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x91{\x01A\x02\x01AU\x01\
B\x0a\x04\0\x08pollable\x03\x01\x01h\0\x01@\x01\x04self\x01\0\x7f\x04\0\x16[meth\
od]pollable.ready\x01\x02\x01@\x01\x04self\x01\x01\0\x04\0\x16[method]pollable.b\
lock\x01\x03\x01p\x01\x01py\x01@\x01\x02in\x04\0\x05\x04\0\x04poll\x01\x06\x03\x01\
\x12wasi:io/poll@0.2.2\x05\0\x02\x03\0\0\x08pollable\x01B\x0f\x02\x03\x02\x01\x01\
\x04\0\x08pollable\x03\0\0\x01w\x04\0\x07instant\x03\0\x02\x01w\x04\0\x08duratio\
n\x03\0\x04\x01@\0\0\x03\x04\0\x03now\x01\x06\x01@\0\0\x05\x04\0\x0aresolution\x01\
\x07\x01i\x01\x01@\x01\x04when\x03\0\x08\x04\0\x11subscribe-instant\x01\x09\x01@\
\x01\x04when\x05\0\x08\x04\0\x12subscribe-duration\x01\x0a\x03\x01!wasi:clocks/m\
onotonic-clock@0.2.2\x05\x02\x01B\x04\x04\0\x05error\x03\x01\x01h\0\x01@\x01\x04\
self\x01\0s\x04\0\x1d[method]error.to-debug-string\x01\x02\x03\x01\x13wasi:io/er\
ror@0.2.2\x05\x03\x02\x03\0\x02\x05error\x01B(\x02\x03\x02\x01\x04\x04\0\x05erro\
r\x03\0\0\x02\x03\x02\x01\x01\x04\0\x08pollable\x03\0\x02\x01i\x01\x01q\x02\x15l\
ast-operation-failed\x01\x04\0\x06closed\0\0\x04\0\x0cstream-error\x03\0\x05\x04\
\0\x0cinput-stream\x03\x01\x04\0\x0doutput-stream\x03\x01\x01h\x07\x01p}\x01j\x01\
\x0a\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0b\x04\0\x19[method]input-stream.re\
ad\x01\x0c\x04\0\"[method]input-stream.blocking-read\x01\x0c\x01j\x01w\x01\x06\x01\
@\x02\x04self\x09\x03lenw\0\x0d\x04\0\x19[method]input-stream.skip\x01\x0e\x04\0\
\"[method]input-stream.blocking-skip\x01\x0e\x01i\x03\x01@\x01\x04self\x09\0\x0f\
\x04\0\x1e[method]input-stream.subscribe\x01\x10\x01h\x08\x01@\x01\x04self\x11\0\
\x0d\x04\0![method]output-stream.check-write\x01\x12\x01j\0\x01\x06\x01@\x02\x04\
self\x11\x08contents\x0a\0\x13\x04\0\x1b[method]output-stream.write\x01\x14\x04\0\
.[method]output-stream.blocking-write-and-flush\x01\x14\x01@\x01\x04self\x11\0\x13\
\x04\0\x1b[method]output-stream.flush\x01\x15\x04\0$[method]output-stream.blocki\
ng-flush\x01\x15\x01@\x01\x04self\x11\0\x0f\x04\0\x1f[method]output-stream.subsc\
ribe\x01\x16\x01@\x02\x04self\x11\x03lenw\0\x13\x04\0\"[method]output-stream.wri\
te-zeroes\x01\x17\x04\05[method]output-stream.blocking-write-zeroes-and-flush\x01\
\x17\x01@\x03\x04self\x11\x03src\x09\x03lenw\0\x0d\x04\0\x1c[method]output-strea\
m.splice\x01\x18\x04\0%[method]output-stream.blocking-splice\x01\x18\x03\x01\x15\
wasi:io/streams@0.2.2\x05\x05\x02\x03\0\x01\x08duration\x02\x03\0\x03\x0cinput-s\
tream\x02\x03\0\x03\x0doutput-stream\x01B\xc1\x01\x02\x03\x02\x01\x06\x04\0\x08d\
uration\x03\0\0\x02\x03\x02\x01\x07\x04\0\x0cinput-stream\x03\0\x02\x02\x03\x02\x01\
\x08\x04\0\x0doutput-stream\x03\0\x04\x02\x03\x02\x01\x04\x04\0\x08io-error\x03\0\
\x06\x02\x03\x02\x01\x01\x04\0\x08pollable\x03\0\x08\x01q\x0a\x03get\0\0\x04head\
\0\0\x04post\0\0\x03put\0\0\x06delete\0\0\x07connect\0\0\x07options\0\0\x05trace\
\0\0\x05patch\0\0\x05other\x01s\0\x04\0\x06method\x03\0\x0a\x01q\x03\x04HTTP\0\0\
\x05HTTPS\0\0\x05other\x01s\0\x04\0\x06scheme\x03\0\x0c\x01ks\x01k{\x01r\x02\x05\
rcode\x0e\x09info-code\x0f\x04\0\x11DNS-error-payload\x03\0\x10\x01k}\x01r\x02\x08\
alert-id\x12\x0dalert-message\x0e\x04\0\x1aTLS-alert-received-payload\x03\0\x13\x01\
ky\x01r\x02\x0afield-name\x0e\x0afield-size\x15\x04\0\x12field-size-payload\x03\0\
\x16\x01kw\x01k\x17\x01q'\x0bDNS-timeout\0\0\x09DNS-error\x01\x11\0\x15destinati\
on-not-found\0\0\x17destination-unavailable\0\0\x19destination-IP-prohibited\0\0\
\x19destination-IP-unroutable\0\0\x12connection-refused\0\0\x15connection-termin\
ated\0\0\x12connection-timeout\0\0\x17connection-read-timeout\0\0\x18connection-\
write-timeout\0\0\x18connection-limit-reached\0\0\x12TLS-protocol-error\0\0\x15T\
LS-certificate-error\0\0\x12TLS-alert-received\x01\x14\0\x13HTTP-request-denied\0\
\0\x1cHTTP-request-length-required\0\0\x16HTTP-request-body-size\x01\x18\0\x1bHT\
TP-request-method-invalid\0\0\x18HTTP-request-URI-invalid\0\0\x19HTTP-request-UR\
I-too-long\0\0\x20HTTP-request-header-section-size\x01\x15\0\x18HTTP-request-hea\
der-size\x01\x19\0!HTTP-request-trailer-section-size\x01\x15\0\x19HTTP-request-t\
railer-size\x01\x17\0\x18HTTP-response-incomplete\0\0!HTTP-response-header-secti\
on-size\x01\x15\0\x19HTTP-response-header-size\x01\x17\0\x17HTTP-response-body-s\
ize\x01\x18\0\"HTTP-response-trailer-section-size\x01\x15\0\x1aHTTP-response-tra\
iler-size\x01\x17\0\x1dHTTP-response-transfer-coding\x01\x0e\0\x1cHTTP-response-\
content-coding\x01\x0e\0\x15HTTP-response-timeout\0\0\x13HTTP-upgrade-failed\0\0\
\x13HTTP-protocol-error\0\0\x0dloop-detected\0\0\x13configuration-error\0\0\x0ei\
nternal-error\x01\x0e\0\x04\0\x0aerror-code\x03\0\x1a\x01q\x03\x0einvalid-syntax\
\0\0\x09forbidden\0\0\x09immutable\0\0\x04\0\x0cheader-error\x03\0\x1c\x01s\x04\0\
\x09field-key\x03\0\x1e\x04\0\x0afield-name\x03\0\x1f\x01p}\x04\0\x0bfield-value\
\x03\0!\x04\0\x06fields\x03\x01\x04\0\x07headers\x03\0#\x04\0\x08trailers\x03\0#\
\x04\0\x10incoming-request\x03\x01\x04\0\x10outgoing-request\x03\x01\x04\0\x0fre\
quest-options\x03\x01\x04\0\x11response-outparam\x03\x01\x01{\x04\0\x0bstatus-co\
de\x03\0*\x04\0\x11incoming-response\x03\x01\x04\0\x0dincoming-body\x03\x01\x04\0\
\x0ffuture-trailers\x03\x01\x04\0\x11outgoing-response\x03\x01\x04\0\x0doutgoing\
-body\x03\x01\x04\0\x18future-incoming-response\x03\x01\x01i#\x01@\0\02\x04\0\x13\
[constructor]fields\x013\x01o\x02\x20\"\x01p4\x01j\x012\x01\x1d\x01@\x01\x07entr\
ies5\06\x04\0\x18[static]fields.from-list\x017\x01h#\x01p\"\x01@\x02\x04self8\x04\
name\x20\09\x04\0\x12[method]fields.get\x01:\x01@\x02\x04self8\x04name\x20\0\x7f\
\x04\0\x12[method]fields.has\x01;\x01j\0\x01\x1d\x01@\x03\x04self8\x04name\x20\x05\
value9\0<\x04\0\x12[method]fields.set\x01=\x01@\x02\x04self8\x04name\x20\0<\x04\0\
\x15[method]fields.delete\x01>\x01@\x03\x04self8\x04name\x20\x05value\"\0<\x04\0\
\x15[method]fields.append\x01?\x01@\x01\x04self8\05\x04\0\x16[method]fields.entr\
ies\x01@\x01@\x01\x04self8\02\x04\0\x14[method]fields.clone\x01A\x01h&\x01@\x01\x04\
self\xc2\0\0\x0b\x04\0\x1f[method]incoming-request.method\x01C\x01@\x01\x04self\xc2\
\0\0\x0e\x04\0([method]incoming-request.path-with-query\x01D\x01k\x0d\x01@\x01\x04\
self\xc2\0\0\xc5\0\x04\0\x1f[method]incoming-request.scheme\x01F\x04\0\"[method]\
incoming-request.authority\x01D\x01i$\x01@\x01\x04self\xc2\0\0\xc7\0\x04\0\x20[m\
ethod]incoming-request.headers\x01H\x01i-\x01j\x01\xc9\0\0\x01@\x01\x04self\xc2\0\
\0\xca\0\x04\0\x20[method]incoming-request.consume\x01K\x01i'\x01@\x01\x07header\
s\xc7\0\0\xcc\0\x04\0\x1d[constructor]outgoing-request\x01M\x01h'\x01i0\x01j\x01\
\xcf\0\0\x01@\x01\x04self\xce\0\0\xd0\0\x04\0\x1d[method]outgoing-request.body\x01\
Q\x01@\x01\x04self\xce\0\0\x0b\x04\0\x1f[method]outgoing-request.method\x01R\x01\
j\0\0\x01@\x02\x04self\xce\0\x06method\x0b\0\xd3\0\x04\0#[method]outgoing-reques\
t.set-method\x01T\x01@\x01\x04self\xce\0\0\x0e\x04\0([method]outgoing-request.pa\
th-with-query\x01U\x01@\x02\x04self\xce\0\x0fpath-with-query\x0e\0\xd3\0\x04\0,[\
method]outgoing-request.set-path-with-query\x01V\x01@\x01\x04self\xce\0\0\xc5\0\x04\
\0\x1f[method]outgoing-request.scheme\x01W\x01@\x02\x04self\xce\0\x06scheme\xc5\0\
\0\xd3\0\x04\0#[method]outgoing-request.set-scheme\x01X\x04\0\"[method]outgoing-\
request.authority\x01U\x01@\x02\x04self\xce\0\x09authority\x0e\0\xd3\0\x04\0&[me\
thod]outgoing-request.set-authority\x01Y\x01@\x01\x04self\xce\0\0\xc7\0\x04\0\x20\
[method]outgoing-request.headers\x01Z\x01i(\x01@\0\0\xdb\0\x04\0\x1c[constructor\
]request-options\x01\\\x01h(\x01k\x01\x01@\x01\x04self\xdd\0\0\xde\0\x04\0'[meth\
od]request-options.connect-timeout\x01_\x01@\x02\x04self\xdd\0\x08duration\xde\0\
\0\xd3\0\x04\0+[method]request-options.set-connect-timeout\x01`\x04\0*[method]re\
quest-options.first-byte-timeout\x01_\x04\0.[method]request-options.set-first-by\
te-timeout\x01`\x04\0-[method]request-options.between-bytes-timeout\x01_\x04\01[\
method]request-options.set-between-bytes-timeout\x01`\x01i)\x01i/\x01j\x01\xe2\0\
\x01\x1b\x01@\x02\x05param\xe1\0\x08response\xe3\0\x01\0\x04\0\x1d[static]respon\
se-outparam.set\x01d\x01h,\x01@\x01\x04self\xe5\0\0+\x04\0\x20[method]incoming-r\
esponse.status\x01f\x01@\x01\x04self\xe5\0\0\xc7\0\x04\0![method]incoming-respon\
se.headers\x01g\x01@\x01\x04self\xe5\0\0\xca\0\x04\0![method]incoming-response.c\
onsume\x01h\x01h-\x01i\x03\x01j\x01\xea\0\0\x01@\x01\x04self\xe9\0\0\xeb\0\x04\0\
\x1c[method]incoming-body.stream\x01l\x01i.\x01@\x01\x04this\xc9\0\0\xed\0\x04\0\
\x1c[static]incoming-body.finish\x01n\x01h.\x01i\x09\x01@\x01\x04self\xef\0\0\xf0\
\0\x04\0![method]future-trailers.subscribe\x01q\x01i%\x01k\xf2\0\x01j\x01\xf3\0\x01\
\x1b\x01j\x01\xf4\0\0\x01k\xf5\0\x01@\x01\x04self\xef\0\0\xf6\0\x04\0\x1b[method\
]future-trailers.get\x01w\x01@\x01\x07headers\xc7\0\0\xe2\0\x04\0\x1e[constructo\
r]outgoing-response\x01x\x01h/\x01@\x01\x04self\xf9\0\0+\x04\0%[method]outgoing-\
response.status-code\x01z\x01@\x02\x04self\xf9\0\x0bstatus-code+\0\xd3\0\x04\0)[\
method]outgoing-response.set-status-code\x01{\x01@\x01\x04self\xf9\0\0\xc7\0\x04\
\0![method]outgoing-response.headers\x01|\x01@\x01\x04self\xf9\0\0\xd0\0\x04\0\x1e\
[method]outgoing-response.body\x01}\x01h0\x01i\x05\x01j\x01\xff\0\0\x01@\x01\x04\
self\xfe\0\0\x80\x01\x04\0\x1b[method]outgoing-body.write\x01\x81\x01\x01j\0\x01\
\x1b\x01@\x02\x04this\xcf\0\x08trailers\xf3\0\0\x82\x01\x04\0\x1c[static]outgoin\
g-body.finish\x01\x83\x01\x01h1\x01@\x01\x04self\x84\x01\0\xf0\0\x04\0*[method]f\
uture-incoming-response.subscribe\x01\x85\x01\x01i,\x01j\x01\x86\x01\x01\x1b\x01\
j\x01\x87\x01\0\x01k\x88\x01\x01@\x01\x04self\x84\x01\0\x89\x01\x04\0$[method]fu\
ture-incoming-response.get\x01\x8a\x01\x01h\x07\x01k\x1b\x01@\x01\x03err\x8b\x01\
\0\x8c\x01\x04\0\x0fhttp-error-code\x01\x8d\x01\x03\x01\x15wasi:http/types@0.2.2\
\x05\x09\x01B\x05\x01r\x02\x07secondsw\x0bnanosecondsy\x04\0\x08datetime\x03\0\0\
\x01@\0\0\x01\x04\0\x03now\x01\x02\x04\0\x0aresolution\x01\x02\x03\x01\x1cwasi:c\
locks/wall-clock@0.2.2\x05\x0a\x01B\x05\x01p}\x01@\x01\x03lenw\0\0\x04\0\x10get-\
random-bytes\x01\x01\x01@\0\0w\x04\0\x0eget-random-u64\x01\x02\x03\x01\x18wasi:r\
andom/random@0.2.2\x05\x0b\x01B\x05\x02\x03\x02\x01\x08\x04\0\x0doutput-stream\x03\
\0\0\x01i\x01\x01@\0\0\x02\x04\0\x0aget-stdout\x01\x03\x03\x01\x15wasi:cli/stdou\
t@0.2.2\x05\x0c\x01B\x05\x02\x03\x02\x01\x08\x04\0\x0doutput-stream\x03\0\0\x01i\
\x01\x01@\0\0\x02\x04\0\x0aget-stderr\x01\x03\x03\x01\x15wasi:cli/stderr@0.2.2\x05\
\x0d\x01B\x05\x02\x03\x02\x01\x07\x04\0\x0cinput-stream\x03\0\0\x01i\x01\x01@\0\0\
\x02\x04\0\x09get-stdin\x01\x03\x03\x01\x14wasi:cli/stdin@0.2.2\x05\x0e\x02\x03\0\
\x04\x10outgoing-request\x02\x03\0\x04\x0frequest-options\x02\x03\0\x04\x18futur\
e-incoming-response\x02\x03\0\x04\x0aerror-code\x01B\x0f\x02\x03\x02\x01\x0f\x04\
\0\x10outgoing-request\x03\0\0\x02\x03\x02\x01\x10\x04\0\x0frequest-options\x03\0\
\x02\x02\x03\x02\x01\x11\x04\0\x18future-incoming-response\x03\0\x04\x02\x03\x02\
\x01\x12\x04\0\x0aerror-code\x03\0\x06\x01i\x01\x01i\x03\x01k\x09\x01i\x05\x01j\x01\
\x0b\x01\x07\x01@\x02\x07request\x08\x07options\x0a\0\x0c\x04\0\x06handle\x01\x0d\
\x03\x01\x20wasi:http/outgoing-handler@0.2.2\x05\x13\x01B\x0a\x01o\x02ss\x01p\0\x01\
@\0\0\x01\x04\0\x0fget-environment\x01\x02\x01ps\x01@\0\0\x03\x04\0\x0dget-argum\
ents\x01\x04\x01ks\x01@\0\0\x05\x04\0\x0binitial-cwd\x01\x06\x03\x01\x1awasi:cli\
/environment@0.2.2\x05\x14\x01B\x03\x01j\0\0\x01@\x01\x06status\0\x01\0\x04\0\x04\
exit\x01\x01\x03\x01\x13wasi:cli/exit@0.2.2\x05\x15\x01B\x01\x04\0\x0eterminal-i\
nput\x03\x01\x03\x01\x1dwasi:cli/terminal-input@0.2.2\x05\x16\x01B\x01\x04\0\x0f\
terminal-output\x03\x01\x03\x01\x1ewasi:cli/terminal-output@0.2.2\x05\x17\x02\x03\
\0\x0d\x0eterminal-input\x01B\x06\x02\x03\x02\x01\x18\x04\0\x0eterminal-input\x03\
\0\0\x01i\x01\x01k\x02\x01@\0\0\x03\x04\0\x12get-terminal-stdin\x01\x04\x03\x01\x1d\
wasi:cli/terminal-stdin@0.2.2\x05\x19\x02\x03\0\x0e\x0fterminal-output\x01B\x06\x02\
\x03\x02\x01\x1a\x04\0\x0fterminal-output\x03\0\0\x01i\x01\x01k\x02\x01@\0\0\x03\
\x04\0\x13get-terminal-stdout\x01\x04\x03\x01\x1ewasi:cli/terminal-stdout@0.2.2\x05\
\x1b\x01B\x06\x02\x03\x02\x01\x1a\x04\0\x0fterminal-output\x03\0\0\x01i\x01\x01k\
\x02\x01@\0\0\x03\x04\0\x13get-terminal-stderr\x01\x04\x03\x01\x1ewasi:cli/termi\
nal-stderr@0.2.2\x05\x1c\x02\x03\0\x03\x05error\x02\x03\0\x05\x08datetime\x01Br\x02\
\x03\x02\x01\x07\x04\0\x0cinput-stream\x03\0\0\x02\x03\x02\x01\x08\x04\0\x0doutp\
ut-stream\x03\0\x02\x02\x03\x02\x01\x1d\x04\0\x05error\x03\0\x04\x02\x03\x02\x01\
\x1e\x04\0\x08datetime\x03\0\x06\x01w\x04\0\x08filesize\x03\0\x08\x01m\x08\x07un\
known\x0cblock-device\x10character-device\x09directory\x04fifo\x0dsymbolic-link\x0c\
regular-file\x06socket\x04\0\x0fdescriptor-type\x03\0\x0a\x01n\x06\x04read\x05wr\
ite\x13file-integrity-sync\x13data-integrity-sync\x14requested-write-sync\x10mut\
ate-directory\x04\0\x10descriptor-flags\x03\0\x0c\x01n\x01\x0esymlink-follow\x04\
\0\x0apath-flags\x03\0\x0e\x01n\x04\x06create\x09directory\x09exclusive\x08trunc\
ate\x04\0\x0aopen-flags\x03\0\x10\x01w\x04\0\x0alink-count\x03\0\x12\x01k\x07\x01\
r\x06\x04type\x0b\x0alink-count\x13\x04size\x09\x15data-access-timestamp\x14\x1b\
data-modification-timestamp\x14\x17status-change-timestamp\x14\x04\0\x0fdescript\
or-stat\x03\0\x15\x01q\x03\x09no-change\0\0\x03now\0\0\x09timestamp\x01\x07\0\x04\
\0\x0dnew-timestamp\x03\0\x17\x01r\x02\x04type\x0b\x04names\x04\0\x0fdirectory-e\
ntry\x03\0\x19\x01m%\x06access\x0bwould-block\x07already\x0ebad-descriptor\x04bu\
sy\x08deadlock\x05quota\x05exist\x0efile-too-large\x15illegal-byte-sequence\x0bi\
n-progress\x0binterrupted\x07invalid\x02io\x0cis-directory\x04loop\x0etoo-many-l\
inks\x0cmessage-size\x0dname-too-long\x09no-device\x08no-entry\x07no-lock\x13ins\
ufficient-memory\x12insufficient-space\x0dnot-directory\x09not-empty\x0fnot-reco\
verable\x0bunsupported\x06no-tty\x0eno-such-device\x08overflow\x0dnot-permitted\x04\
pipe\x09read-only\x0cinvalid-seek\x0etext-file-busy\x0ccross-device\x04\0\x0aerr\
or-code\x03\0\x1b\x01m\x06\x06normal\x0asequential\x06random\x09will-need\x09don\
t-need\x08no-reuse\x04\0\x06advice\x03\0\x1d\x01r\x02\x05lowerw\x05upperw\x04\0\x13\
metadata-hash-value\x03\0\x1f\x04\0\x0adescriptor\x03\x01\x04\0\x16directory-ent\
ry-stream\x03\x01\x01h!\x01i\x01\x01j\x01$\x01\x1c\x01@\x02\x04self#\x06offset\x09\
\0%\x04\0\"[method]descriptor.read-via-stream\x01&\x01i\x03\x01j\x01'\x01\x1c\x01\
@\x02\x04self#\x06offset\x09\0(\x04\0#[method]descriptor.write-via-stream\x01)\x01\
@\x01\x04self#\0(\x04\0$[method]descriptor.append-via-stream\x01*\x01j\0\x01\x1c\
\x01@\x04\x04self#\x06offset\x09\x06length\x09\x06advice\x1e\0+\x04\0\x19[method\
]descriptor.advise\x01,\x01@\x01\x04self#\0+\x04\0\x1c[method]descriptor.sync-da\
ta\x01-\x01j\x01\x0d\x01\x1c\x01@\x01\x04self#\0.\x04\0\x1c[method]descriptor.ge\
t-flags\x01/\x01j\x01\x0b\x01\x1c\x01@\x01\x04self#\00\x04\0\x1b[method]descript\
or.get-type\x011\x01@\x02\x04self#\x04size\x09\0+\x04\0\x1b[method]descriptor.se\
t-size\x012\x01@\x03\x04self#\x15data-access-timestamp\x18\x1bdata-modification-\
timestamp\x18\0+\x04\0\x1c[method]descriptor.set-times\x013\x01p}\x01o\x024\x7f\x01\
j\x015\x01\x1c\x01@\x03\x04self#\x06length\x09\x06offset\x09\06\x04\0\x17[method\
]descriptor.read\x017\x01j\x01\x09\x01\x1c\x01@\x03\x04self#\x06buffer4\x06offse\
t\x09\08\x04\0\x18[method]descriptor.write\x019\x01i\"\x01j\x01:\x01\x1c\x01@\x01\
\x04self#\0;\x04\0![method]descriptor.read-directory\x01<\x04\0\x17[method]descr\
iptor.sync\x01-\x01@\x02\x04self#\x04paths\0+\x04\0&[method]descriptor.create-di\
rectory-at\x01=\x01j\x01\x16\x01\x1c\x01@\x01\x04self#\0>\x04\0\x17[method]descr\
iptor.stat\x01?\x01@\x03\x04self#\x0apath-flags\x0f\x04paths\0>\x04\0\x1a[method\
]descriptor.stat-at\x01@\x01@\x05\x04self#\x0apath-flags\x0f\x04paths\x15data-ac\
cess-timestamp\x18\x1bdata-modification-timestamp\x18\0+\x04\0\x1f[method]descri\
ptor.set-times-at\x01A\x01@\x05\x04self#\x0eold-path-flags\x0f\x08old-paths\x0en\
ew-descriptor#\x08new-paths\0+\x04\0\x1a[method]descriptor.link-at\x01B\x01i!\x01\
j\x01\xc3\0\x01\x1c\x01@\x05\x04self#\x0apath-flags\x0f\x04paths\x0aopen-flags\x11\
\x05flags\x0d\0\xc4\0\x04\0\x1a[method]descriptor.open-at\x01E\x01j\x01s\x01\x1c\
\x01@\x02\x04self#\x04paths\0\xc6\0\x04\0\x1e[method]descriptor.readlink-at\x01G\
\x04\0&[method]descriptor.remove-directory-at\x01=\x01@\x04\x04self#\x08old-path\
s\x0enew-descriptor#\x08new-paths\0+\x04\0\x1c[method]descriptor.rename-at\x01H\x01\
@\x03\x04self#\x08old-paths\x08new-paths\0+\x04\0\x1d[method]descriptor.symlink-\
at\x01I\x04\0![method]descriptor.unlink-file-at\x01=\x01@\x02\x04self#\x05other#\
\0\x7f\x04\0![method]descriptor.is-same-object\x01J\x01j\x01\x20\x01\x1c\x01@\x01\
\x04self#\0\xcb\0\x04\0\x20[method]descriptor.metadata-hash\x01L\x01@\x03\x04sel\
f#\x0apath-flags\x0f\x04paths\0\xcb\0\x04\0#[method]descriptor.metadata-hash-at\x01\
M\x01h\"\x01k\x1a\x01j\x01\xcf\0\x01\x1c\x01@\x01\x04self\xce\0\0\xd0\0\x04\03[m\
ethod]directory-entry-stream.read-directory-entry\x01Q\x01h\x05\x01k\x1c\x01@\x01\
\x03err\xd2\0\0\xd3\0\x04\0\x15filesystem-error-code\x01T\x03\x01\x1bwasi:filesy\
stem/types@0.2.2\x05\x1f\x02\x03\0\x12\x0adescriptor\x01B\x07\x02\x03\x02\x01\x20\
\x04\0\x0adescriptor\x03\0\0\x01i\x01\x01o\x02\x02s\x01p\x03\x01@\0\0\x04\x04\0\x0f\
get-directories\x01\x05\x03\x01\x1ewasi:filesystem/preopens@0.2.2\x05!\x01B\x11\x04\
\0\x07network\x03\x01\x01m\x15\x07unknown\x0daccess-denied\x0dnot-supported\x10i\
nvalid-argument\x0dout-of-memory\x07timeout\x14concurrency-conflict\x0fnot-in-pr\
ogress\x0bwould-block\x0dinvalid-state\x10new-socket-limit\x14address-not-bindab\
le\x0eaddress-in-use\x12remote-unreachable\x12connection-refused\x10connection-r\
eset\x12connection-aborted\x12datagram-too-large\x11name-unresolvable\x1atempora\
ry-resolver-failure\x1apermanent-resolver-failure\x04\0\x0aerror-code\x03\0\x01\x01\
m\x02\x04ipv4\x04ipv6\x04\0\x11ip-address-family\x03\0\x03\x01o\x04}}}}\x04\0\x0c\
ipv4-address\x03\0\x05\x01o\x08{{{{{{{{\x04\0\x0cipv6-address\x03\0\x07\x01q\x02\
\x04ipv4\x01\x06\0\x04ipv6\x01\x08\0\x04\0\x0aip-address\x03\0\x09\x01r\x02\x04p\
ort{\x07address\x06\x04\0\x13ipv4-socket-address\x03\0\x0b\x01r\x04\x04port{\x09\
flow-infoy\x07address\x08\x08scope-idy\x04\0\x13ipv6-socket-address\x03\0\x0d\x01\
q\x02\x04ipv4\x01\x0c\0\x04ipv6\x01\x0e\0\x04\0\x11ip-socket-address\x03\0\x0f\x03\
\x01\x1awasi:sockets/network@0.2.2\x05\"\x02\x03\0\x14\x07network\x01B\x05\x02\x03\
\x02\x01#\x04\0\x07network\x03\0\0\x01i\x01\x01@\0\0\x02\x04\0\x10instance-netwo\
rk\x01\x03\x03\x01#wasi:sockets/instance-network@0.2.2\x05$\x02\x03\0\x14\x0aerr\
or-code\x02\x03\0\x14\x11ip-socket-address\x02\x03\0\x14\x11ip-address-family\x01\
BD\x02\x03\x02\x01\x01\x04\0\x08pollable\x03\0\0\x02\x03\x02\x01#\x04\0\x07netwo\
rk\x03\0\x02\x02\x03\x02\x01%\x04\0\x0aerror-code\x03\0\x04\x02\x03\x02\x01&\x04\
\0\x11ip-socket-address\x03\0\x06\x02\x03\x02\x01'\x04\0\x11ip-address-family\x03\
\0\x08\x01p}\x01r\x02\x04data\x0a\x0eremote-address\x07\x04\0\x11incoming-datagr\
am\x03\0\x0b\x01k\x07\x01r\x02\x04data\x0a\x0eremote-address\x0d\x04\0\x11outgoi\
ng-datagram\x03\0\x0e\x04\0\x0audp-socket\x03\x01\x04\0\x18incoming-datagram-str\
eam\x03\x01\x04\0\x18outgoing-datagram-stream\x03\x01\x01h\x10\x01h\x03\x01j\0\x01\
\x05\x01@\x03\x04self\x13\x07network\x14\x0dlocal-address\x07\0\x15\x04\0\x1d[me\
thod]udp-socket.start-bind\x01\x16\x01@\x01\x04self\x13\0\x15\x04\0\x1e[method]u\
dp-socket.finish-bind\x01\x17\x01i\x11\x01i\x12\x01o\x02\x18\x19\x01j\x01\x1a\x01\
\x05\x01@\x02\x04self\x13\x0eremote-address\x0d\0\x1b\x04\0\x19[method]udp-socke\
t.stream\x01\x1c\x01j\x01\x07\x01\x05\x01@\x01\x04self\x13\0\x1d\x04\0\x20[metho\
d]udp-socket.local-address\x01\x1e\x04\0![method]udp-socket.remote-address\x01\x1e\
\x01@\x01\x04self\x13\0\x09\x04\0![method]udp-socket.address-family\x01\x1f\x01j\
\x01}\x01\x05\x01@\x01\x04self\x13\0\x20\x04\0$[method]udp-socket.unicast-hop-li\
mit\x01!\x01@\x02\x04self\x13\x05value}\0\x15\x04\0([method]udp-socket.set-unica\
st-hop-limit\x01\"\x01j\x01w\x01\x05\x01@\x01\x04self\x13\0#\x04\0&[method]udp-s\
ocket.receive-buffer-size\x01$\x01@\x02\x04self\x13\x05valuew\0\x15\x04\0*[metho\
d]udp-socket.set-receive-buffer-size\x01%\x04\0#[method]udp-socket.send-buffer-s\
ize\x01$\x04\0'[method]udp-socket.set-send-buffer-size\x01%\x01i\x01\x01@\x01\x04\
self\x13\0&\x04\0\x1c[method]udp-socket.subscribe\x01'\x01h\x11\x01p\x0c\x01j\x01\
)\x01\x05\x01@\x02\x04self(\x0bmax-resultsw\0*\x04\0([method]incoming-datagram-s\
tream.receive\x01+\x01@\x01\x04self(\0&\x04\0*[method]incoming-datagram-stream.s\
ubscribe\x01,\x01h\x12\x01@\x01\x04self-\0#\x04\0+[method]outgoing-datagram-stre\
am.check-send\x01.\x01p\x0f\x01@\x02\x04self-\x09datagrams/\0#\x04\0%[method]out\
going-datagram-stream.send\x010\x01@\x01\x04self-\0&\x04\0*[method]outgoing-data\
gram-stream.subscribe\x011\x03\x01\x16wasi:sockets/udp@0.2.2\x05(\x02\x03\0\x16\x0a\
udp-socket\x01B\x0c\x02\x03\x02\x01#\x04\0\x07network\x03\0\0\x02\x03\x02\x01%\x04\
\0\x0aerror-code\x03\0\x02\x02\x03\x02\x01'\x04\0\x11ip-address-family\x03\0\x04\
\x02\x03\x02\x01)\x04\0\x0audp-socket\x03\0\x06\x01i\x07\x01j\x01\x08\x01\x03\x01\
@\x01\x0eaddress-family\x05\0\x09\x04\0\x11create-udp-socket\x01\x0a\x03\x01$was\
i:sockets/udp-create-socket@0.2.2\x05*\x01BT\x02\x03\x02\x01\x07\x04\0\x0cinput-\
stream\x03\0\0\x02\x03\x02\x01\x08\x04\0\x0doutput-stream\x03\0\x02\x02\x03\x02\x01\
\x01\x04\0\x08pollable\x03\0\x04\x02\x03\x02\x01\x06\x04\0\x08duration\x03\0\x06\
\x02\x03\x02\x01#\x04\0\x07network\x03\0\x08\x02\x03\x02\x01%\x04\0\x0aerror-cod\
e\x03\0\x0a\x02\x03\x02\x01&\x04\0\x11ip-socket-address\x03\0\x0c\x02\x03\x02\x01\
'\x04\0\x11ip-address-family\x03\0\x0e\x01m\x03\x07receive\x04send\x04both\x04\0\
\x0dshutdown-type\x03\0\x10\x04\0\x0atcp-socket\x03\x01\x01h\x12\x01h\x09\x01j\0\
\x01\x0b\x01@\x03\x04self\x13\x07network\x14\x0dlocal-address\x0d\0\x15\x04\0\x1d\
[method]tcp-socket.start-bind\x01\x16\x01@\x01\x04self\x13\0\x15\x04\0\x1e[metho\
d]tcp-socket.finish-bind\x01\x17\x01@\x03\x04self\x13\x07network\x14\x0eremote-a\
ddress\x0d\0\x15\x04\0\x20[method]tcp-socket.start-connect\x01\x18\x01i\x01\x01i\
\x03\x01o\x02\x19\x1a\x01j\x01\x1b\x01\x0b\x01@\x01\x04self\x13\0\x1c\x04\0![met\
hod]tcp-socket.finish-connect\x01\x1d\x04\0\x1f[method]tcp-socket.start-listen\x01\
\x17\x04\0\x20[method]tcp-socket.finish-listen\x01\x17\x01i\x12\x01o\x03\x1e\x19\
\x1a\x01j\x01\x1f\x01\x0b\x01@\x01\x04self\x13\0\x20\x04\0\x19[method]tcp-socket\
.accept\x01!\x01j\x01\x0d\x01\x0b\x01@\x01\x04self\x13\0\"\x04\0\x20[method]tcp-\
socket.local-address\x01#\x04\0![method]tcp-socket.remote-address\x01#\x01@\x01\x04\
self\x13\0\x7f\x04\0\x1f[method]tcp-socket.is-listening\x01$\x01@\x01\x04self\x13\
\0\x0f\x04\0![method]tcp-socket.address-family\x01%\x01@\x02\x04self\x13\x05valu\
ew\0\x15\x04\0*[method]tcp-socket.set-listen-backlog-size\x01&\x01j\x01\x7f\x01\x0b\
\x01@\x01\x04self\x13\0'\x04\0%[method]tcp-socket.keep-alive-enabled\x01(\x01@\x02\
\x04self\x13\x05value\x7f\0\x15\x04\0)[method]tcp-socket.set-keep-alive-enabled\x01\
)\x01j\x01\x07\x01\x0b\x01@\x01\x04self\x13\0*\x04\0'[method]tcp-socket.keep-ali\
ve-idle-time\x01+\x01@\x02\x04self\x13\x05value\x07\0\x15\x04\0+[method]tcp-sock\
et.set-keep-alive-idle-time\x01,\x04\0&[method]tcp-socket.keep-alive-interval\x01\
+\x04\0*[method]tcp-socket.set-keep-alive-interval\x01,\x01j\x01y\x01\x0b\x01@\x01\
\x04self\x13\0-\x04\0#[method]tcp-socket.keep-alive-count\x01.\x01@\x02\x04self\x13\
\x05valuey\0\x15\x04\0'[method]tcp-socket.set-keep-alive-count\x01/\x01j\x01}\x01\
\x0b\x01@\x01\x04self\x13\00\x04\0\x1c[method]tcp-socket.hop-limit\x011\x01@\x02\
\x04self\x13\x05value}\0\x15\x04\0\x20[method]tcp-socket.set-hop-limit\x012\x01j\
\x01w\x01\x0b\x01@\x01\x04self\x13\03\x04\0&[method]tcp-socket.receive-buffer-si\
ze\x014\x04\0*[method]tcp-socket.set-receive-buffer-size\x01&\x04\0#[method]tcp-\
socket.send-buffer-size\x014\x04\0'[method]tcp-socket.set-send-buffer-size\x01&\x01\
i\x05\x01@\x01\x04self\x13\05\x04\0\x1c[method]tcp-socket.subscribe\x016\x01@\x02\
\x04self\x13\x0dshutdown-type\x11\0\x15\x04\0\x1b[method]tcp-socket.shutdown\x01\
7\x03\x01\x16wasi:sockets/tcp@0.2.2\x05+\x02\x03\0\x18\x0atcp-socket\x01B\x0c\x02\
\x03\x02\x01#\x04\0\x07network\x03\0\0\x02\x03\x02\x01%\x04\0\x0aerror-code\x03\0\
\x02\x02\x03\x02\x01'\x04\0\x11ip-address-family\x03\0\x04\x02\x03\x02\x01,\x04\0\
\x0atcp-socket\x03\0\x06\x01i\x07\x01j\x01\x08\x01\x03\x01@\x01\x0eaddress-famil\
y\x05\0\x09\x04\0\x11create-tcp-socket\x01\x0a\x03\x01$wasi:sockets/tcp-create-s\
ocket@0.2.2\x05-\x02\x03\0\x14\x0aip-address\x01B\x16\x02\x03\x02\x01\x01\x04\0\x08\
pollable\x03\0\0\x02\x03\x02\x01#\x04\0\x07network\x03\0\x02\x02\x03\x02\x01%\x04\
\0\x0aerror-code\x03\0\x04\x02\x03\x02\x01.\x04\0\x0aip-address\x03\0\x06\x04\0\x16\
resolve-address-stream\x03\x01\x01h\x08\x01k\x07\x01j\x01\x0a\x01\x05\x01@\x01\x04\
self\x09\0\x0b\x04\03[method]resolve-address-stream.resolve-next-address\x01\x0c\
\x01i\x01\x01@\x01\x04self\x09\0\x0d\x04\0([method]resolve-address-stream.subscr\
ibe\x01\x0e\x01h\x03\x01i\x08\x01j\x01\x10\x01\x05\x01@\x02\x07network\x0f\x04na\
mes\0\x11\x04\0\x11resolve-addresses\x01\x12\x03\x01!wasi:sockets/ip-name-lookup\
@0.2.2\x05/\x01B\x05\x01p}\x01@\x01\x03lenw\0\0\x04\0\x19get-insecure-random-byt\
es\x01\x01\x01@\0\0w\x04\0\x17get-insecure-random-u64\x01\x02\x03\x01\x1awasi:ra\
ndom/insecure@0.2.2\x050\x01B\x03\x01o\x02ww\x01@\0\0\0\x04\0\x0dinsecure-seed\x01\
\x01\x03\x01\x1fwasi:random/insecure-seed@0.2.2\x051\x02\x03\0\x04\x10incoming-r\
equest\x02\x03\0\x04\x11response-outparam\x01B\x08\x02\x03\x02\x012\x04\0\x10inc\
oming-request\x03\0\0\x02\x03\x02\x013\x04\0\x11response-outparam\x03\0\x02\x01i\
\x01\x01i\x03\x01@\x02\x07request\x04\x0cresponse-out\x05\x01\0\x04\0\x06handle\x01\
\x06\x04\x01\x20wasi:http/incoming-handler@0.2.2\x054\x01B\x03\x01j\0\0\x01@\0\0\
\0\x04\0\x03run\x01\x01\x04\x01\x12wasi:cli/run@0.2.2\x055\x04\x019seungjin:http\
-request-witbindigen/http-request-witbindgen\x04\0\x0b\x1d\x01\0\x17http-request\
-witbindgen\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}