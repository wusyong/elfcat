mod elf;
mod report_gen;
mod utils;

use elf::parser::ParsedElf;

fn main() -> wry::Result<()> {
    let filename = parse_arguments();
    let contents = std::fs::read(&filename).unwrap();
    let elf = ParsedElf::from_bytes(&filename, &contents).unwrap();
    let report_filename = report_gen::construct_filename(&filename);
    let report = report_gen::generate_report(&elf);

    std::fs::write(&report_filename, &report).expect("failed to write report");

    use std::fs::{canonicalize, read};
    use wry::{
        application::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("elfcat")
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_custom_protocol("elf".into(), move |_, path| {
            let path = path.replace("elf://", "");
            let content = read(canonicalize(&path)?)?;

            Ok((content, String::from("text/html")))
        })
        .with_url(&format!("elf://{}", report_filename))?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn parse_arguments() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage(1);
    }

    if args[1] == "-h" || args[1] == "--help" {
        usage(0);
    }

    if args[1] == "-v" || args[1] == "--version" {
        println!("elfcat {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    args[1].clone()
}

fn usage(ret: i32) {
    println!("Usage: elfcat <filename>");
    println!("Writes <filename>.html to CWD.");

    std::process::exit(ret);
}
