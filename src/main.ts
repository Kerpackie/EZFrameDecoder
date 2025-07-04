import { createApp } from "vue";
import App from "./App.vue";
import naive from "naive-ui";
import './assets/scrollbar.css';
import { router } from './router';
import { createPinia } from "pinia";

createApp(App)
    .use(router)
    .use(naive)
    .use(createPinia())
    .mount("#app");
