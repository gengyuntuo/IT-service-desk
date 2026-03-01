<template>
  <div class="settings-page">
    <h1 class="page-title">设置</h1>
    
    <div class="settings-container">
      <div class="settings-nav">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          class="nav-tab"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >
          {{ tab.icon }} {{ tab.label }}
        </button>
      </div>
      
      <div class="settings-content">
        <div v-if="activeTab === 'profile'" class="settings-section">
          <h2 class="section-title">个人资料</h2>
          <form @submit.prevent="saveProfile" class="settings-form">
            <div class="form-group">
              <label>头像</label>
              <div class="avatar-upload">
                <div class="avatar-preview">
                  <span>{{ profile.name.charAt(0) }}</span>
                </div>
                <button type="button" class="upload-btn">更换头像</button>
              </div>
            </div>
            
            <div class="form-row">
              <div class="form-group">
                <label>用户名</label>
                <input v-model="profile.username" type="text" />
              </div>
              <div class="form-group">
                <label>真实姓名</label>
                <input v-model="profile.name" type="text" />
              </div>
            </div>
            
            <div class="form-group">
              <label>邮箱</label>
              <input v-model="profile.email" type="email" />
            </div>
            
            <div class="form-group">
              <label>电话</label>
              <input v-model="profile.phone" type="tel" />
            </div>
            
            <div class="form-group">
              <label>部门</label>
              <input v-model="profile.department" type="text" />
            </div>
            
            <div class="form-actions">
              <button type="submit" class="btn-primary">保存更改</button>
            </div>
          </form>
        </div>
        
        <div v-if="activeTab === 'security'" class="settings-section">
          <h2 class="section-title">安全设置</h2>
          
          <div class="security-card">
            <h3 class="card-subtitle">修改密码</h3>
            <form @submit.prevent="changePassword" class="settings-form">
              <div class="form-group">
                <label>当前密码</label>
                <input v-model="password.current" type="password" />
              </div>
              
              <div class="form-group">
                <label>新密码</label>
                <input v-model="password.new" type="password" />
              </div>
              
              <div class="form-group">
                <label>确认新密码</label>
                <input v-model="password.confirm" type="password" />
              </div>
              
              <div class="form-actions">
                <button type="submit" class="btn-primary">更新密码</button>
              </div>
            </form>
          </div>
          
          <div class="security-card">
            <h3 class="card-subtitle">两步验证</h3>
            <div class="toggle-item">
              <div class="toggle-info">
                <div class="toggle-label">启用两步验证</div>
                <div class="toggle-desc">增加账户安全性</div>
              </div>
              <label class="toggle-switch">
                <input type="checkbox" v-model="security.twoFactor" />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>
        </div>
        
        <div v-if="activeTab === 'notifications'" class="settings-section">
          <h2 class="section-title">通知设置</h2>
          
          <div class="notification-list">
            <div v-for="item in notifications" :key="item.id" class="notification-item">
              <div class="notification-info">
                <div class="notification-label">{{ item.label }}</div>
                <div class="notification-desc">{{ item.desc }}</div>
              </div>
              <label class="toggle-switch">
                <input type="checkbox" v-model="item.enabled" />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>
        </div>
        
        <div v-if="activeTab === 'system'" class="settings-section">
          <h2 class="section-title">系统设置</h2>
          
          <div class="system-settings">
            <div class="setting-item">
              <div class="setting-info">
                <div class="setting-label">语言</div>
                <div class="setting-desc">选择界面语言</div>
              </div>
              <select v-model="system.language" class="setting-select">
                <option value="zh-CN">简体中文</option>
                <option value="en-US">English</option>
              </select>
            </div>
            
            <div class="setting-item">
              <div class="setting-info">
                <div class="setting-label">主题</div>
                <div class="setting-desc">选择界面主题</div>
              </div>
              <select v-model="system.theme" class="setting-select">
                <option value="light">浅色</option>
                <option value="dark">深色</option>
                <option value="auto">跟随系统</option>
              </select>
            </div>
            
            <div class="setting-item">
              <div class="setting-info">
                <div class="setting-label">每页显示条数</div>
                <div class="setting-desc">列表每页显示的记录数</div>
              </div>
              <select v-model="system.pageSize" class="setting-select">
                <option value="10">10</option>
                <option value="20">20</option>
                <option value="50">50</option>
              </select>
            </div>
            
            <div class="setting-item">
              <div class="setting-info">
                <div class="setting-label">自动保存</div>
                <div class="setting-desc">自动保存编辑内容</div>
              </div>
              <label class="toggle-switch">
                <input type="checkbox" v-model="system.autoSave" />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('profile')

