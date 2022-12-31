import "./style.css";
import App from "./App.svelte";
import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
import './app.postcss';

const app = new App({
  target: document.getElementById("app"),
});

export default app;
