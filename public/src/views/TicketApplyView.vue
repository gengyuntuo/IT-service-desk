<template>
  <div class="ticket-apply">
    <div class="page-header">
      <h1 class="page-title">工单申请</h1>
      <button class="create-btn" @click="showModal = true">
        ➕ 创建申请
      </button>
    </div>
    
    <div class="filter-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索工单..."
        class="search-input"
      />
      <select v-model="filterType" class="filter-select">
        <option value="">全部类型</option>
        <option value="hardware">硬件问题</option>
        <option value="software">软件问题</option>
        <option value="network">网络问题</option>
        <option value="account">账号权限</option>
        <option value="other">其他</option>
      </select>
      <select v-model="filterStatus" class="filter-select">
        <option value="">全部状态</option>
        <option value="pending">待审批</option>
        <option value="approved">已通过</option>
        <option value="rejected">已拒绝</option>
      </select>
    </div>
    
    <div class="table-container">
      <table class="ticket-table">
        <thead>
          <tr>
            <th>工单号</th>
            <th>标题</th>
            <th>类型</th>
            <th>优先级</th>
            <th>申请时间</th>
            <th>状态</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="ticket in filteredTickets" :key="ticket.id">
            <td>#{{ ticket.id }}</td>
            <td>{{ ticket.title }}</td>
            <td>{{ getTypeText(ticket.type) }}</td>
            <td>
              <span class="priority-badge" :class="ticket.priority">
                {{ getPriorityText(ticket.priority) }}
              </span>
            </td>
            <td>{{ ticket.applyTime }}</td>
            <td>
              <span class="status-badge" :class="ticket.status">
                {{ getStatusText(ticket.status) }}
              </span>
            </td>
            <td>
              <button class="action-btn view" @click="viewTicket(ticket)">查看</button>
            </td>
          </tr>
          <tr v-if="filteredTickets.length === 0">
            <td colspan="7" class="empty-cell">
              <div class="empty-state">
                <div class="empty-icon">📭</div>
                <div class="empty-text">暂无工单记录</div>
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
    
    <div v-if="showModal" class="modal-overlay" @click="showModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">创建工单申请</h2>
          <button class="close-btn" @click="showModal = false">✕</button>
        </div>
        
        <form @submit.prevent="submitTicket" class="modal-form">
          <div class="form-row">
            <div class="form-group">
              <label>工单标题 <span class="required">*</span></label>
              <input v-model="form.title" type="text" placeholder="请输入工单标题" required />
            </div>
            <div class="form-group">
              <label>工单类型 <span class="required">*</span></label>
              <select v-model="form.type" required>
                <option value="">请选择类型</option>
                <option value="hardware">硬件问题</option>
                <option value="software">软件问题</option>
                <option value="network">网络问题</option>
                <option value="account">账号权限</option>
                <option value="other">其他</option>
              </select>
            </div>
          </div>
          
          <div class="form-row">
            <div class="form-group">
              <label>优先级 <span class="required">*</span></label>
              <select v-model="form.priority" required>
                <option value="">请选择优先级</option>
                <option value="low">低</option>
                <option value="medium">中</option>
                <option value="high">高</option>
                <option value="urgent">紧急</option>
              </select>
            </div>
            <div class="form-group">
              <label>期望处理时间</label>
              <input v-model="form.expectedTime" type="date" />
            </div>
          </div>
          
          <div class="form-group">
            <label>问题描述 <span class="required">*</span></label>
            <textarea
              v-model="form.description"
              rows="4"
              placeholder="请详细描述您遇到的问题..."
              required
            ></textarea>
          </div>
          
          <div class="form-group">
            <label>附件上传</label>
            <div class="upload-area" @click="triggerFileInput">
              <input
                ref="fileInput"
                type="file"
                multiple
                @change="handleFileChange"
                style="display: none"
              />
              <div class="upload-placeholder">
                <span class="upload-icon">📎</span>
                <span>点击上传文件或拖拽文件到此处</span>
              </div>
            </div>
            <div v-if="files.length > 0" class="file-list">
              <div v-for="(file, index) in files" :key="index" class="file-item">
                <span class="file-name">📄 {{ file.name }}</span>
                <button type="button" class="remove-file" @click="removeFile(index)">✕</button>
              </div>
            </div>
          </div>
          
          <div class="modal-actions">
            <button type="button" class="btn-secondary" @click="showModal = false">取消</button>
            <button type="submit" class="btn-primary" :disabled="submitting">
              {{ submitting ? '提交中...' : '提交申请' }}
            </button>
          </div>
        </form>
      </div>
    </div>
    
    <div v-if="showDetailModal" class="modal-overlay" @click="showDetailModal = false">
      <div class="modal detail-modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">工单详情</h2>
          <button class="close-btn" @click="showDetailModal = false">✕</button>
        </div>
        
        <div class="detail-content" v-if="selectedTicket">
          <div class="detail-row">
            <span class="detail-label">工单号：</span>
            <span class="detail-value">#{{ selectedTicket.id }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">标题：</span>
            <span class="detail-value">{{ selectedTicket.title }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">类型：</span>
            <span class="detail-value">{{ getTypeText(selectedTicket.type) }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">优先级：</span>
            <span class="detail-value">
              <span class="priority-badge" :class="selectedTicket.priority">
                {{ getPriorityText(selectedTicket.priority) }}
              </span>
            </span>
          </div>
          <div class="detail-row">
            <span class="detail-label">状态：</span>
            <span class="detail-value">
              <span class="status-badge" :class="selectedTicket.status">
                {{ getStatusText(selectedTicket.status) }}
              </span>
            </span>
          </div>
          <div class="detail-row">
            <span class="detail-label">申请时间：</span>
            <span class="detail-value">{{ selectedTicket.applyTime }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">期望处理时间：</span>
            <span class="detail-value">{{ selectedTicket.expectedTime || '未指定' }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">问题描述：</span>
          </div>
          <div class="detail-description">
            {{ selectedTicket.description }}
          </div>
          <div v-if="selectedTicket.approvalComment" class="detail-row">
            <span class="detail-label">审批意见：</span>
            <span class="detail-value">{{ selectedTicket.approvalComment }}</span>
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

interface Ticket {
  id: number
  title: string
  type: string
  priority: string
  status: string
  applyTime: string
  expectedTime: string
  description: string
  approvalComment?: string
}

const fileInput = ref<HTMLInputElement | null>(null)
const submitting = ref(false)
const showModal = ref(false)
const showDetailModal = ref(false)
const selectedTicket = ref<Ticket | null>(null)
const files = ref<File[]>([])
const searchQuery = ref('')
const filterType = ref('')
const filterStatus = ref('')
const currentPage = ref(1)

const form = reactive({
  title: '',
  type: '',
  priority: '',
  expectedTime: '',
  description: ''
})

const tickets = ref<Ticket[]>([
  {
    id: 1001,
    title: '无法连接VPN',
    type: 'network',
    priority: 'high',
    status: 'pending',
    applyTime: '2024-03-01 09:30',
    expectedTime: '2024-03-01 12:00',
    description: '早上上班时发现无法连接公司VPN，尝试重启电脑和重新安装VPN客户端都无法解决。'
  },
  {
    id: 1002,
    title: '打印机卡纸',
    type: 'hardware',
    priority: 'medium',
    status: 'approved',
    applyTime: '2024-02-28 10:15',
    expectedTime: '2024-02-28 14:00',
    description: '办公室3楼的HP打印机经常卡纸，已经清理过多次，但问题依然存在。',
    approvalComment: '已安排IT人员处理'
  },
  {
    id: 1003,
    title: '申请开通ERP系统权限',
    type: 'account',
    priority: 'medium',
    status: 'rejected',
    applyTime: '2024-02-27 14:20',
    expectedTime: '2024-02-28 10:00',
    description: '因工作需要，申请开通ERP系统的采购模块权限。',
    approvalComment: '权限申请需部门经理审批'
  }
])

const filteredTickets = computed(() => {
  return tickets.value.filter(ticket => {
    const matchesSearch = ticket.title.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesType = !filterType.value || ticket.type === filterType.value
    const matchesStatus = !filterStatus.value || ticket.status === filterStatus.value
    return matchesSearch && matchesType && matchesStatus
  })
})

const totalPages = computed(() => Math.ceil(filteredTickets.value.length / 10))

const getTypeText = (type: string) => {
  const map: Record<string, string> = {
    hardware: '硬件问题',
    software: '软件问题',
    network: '网络问题',
    account: '账号权限',
    other: '其他'
  }
  return map[type] || type
}

const getPriorityText = (priority: string) => {
  const map: Record<string, string> = {
    low: '低',
    medium: '中',
    high: '高',
    urgent: '紧急'
  }
  return map[priority] || priority
}

const getStatusText = (status: string) => {
  const map: Record<string, string> = {
    pending: '待审批',
    approved: '已通过',
    rejected: '已拒绝'
  }
  return map[status] || status
}

const triggerFileInput = () => {
  fileInput.value?.click()
}

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files) {
    files.value = [...files.value, ...Array.from(target.files)]
  }
}

const removeFile = (index: number) => {
  files.value.splice(index, 1)
}

const resetForm = () => {
  form.title = ''
  form.type = ''
  form.priority = ''
  form.expectedTime = ''
  form.description = ''
  files.value = []
}

const submitTicket = async () => {
  submitting.value = true
  
  try {
    await new Promise(resolve => setTimeout(resolve, 1000))
    const newId = Math.max(...tickets.value.map(t => t.id)) + 1
    tickets.value.unshift({
      id: newId,
      title: form.title,
      type: form.type,
      priority: form.priority,
      status: 'pending',
      applyTime: new Date().toLocaleString('zh-CN'),
      expectedTime: form.expectedTime,
      description: form.description
    })
    alert('工单提交成功！')
    resetForm()
    showModal.value = false
  } catch (error) {
    alert('工单提交失败，请重试')
  } finally {
    submitting.value = false
  }
}

const viewTicket = (ticket: Ticket) => {
  selectedTicket.value = ticket
  showDetailModal.value = true
}
</script>

<style scoped>
.ticket-apply {
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

.ticket-table {
  width: 100%;
  border-collapse: collapse;
}

.ticket-table th {
  background: #f8f9fa;
  padding: 16px;
  text-align: left;
  font-weight: 600;
  color: #333;
  border-bottom: 2px solid #eee;
}

.ticket-table td {
  padding: 16px;
  border-bottom: 1px solid #eee;
  color: #555;
}

.ticket-table tr:hover {
  background: #f8f9fa;
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

.priority-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.priority-badge.low {
  background: #dcfce7;
  color: #15803d;
}

.priority-badge.medium {
  background: #fef3c7;
  color: #d97706;
}

.priority-badge.high {
  background: #fed7aa;
  color: #c2410c;
}

.priority-badge.urgent {
  background: #fee2e2;
  color: #dc2626;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.pending {
  background: #fef3c7;
  color: #d97706;
}

.status-badge.approved {
  background: #d1fae5;
  color: #059669;
}

.status-badge.rejected {
  background: #fee2e2;
  color: #dc2626;
}

.action-btn {
  padding: 6px 12px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  transition: opacity 0.3s;
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

.upload-area {
  border: 2px dashed #ddd;
  border-radius: 8px;
  padding: 24px;
  text-align: center;
  cursor: pointer;
  transition: border-color 0.3s;
}

.upload-area:hover {
  border-color: #667eea;
}

.upload-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  color: #999;
}

.upload-icon {
  font-size: 24px;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 12px;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: #f8f9fa;
  border-radius: 6px;
}

.file-name {
  font-size: 13px;
  color: #555;
}

.remove-file {
  background: #ef4444;
  color: white;
  border: none;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  cursor: pointer;
  font-size: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
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

.detail-description {
  font-size: 14px;
  color: #555;
  line-height: 1.6;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-top: 8px;
}
</style>
