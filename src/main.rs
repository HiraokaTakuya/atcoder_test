extern crate reqwest;

use std::io::prelude::*;
use std::process::{Command, Stdio};

fn input(child: &mut std::process::Child, s: &str) {
    let s = s.to_owned() + "\n";
    let stdin = child.stdin.as_mut().unwrap();
    match stdin.write(s.as_bytes()) {
        Ok(_) => {}
        Err(err) => panic!("coundn't write_all to stdin: {}", err),
    };
}

fn output(child: std::process::Child) -> String {
    String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap()
}

struct Engine {
    process: std::process::Child,
}

impl Engine {
    fn new(command_name: &str) -> Engine {
        Engine {
            process: match Command::new(command_name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Ok(process) => process,
                Err(err) => panic!("counldn't spawn {}: {}", command_name, err),
            },
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>(); // target contest problem
    let inouts = {
        let mut res = reqwest::get(&format!(
            "https://atcoder.jp/contests/{0}/tasks/{0}_{1}",
            args[1], args[2]
        ))
        .unwrap();
        let body = res.text().unwrap();
        let mut sample_count = 1; // 1 origin.
        enum State {
            Input,
            Output,
            Other,
        }
        let mut state = State::Other;
        let mut inouts = Vec::<(String, String)>::new();
        for line in body.lines() {
            match state {
                State::Input => {
                    if line.starts_with("</pre>") {
                        state = State::Other;
                    } else {
                        inouts.last_mut().unwrap().0 += line;
                        inouts.last_mut().unwrap().0 += "\n";
                    }
                }
                State::Output => {
                    if line.starts_with("</pre>") {
                        state = State::Other;
                        sample_count += 1;
                    } else {
                        inouts.last_mut().unwrap().1 += line;
                        inouts.last_mut().unwrap().1 += "\n";
                    }
                }
                State::Other => {
                    let input_start = format!("<h3>入力例 {}</h3><pre>", sample_count);
                    if line.starts_with(&input_start) {
                        state = State::Input;
                        inouts.push((String::new(), String::new()));
                        inouts.last_mut().unwrap().0 += &line.replace(&input_start, "");
                        inouts.last_mut().unwrap().0 += "\n";
                    } else {
                        let output_start = format!("<h3>出力例 {}</h3><pre>", sample_count);
                        if line.starts_with(&output_start) {
                            state = State::Output;
                            inouts.last_mut().unwrap().1 += &line.replace(&output_start, "");
                            inouts.last_mut().unwrap().1 += "\n";
                        }
                    }
                }
            }
        }
        inouts
    };
    Command::new("cargo")
        .args(&["build", "--bin", &args[2]])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    for (idx, inout) in inouts.iter().enumerate() {
        let mut engine = Engine::new(&format!("target/debug/{}", args[2]));
        input(&mut engine.process, &inout.0);
        let out = output(engine.process);
        if out == inout.1 {
            println!("test {} is ok", idx + 1);
        } else {
            println!("test {}", idx + 1);
            println!("expected:");
            print!("{}", inout.1);
            println!("found:");
            println!("{}", out);
        }
    }
}
