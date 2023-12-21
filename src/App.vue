<script setup lang="ts">
import {invoke} from '@tauri-apps/api';
import {onMounted, ref} from 'vue'

const activeIndex = ref('1')
const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
onMounted(() => {
  invoke('get_app_path').then(res => {
    console.log(res);
  })
})
</script>

<template>
  <div id="app">
    <el-menu :default-active="activeIndex" class="el-menu-demo" mode="horizontal" :ellipsis="false"
             @select="handleSelect">
      <el-menu-item index="0">
        <img style="width: 100px" src="https://element-plus.gitee.io/images/element-plus-logo.svg" alt="Element logo"/>
      </el-menu-item>
      <div class="flex-grow"/>
      <el-menu-item index="1">操作中心</el-menu-item>
      <h1 class="test"></h1>
      <!--      <el-sub-menu index="2">-->
      <!--        <template #title>Workspace</template>-->
      <!--        <el-menu-item index="2-1">item one</el-menu-item>-->
      <!--        <el-menu-item index="2-2">item two</el-menu-item>-->
      <!--        <el-menu-item index="2-3">item three</el-menu-item>-->
      <!--        <el-sub-menu index="2-4">-->
      <!--          <template #title>item four</template>-->
      <!--          <el-menu-item index="2-4-1">item one</el-menu-item>-->
      <!--          <el-menu-item index="2-4-2">item two</el-menu-item>-->
      <!--          <el-menu-item index="2-4-3">item three</el-menu-item>-->
      <!--        </el-sub-menu>-->
      <!--      </el-sub-menu>-->
    </el-menu>
    <router-view/>
  </div>
</template>

<style scoped>
.flex-grow {
  flex-grow: 1;
}
</style>
