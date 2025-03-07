mod crypto;
mod routes;
mod storage;

use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {
    let key: [u8; 32] = *b"this_is_a_strong_32_byte_key12!!"; // ✅ 32 bytes
    let storage = Arc::new(storage::Storage::new("db", &key));

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "create" => {
                if args.len() < 3 {
                    eprintln!("Usage: cargo run -- create \"Your message here\"");
                    return;
                }
                let note = args[2..].join(" "); // Supports spaces in notes
                let id = storage.save_note(&note);
                println!(
                    "Note created! Access it using:\nhttp://127.0.0.1:3030/read/{}",
                    id
                );
            }
            "read" => {
                if args.len() != 3 {
                    eprintln!("Usage: cargo run -- read <note_id>");
                    return;
                }
                let id = &args[2];
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    if let Some(note) = storage.get_and_delete(id) {
                        println!("Note: {}", note);
                    } else {
                        println!("Error: Note not found.");
                    }
                });
            }
            _ => {
                eprintln!(
                    "Unknown command. Usage:\n  cargo run -- create \"Your message\"\n  cargo run -- read <note_id>"
                );
            }
        }
        return;
    }

    // Run as a server if no CLI args are passed
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let routes = routes::note_routes(storage);
        println!("Server running on http://127.0.0.1:3030/");
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });
}
