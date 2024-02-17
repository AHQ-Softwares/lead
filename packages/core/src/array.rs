use interpreter::{function, get_mut, methods, module, parse, pkg_name, types::BufValue};

module!(
  Array,
  pkg_name! { "📦 Core / Array" }
  methods! {
    function!("array::malloc", |args, heap, _| {
      parse!(heap + args: > array);

      let _ = heap.set(array.into(), BufValue::Array(vec![]));
    }),
    function!("array::push", |args, heap, _| {
      parse!(heap + args: str arr, -> value);

      get_mut!(heap: Array arr);

      arr.push(value);
    })
  }
);

// use interpreter::types::BufValue;
// use interpreter::val::error;
// use interpreter::{types::MethodRes, Package};

// pub struct Array;

// impl Package for Array {
//   fn name(&self) -> &'static [u8] {
//     "📦 Lead Programming Language / Core / Array".as_bytes()
//   }

//   fn methods(&self) -> MethodRes {
//     &[
//       ("array::malloc", |args, val, _| {
//         let [_, a] = &args[..] else {
//           error(
//             r#"Invalid arguments in :array::malloc
//       Format ---
//       - array::malloc $1"#,
//           )
//         };

//         val.set(a.clone(), BufValue::Array(vec![]));
//       }),
//       ("array::push", |args, heap, _| {
//         let [_, arr, val] = &args[..] else {
//           error(
//             r#"Invalid arguments in array::malloc
//       Format ---
//       - array::push $arr $myval"#,
//           )
//         };

//         let val = heap
//           .remove(val)
//           .unwrap_or_else(|| error("Invalid value provided"))
//           .unwrap_or_else(|| error("Pointer / Invalid variable"));

//         heap.get_mut(arr).map_or_else(
//           || error("Invalid array"),
//           |x| match x {
//             BufValue::Array(arr) => arr.push(val),
//             _ => error("Not an array!"),
//           },
//         );
//       }),
//     ]
//   }
// }
