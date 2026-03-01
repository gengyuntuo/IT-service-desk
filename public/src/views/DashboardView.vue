<template>
  <div class="dashboard">
    <h1 class="page-title">仪表盘</h1>
    
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon" style="background: #667eea;">
          📋
        </div>
        <div class="stat-info">
          <div class="stat-label">总工单</div>
          <div class="stat-value">128</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon" style="background: #f59e0b;">
          ⏳
        </div>
        <div class="stat-info">
          <div class="stat-label">待处理</div>
          <div class="stat-value">24</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon" style="background: #10b981;">
          ✅
        </div>
        <div class="stat-info">
          <div class="stat-label">已完成</div>
          <div class="stat-value">96</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon" style="background: #ef4444;">
          ⚠️
        </div>
        <div class="stat-info">
          <div class="stat-label">紧急</div>
          <div class="stat-value">8</div>
        </div>
      </div>
    </div>
    
    <div class="dashboard-grid">
      <div class="card">
        <h2 class="card-title">最近工单</h2>
        <div class="ticket-list">
          <div class="ticket-item" v-for="ticket in recentTickets" :key="ticket.id">
            <div class="ticket-header">
              <span class="ticket-title">{{ ticket.title }}</span>
              <span class="ticket-status" :class="ticket.statusClass">{{ ticket.status }}</span>
            </div>
            <div class="ticket-meta">
              <span>👤 {{ ticket.user }}</span>
              <span>🕐 {{ ticket.time }}</span>
            </div>
          </div>
        </div>
      </div>
      
      <div class="card">
        <h2 class="card-title">系统状态</h2>
        <div class="system-status">
          <div class="status-item">
            <div class="status-label">服务器状态</div>
            <div class="status-indicator online">正常运行</div>
          </div>
          <div class="status-item">
            <div class="status-label">数据库</div>
            <div class="status-indicator online">正常运行</div>
          </div>
          <div class="status-item">
            <div class="status-label">API服务</div>
            <div class="status-indicator online">正常运行</div>
          </div>
          <div class="status-item">
            <div class="status-label">备份服务</div>
            <div class="status-indicator warning">需要关注</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const recentTickets = ref([
  { id: 1, title: '无法连接VPN', user: '张三', time: '10分钟前', status: '待处理', statusClass: 'pending' },
  { id: 2, title: '打印机卡纸', user: '李四', time: '30分钟前', status: '处理中', statusClass: 'processing' },
  { id: 3, title: '邮箱无法发送', user: '王五', time: '1小时前', status: '已解决', statusClass: 'resolved' },
  { id: 4, title: '系统升级申请', user: '赵六', time: '2小时前', status: '待处理', statusClass: 'pending' },
  { id: 5, title: '权限申请', user: '钱七', time: '3小时前', status: '已解决', statusClass: 'resolved' }
])
</script>

<style scoped>
.dashboard {
  max-width: 1400px;
  margin: 0 auto;
}

.page-title {
  font-size: 28px;
  color: #333;
  margin-bottom: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 20px;
  margin-bottom: 24px;
}

.stat-card {
  background: white;
  padding: 24px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 60px;
  height: 60px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  color: white;
}

.stat-info {
  flex: 1;
}

.stat-label {
  font-size: 14px;
  color: #666;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: #333;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 20px;
}

.card {
  background: white;
  padding: 24px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.card-title {
  font-size: 18px;
  color: #333;
  margin-bottom: 20px;
  padding-bottom: 12px;
  border-bottom: 1px solid #eee;
}

.ticket-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ticket-item {
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  transition: background 0.3s;
}

.ticket-item:hover {
  background: #f0f1f2;
}

.ticket-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.ticket-title {
  font-weight: 500;
  color: #333;
}

.ticket-status {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.ticket-status.pending {
  background: #fef3c7;
  color: #d97706;
}

.ticket-status.processing {
  background: #dbeafe;
  color: #2563eb;
}

.ticket-status.resolved {
  background: #d1fae5;
  color: #059669;
}

.ticket-meta {
  display: flex;
  gap: 16px;
  font-size: 13px;
  color: #666;
}

.system-status {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.status-label {
  font-size: 14px;
  color: #333;
  font-weight: 500;
}

.status-indicator {
  padding: 6px 16px;
  border-radius: 12px;
  font-size: 13px;
  font-weight: 500;
}

.status-indicator.online {
  background: #d1fae5;
  color: #059669;
}

.status-indicator.warning {
  background: #fef3c7;
  color: #d97706;
}
</style>
