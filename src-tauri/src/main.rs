#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

const QUESTIONS: &[&str] = &[
    "Is your device from the Apple brand?",
    "Do you have the charger for your device?",
];

struct Questionaire {
    questions: Vec<&'static str>,
    answers: Mutex<Vec<bool>>,
    index: Mutex<usize>,
}

impl Questionaire {
    fn new() -> Self {
        Questionaire {
            questions: QUESTIONS.to_vec(),
            answers: Mutex::new(Vec::with_capacity(QUESTIONS.len())),
            index: Mutex::new(QUESTIONS.len()),
        }
    }
}

#[tauri::command]
// get answer from ui.
fn answer(value: bool, questionaire: tauri::State<Questionaire>) {
    questionaire
        .answers
        .lock()
        .unwrap()
        .insert(*questionaire.index.lock().unwrap(), value);
}

#[tauri::command]
// get next question from DB? / maybe hard-coded?
fn next_question(questionaire: tauri::State<'_, Questionaire>) -> String {
    *questionaire.index.lock().unwrap() += 1;
    if *questionaire.index.lock().unwrap() > questionaire.questions.len() - 1 {
        *questionaire.index.lock().unwrap() = 0;
    }
    questionaire.questions[*questionaire.index.lock().unwrap()].to_owned()
}

fn main() {
    tauri::Builder::default()
        .manage(Questionaire::new())
        .invoke_handler(tauri::generate_handler![answer, next_question])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
