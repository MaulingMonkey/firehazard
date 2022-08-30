use sandbox_windows_ffi::*;
use abistr::*;
use wasmtime::*;

fn main() {
    let _ = std::collections::HashMap::<u32, u32>::new(); // seed thread's std PRNG
    output_debug_string_a(cstr!("sandbox"));
    revert_to_self().unwrap();
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))
            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat).unwrap();

    let mut store = Store::new(&engine, 4);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    });

    let instance = Instance::new(&mut store, &module, &[host_hello.into()]).unwrap();

    let hello = instance.get_typed_func::<(), (), _>(&mut store, "hello").unwrap();
    hello.call(&mut store, ()).unwrap();
}
