import { createRouter, createWebHashHistory } from 'vue-router';
import DecoderPage from '../pages/DecoderPage.vue';
import AboutPage from '../pages/AboutPage.vue';
import AddCommandPage from '../pages/AddCommandPage.vue';

const routes = [
    { path: '/', component: DecoderPage, name: 'Decoder' },
    { path: '/about', component: AboutPage, name: 'About' },
    { path: '/add-command', component: AddCommandPage, name: 'AddCommand' }
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
