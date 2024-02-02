<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {computed, onMounted, ref} from "vue";
import Home from "./components/Home.vue";
import TrayMenu from "./components/TrayMenu.vue";
onMounted(async ()=> {
  // await appWindow.setAlwaysOnTop(true)
  window.addEventListener("contextmenu",  (e) => {e.preventDefault()},false)
})
const routes = {
  '/': Home,
  '/trayMenu': TrayMenu
}
const currentPath = ref(window.location.hash)
window.addEventListener('hashchange', () => {
  currentPath.value = window.location.hash
})
const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || '/']
})
</script>

<template>
<!--  <div class="window" v-if="now_window_size" :style=" {width:(now_window_size.width-25)+ 'px',height:(now_window_size.height-10)  + 'px'}">-->
<!--  <div class="window">-->
<!--  </div>-->
<!--  <a href="#/">Home</a> |-->
<!--  <a href="#/trayMenu">About</a> |-->
  <component :is="currentView" />
</template>

<style scoped>

.window {
  background-color: #b4b4e7;
}


</style>
