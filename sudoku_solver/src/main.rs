// src/main.rs
use actix_web::{post, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
mod sudoku;

#[derive(Serialize, Deserialize)]
struct SudokuInput {
    cells: [[u32; 9]; 9],
}

#[post("/solve")]
async fn solve_sudoku(sudoku_data: web::Json<SudokuInput>) -> HttpResponse {
    let mut sudoku = sudoku::Sudoku::new(sudoku_data.cells);
    
    // Log the board to see if the input is correct
    println!("Received board: {:?}", sudoku.board);

    if sudoku.solve() {
        // Return the steps in the response
        HttpResponse::Ok().json(sudoku.steps)
    } else {
        HttpResponse::BadRequest().body("No solution exists")
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(solve_sudoku)
            .service(actix_files::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
