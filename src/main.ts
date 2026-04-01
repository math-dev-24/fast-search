import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import router from "./route";
import "./assets/main.css";
import { i18n } from "./i18n";
import { useLocaleStore } from "./shared";

const pinia = createPinia();
const app = createApp(App);

app.use(pinia);
const localeStore = useLocaleStore(pinia);
localeStore.init();
app.use(i18n);
app.use(router);
app.mount("#app");
