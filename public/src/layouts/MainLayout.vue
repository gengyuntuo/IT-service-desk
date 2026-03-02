<template>
  <div class="main-layout">
    <div class="sidebar" :class="{ collapsed: isCollapsed, hidden: isHidden }">
      <div class="sidebar-header">
        <div class="logo" v-if="!isCollapsed">
          <span class="logo-icon">🖥️</span>
          <span class="logo-text">IT服务台</span>
        </div>
        <div class="logo-icon-only" v-else>
          <span>🖥️</span>
        </div>
        <button class="toggle-btn" @click="toggleCollapse">
          {{ isCollapsed ? '→' : '←' }}
        </button>
      </div>
      
      <nav class="sidebar-nav">
        <router-link
          v-for="item in menuItems"
          :key="item.path"
          :to="item.path"
          class="nav-item"
          :class="{ active: $route.path === item.path }"
        >
          <span class="nav-icon">{{ item.icon }}</span>
          <span class="nav-text" v-if="!isCollapsed">{{ item.title }}</span>
        </router-link>
      </nav>
      
      <div class="sidebar-footer">
        <button class="hide-btn" @click="toggleHide">
          {{ isCollapsed ? '隐藏导航栏' : '隐藏导航栏' }}
        </button>
      </div>
    </div>
    
    <div class="main-content" :class="{ expanded: isHidden }">
      <header class="top-header">
        <div class="header-left">
          <button v-if="isHidden" class="show-sidebar-btn" @click="toggleHide">
            ☰ 显示导航栏
          </button>
        </div>
        <div class="header-right">
          <span class="user-info">管理员</span>
          <button class="logout-btn" @click="handleLogout">退出</button>
        </div>
      </header>
      
      <main class="content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const isCollapsed = ref(false)
const isHidden = ref(false)

const menuItems = [
  { path: '/dashboard', title: 'Dashboard', icon: '📊' },
  { path: '/ticket-apply', title: '工单申请', icon: '📝' },
  { path: '/ticket-approve', title: '工单审批', icon: '✅' },
  { path: '/aws-account', title: 'AWS账号管理', icon: '☁️' },
  { path: '/resource-management', title: '资源管理', icon: '📦' },
  { path: '/users', title: '用户管理', icon: '👥' },
  { path: '/settings', title: '设置', icon: '⚙️' }
]

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}

const toggleHide = () => {
  isHidden.value = !isHidden.value
  if (!isHidden.value) {
    isCollapsed.value = false
  }
}

const handleLogout = () => {
  if (confirm('确定要退出登录吗？')) {
    router.push('/')
  }
}
</script>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.sidebar {
  width: 260px;
  background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease, transform 0.3s ease;
  color: white;
  flex-shrink: 0;
}

.sidebar.collapsed {
  width: 80px;
}

.sidebar.hidden {
  width: 0;
  min-width: 0;
  overflow: hidden;
  padding: 0;
  border: none;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo-icon {
  font-size: 24px;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
}

.logo-icon-only {
  display: flex;
  justify-content: center;
  font-size: 24px;
}

.toggle-btn {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: white;
  width: 28px;
  height: 28px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.3s;
}

.toggle-btn:hover {
  background: rgba(255, 255, 255, 0.3);
}

.sidebar-nav {
  flex: 1;
  padding: 20px 0;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 20px;
  color: rgba(255, 255, 255, 0.8);
  text-decoration: none;
  transition: all 0.3s;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.nav-item.active {
  background: rgba(255, 255, 255, 0.15);
  color: white;
  border-left-color: white;
}

.nav-icon {
  font-size: 20px;
  min-width: 20px;
}

.nav-text {
  font-size: 14px;
  white-space: nowrap;
}

.sidebar-footer {
  padding: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.hide-btn {
  width: 100%;
  padding: 10px;
  background: rgba(255, 255, 255, 0.15);
  border: none;
  color: white;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.3s;
}

.hide-btn:hover {
  background: rgba(255, 255, 255, 0.25);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: margin-left 0.3s ease;
}

.top-header {
  background: white;
  padding: 16px 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.header-left {
  display: flex;
  align-items: center;
}

.show-sidebar-btn {
  padding: 8px 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.show-sidebar-btn:hover {
  opacity: 0.9;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.user-info {
  color: #333;
  font-size: 14px;
}

.logout-btn {
  padding: 8px 16px;
  background: #f44336;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: background 0.3s;
}

.logout-btn:hover {
  background: #d32f2f;
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  background: #f5f5f5;
}
</style>