const tabs = [
  { id: 'profile', label: '个人资料', icon: '👤' },
  { id: 'security', label: '安全设置', icon: '🔒' },
  { id: 'notifications', label: '通知设置', icon: '🔔' },
  { id: 'system', label: '系统设置', icon: '⚙️' }
]

const profile = ref({
  username: 'admin',
  name: '管理员',
  email: 'admin@example.com',
  phone: '13800138000',
  department: 'IT部门'
})

const password = ref({
  current: '',
  new: '',
  confirm: ''
})

const security = ref({
  twoFactor: false
})

const notifications = ref([
  { id: 1, label: '邮件通知', desc: '接收重要邮件通知', enabled: true },
  { id: 2, label: '工单提醒', desc: '新工单提醒', enabled: true },
  { id: 3, label: '系统公告', desc: '接收系统公告', enabled: true },
  { id: 4, label: '周报推送', desc: '每周推送周报', enabled: false }
])

const system = ref({
  language: 'zh-CN',
  theme: 'light',
  pageSize: '10',
  autoSave: true
})

const saveProfile = () => {
  alert('个人资料已保存')
}

const changePassword = () => {
  if (password.value.new !== password.value.confirm) {
    alert('两次输入的密码不一致')
    return
  }
  alert('密码已更新')
  password.value = { current: '', new: '', confirm: '' }
}
</script>

<style scoped>
.settings-page {
  max-width: 1200px;
  margin: 0 auto;
}

.page-title {
  font-size: 28px;
  color: #333;
  margin-bottom: 24px;
}

.settings-container {
  display: flex;
  gap: 24px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  overflow: hidden;
}

.settings-nav {
  width: 240px;
  background: #f8f9fa;
  padding: 24px 0;
  border-right: 1px solid #eee;
}

.nav-tab {
  width: 100%;
  padding: 14px 24px;
  text-align: left;
  background: transparent;
  border: none;
  border-left: 3px solid transparent;
  cursor: pointer;
  font-size: 14px;
  color: #666;
  transition: all 0.3s;
}

.nav-tab:hover {
  background: rgba(102, 126, 234, 0.1);
  color: #667eea;
}

.nav-tab.active {
  background: rgba(102, 126, 234, 0.1);
  color: #667eea;
  border-left-color: #667eea;
  font-weight: 500;
}

.settings-content {
  flex: 1;
  padding: 32px;
}

.section-title {
  font-size: 20px;
  color: #333;
  margin-bottom: 24px;
  padding-bottom: 12px;
  border-bottom: 1px solid #eee;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 14px;
  color: #555;
  font-weight: 500;
}

.form-group input,
.form-group select {
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: #667eea;
}

.avatar-upload {
  display: flex;
  align-items: center;
  gap: 16px;
}

.avatar-preview {
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: 600;
}

.upload-btn {
  padding: 8px 16px;
  background: #f3f4f6;
  color: #333;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.form-actions {
  margin-top: 8px;
}

.btn-primary {
  padding: 12px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: opacity 0.3s;
}

.btn-primary:hover {
  opacity: 0.9;
}

.security-card {
  background: #f8f9fa;
  padding: 24px;
  border-radius: 8px;
  margin-bottom: 20px;
}

.card-subtitle {
  font-size: 16px;
  color: #333;
  margin-bottom: 20px;
}

.toggle-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toggle-info {
  flex: 1;
}

.toggle-label {
  font-size: 14px;
  color: #333;
  font-weight: 500;
  margin-bottom: 4px;
}

.toggle-desc {
  font-size: 13px;
  color: #666;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 24px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

.toggle-switch input:checked + .toggle-slider {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.toggle-switch input:checked + .toggle-slider:before {
  transform: translateX(24px);
}

.notification-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.notification-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.notification-info {
  flex: 1;
}

.notification-label {
  font-size: 14px;
  color: #333;
  font-weight: 500;
  margin-bottom: 4px;
}

.notification-desc {
  font-size: 13px;
  color: #666;
}

.system-settings {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.setting-info {
  flex: 1;
}

.setting-label {
  font-size: 14px;
  color: #333;
  font-weight: 500;
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 13px;
  color: #666;
}

.setting-select {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
  background: white;
  cursor: pointer;
}
</style>
