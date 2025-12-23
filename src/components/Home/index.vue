<template>
    <el-container class="container">
      <el-header class="app-header">
        <div class="header-content">
          <!-- 左侧：Logo 和应用名称 -->
          <div class="brand-section">
            <div class="logo">
              <el-icon :size="24">
                <Picture />
              </el-icon>
            </div>
            <div class="brand-info">
              <h1 class="app-title">Wallhaven</h1>
              <span class="app-subtitle">壁纸管理器</span>
            </div>
          </div>

          <!-- 中间：导航菜单 -->
          <el-menu
              :default-active="activeIndex"
              mode="horizontal"
              :router="true"
              class="nav-menu"
              @select="handleSelect"
          >
            <el-menu-item index="/index/latest">
              <el-icon><HomeFilled /></el-icon>
              <span>最新</span>
            </el-menu-item>
            <el-menu-item index="/index/top">
              <el-icon><TrendCharts /></el-icon>
              <span>排行榜</span>
            </el-menu-item>
            <el-menu-item index="/index/hot">
              <el-icon><Histogram /></el-icon>
              <span>热门</span>
            </el-menu-item>
            <el-menu-item index="/index/random">
              <el-icon><MagicStick /></el-icon>
              <span>随机</span>
            </el-menu-item>
          </el-menu>

          <!-- 右侧：搜索和操作按钮 -->
          <div class="action-section">
            <!-- 搜索框 -->
            <el-input
                v-model="searchQuery"
                placeholder="搜索壁纸..."
                :prefix-icon="Search"
                class="search-input"
                clearable
                @keyup.enter="handleSearch"
            />

            <!-- 操作按钮组 -->
            <div class="action-buttons">
              <el-tooltip content="刷新" placement="bottom">
                <el-button circle :icon="Refresh" @click="handleRefresh" />
              </el-tooltip>

              <el-tooltip content="设置" placement="bottom">
                <el-button circle :icon="Setting" @click="openConfig" />
              </el-tooltip>

              <el-tooltip content="GitHub" placement="bottom">
                <el-button 
                  circle 
                  @click="openGithub"
                >
                  <el-icon><svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                    <path d="M511.6 76.3C264.3 76.2 64 276.4 64 523.5 64 718.9 189.3 885 363.8 946c23.5 5.9 19.9-10.8 19.9-22.2v-77.5c-135.7 15.9-141.2-73.9-150.3-88.9C215 726 171.5 718 184.5 703c30.9-15.9 62.4 4 98.9 57.9 26.4 39.1 77.9 32.5 104 26 5.7-23.5 17.9-44.5 34.7-60.8-140.6-25.2-199.2-111-199.2-213 0-49.5 16.3-95 48.3-131.7-20.4-60.5 1.9-112.3 4.9-120 58.1-5.2 118.5 41.6 123.2 45.3 33-8.9 70.7-13.6 112.9-13.6 42.4 0 80.2 4.9 113.5 13.9 11.3-8.6 67.3-48.8 121.3-43.9 2.9 7.7 24.7 58.3 5.5 118 32.4 36.8 48.9 82.7 48.9 132.3 0 102.2-59 188.1-200 212.9a127.5 127.5 0 0 1 38.1 91v112.5c.8 9 0 17.9 15 17.9 177.1-59.7 304.6-227 304.6-424.1 0-247.2-200.4-447.3-447.5-447.3z"/>
                  </svg></el-icon>
                </el-button>
              </el-tooltip>
            </div>
          </div>
        </div>
      </el-header>
      <el-main style="overflow: hidden">
          <router-view/>
      </el-main>
    </el-container>
</template>

<script lang="ts" setup>
import {
  Setting, 
  Search, 
  Refresh, 
  HomeFilled, 
  TrendCharts, 
  Histogram, 
  MagicStick,
  Picture
} from '@element-plus/icons-vue'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { WebviewWindow } from "@tauri-apps/api/window"
import { open } from '@tauri-apps/api/shell'
import { ElMessage } from 'element-plus'

const router = useRouter()
const activeIndex = ref("/index/latest")
const searchQuery = ref('')

const handleSelect = (index: string) => {
  activeIndex.value = index
}

