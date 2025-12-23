<template>
  <div class="config-container">
    <!-- 顶部标题栏 -->
    <div class="config-header">
      <div class="header-title">
        <el-icon :size="24" color="#409EFF">
          <Setting />
        </el-icon>
        <h2>应用设置</h2>
      </div>
      <el-tag type="info" size="small">v1.0.0</el-tag>
    </div>

    <!-- 设置内容区域 -->
    <div class="config-content">
      <el-tabs v-model="activeTab" class="config-tabs">
        <!-- 基础设置 -->
        <el-tab-pane label="基础设置" name="basic">
          <div class="tab-content">
            <el-form :model="form" label-width="120px" label-position="left">
              <!-- 下载路径设置 -->
              <el-form-item label="下载保存路径">
                <el-input
                    v-model="form.download_path"
                    :placeholder="default_path || '选择保存路径'"
                    readonly
                    class="path-input"
                >
                  <template #append>
                    <el-button :icon="FolderOpened" @click="open_path">
                      选择路径
                    </el-button>
                  </template>
                </el-input>
                <div class="form-hint">
                  <el-icon><InfoFilled /></el-icon>
                  <span>选择壁纸下载后的保存位置</span>
                </div>
              </el-form-item>

              <!-- 文件命名规则 -->
              <el-form-item label="文件命名规则">
                <el-radio-group v-model="form.naming_rule">
                  <el-radio label="original">原始文件名</el-radio>
                  <el-radio label="timestamp">时间戳命名</el-radio>
                  <el-radio label="custom">自定义前缀</el-radio>
                </el-radio-group>
              </el-form-item>

              <el-form-item label="自定义前缀" v-if="form.naming_rule === 'custom'">
                <el-input
                    v-model="form.custom_prefix"
                    placeholder="例如: wallpaper_"
                    clearable
                />
              </el-form-item>

              <!-- 自动打开保存目录 -->
              <el-form-item label="下载完成后">
                <el-switch
                    v-model="form.open_folder_after_download"
                    active-text="自动打开保存目录"
                    inactive-text="不打开"
                />
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>

        <!-- 界面设置 -->
        <el-tab-pane label="界面设置" name="appearance">
          <div class="tab-content">
            <el-form :model="form" label-width="120px" label-position="left">
              <!-- 主题设置 -->
              <el-form-item label="主题模式">
                <el-radio-group v-model="form.theme">
                  <el-radio-button label="light">浅色</el-radio-button>
                  <el-radio-button label="dark">深色</el-radio-button>
                  <el-radio-button label="auto">跟随系统</el-radio-button>
                </el-radio-group>
              </el-form-item>

              <!-- 缩略图大小 -->
              <el-form-item label="缩略图尺寸">
                <el-slider
                    v-model="form.thumbnail_size"
                    :min="150"
                    :max="300"
                    :step="10"
                    show-stops
                    :marks="{ 150: '小', 225: '中', 300: '大' }"
                />
              </el-form-item>

              <!-- 侧边栏宽度 -->
              <el-form-item label="侧边栏宽度">
                <el-slider
                    v-model="form.sidebar_width"
                    :min="250"
                    :max="500"
                    :step="10"
                    show-input
                />
              </el-form-item>

              <!-- 显示图片信息 -->
              <el-form-item label="图片信息显示">
                <el-checkbox-group v-model="form.show_info">
                  <el-checkbox label="resolution">分辨率</el-checkbox>
                  <el-checkbox label="size">文件大小</el-checkbox>
                  <el-checkbox label="views">浏览次数</el-checkbox>
                </el-checkbox-group>
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>

        <!-- 下载设置 -->
        <el-tab-pane label="下载设置" name="download">
          <div class="tab-content">
            <el-form :model="form" label-width="120px" label-position="left">
              <!-- 同时下载数 -->
              <el-form-item label="并发下载数">
                <el-input-number
                    v-model="form.concurrent_downloads"
                    :min="1"
                    :max="10"
                    :step="1"
                />
                <div class="form-hint">
                  <el-icon><InfoFilled /></el-icon>
                  <span>同时下载的最大图片数量</span>
                </div>
              </el-form-item>

              <!-- 自动去重 -->
              <el-form-item label="下载去重">
                <el-switch
                    v-model="form.auto_deduplicate"
                    active-text="自动跳过已存在的文件"
                    inactive-text="允许重复下载"
                />
              </el-form-item>

              <!-- 下载完成通知 -->
              <el-form-item label="下载通知">
                <el-switch
                    v-model="form.download_notification"
                    active-text="显示下载完成通知"
                    inactive-text="不显示通知"
                />
              </el-form-item>
            </el-form>
          </div>
        </el-tab-pane>

        <!-- 关于 -->
        <el-tab-pane label="关于" name="about">
          <div class="tab-content about-section">
            <div class="about-card">
              <el-icon :size="48" color="#409EFF">
                <Picture />
              </el-icon>
              <h3>Wallhaven 壁纸管理器</h3>
              <p class="version">版本 1.0.0</p>
              <p class="description">
                一个基于 Tauri 和 Vue 3 构建的现代化壁纸管理应用，
                支持从 Wallhaven 浏览和下载高质量壁纸。
              </p>
              
              <div class="about-links">
                <el-button type="primary" :icon="Link" @click="openGithub">
                  查看源码
                </el-button>
                <el-button :icon="Document" @click="openDocs">
                  使用文档
                </el-button>
              </div>

              <div class="about-info">
                <el-descriptions :column="1" border size="small">
                  <el-descriptions-item label="框架">Vue 3 + TypeScript</el-descriptions-item>
                  <el-descriptions-item label="UI 库">Element Plus</el-descriptions-item>
                  <el-descriptions-item label="后端">Tauri + Rust</el-descriptions-item>
                  <el-descriptions-item label="开源协议">MIT</el-descriptions-item>
                </el-descriptions>
              </div>
            </div>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>

    <!-- 底部操作栏 -->
    <div class="config-footer">
      <div class="footer-actions">
        <el-button @click="resetToDefault">恢复默认</el-button>
        <div class="right-actions">
          <el-button @click="handleCancel">取消</el-button>
          <el-button type="primary" @click="onSubmit" :loading="saving">
            保存设置
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, reactive, ref} from 'vue'
import {dialog, invoke} from "@tauri-apps/api"
import {ElMessage, ElMessageBox} from 'element-plus'
import {open} from '@tauri-apps/api/shell'
import {
  Setting,
  FolderOpened,
  InfoFilled,
  Picture,
  Link,
  Document
} from '@element-plus/icons-vue'

