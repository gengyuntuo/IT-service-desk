<template>
  <div class="aws-account">
    <div class="page-header">
      <h1 class="page-title">AWS账号管理</h1>
      <button class="create-btn" @click="showModal = true">
        ➕ 添加账号
      </button>
    </div>

    <div class="filter-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索账号名称、账号ID..."
        class="search-input"
      />
      <select v-model="filterStatus" class="filter-select">
        <option value="">全部状态</option>
        <option value="active">启用</option>
        <option value="inactive">禁用</option>
      </select>
      <select v-model="filterType" class="filter-select">
        <option value="">全部类型</option>
        <option value="root">根账号</option>
        <option value="iam">IAM用户</option>
        <option value="role">IAM角色</option>
      </select>
    </div>

    <div class="table-container">
      <table class="account-table">
        <thead>
          <tr>
            <th>账号名称</th>
            <th>账号ID</th>
            <th>账号类型</th>
            <th>区域</th>
            <th>创建时间</th>
            <th>状态</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="account in paginatedAccounts" :key="account.id">
            <td>
              <div class="account-name">
                <span class="account-icon">☁️</span>
                <span>{{ account.name }}</span>
              </div>
            </td>
            <td class="account-id">{{ account.accountId }}</td>
            <td>
              <span class="type-badge" :class="account.type">
                {{ getTypeText(account.type) }}
              </span>
            </td>
            <td>{{ account.region }}</td>
            <td>{{ account.createTime }}</td>
            <td>
              <span class="status-badge" :class="account.status">
                {{ getStatusText(account.status) }}
              </span>
            </td>
            <td>
              <button class="action-btn view" @click="viewAccount(account)">查看</button>
              <button class="action-btn edit" @click="editAccount(account)">编辑</button>
              <button class="action-btn delete" @click="deleteAccount(account.id)">删除</button>
            </td>
          </tr>
          <tr v-if="paginatedAccounts.length === 0">
            <td colspan="7" class="empty-cell">
              <div class="empty-state">
                <div class="empty-icon">☁️</div>
                <div class="empty-text">暂无AWS账号</div>
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

    <!-- 添加/编辑账号对话框 -->
    <div v-if="showModal" class="modal-overlay" @click="showModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">{{ isEditing ? '编辑AWS账号' : '添加AWS账号' }}</h2>
          <button class="close-btn" @click="showModal = false">✕</button>
        </div>

        <form @submit.prevent="saveAccount" class="modal-form">
          <div class="form-row">
            <div class="form-group">
              <label>账号名称 <span class="required">*</span></label>
              <input v-model="form.name" type="text" placeholder="请输入账号名称" required />
            </div>
            <div class="form-group">
              <label>账号ID <span class="required">*</span></label>
              <input v-model="form.accountId" type="text" placeholder="12位数字账号ID" maxlength="12" required />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>账号类型 <span class="required">*</span></label>
              <select v-model="form.type" required>
                <option value="">请选择类型</option>
                <option value="root">根账号</option>
                <option value="iam">IAM用户</option>
                <option value="role">IAM角色</option>
              </select>
            </div>
            <div class="form-group">
              <label>默认区域 <span class="required">*</span></label>
              <select v-model="form.region" required>
                <option value="">请选择区域</option>
                <option value="ap-northeast-1">亚太（东京）</option>
                <option value="ap-southeast-1">亚太（新加坡）</option>
                <option value="ap-southeast-2">亚太（悉尼）</option>
                <option value="cn-north-1">中国（北京）</option>
                <option value="cn-northwest-1">中国（宁夏）</option>
                <option value="us-east-1">美国东部（弗吉尼亚）</option>
                <option value="us-west-2">美国西部（俄勒冈）</option>
                <option value="eu-west-1">欧洲（爱尔兰）</option>
              </select>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Access Key ID</label>
              <input v-model="form.accessKeyId" type="text" placeholder="AKIA..." />
            </div>
            <div class="form-group">
              <label>Secret Access Key</label>
              <input v-model="form.secretAccessKey" type="password" placeholder="********" />
            </div>
          </div>

          <div class="form-group">
            <label>备注</label>
            <textarea v-model="form.remark" rows="3" placeholder="请输入备注信息..."></textarea>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="form.status" true-value="active" false-value="inactive" />
              <span>启用账号</span>
            </label>
          </div>

          <div class="modal-actions">
            <button type="button" class="btn-secondary" @click="showModal = false">取消</button>
            <button type="submit" class="btn-primary" :disabled="submitting">
              {{ submitting ? '保存中...' : (isEditing ? '保存修改' : '添加账号') }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- 查看详情对话框 -->
    <div v-if="showDetailModal" class="modal-overlay" @click="showDetailModal = false">
      <div class="modal detail-modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">AWS账号详情</h2>
          <button class="close-btn" @click="showDetailModal = false">✕</button>
        </div>

        <div class="detail-content" v-if="selectedAccount">
          <div class="detail-row">
            <span class="detail-label">账号名称：</span>
            <span class="detail-value">{{ selectedAccount.name }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">账号ID：</span>
            <span class="detail-value">{{ selectedAccount.accountId }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">账号类型：</span>
            <span class="detail-value">
              <span class="type-badge" :class="selectedAccount.type">
                {{ getTypeText(selectedAccount.type) }}
              </span>
            </span>
          </div>
          <div class="detail-row">
            <span class="detail-label">默认区域：</span>
            <span class="detail-value">{{ getRegionText(selectedAccount.region) }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Access Key ID：</span>
            <span class="detail-value">{{ selectedAccount.accessKeyId || '未配置' }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">状态：</span>
            <span class="detail-value">
              <span class="status-badge" :class="selectedAccount.status">
                {{ getStatusText(selectedAccount.status) }}
              </span>
            </span>
          </div>
          <div class="detail-row">
            <span class="detail-label">创建时间：</span>
            <span class="detail-value">{{ selectedAccount.createTime }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">更新时间：</span>
            <span class="detail-value">{{ selectedAccount.updateTime || selectedAccount.createTime }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">备注：</span>
          </div>
          <div class="detail-remark">
            {{ selectedAccount.remark || '无' }}
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

interface AwsAccount {
  id: number
  name: string
  accountId: string
  type: 'root' | 'iam' | 'role'
  region: string
  accessKeyId?: string
  secretAccessKey?: string
  status: 'active' | 'inactive'
  createTime: string
  updateTime?: string
  remark?: string
}

const searchQuery = ref('')
const filterStatus = ref('')
const filterType = ref('')
const currentPage = ref(1)
const pageSize = 10
const showModal = ref(false)
const showDetailModal = ref(false)
const selectedAccount = ref<AwsAccount | null>(null)
const isEditing = ref(false)
const submitting = ref(false)
const editId = ref<number | null>(null)

const form = reactive({
  name: '',
  accountId: '',
  type: '',
  region: '',
  accessKeyId: '',
  secretAccessKey: '',
  status: 'active' as 'active' | 'inactive',
  remark: ''
})

const accounts = ref<AwsAccount[]>([
  {
    id: 1,
    name: '生产环境主账号',
    accountId: '123456789012',
    type: 'root',
    region: 'ap-northeast-1',
    accessKeyId: 'AKIAIOSFODNN7EXAMPLE',
    status: 'active',
    createTime: '2024-01-15 10:30',
    updateTime: '2024-02-20 14:20',
    remark: '用于生产环境部署'
  },
  {
    id: 2,
    name: '测试环境账号',
    accountId: '234567890123',
    type: 'iam',
    region: 'ap-southeast-1',
    accessKeyId: 'AKIAI44QH8DHBEXAMPLE',
    status: 'active',
    createTime: '2024-01-20 09:15',
    remark: '开发和测试使用'
  },
  {
    id: 3,
    name: '运维管理账号',
    accountId: '345678901234',
    type: 'role',
    region: 'cn-north-1',
    status: 'active',
    createTime: '2024-02-01 16:45',
    remark: '运维团队使用'
  },
  {
    id: 4,
    name: '备份账号',
    accountId: '456789012345',
    type: 'iam',
    region: 'us-west-2',
    accessKeyId: 'AKIAIOSFODNN7BACKUP',
    status: 'inactive',
    createTime: '2024-02-10 11:00',
    remark: '数据备份专用'
  }
])

const filteredAccounts = computed(() => {
  return accounts.value.filter(account => {
    const matchesSearch = account.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         account.accountId.includes(searchQuery.value)
    const matchesStatus = !filterStatus.value || account.status === filterStatus.value
    const matchesType = !filterType.value || account.type === filterType.value
    return matchesSearch && matchesStatus && matchesType
  })
})

const totalPages = computed(() => Math.ceil(filteredAccounts.value.length / pageSize))

const paginatedAccounts = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return filteredAccounts.value.slice(start, end)
})

const getTypeText = (type: string) => {
  const map: Record<string, string> = {
    root: '根账号',
    iam: 'IAM用户',
    role: 'IAM角色'
  }
  return map[type] || type
}

const getStatusText = (status: string) => {
  const map: Record<string, string> = {
    active: '启用',
    inactive: '禁用'
  }
  return map[status] || status
}

const getRegionText = (region: string) => {
  const map: Record<string, string> = {
    'ap-northeast-1': '亚太（东京）',
    'ap-southeast-1': '亚太（新加坡）',
    'ap-southeast-2': '亚太（悉尼）',
    'cn-north-1': '中国（北京）',
    'cn-northwest-1': '中国（宁夏）',
    'us-east-1': '美国东部（弗吉尼亚）',
    'us-west-2': '美国西部（俄勒冈）',
    'eu-west-1': '欧洲（爱尔兰）'
  }
  return map[region] || region
}

const resetForm = () => {
  form.name = ''
  form.accountId = ''
  form.type = ''
  form.region = ''
  form.accessKeyId = ''
  form.secretAccessKey = ''
  form.status = 'active'
  form.remark = ''
  editId.value = null
  isEditing.value = false
}

const saveAccount = async () => {
  submitting.value = true

  try {
    await new Promise(resolve => setTimeout(resolve, 800))

    if (isEditing.value && editId.value) {
      const index = accounts.value.findIndex(a => a.id === editId.value)
      if (index !== -1) {
        accounts.value[index] = {
          ...accounts.value[index],
          name: form.name,
          accountId: form.accountId,
          type: form.type as 'root' | 'iam' | 'role',
          region: form.region,
          accessKeyId: form.accessKeyId,
          secretAccessKey: form.secretAccessKey,
          status: form.status,
          remark: form.remark,
          updateTime: new Date().toLocaleString('zh-CN')
        }
      }
      alert('账号修改成功！')
    } else {
      const newId = Math.max(...accounts.value.map(a => a.id), 0) + 1
      accounts.value.unshift({
        id: newId,
        name: form.name,
        accountId: form.accountId,
        type: form.type as 'root' | 'iam' | 'role',
        region: form.region,
        accessKeyId: form.accessKeyId,
        secretAccessKey: form.secretAccessKey,
        status: form.status,
        remark: form.remark,
        createTime: new Date().toLocaleString('zh-CN')
      })
      alert('账号添加成功！')
    }

    resetForm()
    showModal.value = false
  } catch (error) {
    alert('操作失败，请重试')
  } finally {
    submitting.value = false
  }
}

const viewAccount = (account: AwsAccount) => {
  selectedAccount.value = account
  showDetailModal.value = true
}

const editAccount = (account: AwsAccount) => {
  isEditing.value = true
  editId.value = account.id
  form.name = account.name
  form.accountId = account.accountId
  form.type = account.type
  form.region = account.region
  form.accessKeyId = account.accessKeyId || ''
  form.secretAccessKey = account.secretAccessKey || ''
  form.status = account.status
  form.remark = account.remark || ''
  showModal.value = true
}

const deleteAccount = (id: number) => {
  if (confirm('确定要删除该AWS账号吗？此操作不可恢复。')) {
    accounts.value = accounts.value.filter(a => a.id !== id)
    alert('账号已删除')
  }
}
</script>

<style scoped>
.aws-account {
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

.account-table {
  width: 100%;
  border-collapse: collapse;
}

.account-table th {
  background: #f8f9fa;
  padding: 16px;
  text-align: left;
  font-weight: 600;
  color: #333;
  border-bottom: 2px solid #eee;
}

.account-table td {
  padding: 16px;
  border-bottom: 1px solid #eee;
  color: #555;
}

.account-table tr:hover {
  background: #f8f9fa;
}

.account-name {
  display: flex;
  align-items: center;
  gap: 10px;
}

.account-icon {
  font-size: 20px;
}

.account-id {
  font-family: 'Courier New', monospace;
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

.type-badge.root {
  background: #dbeafe;
  color: #2563eb;
}

.type-badge.iam {
  background: #e0f2fe;
  color: #0369a1;
}

.type-badge.role {
  background: #f3e8ff;
  color: #9333ea;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.active {
  background: #d1fae5;
  color: #059669;
}

.status-badge.inactive {
  background: #fee2e2;
  color: #dc2626;
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
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
}

.detail-modal {
  max-width: 500px;
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

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
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

.detail-row {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.detail-label {
  font-size: 14px;
  color: #666;
  min-width: 100px;
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