const handleSearch = () => {
  if (searchQuery.value.trim()) {
    ElMessage.info(`搜索功能开发中: ${searchQuery.value}`)
    // TODO: 实现搜索功能
  }
}

const handleRefresh = () => {
  // 刷新当前页面
  router.go(0)
  ElMessage.success('页面已刷新')
}

const openConfig = async () => {
  const webview = new WebviewWindow('theUniqueLabel', {
    url: '/config',
    title: '应用设置',
    width: 800,
    height: 600,
    resizable: true,
    center: true,
  });
  await webview.once('tauri://created', function () {
    console.log('webview created')
  })
}

const openGithub = async () => {
  await open('https://github.com/Sunnnner/wallhaven_rs')
}

</script>

<style scoped lang="scss">
  .container {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .app-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
    padding: 0;
    height: 70px !important;
    position: relative;
    z-index: 100;

    .header-content {
      height: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 24px;
      gap: 24px;
    }

    // 品牌区域
    .brand-section {
      display: flex;
      align-items: center;
      gap: 12px;
      min-width: 200px;

      .logo {
        width: 40px;
        height: 40px;
        background: rgba(255, 255, 255, 0.2);
        border-radius: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #fff;
        backdrop-filter: blur(10px);
        transition: all 0.3s;

        &:hover {
          background: rgba(255, 255, 255, 0.3);
          transform: scale(1.05);
        }
      }

      .brand-info {
        display: flex;
        flex-direction: column;
        gap: 2px;

        .app-title {
          margin: 0;
          font-size: 20px;
          font-weight: 700;
          color: #fff;
          letter-spacing: 0.5px;
          text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }

        .app-subtitle {
          font-size: 12px;
          color: rgba(255, 255, 255, 0.85);
          font-weight: 500;
        }
      }
    }

    // 导航菜单
    .nav-menu {
      flex: 1;
      background: transparent !important;
      border: none !important;
      max-width: 600px;

      :deep(.el-menu-item) {
        color: rgba(255, 255, 255, 0.85);
        border: none !important;
        font-weight: 500;
        font-size: 15px;
        transition: all 0.3s;
        border-radius: 8px;
        margin: 0 4px;

        &:hover {
          background: rgba(255, 255, 255, 0.15) !important;
          color: #fff !important;
        }

        &.is-active {
          background: rgba(255, 255, 255, 0.25) !important;
          color: #fff !important;
          font-weight: 600;
        }

        .el-icon {
          margin-right: 6px;
        }
      }
    }

    // 操作区域
    .action-section {
      display: flex;
      align-items: center;
      gap: 12px;

      .search-input {
        width: 220px;
        
        :deep(.el-input__wrapper) {
          background: rgba(255, 255, 255, 0.2);
          backdrop-filter: blur(10px);
          border: 1px solid rgba(255, 255, 255, 0.3);
          box-shadow: none;
          transition: all 0.3s;

          &:hover, &.is-focus {
            background: rgba(255, 255, 255, 0.3);
            border-color: rgba(255, 255, 255, 0.5);
          }

          .el-input__inner {
            color: #fff;
            
            &::placeholder {
              color: rgba(255, 255, 255, 0.7);
            }
          }

          .el-input__prefix {
            color: rgba(255, 255, 255, 0.8);
          }
        }
      }

      .action-buttons {
        display: flex;
        gap: 8px;

        .el-button {
          background: rgba(255, 255, 255, 0.2);
          border: 1px solid rgba(255, 255, 255, 0.3);
          color: #fff;
          backdrop-filter: blur(10px);
          transition: all 0.3s;

          &:hover {
            background: rgba(255, 255, 255, 0.3);
            border-color: rgba(255, 255, 255, 0.5);
            transform: translateY(-2px);
          }

          &:active {
            transform: translateY(0);
          }
        }
      }
    }
  }

  // 响应式设计
  @media (max-width: 1200px) {
    .app-header {
      .search-input {
        width: 180px;
      }
    }
  }

  @media (max-width: 992px) {
    .app-header {
      .brand-info .app-subtitle {
        display: none;
      }

      .search-input {
        width: 150px;
      }
    }
  }
</style>
