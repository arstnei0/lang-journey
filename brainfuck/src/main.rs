use clap::{arg, command};
use std::fs;

pub mod run {
    use std::io::{stdin, Read};

    pub enum Operator {
        Left,
        Right,
        Add,
        Minus,
        Out,
        In,
        Start,
        End,
    }

    pub fn get_operator_from_char(char: char) -> Option<Operator> {
        if char == '<' {
            Some(Operator::Left)
        } else if char == '>' {
            Some(Operator::Right)
        } else if char == '+' {
            Some(Operator::Add)
        } else if char == '-' {
            Some(Operator::Minus)
        } else if char == '.' {
            Some(Operator::Out)
        } else if char == ',' {
            Some(Operator::In)
        } else if char == '[' {
            Some(Operator::Start)
        } else if char == ']' {
            Some(Operator::End)
        } else {
            None
        }
    }

    pub fn exe(content: String, debug: bool, interval: u32) {
        let mut arr: Vec<i16> = vec![0; 10000];
        let mut pos = 0;
        // The loop stack contains the starting point of the loops
        let mut loop_stack: Vec<usize> = vec![];

        let mut max_pos = 0;
        let update_max_pos = |max_pos: &mut usize, pos: &usize| {
            if debug {
                if *max_pos < *pos {
                    *max_pos = pos.clone();
                }
            }
        };
        let mut output = "".to_string();

        let mut char_pos = 0;
        let chars = content.chars();
        let total = content.len();

        loop {
            match chars.clone().nth(char_pos) {
                Some(char) => {
                    let operator = get_operator_from_char(char);
                    match operator {
                        Some(operator) => {
                            if debug {
                                std::thread::sleep(std::time::Duration::from_millis(
                                    interval.into(),
                                ));
                                println!("{} {} {:?}", char, pos, &arr[0..max_pos + 1]);
                            }

                            match operator {
                                Operator::Add => {
                                    arr[pos] += 1;
                                }
                                Operator::Minus => {
                                    arr[pos] -= 1;
                                }
                                Operator::Out => {
                                    let character = char::from_u32(match arr[pos].try_into() {
                                        Ok(n) => n,
                                        Err(_err) => {
                                            break;
                                        }
                                    })
                                    .unwrap();

                                    if debug {
                                        output.push(character);
                                        println!("************* Output: {} **************", output);
                                    } else {
                                        print!("{}", character);
                                    }
                                }
                                Operator::In => {
                                    let mut input = [0; 1];
                                    let result = stdin().read(&mut input);
                                    match result {
                                        Ok(_data) => {
                                            arr[pos] = (*input.get(0).unwrap()).into();
                                        }
                                        Err(_err) => {
                                            println!("Input not legal!");
                                            break;
                                        }
                                    }
                                }
                                Operator::Right => {
                                    pos += 1;
                                    update_max_pos(&mut max_pos, &pos);
                                }
                                Operator::Left => {
                                    pos -= 1;
                                    update_max_pos(&mut max_pos, &pos);
                                }
                                Operator::Start => {
                                    loop_stack.push(char_pos);
                                }
                                Operator::End => {
                                    if arr[pos] == 0 {
                                        loop_stack.pop();
                                    } else {
                                        char_pos = *loop_stack.last().unwrap();
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                None => break,
            }

            char_pos += 1;

            if char_pos == total {
                break;
            }
        }
    }
}

fn main() {
    let matches = command!()
        .arg(arg!(<file> "The brainfuck file (*.bf) that you want to run"))
        .arg(arg!(
        -d --debug "Set debug mode"
        ))
        .arg(arg!(
        -i --interval <VALUE> "The duration the program stops for debugging (milliseconds) (default 30)"
        ))
        .get_matches();

    let debug = {
        match matches.get_one::<bool>("debug") {
            Some(d) => *d,
            None => false,
        }
    };

    if debug {
        println!("The debug format is: <current_operation> <pointer_position> <the_array> (The current position starts from 0)")
    }

    let interval: u32 = {
        match matches.get_one::<String>("interval") {
            Some(d) => match d.parse() {
                Ok(i) => i,
                Err(_err) => 30,
            },
            None => 30,
        }
    };

    let filepath = matches
        .get_one::<String>("file")
        .expect("No file specified");

    let content = fs::read_to_string(filepath).expect("Unable to read that file");

    run::exe(content, debug, interval);
    println!("");
}
