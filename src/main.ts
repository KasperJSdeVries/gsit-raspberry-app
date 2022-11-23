import {invoke} from "@tauri-apps/api/tauri";

let questionEl: HTMLParagraphElement | null;

async function give_answer(answer: boolean) {
    await invoke("answer", {
        value: answer,
    });
    await update_question();

}

async function update_question() {
    if (questionEl) {
        questionEl.textContent = await invoke("next_question");
    }
}

window.addEventListener("DOMContentLoaded", async () => {
    questionEl = document.querySelector("#question-paragraph");
    await update_question();
    document
        .querySelector("#yes-button")
        ?.addEventListener("click", async () => {
            await give_answer(true);
        });
    document.querySelector("#no-button")?.addEventListener("click", async () => {
        await give_answer(false);
    });
});
