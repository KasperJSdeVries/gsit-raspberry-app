//#![cfg_attr(
//    all(not(debug_assertions), target_os = "windows"),
//    windows_subsystem = "windows"
//)]

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use tauri::Window;

const LEDS: &[u8] = &[17, 27, 22];
const MOTORS: &[u8] = &[23, 24, 25];

struct DrawerControls {
    leds: Vec<OutputPin>,
    motors: Vec<OutputPin>,
}

impl DrawerControls {
    fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        let mut leds = Vec::new();
        let mut motors = Vec::new();

        for &led in LEDS {
            let mut led = gpio.get(led).unwrap().into_output();
            led.set_low();
            leds.push(led);
        }
        for &motor in MOTORS {
            let mut motor = gpio.get(motor).unwrap().into_output();
            motor.set_low();
            motors.push(motor);
        }

        Self { leds, motors }
    }

    fn open<'a>(&'a mut self, idx: usize) {
        let led = &mut self.leds[idx % 3];
        let motor = &mut self.motors[idx % 3];

        led.set_high();
        motor.set_high();
        thread::sleep(Duration::from_secs(1));
        motor.set_low();
        thread::sleep(Duration::from_secs(5));
        led.set_low();
    }
}

const QUESTIONS: &[&str] = &[
    "Is your device from the Apple brand?",
    "Do you have the charger for your device?",
];

struct Questionaire {
    questions: Vec<&'static str>,
    answers: Mutex<Vec<bool>>,
    index: Mutex<usize>,
    drawers: Mutex<DrawerControls>,
    drawer_idx: Mutex<usize>,
}

impl Questionaire {
    fn new() -> Self {
        Questionaire {
            questions: QUESTIONS.to_vec(),
            answers: Mutex::new(Vec::with_capacity(QUESTIONS.len())),
            index: Mutex::new(QUESTIONS.len()),
            drawers: Mutex::new(DrawerControls::new()),
            drawer_idx: Mutex::new(0),
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

#[derive(serde::Serialize, Clone)]
struct FinishedQuestionairePayload {
    drawer_idx: usize,
}

#[tauri::command]
// get next question from DB? / maybe hard-coded?
fn next_question(questionaire: tauri::State<'_, Questionaire>, window: Window) -> String {
    *questionaire.index.lock().unwrap() += 1;
    if *questionaire.index.lock().unwrap() > questionaire.questions.len() - 1 {
        window
            .emit(
                "finished-questionaire",
                FinishedQuestionairePayload {
                    drawer_idx: *questionaire.drawer_idx.lock().unwrap(),
                },
            )
            .unwrap();
        questionaire
            .drawers
            .lock()
            .unwrap()
            .open(*questionaire.drawer_idx.lock().unwrap() % 3);
        *questionaire.drawer_idx.lock().unwrap() += 1;
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
