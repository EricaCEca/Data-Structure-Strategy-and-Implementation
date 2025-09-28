use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};
use clap::Parser;
use anyhow::Result;
use mega_store_search::{indexer::InvertedIndex, models::Product, search::SearchEngine};

#[derive(Parser)]
struct Args {
    /// caminho para o dataset JSON (array de produtos)
    #[arg(short, long)]
    dataset: Option<String>,
}

/// Carrega produtos de um arquivo JSON
fn load_products_from_file(path: &str) -> Result<Vec<Product>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let products: Vec<Product> = serde_json::from_reader(reader)?;
    Ok(products)
}

/// Loop interativo de busca
fn interactive_loop(engine: &SearchEngine) {
    println!("Modo interativo: digite uma query (ou 'sair' para terminar)");
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            println!("Erro ao ler input");
            continue;
        }

        let query = line.trim();
        if query.eq_ignore_ascii_case("sair") {
            println!("Encerrando...");
            break;
        }

        let results = engine.search(query, 10, "AND"); // top_k = 10, modo AND
        if results.is_empty() {
            println!("Nenhum produto encontrado para '{}'", query);
        } else {
            println!("Produtos encontrados:");
            for p in results {
                println!("ID: {}, Nome: {}, Categoria: {}", p.id, p.name, p.category);
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dataset_path = match args.dataset {
        Some(p) => p,
        None => {
            println!("Nenhum dataset fornecido. Use --dataset <caminho>");
            return Ok(());
        }
    };

    let products = load_products_from_file(&dataset_path)?;
    let engine = SearchEngine::new(products);

    interactive_loop(&engine);

    Ok(())
}
