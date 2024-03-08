import { defineClientConfig } from 'vuepress/client'
import { useRouter } from 'vue-router'
import { onMounted } from 'vue';
import { theme } from 'ant-design-vue';
import Antd from 'ant-design-vue';
import Confirmation from './components/Confirmation.vue'
import FileSelect from './components/FileSelect.vue'
import GameTypeImage from './components/GameTypeImage.vue'
import RandomizerForm from './components/RandomizerForm.vue'

function runRenderUpdate() {
    // Update API for decorative things after update.
    new Promise((resolve) => void setTimeout(resolve, 450))
        .then(() => {
            requestAnimationFrame(() => {
                document.querySelectorAll('h1,h2,h3,h4,h5,h6')
                    .forEach(updateSheikaElement);
            });
        });
}

function updateSheikaElement(el) {
    if (el.classList.contains('splitting')) {
        return;
    }
    el.classList.add('sheika-text');
    // Guard against non-presence in case the CDN goes down.
    window.Splitting && window.Splitting({ target: el, by: 'chars' });
}

export default defineClientConfig({
    enhance({ app }) {
        theme.colorPrimary = "black";

        app.use(Antd);
        app.component('Confirmation', Confirmation);
        app.component('FileSelect', FileSelect);
        app.component('GameTypeImage', GameTypeImage);
        app.component('RandomizerForm', RandomizerForm);
    },
    setup() {
        const router = useRouter();
        router.afterEach(runRenderUpdate);
        onMounted(runRenderUpdate);
    },
});