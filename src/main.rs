use lean_sys::*;
mod callee {
	#[link(name = "Callee")]
	extern "C" {
		#[link_name = "initialize_Callee"]
		pub fn initialize(builtin: u8, world: lean_sys::lean_obj_arg) -> lean_sys::lean_obj_res;
        pub fn print_hello(world: lean_sys::lean_obj_arg) -> lean_sys::lean_obj_res;
	}
}

// https://lean-lang.org/lean4/doc/dev/ffi.html#initialization
// https://git.leni.sh/aniva/RustCallLean/src/branch/main/src/main.rs#L30
fn initialize_lean_environment() {
    unsafe {
        lean_initialize_runtime_module();
        lean_initialize();  // necessary if you (indirectly) access the `Lean` package
        let builtin: u8 = 1;
        let res = callee::initialize(builtin, lean_io_mk_world());
        if lean_io_result_is_ok(res) {
            lean_dec_ref(res);
        } else {
            lean_io_result_show_error(res);
            lean_dec(res);
            panic!("Failed to load callee!");
        }
        //lean_init_task_manager(); // necessary if you (indirectly) use `Task`
        lean_io_mark_end_initialization();
    }
}

fn print_hello() {
    unsafe {
        let res = callee::print_hello(lean_io_mk_world());
		if lean_io_result_is_ok(res) {
			lean_dec_ref(res);
		} else {
			lean_io_result_show_error(res);
			lean_dec(res);
			panic!("IO Monad execution failed");
		}
    }
}

fn main() {
    initialize_lean_environment();
    print_hello();
}
