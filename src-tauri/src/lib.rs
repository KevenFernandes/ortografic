// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use bk_tree::{BKTree, Metric};
use serde::Serialize;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use strsim::levenshtein;
use tauri::State;

// Estrutura do estado global do backend
pub struct AppState {
    pub icf_map: HashMap<String, f32>,
    pub tree: BKTree<String, LevenshteinMetric>,
}

// Estrutura de resposta JSON para o react
#[derive(Serialize)]
pub struct RespostaVerificacao {
    pub status: String,
    pub sugestoes: Vec<String>, //vetor []
}

pub struct LevenshteinMetric;
impl Metric<String> for LevenshteinMetric {
    fn distance(&self, a: &String, b: &String) -> u32 {
        levenshtein(a, b) as u32
    }

    fn threshold_distance(&self, a: &String, b: &String, threshold: u32) -> Option<u32> {
        let dist = self.distance(a, b);
        if dist <= threshold { Some(dist) } else { None }
    }
}

fn gerar_sugestoes_icf(
    palavra: &str,
    icf_map: &HashMap<String, f32>,
    tree: &BKTree<String, LevenshteinMetric>,
) -> Vec<String> {
    let palavra_string = palavra.to_string();
    let resultados = tree.find(&palavra_string, 2);

    let mut sugestoes: Vec<(&String, u32, f32, usize)> = Vec::new();

    for (distancia, palavra_encontrada) in resultados {
        let icf = *icf_map.get(palavra_encontrada).unwrap_or(&0.0);

        let prefixo_comum = palavra.chars().zip(palavra_encontrada.chars()).take_while(|(a, b)| a == b).count();

        sugestoes.push((palavra_encontrada, distancia, icf, prefixo_comum));
    }

    sugestoes.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| b.3.cmp(&a.3))
            .then_with(|| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal))
    });

    sugestoes.into_iter().take(5).map(|(p, _, _, _)| p.clone()).collect()
}

fn carregar_dados() -> AppState {
    let mut icf = HashMap::new();
    let mut tree = BKTree::new(LevenshteinMetric);

    if let Ok(arquivo) = File::open("dicionarios/icf.txt") {
        let leitor = BufReader::new(arquivo);
        for linha in leitor.lines().flatten() {
            let linha_str = linha.trim();
            if linha_str.is_empty() {
                continue;
            }

            let mut partes = linha_str.split(',');

            if let (Some(palavra_raw), Some(valor_str)) = (partes.next(), partes.next()) {
                let palavra = palavra_raw.trim().to_lowercase();
                let valor_formatado = valor_str.trim().replace(',', ".");

                if let Ok(valor) = valor_formatado.parse::<f32>() {
                    tree.add(palavra.clone());
                    icf.insert(palavra, valor);
                }
            }
        }
    } else {
        eprintln!("Aviso: Não foi possível abrir o arquivo dicionarios/icf.txt");
    }

    println!(
        "Carregamento concluído: {} entradas no ICF.",
        icf.len()
    );

    AppState {
        icf_map: icf,
        tree,
    }
}

#[tauri::command]
fn verificar_texto(
    palavra_digitada: String,
    state: State<'_, Mutex<AppState>>,
) -> RespostaVerificacao {
    let palavra = palavra_digitada.trim().to_lowercase();
    let app_state = state.lock().unwrap();

    if app_state.icf_map.contains_key(&palavra) {
        RespostaVerificacao {
            status: "correto".to_string(),
            sugestoes: vec![],
        }
    } else {
        let sugestoes = gerar_sugestoes_icf(&palavra, &app_state.icf_map, &app_state.tree);
        RespostaVerificacao {
            status: "error".to_string(),
            sugestoes,
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let estado_inicial = carregar_dados();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(estado_inicial))
        .invoke_handler(tauri::generate_handler![verificar_texto])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
