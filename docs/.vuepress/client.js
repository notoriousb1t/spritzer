import { defineClientConfig } from 'vuepress/client'
import Confirmation from './components/Confirmation.vue'
import FileSelect from './components//FileSelect.vue'
import GameTypeImage from './components/GameTypeImage.vue'
import RandomizerForm from './components//RandomizerForm.vue'
import Antd from 'ant-design-vue';


export default defineClientConfig({
    enhance({ app }) {
        app.use(Antd);
        app.component('Confirmation', Confirmation);
        app.component('FileSelect', FileSelect);
        app.component('GameTypeImage', GameTypeImage);
        app.component('RandomizerForm', RandomizerForm);
    },
});