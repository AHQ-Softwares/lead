use interpreter::{document, function, methods, module, pkg_name, types::BufValue, runtime::RuntimeValue, types::*};
use std::collections::HashMap;

module!(
  IO,
  pkg_name! {"📦 Lead Programming Language / IO"}
  methods! {
    function! {
      "os::name",
      document!(""),
      |_, _, _, opt| {
        opt.set_return_val(if cfg!(windows) {
          BufValue::Str("Win32".into())
        } else if cfg!(target_os = "macos") {
          BufValue::Str("MacOS".into())
        } else if cfg!(target_os = "linux") {
          BufValue::Str("Linux".into())
        } else {
          BufValue::Str("Unknown".into())
        });
      }
    }
  }
);

module!(
  AHQ,
  pkg_name! {"📦 Lead Programming Language / AHQ"}
  methods! {
    function! {
      "ahq::mk",
      document!(""),
      |_, _, _, opt| {
        opt.set_r_runtime(RuntimeValue::new("core/str_string", {
          let mut map: HashMap<&'static _, (&'static _, for<'a, 'b, 'c, 'd, 'e> fn(&'a Vec<String>, &'b mut Heap, &'c mut Heap, &'d String, &'e mut Options))> = HashMap::new();

          map.insert("test", ("", |_, _, _, _, _| {
            println!("This is a test");
          }));

          map
        }));
      }
    }
  }
);