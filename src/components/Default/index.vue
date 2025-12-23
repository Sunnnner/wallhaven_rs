<template>
  <div class="wallpaper-browser">
    <!-- 左侧缩略图列表 -->
    <div class="left" :style="{ width: sidebarWidth + 'px' }">
      <div class="sidebar-header">
        <span class="title">壁纸列表 ({{ fits.length }})</span>
        <el-button size="small" @click="clearSelection" v-if="selectedImages.size > 0">
          清除选择 ({{ selectedImages.size }})
        </el-button>
      </div>
      <ul v-infinite-scroll="load" class="infinite-list" :infinite-scroll-immediate="immediate">
        <li v-for="fit in fits" :key="fit.name" class="thumbnail-container"
            :class="{ 'selected': selectedImages.has(fit.name), 'active': full_name === fit.name }">
          <div class="thumbnail-wrapper" @click="selectImage(fit)">
            <el-image
                class="thumbnail-image"
                :src="fit.url"
                fit="cover"
                lazy
            >
              <template #error>
                <div class="image-error">
                  <el-icon><Picture /></el-icon>
                </div>
              </template>
            </el-image>
            <div class="thumbnail-overlay">
              <el-checkbox 
                :model-value="selectedImages.has(fit.name)"
                @click.stop="toggleSelection(fit)"
                class="select-checkbox"
              />
              <div class="thumbnail-info">
                <span class="resolution">{{ fit.resolution }}</span>
              </div>
            </div>
          </div>
        </li>
      </ul>
      <!-- 侧边栏拖动调整宽度 -->
      <div class="resize-handle" @mousedown="startResize"></div>
    </div>

    <!-- 右侧大图展示区 -->
    <div class="right-content">
      <!-- 空状态 -->
      <div v-if="!full_url" class="empty-state">
        <el-empty :image-size="200" description="选择左侧图片以查看详情" />
      </div>

      <!-- 图片展示 -->
      <div v-else class="image-viewer">
        <!-- 顶部工具栏 -->
        <div class="toolbar">
          <div class="image-info">
            <el-tag type="info" size="small">{{ full_name }}</el-tag>
            <el-tag type="success" size="small" v-if="currentImage">
              {{ currentImage.resolution }}
            </el-tag>
          </div>
          <div class="toolbar-actions">
            <el-button-group>
              <el-tooltip content="下载当前图片" placement="bottom">
                <el-button 
                  :icon="Download" 
                  @click="download_image"
                  :loading="downloading"
                >
                  下载
                </el-button>
              </el-tooltip>
              <el-tooltip content="批量下载选中图片" placement="bottom" v-if="selectedImages.size > 0">
                <el-button 
                  :icon="Download" 
                  @click="downloadSelected"
                  type="primary"
                >
                  下载选中 ({{ selectedImages.size }})
                </el-button>
              </el-tooltip>
              <el-tooltip content="在浏览器中打开" placement="bottom">
                <el-button :icon="Link" @click="openInBrowser">
                  打开链接
                </el-button>
              </el-tooltip>
              <el-tooltip content="复制图片链接" placement="bottom">
                <el-button :icon="CopyDocument" @click="copyLink">
                  复制链接
                </el-button>
              </el-tooltip>
            </el-button-group>
          </div>
        </div>

        <!-- 图片显示区域 -->
        <div class="full-image-container">
          <el-image
              :src="full_url"
              alt=""
              class="full-image"
              :fit="imageFit"
              v-loading="loading"
              @load="loading = false"
              :preview-src-list="[full_url]"
              :initial-index="0"
          />
        </div>

        <!-- 底部控制栏 -->
        <div class="bottom-controls">
          <div class="fit-controls">
            <el-radio-group v-model="imageFit" size="small">
              <el-radio-button label="contain">适应</el-radio-button>
              <el-radio-button label="cover">填充</el-radio-button>
              <el-radio-button label="fill">拉伸</el-radio-button>
              <el-radio-button label="scale-down">缩小</el-radio-button>
            </el-radio-group>
          </div>
          <div class="navigation">
            <el-button 
              size="small" 
              :icon="ArrowLeft" 
              @click="previousImage"
              :disabled="currentIndex <= 0"
            >
              上一张
            </el-button>
            <span class="nav-info">{{ currentIndex + 1 }} / {{ fits.length }}</span>
            <el-button 
              size="small" 
              @click="nextImage"
              :disabled="currentIndex >= fits.length - 1"
            >
              下一张
              <el-icon class="el-icon--right"><ArrowRight /></el-icon>
            </el-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {invoke} from "@tauri-apps/api";
