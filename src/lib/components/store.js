// @ts-ignore
import { writable } from "svelte/store";

const storedFolder = localStorage.getItem("folder");
export const folder = writable(storedFolder);
folder.subscribe((value) => {
  localStorage.setItem("folder", value);
});

export let files = writable([]);

const storedOutputFolder = localStorage.getItem("output_folder");
export const output_folder = writable(storedOutputFolder);
output_folder.subscribe((value) => {
  localStorage.setItem("output_folder", value);
});
