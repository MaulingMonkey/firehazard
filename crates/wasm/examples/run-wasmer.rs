use sandbox_windows_ffi::*;
use abistr::*;
use wasmer::*;

fn main() {
    let _ = std::collections::HashMap::<u32, u32>::new(); // seed thread's std PRNG
    output_debug_string_a(cstr!("sandbox"));
    revert_to_self().unwrap();
    let module_wat = r#"
    (module
        (type $t0 (func (param i32) (result i32)))
        (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
            get_local $p0
            i32.const 1
            i32.add))
    "#;

    let store = Store::default();
    let module = Module::new(&store, &module_wat).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();

    let add_one = instance.exports.get_function("add_one").unwrap();
    let result = add_one.call(&[Value::I32(42)]).unwrap();
    assert_eq!(result[0], Value::I32(43));
}
