import { createApp } from 'vue'
import App from './App.vue'
import Emitter from 'tiny-emitter';
import store from './store';
import router from './router';

const app = createApp(App);
app.config.globalProperties.$msalInstance = {};
app.config.globalProperties.$emitter = new Emitter();

app.use(store)
    .use(router)
    .mount('#app');
