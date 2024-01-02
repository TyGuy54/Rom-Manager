import { invoke } from "@tauri-apps/api/tauri";

// calls a function from the backend that loads a rom to play it
export const loadRom = async (romName) => {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("load_rom", { romName })
    .then((data) => {
        console.log(`This is from loadRom ${data}`);
    })
    .catch((error) => {
        console.log(error);
    });
}