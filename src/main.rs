use std::{env::{self}, fs, thread, time::Duration};
use gfxsrc::{self, Screen};

struct Matrix{
    color: Box<str>,
    repeat:bool,
    app: Screen,
    content: String,
}

fn prepare_matrix() -> Matrix {
    let args: Vec<String> = env::args().collect();
    let mut color = "#FFFFFF".into();
    let mut repeat = false;
    for i in &args {
        let u = i.split('=').collect::<Vec<&str>>();
        if u[0] == "--loop" {
            repeat = true;
        } else if u[0] == "--matrixcolor" {
            color = u[1].into();
        }
    }
    let path = &args[args.len()-1];
    let content = fs::read_to_string(path).expect("Path not found");
    Matrix{
        color: color,
        repeat: repeat,
        app: gfxsrc::Screen::new(16, 8, ' '.to_string()),
        content: content,
    }
}


fn showmatrix(mut matrix: Matrix){
    if matrix.repeat == true {
        loop {
            for line in matrix.content.lines() {
                let parts = line.split('-').collect::<Vec<&str>>();
                if parts[0] == "OFF" {
                    for i in 1..8{
                        for u in 1..8{
                            matrix.app.addstring(i*2, u, " ", "#FFFFFF")
                        }
                    }
                } else if parts[0] == "ON" {
                    for i in 1..8{
                        for u in 1..8{
                            matrix.app.addstring(i*2, u, "#", &matrix.color)
                        }
                    }
                } else {
                    for pixel in parts[0].split(',') {
                        let possitions = pixel.split_whitespace().collect::<Vec<&str>>();
                        matrix.app.addstring(((possitions[0].parse::<usize>().unwrap())+1)*2,(possitions[1].parse::<usize>().unwrap())+1, "#", &matrix.color);
                    }
                }
                matrix.app.print();
                thread::sleep(Duration::from_millis(parts[1].parse::<u64>().unwrap()));
            }
        }
    } else {
        for line in matrix.content.lines() {
            let parts = line.split('-').collect::<Vec<&str>>();
            if parts[0] == "OFF" {
                for i in 1..8{
                    for u in 1..8{
                        matrix.app.addstring(i*2, u, " ", "#FFFFFF")
                    }
                }
            } else if parts[0] == "ON" {
                for i in 1..8{
                    for u in 1..8{
                        matrix.app.addstring(i*2, u, "#", &matrix.color)
                    }
                }
            } else {
                for pixel in parts[0].split(',') {
                    let possitions = pixel.split_whitespace().collect::<Vec<&str>>();
                    matrix.app.addstring(((possitions[0].parse::<usize>().unwrap())+1)*2,(possitions[1].parse::<usize>().unwrap())+1, "#", &matrix.color);
                }
            }
            matrix.app.print();
            thread::sleep(Duration::from_millis(parts[1].parse::<u64>().unwrap()));
        }
    }
}


fn main() {
    let matrix = prepare_matrix();
    showmatrix(matrix);
}
