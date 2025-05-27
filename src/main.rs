mod utils;
mod ais;
mod buddy;
mod error;
mod mistral;

pub use self::error::{Error, Result};

use crate::utils::prompt;
use crate::mistral::mistral_chat;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	println!();

	match start().await {
		Ok(_) => println!("\n👋 Encerrado.\n"),
		Err(e) => println!("\n❌ Erro: {e:?}\n"),
	}
}

// commands

#[derive(Debug)]
enum Cmd {
	Quit, 
	SelectPatient(String),
	AddSymptom(String),
	GenerateDiagnosis,
	Save,
	FreeChat(String)
}

impl Cmd {
	fn from_input(input: impl Into<String>) -> Self {
		let input = input.into();
		let trimmed = input.trim();

		if trimmed == "/q" {
			Self::Quit
		} else if trimmed == "/dx" {
			Self::GenerateDiagnosis
		} else if trimmed == "/save" {
			Self::Save
		} else if trimmed.starts_with("/patient") {
			let id = trimmed.replacen("/patient", "", 1).to_string();
			Self::SelectPatient(id)
		} else if trimmed.starts_with("/add") {
			let symptom =  trimmed.replacen("/add", "", 1).to_string();
			Self::AddSymptom(symptom)
		} else {
			Self::FreeChat(trimmed.to_string())
		}
	}
}

// patient structure

#[derive(Debug)]
struct Patient {
	id: String,
	symptom: Vec<String>,
	diagnosis: Option<String>,
}

impl Patient {
	fn new(id: &str) -> Self {
		Self {
			id: id.to_string(),
			symptom: vec![],
			diagnosis: None,
		}
	}
}


async fn start() -> Result<()> {
    let mut patient: HashMap<String, Patient> = HashMap::new();
    let mut actual_patient: Option<String> = None;
    
    loop {
		println!("\n---");

        let input = prompt("🤖 Digite um dos comandos ou pergunte")?;
        let cmd = Cmd::from_input(input);

        match cmd {
            Cmd::Quit => break,

            Cmd::SelectPatient(id) => {
                let id = id.trim().to_string();
                println!("📁 Paciente selecionado: {id}");
                patient.entry(id.clone()).or_insert(Patient::new(&id));
                actual_patient = Some(id);
            }

            Cmd::AddSymptom(symptom) => {
                let symptom = symptom.trim().to_string();
                if let Some(id) = &actual_patient {
                    if let Some(p) = patient.get_mut(id) {
                        p.symptom.push(symptom.clone());
                        println!("✅ Sintoma \"{symptom}\" adicionado ao paciente \"{id}\"");
                    }
                } else {
                    println!("⚠️ Nenhum paciente selecionado. Use /patient <id> primeiro.");
                }
            }

            Cmd::GenerateDiagnosis => {
                if let Some(id) = &actual_patient {
                    if let Some(p) = patient.get_mut(id) {
                        let diag = format!(
                            "Diagnóstico simulado com base em {} sintoma(s).",
                            p.symptom.len()
                        );
                        p.diagnosis = Some(diag.clone());
                        println!("🧠 Diagnóstico gerado para \"{id}\": {diag}");
                    }
                } else {
                    println!("⚠️ Nenhum paciente selecionado.");
                }
            }

            Cmd::Save => {
                if let Some(id) = &actual_patient {
                    if let Some(p) = patient.get(id) {
                        println!("💾 Dados salvos para paciente \"{id}\":\n{:#?}", p);
                        // Futuro: salvar realmente em arquivo ou banco
                    }
                } else {
                    println!("⚠️ Nenhum paciente selecionado.");
                }
            }

            Cmd::FreeChat(msg) => {
                // Aqui usamos a função mistral_chat para chamar a IA
                match mistral_chat(&msg).await {
                    Ok(answer) => println!("\n🤖 Resposta IA:\n{}", answer),
                    Err(e) => println!("❌ Erro ao chamar IA: {}", e),
                }
            }
        }
    }

    Ok(())
}