extern crate regex;

use std::fs::File;
use std::process::{Command, Stdio};
use std::collections::BinaryHeap;
use std::fs;
use regex::Regex;
use std::io::{self, Read, Write};

#[derive(Debug)]
struct Testcase {
    input: String,
    output: String,
}

fn get_testcases() -> Result<Vec<Testcase>, io::Error> {
    let input_re = Regex::new(r"input.*").unwrap();
    let output_re = Regex::new(r"output.*").unwrap();

    let mut testcases = vec![];
    let mut inputs = BinaryHeap::new();
    let mut outputs = BinaryHeap::new();

    for entry in try!(fs::read_dir(".")) {
        let path = try!(entry);
        let path = path.path().into_os_string();
        if let Some(input) = input_re.captures(path.to_str().unwrap()) {
            inputs.push(input[0].to_string());
        }
        if let Some(output) = output_re.captures(path.to_str().unwrap()) {
            outputs.push(output[0].to_string());
        }
    }
    if inputs.len() != outputs.len() {
        return Err(io::Error::new(io::ErrorKind::Other, "mismatched input/output"));
    }
    while !inputs.is_empty() {
        testcases.push(Testcase {
            input: inputs.pop().unwrap(),
            output: outputs.pop().unwrap(),
        });
    }
    Ok(testcases)
}

fn run_testcase(binary: &str, testcase: &Testcase) -> Result<(), io::Error> {
    let process = Command::new(binary).stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut f = File::open(&testcase.input)?;
    let mut inp = String::new();
    f.read_to_string(&mut inp)?;
    process.stdin.unwrap().write_all(inp.as_bytes())?;

    let mut outf = File::open(&testcase.output)?;
    let mut out_expected = String::new();
    outf.read_to_string(&mut out_expected)?;

    let mut out = String::new();
    process.stdout.unwrap().read_to_string(&mut out)?;
    if out == out_expected {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other,
                           format!("  Expected: '{}'
  Actual:   '{}'",
                                   out_expected,
                                   out)))
    }
}

fn main() {
    let binary = "./a.out";

    let mut failed_testcases = vec![];
    match get_testcases() {
        Ok(testcases) => {
            for testcase in testcases {
                match run_testcase(binary, &testcase) {
                    Ok(_) => print!("."),
                    Err(e) => {
                        failed_testcases.push((testcase, e));
                        print!("E");
                    }
                }
            }
            println!();
        }
        Err(e) => println!("{}", e),
    }
    if !failed_testcases.is_empty() {
        println!("Failed testcases:");
        for (testcase, e) in failed_testcases {
            println!("{:?}:\n{}\n", testcase, e);
        }
    } else {
        println!("OK");
    }
}
