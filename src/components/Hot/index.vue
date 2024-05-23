<template>
  <div class="wallpaper-browser">
    <el-row>
      <el-col :span="5">
        <ul v-infinite-scroll="load" class="infinite-list" :style="{overflow: 'auto', height: sc_height}"
            :infinite-scroll-immediate="immediate">
          <li v-for="fit in fits" :key="fit.name" class="thumbnail-container">
            <el-image
                class="thumbnail-image"
                :src="fit.url"
                @click="selectImage(fit)"
            />
          </li>
        </ul>
      </el-col>
      <el-col :span="19">
        <div class="full-image-container">
          <el-image :src="full_url" class="full-image"/>
        </div>
      </el-col>
    </el-row>
    <el-tooltip content="下载到本地" placement="top">

      <el-backtop :bottom="400" :right="0" :visibility-height="0" class="float-button">
        <template #default>
          <el-icon>
            <el-button :icon="Download" circle @click="download_image"/>
          </el-icon>
        </template>
      </el-backtop>
    </el-tooltip>
    <!--    <div class="wallpaper-page">-->
    <!--      <el-button type="primary" @click="invokeAPI('get_top_wallpapers', 1)">首页</el-button>-->
    <!--    </div>-->
  </div>
  <!--  <el-button type="primary" style="margin-left: 16px" @click="drawer2 = true">-->
  <!--    选项-->
  <!--  </el-button>-->

  <el-drawer
      v-model="drawer"
      title="I am the title"
      :direction="direction"
      :before-close="handleClose"
  >
    <span>Hi, there!</span>
  </el-drawer>
  <el-drawer v-model="drawer2" :direction="direction">
    <template #header>
      <h4>set title by slot</h4>
    </template>
    <template #default>
      <div>
        <el-radio v-model="radio1" value="Option 1" size="large">
          Option 1
        </el-radio>
        <el-radio v-model="radio1" value="Option 2" size="large">
          Option 2
        </el-radio>
      </div>
    </template>
    <template #footer>
      <div style="flex: auto">
        <el-button @click="cancelClick">cancel</el-button>
        <el-button type="primary" @click="confirmClick">confirm</el-button>
      </div>
    </template>
  </el-drawer>
</template>

<script lang="ts" setup>
import {invoke} from "@tauri-apps/api";
import {ref, onMounted, reactive} from "vue";
import {Download} from '@element-plus/icons-vue'
import {sendNotification} from "@tauri-apps/api/notification";
import {ElMessageBox} from 'element-plus'
import type {DrawerProps} from 'element-plus'

const fits = reactive<any[]>([]);
const full_url = ref('');
const full_name = ref('');
const sc_height = ref((window.innerHeight - 200) + 'px');
const drawer = ref(false)
const drawer2 = ref(false)
const direction = ref<DrawerProps['direction']>('btt')
const radio1 = ref('Option 1')
const count = ref(1)
const immediate = ref(false)
const load = () => {
  invokeAPI('get_top_wallpapers', count.value)

}

function invokeAPI(apiMethod: string, newPage: number) {
  // console.log(new Error("invokeAPI called").stack);
  console.log('invokeAPI', newPage);
  invoke(apiMethod, {
    params: {
      categories: 111,
      purity: 110,
      topRange: '',
      sorting: "hot",
      order: "asc",
      ai_art_filter: 1,
      page: newPage
    }
  }).then((res: any) => {
    console.log(res);
    res.forEach((fit: any) => {
      fits.push(fit);
    });
    full_url.value = fits[0].full_url; // 默认选中第一张图片
    immediate.value = true
    count.value += 1
  });
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


const handleClose = (done: () => void) => {
  ElMessageBox.confirm('Are you sure you want to close this?')
      .then(() => {
        done()
      })
      .catch(() => {
        // catch error
      })
}

function cancelClick() {
  drawer2.value = false
}

function confirmClick() {
  ElMessageBox.confirm(`Are you confirm to chose ${radio1.value} ?`)
      .then(() => {
        drawer2.value = false
      })
      .catch(() => {
        // catch error
      })
}

onMounted(() => {
  invokeAPI('get_top_wallpapers', 1);
});

</script>

<style lang="scss" scoped>
.wallpaper-browser {
  .infinite-list {
    padding: 0;
    margin: 0;
    list-style: none;
  }

  .drawer-button {
    position: fixed;
    top: 20px;
    right: 10px;
  }

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

  .wallpaper-page {
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

  .config-button {
    position: fixed;
    bottom: 100px;
    right: 10px;
  }
}
</style>