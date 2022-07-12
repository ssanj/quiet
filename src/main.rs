use std::io::{self, BufRead};
use serde_json::Result as JsonResult;
use crate::models::{CompilerMessage, Reason};
mod models;

fn main() -> JsonResult<()>{
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
      let line = line_result.unwrap();
      let reason: Reason = serde_json::from_str(&line)?;

      //if  type of reason is compiler-message, then we want the full payload otherwise ignore?
      // we also want the build-finished
      if reason.reason == "compiler-message" {
        println!("Line ---> {}", &line);
        let compiler_message: CompilerMessage = serde_json::from_str(&line)?;
        println!("*** {:?}", compiler_message);
      } else {
        //do nothing
      }
    }

    Ok(())
}
