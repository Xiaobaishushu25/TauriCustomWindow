<script setup lang="ts">

</script>

<template>
  <div class="container">
    <div class="header">
      <span> 通知中心</span>
    </div>
    <div class="content">
      <div class="item">
        <img src="../assets/tauri.svg"  alt="QQ" />
        <span>QQ活动</span>
        <span class="time">1天前</span>
      </div>
      <div class="item">
        <img src="../assets/vue.svg" alt="QQ" />
        <span>会员积分</span>
        <span class="time">2天前</span>
      </div>
      <div class="item">
        <img src="../assets/tauri.svg" alt="QQ" />
        <span>会员成长</span>
        <span class="time">1天前</span>
      </div>
      <div class="item">
        <img src="../assets/tauri.svg" alt="QQ" />
        <span>会员成长</span>
        <span class="time">1天前</span>
      </div>
      <div class="item">
        <button>查看所有通知</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  width: 145px;
  font-family: Arial, sans-serif;
}

.header {
  background-color: #f5f5f5;
  padding: 10px;
  text-align: center;
}

.content {
  background-color: #fff;
  padding: 10px;
  border: 1px solid #ddd;
}

.item {
  display: flex;
  align-items: center;
  font-size: 12px;
  margin-bottom: 5px;
}

.item img {
  width: 25px;
  height: 25px;
  margin-left: 5px;
}

.item span {
  margin-left: 5px;
}

.time {
  color: #999;
  font-size: 12px;
}

button {
  background-color: #007aff;
  color: #fff;
  border: none;
  padding: 10px 30px;
  border-radius: 5px;
  cursor: pointer;
  margin-top: 10px;
}
</style>