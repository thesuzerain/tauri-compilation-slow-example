import { createApp } from "vue";
import router from "@/routes";
import App from "@/App.vue";
import { createPinia } from "pinia";
import "omorphia/dist/style.css";
import "@/assets/stylesheets/global.scss";
import "floating-vue/dist/style.css";
import FloatingVue from "floating-vue";
import loadCssMixin from "./mixins/macCssFix.js";

const pinia = createPinia();

let app = createApp(App);
app.use(router);
app.use(pinia);
app.use(FloatingVue);
app.mixin(loadCssMixin);

const mountedApp = app.mount("#app");

mountedApp.initialize();
