import { createRouter, createWebHashHistory } from 'vue-router';
import DecoderPage from '../pages/DecoderPage.vue';
import AboutPage from '../pages/AboutPage.vue';
import SpecEditorPage from '../pages/SpecEditorPage.vue'; // Import the new page

const routes = [
    { path: '/', component: DecoderPage, name: 'Decoder' },
    { path: '/about', component: AboutPage, name: 'About' },
    // Replace the old edit/add pages with the new unified spec editor
    { path: '/spec-editor', component: SpecEditorPage, name: 'Spec Editor' },
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
