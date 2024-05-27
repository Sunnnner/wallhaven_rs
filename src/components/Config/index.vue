<template>
  <el-form :model="form" label-width="auto" style="max-width: 600px; background: #f9f9f9; padding: 20px; border-radius: 8px; box-shadow: 0 4px 16px rgba(0,0,0,0.1);">
    <el-form-item label="保存路径">
      <el-input v-model="form.name" :placeholder="default_path" @click="open_path" class="input-with-transition"/>
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="onSubmit" class="button-with-transition">保存</el-button>
    </el-form-item>
  </el-form>
</template>

<script lang="ts" setup>
import {onMounted, reactive, ref} from 'vue'
import {dialog, invoke} from "@tauri-apps/api";
import { isPermissionGranted, sendNotification } from '@tauri-apps/api/notification';

const default_path = ref('');

const form = reactive({
  name: '',
  region: '',
  date1: '',
  date2: '',
  delivery: false,
  type: [],
  resource: '',
  desc: '',
})

async function open_path() {
  const result: any = await dialog.open({
    directory: true,
    defaultPath: '.'
  });
  if (result) {
    form.name = result;
  }
}

const onSubmit = () => {
  invoke('save_config', {path: form.name}).then(() => {
    sendNotification('保存成功!');
  });
}

async function get_default_path() {
  const ret: any = await invoke('load_config');
  default_path.value = ret.download_path;
}

async function get_permission() {
  await isPermissionGranted();
}

onMounted(async () => {
  await get_default_path();
  await get_permission();
})

</script>

<style scoped lang="scss">
.input-with-transition, .button-with-transition {
  transition: all 0.3s ease-in-out;
}

.input-with-transition:focus, .button-with-transition:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(0,0,0,0.2);
}
</style>