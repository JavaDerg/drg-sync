import { defineConfig } from 'windicss/helpers';
import aspectPlugin from 'windicss/plugin/aspect-ratio';

export default defineConfig({
    darkMode: 'class',
    plugins: [
        aspectPlugin,
    ],
});
