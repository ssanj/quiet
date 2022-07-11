use std::io::{self, BufRead};
use serde_json::Result as JsonResult;
use crate::models::Reason;
mod models;

//{"reason":"build-finished","success":true}

// {
//     "reason": "compiler-message",
//     "package_id": "purs 0.3.11 (path+file:///Volumes/Work/projects/code/rust/toy/purs)",
//     "manifest_path": "/Volumes/Work/projects/code/rust/toy/purs/Cargo.toml",
//     "target":
//     {
//         "kind":
//         [
//             "bin"
//         ],
//         "crate_types":
//         [
//             "bin"
//         ],
//         "name": "avatar_cache",
//         "src_path": "/Volumes/Work/projects/code/rust/toy/purs/src/avatar_cache.rs",
//         "edition": "2021",
//         "doc": true,
//         "doctest": false,
//         "test": true
//     },
//     "message":
//     {
//         "rendered": "error: aborting due to previous error\n\n",
//         "children":
//         [],
//         "code": null,
//         "level": "error",
//         "message": "aborting due to previous error",
//         "spans":
//         []
//     }
// }

//type 2:
// {
//     "reason": "compiler-message",
//     "package_id": "purs 0.3.11 (path+file:///Volumes/Work/projects/code/rust/toy/purs)",
//     "manifest_path": "/Volumes/Work/projects/code/rust/toy/purs/Cargo.toml",
//     "target":
//     {
//         "kind":
//         [
//             "bin"
//         ],
//         "crate_types":
//         [
//             "bin"
//         ],
//         "name": "avatar_cache",
//         "src_path": "/Volumes/Work/projects/code/rust/toy/purs/src/avatar_cache.rs",
//         "edition": "2021",
//         "doc": true,
//         "doctest": false,
//         "test": true
//     },
//     "message":
//     {
//         "rendered": "error[E0432]: unresolved import `crate::log`\n --> src/avatar.rs:8:12\n  |\n8 | use crate::log::print_errors;\n  |            ^^^ could not find `log` in the crate root\n\n",
//         "children":
//         [],
//         "code":
//         {
//             "code": "E0432",
//             "explanation": "An import was unresolved.\n\nErroneous code example:\n\n```compile_fail,E0432\nuse something::Foo; // error: unresolved import `something::Foo`.\n```\n\nIn Rust 2015, paths in `use` statements are relative to the crate root. To\nimport items relative to the current and parent modules, use the `self::` and\n`super::` prefixes, respectively.\n\nIn Rust 2018, paths in `use` statements are relative to the current module\nunless they begin with the name of a crate or a literal `crate::`, in which\ncase they start from the crate root. As in Rust 2015 code, the `self::` and\n`super::` prefixes refer to the current and parent modules respectively.\n\nAlso verify that you didn't misspell the import name and that the import exists\nin the module from where you tried to import it. Example:\n\n```\nuse self::something::Foo; // Ok.\n\nmod something {\n    pub struct Foo;\n}\n# fn main() {}\n```\n\nIf you tried to use a module from an external crate and are using Rust 2015,\nyou may have missed the `extern crate` declaration (which is usually placed in\nthe crate root):\n\n```edition2015\nextern crate core; // Required to use the `core` crate in Rust 2015.\n\nuse core::any;\n# fn main() {}\n```\n\nIn Rust 2018 the `extern crate` declaration is not required and you can instead\njust `use` it:\n\n```edition2018\nuse core::any; // No extern crate required in Rust 2018.\n# fn main() {}\n```\n"
//         },
//         "level": "error",
//         "message": "unresolved import `crate::log`",
//         "spans":
//         [
//             {
//                 "byte_end": 246,
//                 "byte_start": 243,
//                 "column_end": 15,
//                 "column_start": 12,
//                 "expansion": null,
//                 "file_name": "src/avatar.rs",
//                 "is_primary": true,
//                 "label": "could not find `log` in the crate root",
//                 "line_end": 8,
//                 "line_start": 8,
//                 "suggested_replacement": null,
//                 "suggestion_applicability": null,
//                 "text":
//                 [
//                     {
//                         "highlight_end": 15,
//                         "highlight_start": 12,
//                         "text": "use crate::log::print_errors;"
//                     }
//                 ]
//             }
//         ]
//     }
// }

fn main() -> JsonResult<()>{
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
      let line = line_result.unwrap();
      let reason: Reason = serde_json::from_str(&line)?;

      //if  type of reason is compiler-message, then we want the full payload otherwise ignore?
      // we also want the build-finished
        println!("*** {}", reason);
    }

    Ok(())
}
