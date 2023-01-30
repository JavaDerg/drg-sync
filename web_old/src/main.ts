import 'virtual:windi.css';
import './app.pcss';
import App from './App.svelte';
import { init } from "./dark.ts";


init();

const app = new App({
    target: document.getElementById('app'),
});

export default app;
