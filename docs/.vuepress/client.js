import { defineClientConfig } from 'vuepress/client'
import RandomizerForm from './components//RandomizerForm.vue'


export default defineClientConfig({
    enhance({ app }) {
        app.component('RandomizerForm', RandomizerForm);
    },
});