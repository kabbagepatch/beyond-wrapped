import { createApp } from "vue";
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from "./App.vue";
import { setTheme } from "./themes.ts";
import Settings from "./Settings.vue";
import ExtendedStats from "./pages/ExtendedStats.vue";
import YearlyStats from "./pages/YearlyStats.vue";
import MonthlyStats from "./pages/MonthlyStats.vue";
import TrackStats from "./pages/TrackStats.vue";
import ArtistStats from "./pages/ArtistStats.vue";
import CustomStats from "./pages/CustomStats.vue";
import AlbumStats from "./pages/AlbumStats.vue";

const routes = [
  { path: '/year/:year', component: YearlyStats },
  { path: '/year/:year/:month', component: MonthlyStats },
  { path: '/tracks/:track', component: TrackStats },
  { path: '/artists/:artist', component: ArtistStats },
  { path: '/albums/:album', component: AlbumStats },
  { path: '/custom', component: CustomStats },
  { path: '/settings', component: Settings },
  { path: '/', component: ExtendedStats },
]

const pinia = createPinia()
const router = createRouter({
  history: createWebHistory(),
  routes,
})
const app = createApp(App)

app.use(pinia);
app.use(router);
app.mount("#app");

let curTheme = localStorage.getItem('theme') || 'Forest';
setTheme(curTheme)
