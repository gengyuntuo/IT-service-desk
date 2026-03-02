<template>
  <div class="resource-management">
    <div class="page-header">
      <h1 class="page-title">资源管理</h1>
      <button class="create-btn" @click="showModal = true">
        ➕ 创建资源
      </button>
    </div>

    <div class="stats-cards">
      <div class="stat-card">
        <div class="stat-icon server">🖥️</div>
        <div class="stat-info">
          <div class="stat-value">{{ resourceStats.servers }}</div>
          <div class="stat-label">服务器</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon database">🗄️</div>
        <div class="stat-info">
          <div class="stat-value">{{ resourceStats.databases }}</div>
          <div class="stat-label">数据库</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon storage">💾</div>
        <div class="stat-info">
          <div class="stat-value">{{ resourceStats.storage }}</div>
          <div class="stat-label">存储</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon network">🌐</div>
        <div class="stat-info">
          <div class="stat-value">{{ resourceStats.network }}</div>
          <div class="stat-label">网络设备</div>
        </div>
      </div>
    </div>

    <div class="filter-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索资源名称、IP地址..."
        class="search-input"
      />
      <select v-model="filterType" class="filter-select">
        <option value="">全部类型</option>
        <option value="server">服务器</option>
        <option value="database">数据库</option>
        <option value="storage">存储</option>
        <option value="network">网络设备</option>
      </select>
      <select v-model="filterStatus" class="filter-select">
        <option value="">全部状态</option>
        <option value="running">运行中</option>
        <option value="stopped">已停止</option>
        <option value="maintenance">维护中</option>
      </select>
      <select v-model="filterEnv" class="filter-select">
        <option value="">全部环境</option>
        <option value="production">生产环境</option>
        <option value="staging">预发布</option>
        <option value="development">开发环境</option>
        <option value="test">测试环境</option>
      </select>
    </div>

    <div class="table-container">
      <table class="resource-table">
        <thead>
          <tr>
            <th>资源名称</th>
            <th>类型</th>
            <th>IP地址</th>
            <th>环境</th>
            <th>配置</th>
            <th>状态</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="resource in paginatedResources" :key="resource.id">
            <td>
              <div class="resource-name">
                <span class="resource-icon">{{ getTypeIcon(resource.type) }}</span>
                <span>{{ resource.name }}</span>
              </div>
            </td>
            <td>
              <span class="type-badge" :class="resource.type">
                {{ getTypeText(resource.type) }}
              </span>
            </td>
            <td class="ip-address">{{ resource.ip }}</td>
            <td>
              <span class="env-badge" :class="resource.environment">
                {{ getEnvText(resource.environment) }}
              </span>
            </td>
            <td class="config-cell">{{ resource.config }}</td>
            <td>
              <span class="status-badge" :class="resource.status">
                {{ getStatusText(resource.status) }}
              </span>
            </td>
            <td>{{ resource.createTime }}</td>
            <td>
              <button class="action-btn view" @click="viewResource(resource)">查看</button>
              <button class="action-btn edit" @click="editResource(resource)">编辑</button>
              <button class="action-btn delete" @click="deleteResource(resource.id)">删除</button>
            </td>
          </tr>
          <tr v-if="paginatedResources.length === 0">
            <td colspan="8" class="empty-cell">
              <div class="empty-state">
                <div class="empty-icon">📦</div>
                <div class="empty-text">暂无资源数据</div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="pagination">
      <button class="page-btn" :disabled="currentPage === 1" @click="currentPage--">
        上一页
      </button>
      <span class="page-info">第 {{ currentPage }} 页，共 {{ totalPages }} 页</span>
      <button class="page-btn" :disabled="currentPage === totalPages" @click="currentPage++">
        下一页
      </button>
    </div>

    <!-- 创建/编辑资源对话框 -->
    <div v-if="showModal" class="modal-overlay" @click="showModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">{{ isEditing ? '编辑资源' : '创建资源' }}</h2>
          <button class="close-btn" @click="showModal = false">✕</button>
        </div>

        <form @submit.prevent="saveResource" class="modal-form">
          <div class="form-row">
            <div class="form-group">
              <label>资源名称 <span class="required">*</span></label>
              <input v-model="form.name" type="text" placeholder="请输入资源名称" required />
            </div>
            <div class="form-group">
              <label>资源类型 <span class="required">*</span></label>
              <select v-model="form.type" required>
                <option value="">请选择类型</option>
                <option value="server">服务器</option>
                <option value="database">数据库</option>
                <option value="storage">存储</option>
                <option value="network">网络设备</option>
              </select>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>IP地址 <span class="required">*</span></label>
              <input v-model="form.ip" type="text" placeholder="例如：192.168.1.100" required />
            </div>
            <div class="form-group">
              <label>端口</label>
              <input v-model="form.port" type="text" placeholder="例如：22, 3306, 8080" />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>所属环境 <span class="required">*</span></label>
              <select v-model="form.environment" required>
                <option value="">请选择环境</option>
                <option value="production">生产环境</option>
                <option value="staging">预发布</option>
                <option value="development">开发环境</option>
                <option value="test">测试环境</option>
              </select>
            </div>
            <div class="form-group">
              <label>状态 <span class="required">*</span></label>
              <select v-model="form.status" required>
                <option value="running">运行中</option>
                <option value="stopped">已停止</option>
                <option value="maintenance">维护中</option>
              </select>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>CPU</label>
              <input v-model="form.cpu" type="text" placeholder="例如：4核" />
            </div>
            <div class="form-group">
              <label>内存</label>
              <input v-model="form.memory" type="text" placeholder="例如：16GB" />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>磁盘</label>
              <input v-model="form.disk" type="text" placeholder="例如：500GB SSD" />
            </div>
            <div class="form-group">
              <label>操作系统</label>
              <input v-model="form.os" type="text" placeholder="例如：CentOS 7.9" />
            </div>
          </div>

          <div class="form-group">
            <label>关联AWS账号</label>
            <select v-model="form.awsAccount">
              <option value="">无</option>
              <option v-for="account in awsAccounts" :key="account.id" :value="account.id">
                {{ account.name }} ({{ account.accountId }})
              </option>
            </select>
          </div>

          <div class="form-group">
            <label>备注</label>
            <textarea v-model="form.remark" rows="3" placeholder="请输入备注信息..."></textarea>
          </div>

          <div class="modal-actions">
            <button type="button" class="btn-secondary" @click="showModal = false">取消</button>
            <button type="submit" class="btn-primary" :disabled="submitting">
              {{ submitting ? '保存中...' : (isEditing ? '保存修改' : '创建资源') }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- 查看详情对话框 -->
    <div v-if="showDetailModal" class="modal-overlay" @click="showDetailModal = false">
      <div class="modal detail-modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">资源详情</h2>
          <button class="close-btn" @click="showDetailModal = false">✕</button>
        </div>

        <div class="detail-content" v-if="selectedResource">
          <div class="detail-header">
            <span class="detail-icon">{{ getTypeIcon(selectedResource.type) }}</span>
            <div class="detail-title-info">
              <h3>{{ selectedResource.name }}</h3>
              <span class="type-badge" :class="selectedResource.type">
                {{ getTypeText(selectedResource.type) }}
              </span>
            </div>
          </div>

          <div class="detail-section">
            <h4>基本信息</h4>
            <div class="detail-row">
              <span class="detail-label">IP地址：</span>
              <span class="detail-value">{{ selectedResource.ip }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">端口：</span>
              <span class="detail-value">{{ selectedResource.port || '未配置' }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">环境：</span>
              <span class="detail-value">
                <span class="env-badge" :class="selectedResource.environment">
                  {{ getEnvText(selectedResource.environment) }}
                </span>
              </span>
            </div>
            <div class="detail-row">
              <span class="detail-label">状态：</span>
              <span class="detail-value">
                <span class="status-badge" :class="selectedResource.status">
                  {{ getStatusText(selectedResource.status) }}
                </span>
              </span>
            </div>
          </div>

          <div class="detail-section">
            <h4>配置信息</h4>
            <div class="detail-row">
              <span class="detail-label">CPU：</span>
              <span class="detail-value">{{ selectedResource.cpu || '未配置' }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">内存：</span>
              <span class="detail-value">{{ selectedResource.memory || '未配置' }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">磁盘：</span>
              <span class="detail-value">{{ selectedResource.disk || '未配置' }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">操作系统：</span>
              <span class="detail-value">{{ selectedResource.os || '未配置' }}</span>
            </div>
          </div>

          <div class="detail-section">
            <h4>其他信息</h4>
            <div class="detail-row">
              <span class="detail-label">创建时间：</span>
              <span class="detail-value">{{ selectedResource.createTime }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">更新时间：</span>
              <span class="detail-value">{{ selectedResource.updateTime || selectedResource.createTime }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">备注：</span>
            </div>
            <div class="detail-remark">
              {{ selectedResource.remark || '无' }}
            </div>
          </div>
        </div>

        <div class="modal-actions">
          <button type="button" class="btn-secondary" @click="showDetailModal = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'

interface Resource {
  id: number
  name: string
  type: 'server' | 'database' | 'storage' | 'network'
  ip: string
  port?: string
  environment: 'production' | 'staging' | 'development' | 'test'
  status: 'running' | 'stopped' | 'maintenance'
  config: string
  cpu?: string
  memory?: string
  disk?: string
  os?: string
  awsAccount?: number
  createTime: string
  updateTime?: string
  remark?: string
}

interface AwsAccount {
  id: number
  name: string
  accountId: string
}

const searchQuery = ref('')
const filterType = ref('')
const filterStatus = ref('')
const filterEnv = ref('')
const currentPage = ref(1)
const pageSize = 10
const showModal = ref(false)
const showDetailModal = ref(false)
const selectedResource = ref<Resource | null>(null)
const isEditing = ref(false)
const submitting = ref(false)
const editId = ref<number | null>(null)

const form = reactive({
  name: '',
  type: '',
  ip: '',
  port: '',
  environment: '',
  status: 'running' as 'running' | 'stopped' | 'maintenance',
  cpu: '',
  memory: '',
  disk: '',
  os: '',
  awsAccount: '',
  remark: ''
})

const awsAccounts = ref<AwsAccount[]>([
  { id: 1, name: '生产环境主账号', accountId: '123456789012' },
  { id: 2, name: '测试环境账号', accountId: '234567890123' }
])

const resources = ref<Resource[]>([
  {
    id: 1,
    name: 'Web服务器-01',
    type: 'server',
    ip: '192.168.1.100',
    port: '22, 80, 443',
    environment: 'production',
    status: 'running',
    config: '4核/16GB/500GB',
    cpu: '4核',
    memory: '16GB',
    disk: '500GB SSD',
    os: 'CentOS 7.9',
    awsAccount: 1,
    createTime: '2024-01-10 09:00',
    remark: '主Web服务器'
  },
  {
    id: 2,
    name: 'MySQL主库',
    type: 'database',
    ip: '192.168.1.101',
    port: '3306',
    environment: 'production',
    status: 'running',
    config: '8核/32GB/1TB',
    cpu: '8核',
    memory: '32GB',
    disk: '1TB SSD',
    os: 'CentOS 7.9',
    awsAccount: 1,
    createTime: '2024-01-10 10:30',
    remark: 'MySQL 8.0主数据库'
  },
  {
    id: 3,
    name: 'NAS存储',
    type: 'storage',
    ip: '192.168.1.102',
    environment: 'production',
    status: 'running',
    config: '10TB RAID5',
    disk: '10TB RAID5',
    createTime: '2024-01-15 14:00',
    remark: '文件共享存储'
  },
  {
    id: 4,
    name: '核心交换机',
    type: 'network',
    ip: '192.168.1.1',
    port: '22, 161',
    environment: 'production',
    status: 'running',
    config: '48口千兆',
    createTime: '2024-01-05 08:00',
    remark: '核心网络设备'
  },
  {
    id: 5,
    name: '测试服务器-01',
    type: 'server',
    ip: '192.168.2.100',
    port: '22, 8080',
    environment: 'test',
    status: 'stopped',
    config: '2核/8GB/200GB',
    cpu: '2核',
    memory: '8GB',
    disk: '200GB SSD',
    os: 'Ubuntu 20.04',
    awsAccount: 2,
    createTime: '2024-02-01 11:00',
    remark: '测试环境服务器'
  },
  {
    id: 6,
    name: 'Redis缓存',
    type: 'database',
    ip: '192.168.1.103',
    port: '6379',
    environment: 'production',
    status: 'running',
    config: '4核/16GB',
    cpu: '4核',
    memory: '16GB',
    createTime: '2024-01-20 16:00',
    remark: 'Redis 7.0缓存服务'
  }
])

const resourceStats = computed(() => ({
  servers: resources.value.filter(r => r.type === 'server').length,
  databases: resources.value.filter(r => r.type === 'database').length,
  storage: resources.value.filter(r => r.type === 'storage').length,
  network: resources.value.filter(r => r.type === 'network').length
}))

const filteredResources = computed(() => {
  return resources.value.filter(resource => {
    const matchesSearch = resource.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         resource.ip.includes(searchQuery.value)
    const matchesType = !filterType.value || resource.type === filterType.value
    const matchesStatus = !filterStatus.value || resource.status === filterStatus.value
    const matchesEnv = !filterEnv.value || resource.environment === filterEnv.value
    return matchesSearch && matchesType && matchesStatus && matchesEnv
  })
})

const totalPages = computed(() => Math.ceil(filteredResources.value.length / pageSize))

const paginatedResources = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return filteredResources.value.slice(start, end)
})

const getTypeIcon = (type: string) => {
  const map: Record<string, string> = {
    server: '🖥️',
    database: '🗄️',
    storage: '💾',
    network: '🌐'
  }
  return map[type] || '📦'
}

const getTypeText = (type: string) => {
  const map: Record<string, string> = {
    server: '服务器',
    database: '数据库',
    storage: '存储',
    network: '网络设备'
  }
  return map[type] || type
}

const getEnvText = (env: string) => {
  const map: Record<string, string> = {
    production: '生产环境',
    staging: '预发布',
    development: '开发环境',
    test: '测试环境'
  }
  return map[env] || env
}

const getStatusText = (status: string) => {
  const map: Record<string, string> = {
    running: '运行中',
    stopped: '已停止',
    maintenance: '维护中'
  }
  return map[status] || status
}

const resetForm = () => {
  form.name = ''
  form.type = ''
  form.ip = ''
  form.port = ''
  form.environment = ''
  form.status = 'running'
  form.cpu = ''
  form.memory = ''
  form.disk = ''
  form.os = ''
  form.awsAccount = ''
  form.remark = ''
  editId.value = null
  isEditing.value = false
}

const saveResource = async () => {
  submitting.value = true

  try {
    await new Promise(resolve => setTimeout(resolve, 800))

    const configStr = [form.cpu, form.memory, form.disk].filter(Boolean).join('/')

    if (isEditing.value && editId.value) {
      const index = resources.value.findIndex(r => r.id === editId.value)
      if (index !== -1) {
        resources.value[index] = {
          ...resources.value[index],
          name: form.name,
          type: form.type as 'server' | 'database' | 'storage' | 'network',
          ip: form.ip,
          port: form.port,
          environment: form.environment as 'production' | 'staging' | 'development' | 'test',
          status: form.status,
          config: configStr || '未配置',
          cpu: form.cpu,
          memory: form.memory,
          disk: form.disk,
          os: form.os,
          awsAccount: form.awsAccount ? parseInt(form.awsAccount) : undefined,
          remark: form.remark,
          updateTime: new Date().toLocaleString('zh-CN')
        }
      }
      alert('资源修改成功！')
    } else {
      const newId = Math.max(...resources.value.map(r => r.id), 0) + 1
      resources.value.unshift({
        id: newId,
        name: form.name,
        type: form.type as 'server' | 'database' | 'storage' | 'network',
        ip: form.ip,
        port: form.port,
        environment: form.environment as 'production' | 'staging' | 'development' | 'test',
        status: form.status,
        config: configStr || '未配置',
        cpu: form.cpu,
        memory: form.memory,
        disk: form.disk,
        os: form.os,
        awsAccount: form.awsAccount ? parseInt(form.awsAccount) : undefined,
        remark: form.remark,
        createTime: new Date().toLocaleString('zh-CN')
      })
      alert('资源创建成功！')
    }

    resetForm()
    showModal.value = false
  } catch (error) {
    alert('操作失败，请重试')
  } finally {
    submitting.value = false
  }
}

const viewResource = (resource: Resource) => {
  selectedResource.value = resource
  showDetailModal.value = true
}

const editResource = (resource: Resource) => {
  isEditing.value = true
  editId.value = resource.id
  form.name = resource.name
  form.type = resource.type
  form.ip = resource.ip
  form.port = resource.port || ''
  form.environment = resource.environment
  form.status = resource.status
  form.cpu = resource.cpu || ''
  form.memory = resource.memory || ''
  form.disk = resource.disk || ''
  form.os = resource.os || ''
  form.awsAccount = resource.awsAccount ? String(resource.awsAccount) : ''
  form.remark = resource.remark || ''
  showModal.value = true
}

const deleteResource = (id: number) => {
  if (confirm('确定要删除该资源吗？此操作不可恢复。')) {
    resources.value = resources.value.filter(r => r.id !== id)
    alert('资源已删除')
  }
}
</script>

<style scoped>
.resource-management {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.page-title {
  font-size: 28px;
  color: #333;
}

.create-btn {
  padding: 10px 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: opacity 0.3s;
}

.create-btn:hover {
  opacity: 0.9;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: white;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.stat-icon.server {
  background: #dbeafe;
}

.stat-icon.database {
  background: #fce7f3;
}

.stat-icon.storage {
  background: #d1fae5;
}

.stat-icon.network {
  background: #e0e7ff;
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  color: #333;
}

.stat-label {
  font-size: 14px;
  color: #666;
  margin-top: 4px;
}

.filter-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.search-input {
  flex: 1;
  padding: 10px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: #667eea;
}

.filter-select {
  padding: 10px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  background: white;
  cursor: pointer;
}

.table-container {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  overflow: hidden;
  margin-bottom: 24px;
}

.resource-table {
  width: 100%;
  border-collapse: collapse;
}

.resource-table th {
  background: #f8f9fa;
  padding: 16px;
  text-align: left;
  font-weight: 600;
  color: #333;
  border-bottom: 2px solid #eee;
}

.resource-table td {
  padding: 16px;
  border-bottom: 1px solid #eee;
  color: #555;
}

.resource-table tr:hover {
  background: #f8f9fa;
}

.resource-name {
  display: flex;
  align-items: center;
  gap: 10px;
}

.resource-icon {
  font-size: 20px;
}

.ip-address {
  font-family: 'Courier New', monospace;
  font-size: 13px;
  color: #666;
}

.config-cell {
  font-size: 13px;
  color: #666;
}

.empty-cell {
  text-align: center;
  padding: 60px;
}

.empty-state {
  color: #999;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-text {
  font-size: 14px;
}

.type-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.type-badge.server {
  background: #dbeafe;
  color: #2563eb;
}

.type-badge.database {
  background: #fce7f3;
  color: #db2777;
}

.type-badge.storage {
  background: #d1fae5;
  color: #059669;
}

.type-badge.network {
  background: #e0e7ff;
  color: #4f46e5;
}

.env-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.env-badge.production {
  background: #fee2e2;
  color: #dc2626;
}

.env-badge.staging {
  background: #fef3c7;
  color: #d97706;
}

.env-badge.development {
  background: #dbeafe;
  color: #2563eb;
}

.env-badge.test {
  background: #e0e7ff;
  color: #4f46e5;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.running {
  background: #d1fae5;
  color: #059669;
}

.status-badge.stopped {
  background: #fee2e2;
  color: #dc2626;
}

.status-badge.maintenance {
  background: #fef3c7;
  color: #d97706;
}

.action-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  margin-right: 6px;
  transition: opacity 0.3s;
}

.action-btn.view {
  background: #3b82f6;
  color: white;
}

.action-btn.edit {
  background: #f59e0b;
  color: white;
}

.action-btn.delete {
  background: #ef4444;
  color: white;
}

.action-btn:hover {
  opacity: 0.9;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
}

.page-btn {
  padding: 8px 16px;
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.page-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-info {
  color: #666;
  font-size: 14px;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: white;
  border-radius: 12px;
  width: 100%;
  max-width: 650px;
  max-height: 90vh;
  overflow-y: auto;
}

.detail-modal {
  max-width: 550px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #eee;
}

.modal-title {
  font-size: 18px;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 20px;
  color: #999;
  cursor: pointer;
}

.modal-form {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
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

.required {
  color: #ef4444;
}

.form-group input,
.form-group select,
.form-group textarea {
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  font-family: inherit;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #667eea;
}

.modal-actions {
  display: flex;
  gap: 12px;
  padding: 20px 24px;
  border-top: 1px solid #eee;
}

.btn-secondary,
.btn-primary {
  flex: 1;
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: opacity 0.3s;
}

.btn-secondary {
  background: #f3f4f6;
  color: #333;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover,
.btn-secondary:hover {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.detail-content {
  padding: 24px;
}

.detail-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #eee;
}

.detail-icon {
  font-size: 40px;
}

.detail-title-info h3 {
  margin: 0 0 8px 0;
  font-size: 20px;
  color: #333;
}

.detail-section {
  margin-bottom: 20px;
}

.detail-section h4 {
  font-size: 14px;
  color: #666;
  margin: 0 0 12px 0;
  padding-bottom: 8px;
  border-bottom: 1px solid #f0f0f0;
}

.detail-row {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}

.detail-label {
  font-size: 14px;
  color: #666;
  min-width: 80px;
}

.detail-value {
  font-size: 14px;
  color: #333;
}

.detail-remark {
  font-size: 14px;
  color: #555;
  line-height: 1.6;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-top: 8px;
}
</style>
