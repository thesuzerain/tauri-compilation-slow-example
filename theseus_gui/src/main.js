import { createApp } from 'vue'
import App from '@/App.vue'
import { createPinia } from 'pinia'
import FloatingVue from 'floating-vue'

const pinia = createPinia()

let app = createApp(App)
app.use(pinia)
app.use(FloatingVue)

const mountedApp = app.mount('#app')

mountedApp.initialize()