const default_path = ref('')
const activeTab = ref('basic')
const saving = ref(false)

const form = reactive({
  // 基础设置
  download_path: '',
  naming_rule: 'original' as 'original' | 'timestamp' | 'custom',
  custom_prefix: '',
  open_folder_after_download: false,

  // 界面设置
  theme: 'light' as 'light' | 'dark' | 'auto',
  thumbnail_size: 180,
  sidebar_width: 320,
  show_info: ['resolution'],

  // 下载设置
  concurrent_downloads: 3,
  auto_deduplicate: true,
  download_notification: true,
})

// 默认配置
const defaultConfig = {
  download_path: '',
  naming_rule: 'original',
  custom_prefix: '',
  open_folder_after_download: false,
  theme: 'light',
  thumbnail_size: 180,
  sidebar_width: 320,
  show_info: ['resolution'],
  concurrent_downloads: 3,
  auto_deduplicate: true,
  download_notification: true,
}

async function open_path() {
  try {
    const result: any = await dialog.open({
      directory: true,
      defaultPath: form.download_path || '.'
    })
    if (result) {
      form.download_path = result
      ElMessage.success('路径已选择')
    }
  } catch (error) {
    ElMessage.error('选择路径失败')
  }
}

const onSubmit = async () => {
  // 验证必填项
  if (!form.download_path) {
    ElMessage.warning('请先选择下载保存路径')
    activeTab.value = 'basic'
    return
  }

  saving.value = true
  try {
    // 保存所有配置
    await invoke('save_config', {
      path: form.download_path,
      config: form
    })

    ElMessage.success('设置保存成功！')
  } catch (error) {
    ElMessage.error('保存设置失败：' + error)
  } finally {
    saving.value = false
  }
}

const handleCancel = () => {
  // 关闭窗口或返回
  window.close()
}

const resetToDefault = async () => {
  try {
    await ElMessageBox.confirm(
        '确定要恢复所有默认设置吗？此操作不可撤销。',
        '恢复默认设置',
        {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning',
        }
    )

    Object.assign(form, defaultConfig)
    ElMessage.success('已恢复默认设置')
  } catch {
    // 用户取消
  }
}

