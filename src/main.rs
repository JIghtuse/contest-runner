extern crate diff;
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

enum Error {
    Io(io::Error),
    TooManyInputs((BinaryHeap<String>, BinaryHeap<String>)),
    TooManyOutputs((BinaryHeap<String>, BinaryHeap<String>)),
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

fn get_testcases() -> Result<Vec<Testcase>, Error> {
    let input_re = Regex::new(r"input.*").unwrap();
    let output_re = Regex::new(r"output.*").unwrap();

    let mut testcases = vec![];
    let mut inputs = BinaryHeap::new();
    let mut outputs = BinaryHeap::new();

    for entry in fs::read_dir(".")? {
        let path = entry?.path().into_os_string();
        if let Some(input) = input_re.captures(path.to_str().unwrap()) {
            inputs.push(input[0].to_string());
        }
        if let Some(output) = output_re.captures(path.to_str().unwrap()) {
            outputs.push(output[0].to_string());
        }
    }
    if inputs.len() < outputs.len() {
        return Err(Error::TooManyOutputs((inputs, outputs)));
    } else if inputs.len() > outputs.len() {
        return Err(Error::TooManyInputs((inputs, outputs)));
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
        .stderr(Stdio::null())
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
        let mut s = String::new();
        for diff in diff::lines(&out, &out_expected) {
            s += &match diff {
                diff::Result::Left(l)    => format!("-{}\n", l),
                diff::Result::Both(l, _) => format!(" {}\n", l),
                diff::Result::Right(r)   => format!("+{}\n", r),
            };
        }
        Err(io::Error::new(io::ErrorKind::Other, s))
    }
}

fn main() {
    let binary = if let Some(name) = std::env::args().nth(1) {
        name
    } else {
        String::from("./a.out")
    };

    if !std::path::Path::new(&binary).exists() {
        println!("Binary {} does not exist", binary);
        std::process::exit(1);
    }

    let mut failed_testcases = vec![];
    match get_testcases() {
        Ok(testcases) => {
            println!("{} testcases found", testcases.len());
            for testcase in testcases {
                match run_testcase(&binary, &testcase) {
                    Ok(_) => print!("."),
                    Err(e) => {
                        failed_testcases.push((testcase, e));
                        print!("E");
                    }
                }
            }
            println!();
        }
        Err(Error::Io(e)) => println!("I/O error: {}", e),
        Err(Error::TooManyInputs((i, o))) => {
            println!("There are more inputs than outputs.
Inputs : {:?}
Outputs: {:?}",
                     i,
                     o);
        }
        Err(Error::TooManyOutputs((i, o))) => {
            println!("There are more outputs than inputs.
Inputs : {:?}
Outputs: {:?}",
                     i,
                     o);
        }
    }
    if !failed_testcases.is_empty() {
        println!("Failed testcases:");
        for (testcase, e) in failed_testcases {
            println!("{:?}:\n{}\n", testcase, e);
        }
        std::process::exit(1);
    } else {
        println!("OK");
    }
}
