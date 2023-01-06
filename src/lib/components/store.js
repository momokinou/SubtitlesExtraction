// @ts-ignore
import { writable } from "svelte/store";

const storedFolder = localStorage.getItem("folder");
export const folder = writable(storedFolder);
folder.subscribe((value) => {
  localStorage.setItem("folder", value);
});