async function get_default_path() {
  try {
    const ret: any = await invoke('load_config')
    if (ret) {
      default_path.value = ret.download_path
      form.download_path = ret.download_path

      // 加载其他配置
      if (ret.config) {
        Object.assign(form, ret.config)
      }
    }
  } catch (error) {
    console.error('加载配置失败:', error)
  }
}



const openGithub = async () => {
  await open('https://github.com/Sunnnner/wallhaven_rs')
}

const openDocs = async () => {
  ElMessage.info('文档页面开发中...')
}

onMounted(async () => {
  await get_default_path()
})

</script>

<style scoped lang="scss">
.config-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #f5f7fa 0%, #e8eef5 100%);
  overflow: hidden;

  // 顶部标题栏
  .config-header {
    padding: 20px 24px;
    background: #fff;
    border-bottom: 1px solid #e4e7ed;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);

    .header-title {
      display: flex;
      align-items: center;
      gap: 12px;

      h2 {
        margin: 0;
        font-size: 20px;
        font-weight: 600;
        color: #303133;
      }
    }
  }

  // 内容区域
  .config-content {
    flex: 1;
    overflow: hidden;
    padding: 24px;

    .config-tabs {
      height: 100%;
      background: #fff;
      border-radius: 8px;
      box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
      padding: 20px;

      :deep(.el-tabs__header) {
        margin-bottom: 24px;
      }

      :deep(.el-tabs__item) {
        font-size: 15px;
        font-weight: 500;
        padding: 0 24px;
      }

      :deep(.el-tabs__content) {
        height: calc(100% - 60px);
        overflow-y: auto;
      }

      .tab-content {
        max-width: 700px;
        margin: 0 auto;

        .el-form {
          .el-form-item {
            margin-bottom: 28px;

            :deep(.el-form-item__label) {
              font-weight: 500;
              color: #606266;
            }
          }

          .path-input {
            :deep(.el-input-group__append) {
              padding: 0;

              .el-button {
                border-radius: 0;
              }
            }
          }

          .form-hint {
            display: flex;
            align-items: center;
            gap: 6px;
            margin-top: 8px;
            font-size: 13px;
            color: #909399;

            .el-icon {
              font-size: 14px;
            }
          }
        }
      }

      // 关于页面样式
      .about-section {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 400px;

        .about-card {
          text-align: center;
          max-width: 500px;

          .el-icon {
            margin-bottom: 16px;
          }

          h3 {
            font-size: 24px;
            font-weight: 600;
            color: #303133;
            margin: 0 0 8px;
          }

          .version {
            font-size: 14px;
            color: #909399;
            margin: 0 0 16px;
          }

          .description {
            font-size: 14px;
            color: #606266;
            line-height: 1.6;
            margin: 0 0 24px;
          }

          .about-links {
            display: flex;
            gap: 12px;
            justify-content: center;
            margin-bottom: 32px;
          }

          .about-info {
            margin-top: 24px;
          }
        }
      }
    }
  }

  // 底部操作栏
  .config-footer {
    padding: 16px 24px;
    background: #fff;
    border-top: 1px solid #e4e7ed;
    box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.05);

    .footer-actions {
      display: flex;
      justify-content: space-between;
      align-items: center;
      max-width: 1200px;
      margin: 0 auto;

      .right-actions {
        display: flex;
        gap: 12px;
      }
    }
  }

  // 滚动条样式
  :deep(.el-tabs__content) {
    &::-webkit-scrollbar {
      width: 8px;
    }

    &::-webkit-scrollbar-track {
      background: #f1f1f1;
      border-radius: 4px;
    }

    &::-webkit-scrollbar-thumb {
      background: #c1c1c1;
      border-radius: 4px;

      &:hover {
        background: #a8a8a8;
      }
    }
  }

  // 动画效果
  .el-button, .el-input, .el-slider {
    transition: all 0.3s ease;
  }
}

// 响应式设计
@media (max-width: 768px) {
  .config-container {
    .config-content {
      padding: 16px;

      .tab-content {
        .el-form {
          :deep(.el-form-item__label) {
            text-align: left;
          }
        }
      }
    }
  }
}
</style>