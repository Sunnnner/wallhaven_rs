<template>
  <div class="wallpaper-browser">
    <div class="left">
      <ul v-infinite-scroll="load" class="infinite-list" :infinite-scroll-immediate="immediate">
        <li v-for="fit in fits" :key="fit.name" class="thumbnail-container">
          <el-image
              class="thumbnail-image"
              :src="fit.url"
              fit="contain"
              @click="selectImage(fit)"
          />
        </li>
      </ul>
    </div>
    <div class="full-image-container">
      <el-empty :image-size="200" description="暂无数据" v-if="!full_url"/>
      <el-image
          :src="full_url"
          alt=""
          class="full-image"
          fit="contain"
          v-if="full_url"
          v-loading="loading"
          @load="loading = false"
      />
    </div>
    <el-tooltip content="下载到本地" placement="top">
      <el-backtop :bottom="400" :right="0" :visibility-height="0" class="float-button">
        <template #default>
          <el-icon>
            <el-button :icon="Download" circle @click="download_image"/>
          </el-icon>
        </template>
      </el-backtop>
    </el-tooltip>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import {ref, onMounted, reactive, toRefs, PropType} from "vue";
import {Download} from '@element-plus/icons-vue'
import {sendNotification} from "@tauri-apps/plugin-notification";

interface infoRef {
  categories: number,
  purity: number,
  topRange: string,
  sorting: string,
  order: string,
  ai_art_filter: number,
  page: number
}


const props = defineProps({
  info: {
    type: Object as PropType<infoRef>,
    required: true
  }
})

const info = toRefs(props.info);
const fits = reactive<any[]>([]);
const full_url = ref('');
const full_name = ref('');
const count = ref(1)
const immediate = ref(false)
const loading = ref(true)

const load = () => {
  invokeAPI('get_top_wallpapers', count.value)
}

function invokeAPI(apiMethod: string, newPage: number) {
  invoke(apiMethod, {
    params: {
      categories: info.categories.value,
      purity: info.purity.value,
      topRange: info.topRange.value,
      sorting: info.sorting.value,
      order: info.order.value,
      ai_art_filter: info.ai_art_filter.value,
      page: newPage
    }
  }).then((res: any) => {
    // console.log(res);
    res.forEach((fit: any) => {
      fits.push(fit);
    });
    immediate.value = true
    count.value += 1
  });
}

function selectImage(fit: any) {
  full_url.value = fit.full_url;
  full_name.value = fit.name;
  loading.value = true
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

onMounted(() => {
  invokeAPI('get_top_wallpapers', 1);
});

</script>

<style lang="scss">
@import '../Style/theme.scss';

</style>