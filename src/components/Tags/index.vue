<template>
  <div>
    <div>
      <el-link type="primary">最多浏览
        <el-icon class="el-icon--right">
          <icon-view/>
        </el-icon>
      </el-link>
      <el-link type="primary">最多收藏
        <el-icon class="el-icon--right">
          <icon-view/>
        </el-icon>
      </el-link>
      <el-link type="primary">最流行
        <el-icon class="el-icon--right">
          <icon-view/>
        </el-icon>
      </el-link>
      <el-link type="primary">最新
        <el-icon class="el-icon--right">
          <icon-view/>
        </el-icon>
      </el-link>
      <el-link type="primary">最多订阅
        <el-icon class="el-icon--right">
          <icon-view/>
        </el-icon>
      </el-link>
    </div>
    <div>
      <ul v-infinite-scroll="load" class="infinite-list" :infinite-scroll-immediate="immediate">
        <li v-for="i in results" :key="i" class="infinite-list-item">
          <el-link href={{i.url}} target="_blank">{{ i.name }}</el-link>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {View as IconView} from '@element-plus/icons-vue'
import {invoke} from "@tauri-apps/api";
import {ref} from "vue";

const count = ref(1)
const results = ref<any[]>([])
const immediate = ref(false)


const load = () => {
  invokeAPI('get_tags', count.value, 'popular')
}

function invokeAPI(apiMethod: string, newPage: number, categories: string) {
  invoke(apiMethod, {
    params: {
      page: newPage,
      categories: categories
    }
  }).then((res: any) => {
    // console.log(res);
    results.value = [...results.value, ...res];
    immediate.value = true
    count.value += 1
  });
}


</script>

<style lang="scss" scoped>
@import '../Style/theme.scss';

.el-link {
  margin-right: 30px;
}

</style>
