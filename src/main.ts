import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import Unicon from 'vue-unicons'
import { uniCopyAlt, uniCopyLandscape,uniWindowSection, uniWindowMaximize, uniCommentMonochrome,uniMultiplyMonochrome,uniMinusSquareFullMonochrome } from 'vue-unicons/dist/icons.js'
import {icons} from "./icons.js"
icons.push(
    uniCopyAlt,
    uniCopyLandscape,
    uniWindowSection,
    uniWindowMaximize,
    uniCommentMonochrome,
    uniMultiplyMonochrome,
    uniMinusSquareFullMonochrome
);
Unicon.add(icons)

createApp(App).use(Unicon).mount("#app");
