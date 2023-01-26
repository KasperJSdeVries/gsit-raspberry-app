import {invoke} from "@tauri-apps/api/tauri";

let finishedQuestionaire = false;
let questionEl: HTMLParagraphElement | null;
let leftButton: HTMLButtonElement | null;
let rightButton: HTMLButtonElement | null;

async function give_answer(answer: boolean) {
    if (finishedQuestionaire) {
        if (leftButton) {
            leftButton.textContent = "Yes";
        }
        if (rightButton) {
            rightButton.hidden = false;
        }
        finishedQuestionaire = false;
        return;
    }
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

window.addEventListener("finished-questionaire", () => {
    finishedQuestionaire = true;
    if (questionEl) {
        questionEl.textContent = "Thank you for finishing the questionaire!";
    }
    if (leftButton) {
        leftButton.textContent = "Restart questionaire";
    }
    if (rightButton) {
        rightButton.hidden = true;
    }
})

window.addEventListener("DOMContentLoaded", async () => {
    questionEl = document.querySelector("#question-paragraph");
    leftButton = document.querySelector("#yes-button")
    rightButton = document.querySelector("#no-button")
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
