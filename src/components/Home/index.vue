<template>
    <el-container class="container">
      <el-header>
          <el-menu
              :default-active="activeIndex"
              mode="horizontal"
              :router="true"
          >
            <el-menu-item index="/index/latest">主页</el-menu-item>
            <el-menu-item index="/index/top">排行榜</el-menu-item>
            <el-menu-item index="/index/hot">热门</el-menu-item>
            <el-menu-item index="/index/random">随机</el-menu-item>
            <el-menu-item index="/index/tags">标签</el-menu-item>
            <el-menu-item @click="openConfig">
              <el-icon style="margin-right: 8px; margin-top: 1px">
                <setting/>
              </el-icon>
              配置
            </el-menu-item>
            <el-menu-item >
                <!--github 链接以及图标-->
                <a href="https://github.com/Sunnnner/wallhaven_rs" target="_blank">
                  <el-icon style="margin-right: 8px; margin-top: 1px">
                    <el-image src="https://img.icons8.com/ios-glyphs/30/000000/github.png"/>
                  </el-icon>
                </a>
            </el-menu-item>
          </el-menu>
      </el-header>
      <el-main style="overflow: hidden">
          <router-view/>
      </el-main>
    </el-container>
</template>

<script lang="ts" setup>
import {Setting} from '@element-plus/icons-vue'
import { ref } from 'vue'
import {WebviewWindow} from "@tauri-apps/api/window";

const activeIndex = ref("/index/latest")

const openConfig = async () => {
  const webview = new WebviewWindow('theUniqueLabel', {
    url: '/config',
    title: '用户配置',
    width: 300,
    height: 200,
    resizable: false,
  });
  await webview.once('tauri://created', function () {
    console.log('webview created')
  })

}

</script>

<style scoped lang="scss">
  .container {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
</style>
