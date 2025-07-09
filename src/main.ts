import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import router from "./route";
import "./assets/main.css";

const pinia = createPinia();

createApp(App).use(pinia).use(router).mount("#app");
