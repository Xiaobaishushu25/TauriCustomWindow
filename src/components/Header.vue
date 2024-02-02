<script setup>
import {ref, watch} from "vue";
import {appWindow, UserAttentionType} from "@tauri-apps/api/window";
const max_state_name = ref('window-maximize')
const max_state= ref(false)
watch(max_state, async (newValue) => {
  if(newValue){ //当前状态是最大化
    max_state_name.value = 'window-restore'
    await appWindow.maximize()
  }else{
    max_state_name.value = 'window-maximize'
    await appWindow.unmaximize()
  }
})

async function window_minimize(){
  await appWindow.minimize()
  // setTimeout(async ()=>{
  //   await appWindow.requestUserAttention(UserAttentionType.Critical)
  //   console.log("请求注意")
  // },5000)
}
function window_maximize(){
  max_state.value =!max_state.value
}
async function window_close(){
  await appWindow.close()
}
</script>

<template>
  <div  data-tauri-drag-region class="titlebar"  >
    <div id="stage-button">
<!--      <unicon class ="top" viewBox="0 0 1024 1024" name="window-on-top" />-->
      <unicon class ="min" viewBox="0 0 1024 1024" name="window-minimize" @click.left="window_minimize" />
      <unicon class ="max" viewBox="0 0 1024 1024" @click.left="window_maximize" :name=max_state_name />
      <unicon class ="close" name="multiply" @click.left="window_close" icon-style="monochrome" />
    </div>
  </div>
</template>

<style >
.titlebar{
  display: flex;
  flex-direction:row;
  height: 30px;
  user-select: none;
  /*top: 0px;
  left: 0;
  right: 0;*/
}
#stage-button{
  display: flex;
  flex-direction:row;
  justify-content: flex-end;
  /*在 Flexbox 中，margin-left: auto; 会将元素推到其容器的末尾，而 margin-right: 0; 在 Flexbox 中不会产生相同的效果。*/
  margin-left: auto;
}
.top{
  width:30px;
  height: 30px;
}
.top path{
  stroke:red;
  stroke-width:30;
}
.top:hover{
  cursor: pointer;
}
.top:hover path{
  stroke: #37ff00;
  stroke-width:30;
}
.min, .max, .close{
  /*font-size: 30px;用这个无法设置大小*/
  width: 30px;
  height: 30px;
}

.min path{
  stroke: black;
  width: 24px;
  stroke-width: 1;
}
.max path{
  transform: scale(0.7);
  transform-origin: center;
  stroke: black;
  stroke-width:20;
}
.min:hover,.max:hover{
  background-color: #33303020;
  border-radius: 5px;
}

.close:hover{
  fill: white;
  background-color: red;
  border-radius: 5px;
}
</style>
