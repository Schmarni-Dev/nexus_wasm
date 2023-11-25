use std::mem::size_of;
use std::{ffi::CStr, mem};

use wasm_api::{Transform, Vec3};
use wasmtime::{Caller, Config, Engine, Extern, Func, Instance, Module, Store, TypedFunc};

pub struct ClientWasmExtension {
    wasm_main: TypedFunc<(), ()>,
    wasm_instance: Instance,
    wasm_store: Store<ClientExtensionState>,
}

impl ClientWasmExtension {
    pub fn main(&mut self) -> eyre::Result<()> {
        self.wasm_main
            .call(&mut self.wasm_store, ())
            .map_err(|e| eyre::eyre!(e))
    }
}

pub struct ClientExtensionState {
    extension_name: String,
}

pub fn print_name(caller: Caller<'_, ClientExtensionState>) {
    println!("{} called print_name", caller.data().extension_name);
}

pub fn print_transform(mut caller: Caller<'_, ClientExtensionState>, ptr: u32) {
    let mem = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => Ok(mem),
        _ => Err(eyre::eyre!(
            "Unable to get wasm memory from: {}",
            caller.data().extension_name
        )),
    }
    .unwrap();
    let str_bytes = mem
        .data(&caller)
        .get(ptr as usize..)
        .unwrap();
    let transform_ptr = str_bytes.as_ptr() as *const Transform;

    println!("Info::{}: {:#?}", caller.data().extension_name, unsafe {
        &*transform_ptr
    },);
}

pub fn wasm_get_transfrom(mut caller: Caller<'_, ClientExtensionState>) -> u32 {
    let alloc = caller
        .get_export("alloc")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<u32, u32>(&caller)
        .unwrap();
    let transform_ptr = alloc
        .call(&mut caller, size_of::<Transform>() as u32)
        .unwrap();
    let transform = Transform {
        pos: Vec3 {
            x: 1.0,
            y: 1.8,
            z: -0.1,
        },
        rotation: wasm_api::Quat {
            x: 0.0,
            y: 0.1,
            z: 1.0,
            w: 0.3,
        },
    };
    let mem = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => Ok(mem),
        _ => Err(eyre::eyre!(
            "Unable to get wasm memory from: {}",
            caller.data().extension_name
        )),
    }
    .unwrap();
    mem.write(&mut caller, transform_ptr as usize, unsafe {
        any_as_u8_slice(&transform)
    })
    .unwrap();

    transform_ptr
}
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn wasm_print(mut caller: Caller<'_, ClientExtensionState>, s: u32) {
    let mem = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => Ok(mem),
        _ => Err(eyre::eyre!(
            "Unable to get wasm memory from: {}",
            caller.data().extension_name
        )),
    }
    .unwrap();
    let str_bytes = mem.data(&caller).get(s as usize..).unwrap();
    let c_str = CStr::from_bytes_until_nul(str_bytes).unwrap();

    println!(
        "Info::{}: {}",
        caller.data().extension_name,
        c_str.to_str().unwrap()
    );
}

pub fn create_extension(
    engine: &Engine,
    module: Module,
    name: &str,
) -> eyre::Result<ClientWasmExtension> {
    let mut store = Store::new(
        engine,
        ClientExtensionState {
            extension_name: name.to_string(),
        },
    );
    let print_name_wasm = Func::wrap(&mut store, print_name);
    let print_wasm = Func::wrap(&mut store, wasm_print);
    let print_transform = Func::wrap(&mut store, print_transform);
    let get_transform = Func::wrap(&mut store, wasm_get_transfrom);

    let imports = [
        print_name_wasm.into(),
        print_wasm.into(),
        get_transform.into(),
        print_transform.into(),
    ];
    let instance = Instance::new(&mut store, &module, &imports).map_err(|e| eyre::eyre!(e))?;
    let run = instance
        .get_typed_func::<(), ()>(&mut store, "main")
        .map_err(|e| eyre::eyre!(e))?;
    Ok(ClientWasmExtension {
        wasm_instance: instance,
        wasm_store: store,
        wasm_main: run,
    })
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let mut config = Config::default();
    config.debug_info(true);
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.debug_info(true);
    config.coredump_on_trap(true);
    let engine = Engine::new(&config).map_err(|e| eyre::eyre!(e))?;
    let module = Module::from_file(
        &engine,
        "./target/wasm32-unknown-unknown/debug/wasm_test.wasm",
    )
    .map_err(|e| eyre::eyre!(e))?;
    let module_as = Module::from_file(&engine, "./assemblyscript_test/build/release.wasm")
        .map_err(|e| eyre::eyre!(e))?;
    for m in module_as.imports() {
        println!("{:#?}", m)
    }
    let mut extension = create_extension(&engine, module, "test")?;
    let mut extension_as = create_extension(&engine, module_as, "assemblyscript")?;
    extension.main()?;
    extension_as.main()?;

    println!("Hello, world!");
    Ok(())
}