import {ref, onMounted, reactive, toRefs, PropType, computed} from "vue";
import {Download, Link, CopyDocument, ArrowLeft, ArrowRight, Picture} from '@element-plus/icons-vue'
import {ElMessage} from 'element-plus'
import {open} from '@tauri-apps/api/shell';

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
const downloading = ref(false)
const imageFit = ref<'contain' | 'cover' | 'fill' | 'scale-down'>('contain')
const selectedImages = reactive(new Set<string>())
const sidebarWidth = ref(320)
const isResizing = ref(false)

// 当前选中的图片对象
const currentImage = computed(() => {
  return fits.find(fit => fit.name === full_name.value)
})

// 当前图片索引
const currentIndex = computed(() => {
  return fits.findIndex(fit => fit.name === full_name.value)
})

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
  if (!full_url.value || !full_name.value) {
    ElMessage.warning('请先选择要下载的图片')
    return
  }
  
  downloading.value = true
  try {
    await invoke('download_wallpaper', {url: full_url.value, file_name: full_name.value})
    ElMessage.success('图片下载成功！')
  } catch (error) {
    ElMessage.error('图片下载失败：' + error)
  } finally {
    downloading.value = false
  }
}

// 批量下载选中的图片
async function downloadSelected() {
  if (selectedImages.size === 0) {
    ElMessage.warning('请先选择要下载的图片')
    return
  }

  const selectedFits = fits.filter(fit => selectedImages.has(fit.name))
  let successCount = 0
  let failCount = 0

  ElMessage.info(`开始下载 ${selectedFits.length} 张图片...`)

  for (const fit of selectedFits) {
    try {
      await invoke('download_wallpaper', {url: fit.full_url, file_name: fit.name})
      successCount++
    } catch (error) {
      failCount++
      console.error(`下载失败: ${fit.name}`, error)
    }
  }

  ElMessage.success(`下载完成！成功: ${successCount} 张, 失败: ${failCount} 张`)
  clearSelection()
}

// 在浏览器中打开
async function openInBrowser() {
  if (full_url.value) {
    await open(full_url.value)
  }
}

// 复制链接
function copyLink() {
  if (full_url.value) {
    navigator.clipboard.writeText(full_url.value).then(() => {
      ElMessage.success('链接已复制到剪贴板')
    }).catch(() => {
      ElMessage.error('复制失败')
    })
  }
}

// 切换选择状态
function toggleSelection(fit: any) {
  if (selectedImages.has(fit.name)) {
    selectedImages.delete(fit.name)
  } else {
    selectedImages.add(fit.name)
  }
}

// 清除所有选择
function clearSelection() {
  selectedImages.clear()
}

// 上一张图片
function previousImage() {
  if (currentIndex.value > 0) {
    selectImage(fits[currentIndex.value - 1])
  }
}

// 下一张图片
function nextImage() {
  if (currentIndex.value < fits.length - 1) {
    selectImage(fits[currentIndex.value + 1])
  }
}

// 侧边栏宽度调整
function startResize(e: MouseEvent) {
  isResizing.value = true
  const startX = e.clientX
  const startWidth = sidebarWidth.value

  const doDrag = (e: MouseEvent) => {
    if (!isResizing.value) return
    const newWidth = startWidth + (e.clientX - startX)
    if (newWidth >= 200 && newWidth <= 600) {
      sidebarWidth.value = newWidth
    }
  }

  const stopDrag = () => {
    isResizing.value = false
    document.removeEventListener('mousemove', doDrag)
    document.removeEventListener('mouseup', stopDrag)
  }

  document.addEventListener('mousemove', doDrag)
  document.addEventListener('mouseup', stopDrag)
}

onMounted(() => {
  invokeAPI('get_top_wallpapers', 1);
});

</script>

<style lang="scss">
@use '../Style/theme.scss';

</style>