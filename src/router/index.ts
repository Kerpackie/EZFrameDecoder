import { createRouter, createWebHashHistory } from 'vue-router';
import DecoderPage from '../pages/DecoderPage.vue';
import AboutPage from '../pages/AboutPage.vue';
import SpecEditorPage from '../pages/SpecEditorPage.vue';
import SettingsPage from '../pages/SettingsPage.vue'; // New
import ViewCommandsPage from '../pages/ViewCommandsPage.vue'; // New

const routes = [
    { path: '/', component: DecoderPage, name: 'Decoder' },
    { path: '/about', component: AboutPage, name: 'About' },
    { path: '/spec-editor', component: SpecEditorPage, name: 'Spec Editor' },
    { path: '/settings', component: SettingsPage, name: 'Settings' },
    { path: '/view-commands', component: ViewCommandsPage, name: 'View Commands' },
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
