<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog"
import {invoke} from "@tauri-apps/api";
import {useRouter} from "vue-router";
const router = useRouter()
async function newXlsx() {
  let file = await open()
  invoke("hello", {path: file}).then(
      resp => {
        console.log(resp)
      }
  )
}
async function toData() {
  router.push("/data")
}
</script>

<template>
  <div class="main">
    <el-row :gutter="12">
      <el-col :span="8">
        <el-card shadow="hover" @click="newXlsx">录入</el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover" @click="toData">总览</el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover">导出</el-card>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped>
.main {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>