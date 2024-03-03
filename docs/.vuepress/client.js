import { defineClientConfig } from 'vuepress/client'
import RandomizerForm from './components//RandomizerForm.vue'
import Antd from 'ant-design-vue';


export default defineClientConfig({
    enhance({ app }) {
        app.use(Antd);
        app.component('RandomizerForm', RandomizerForm);
    },
});