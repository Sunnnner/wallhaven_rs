<template>
  <div class="wallpaper-browser">
    <el-row>
      <el-col :span="5">
        <el-scrollbar :height="sc_height">
          <div v-for="fit in fits" :key="fit.name" class="thumbnail-container">
            <el-image
                class="thumbnail-image"
                :src="fit.url"
                @click="selectImage(fit)"
            />
          </div>
        </el-scrollbar>
      </el-col>
      <el-col :span="19">
        <div class="full-image-container">
          <el-image :src="full_url" class="full-image"/>
        </div>
      </el-col>
    </el-row>
    <div class="wallpaper-page">
      <el-button type="primary" @click="invokeAPI('get_top_wallpapers', 1)">首页</el-button>
      <el-button type="primary" @click="invokeAPI('get_top_wallpapers', page+1)">下一页</el-button>
    </div>
    <div>
      <el-button type="success" :icon="Download" circle class="float-button" @click="download_image" />
    </div>
    <div>
      <el-button type="info" :icon="SetUp" circle class="config-button" @click="open_config" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import {invoke} from "@tauri-apps/api";
import {ref, onMounted} from "vue";
import { Download, SetUp } from '@element-plus/icons-vue'
import {WebviewWindow} from "@tauri-apps/api/window";
import {sendNotification} from "@tauri-apps/api/notification";

const fits = ref();
const full_url = ref('');
const full_name = ref('');
const sc_height = ref((window.innerHeight - 50) + 'px');
const page = ref(1);

async function invokeAPI(apiMethod: string, newPage: number) {
  try {
    fits.value = await invoke(apiMethod, {page: newPage});
    page.value = newPage;
    full_url.value = fits.value[0].full_url; // 默认选中第一张图片
    full_name.value = fits.value[0].name;
  } catch (error) {
    console.error('API call failed:', error);
  }
}

function selectImage(fit: any) {
  full_url.value = fit.full_url;
  full_name.value = fit.name;
}
window.onresize = function () {
  sc_height.value = (window.innerHeight - 50) + 'px'
}

async function download_image() {
  invoke('download_wallpaper', {url: full_url.value, file_name: full_name.value}).then(() => {
    sendNotification({
      title: '通知',
      body: '图片已保存',
      icon: 'tauri://icons/128x128.png'
    });
  });
}

const open_config = async () => {
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
  await webview.close();

}

onMounted(() => {
  invokeAPI('get_top_wallpapers', 1);
});

</script>

<style lang="scss">
.wallpaper-browser {
  //background-color: #CCFFFF; // 添加科技感背景色

  .thumbnail-container {
    margin: 10px;
    display: flex;
    justify-content: center;
  }

  .full-image-container {
    display: flex;
    justify-content: center;
    align-items: center; // 垂直居中
    height: 100%; // 确保容器占满父元素的高度
  }

  .thumbnail-image, .full-image {
    cursor: pointer;
    transition: transform 0.3s ease;
    object-fit: contain; // 保持图片的宽高比

    &:hover {
      transform: scale(1.1);
    }
  }
  .wallpaper-page{
    //固定底部中间
    position: fixed;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
  }
  .float-button, .config-button {
    transition: background-color 0.3s, transform 0.3s;
    &:hover {
      background-color: #a4d7e1; // 鼠标悬停时变更背景色
      transform: translateY(-3px); // 轻微升起效果
    }
  }
  .float-button {
    position: fixed;
    bottom: 200px;
    right: 10px;
  }
  .config-button {
    position: fixed;
    bottom: 100px;
    right: 10px;
  }
}
</style>