use std::{fs, thread, time::Duration, env};
use gfxsrc;

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut color = "#FFFFFF";
    let mut repeat = false;
    for i in &args {
        let u = i.split('=').collect::<Vec<&str>>();
        if u[0] == "--loop" {
            repeat = true;
        } else if u[0] == "--matrixcolor" {
            color = u[1];
        }
    }
    let path = &args[args.len()-1];
    let content = fs::read_to_string(path).expect("Path not found");
    let mut app = gfxsrc::Screen::new(16, 8, ' '.to_string());
    for line in content.lines() {
        let parts = line.split('-').collect::<Vec<&str>>();
        if parts[0] == "OFF" {
            for i in 1..8{
                for u in 1..8{
                    app.addstring(i*2, u, " ", "#FFFFFF")
                }
            }
        } else if parts[0] == "ON" {
            for i in 1..8{
                for u in 1..8{
                    app.addstring(i*2, u, "#", &color)
                }
            }
        } else {
            for pixel in parts[0].split(',') {
                let possitions = pixel.split_whitespace().collect::<Vec<&str>>();
                app.addstring(((possitions[0].parse::<usize>().unwrap())+1)*2,(possitions[1].parse::<usize>().unwrap())+1, "#", &color);
            }
        }
        app.print();
        thread::sleep(Duration::from_millis(parts[1].parse::<u64>().unwrap()));
    }
}
