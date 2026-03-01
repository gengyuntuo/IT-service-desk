<template>
  <div class="ticket-approve">
    <div class="page-header">
      <h1 class="page-title">工单审批</h1>
      <div class="header-actions">
        <select v-model="filterStatus" class="filter-select">
          <option value="">全部状态</option>
          <option value="pending">待审批</option>
          <option value="approved">已通过</option>
          <option value="rejected">已拒绝</option>
        </select>
      </div>
    </div>
    
    <div class="stats-cards">
      <div class="stat-card">
        <div class="stat-icon" style="background: #f59e0b;">⏳</div>
        <div class="stat-info">
          <div class="stat-label">待审批</div>
          <div class="stat-value">{{ pendingCount }}</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #10b981;">✅</div>
        <div class="stat-info">
          <div class="stat-label">已通过</div>
          <div class="stat-value">{{ approvedCount }}</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #ef4444;">❌</div>
        <div class="stat-info">
          <div class="stat-label">已拒绝</div>
          <div class="stat-value">{{ rejectedCount }}</div>
        </div>
      </div>
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
      <select v-model="filterPriority" class="filter-select">
        <option value="">全部优先级</option>
        <option value="low">低</option>
        <option value="medium">中</option>
        <option value="high">高</option>
        <option value="urgent">紧急</option>
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
            <th>申请人</th>
            <th>申请时间</th>
            <th>状态</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="ticket in paginatedTickets" :key="ticket.id">
            <td>#{{ ticket.id }}</td>
            <td>{{ ticket.title }}</td>
            <td>{{ getTypeText(ticket.type) }}</td>
            <td>
              <span class="priority-badge" :class="ticket.priority">
                {{ getPriorityText(ticket.priority) }}
              </span>
            </td>
            <td>{{ ticket.applicant }}</td>
            <td>{{ ticket.applyTime }}</td>
            <td>
              <span class="status-badge" :class="ticket.status">
                {{ getStatusText(ticket.status) }}
              </span>
            </td>
            <td>
              <button class="action-btn view" @click="viewTicket(ticket)">查看</button>
              <button
                v-if="ticket.status === 'pending'"
                class="action-btn approve"
                @click="handleApprove(ticket.id)"
              >
                通过
              </button>
              <button
                v-if="ticket.status === 'pending'"
                class="action-btn reject"
                @click="openRejectModal(ticket.id)"
              >
                拒绝
              </button>
            </td>
          </tr>
          <tr v-if="paginatedTickets.length === 0">
            <td colspan="8" class="empty-cell">
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
            <span class="detail-label">申请人：</span>
            <span class="detail-value">{{ selectedTicket.applicant }}</span>
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
          
          <div v-if="selectedTicket.status !== 'pending'" class="approval-result">
            <h3 class="result-title">审批结果</h3>
            <div class="result-row">
              <span class="result-label">审批人：</span>
              <span class="result-value">{{ selectedTicket.approver }}</span>
            </div>
            <div class="result-row">
              <span class="result-label">审批时间：</span>
              <span class="result-value">{{ selectedTicket.approvalTime }}</span>
            </div>
            <div class="result-row">
              <span class="result-label">审批意见：</span>
              <span class="result-value">{{ selectedTicket.approvalComment }}</span>
            </div>
          </div>
        </div>
        
        <div class="modal-actions">
          <button
            v-if="selectedTicket?.status === 'pending'"
            class="btn-reject"
            @click="openRejectModal(selectedTicket!.id); showDetailModal = false"
          >
            拒绝
          </button>
          <button
            v-if="selectedTicket?.status === 'pending'"
            class="btn-approve"
            @click="handleApprove(selectedTicket!.id); showDetailModal = false"
          >
            通过
          </button>
          <button v-else class="btn-secondary" @click="showDetailModal = false">关闭</button>
        </div>
      </div>
    </div>
    
    <div v-if="showRejectModal" class="modal-overlay" @click="showRejectModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">拒绝工单</h2>
          <button class="close-btn" @click="showRejectModal = false">✕</button>
        </div>
        <form @submit.prevent="confirmReject" class="modal-form">
          <div class="form-group">
            <label>拒绝原因 <span class="required">*</span></label>
            <textarea
              v-model="rejectReason"
              rows="4"
              placeholder="请输入拒绝原因..."
              required
            ></textarea>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn-secondary" @click="showRejectModal = false">取消</button>
            <button type="submit" class="btn-primary">确认拒绝</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Ticket {
  id: number
  title: string
  type: string
  priority: string
  status: string
  applicant: string
  applyTime: string
  expectedTime: string
  description: string
  approver?: string
  approvalTime?: string
  approvalComment?: string
}

const searchQuery = ref('')
const filterStatus = ref('')
const filterType = ref('')
const filterPriority = ref('')
const currentPage = ref(1)
const pageSize = 10
const showDetailModal = ref(false)
const showRejectModal = ref(false)
const selectedTicket = ref<Ticket | null>(null)
const rejectTicketId = ref<number | null>(null)
const rejectReason = ref('')

const tickets = ref<Ticket[]>([
  {
    id: 1001,
    title: '无法连接VPN',
    type: 'network',
    priority: 'high',
    status: 'pending',
    applicant: '张三',
    applyTime: '2024-03-01 09:30',
    expectedTime: '2024-03-01 12:00',
    description: '早上上班时发现无法连接公司VPN，尝试重启电脑和重新安装VPN客户端都无法解决。'
  },
  {
    id: 1002,
    title: '打印机卡纸',
    type: 'hardware',
    priority: 'medium',
    status: 'pending',
    applicant: '李四',
    applyTime: '2024-03-01 10:15',
    expectedTime: '2024-03-01 14:00',
    description: '办公室3楼的HP打印机经常卡纸，已经清理过多次，但问题依然存在。'
  },
  {
    id: 1003,
    title: '申请开通ERP系统权限',
    type: 'account',
    priority: 'medium',
    status: 'approved',
    applicant: '王五',
    applyTime: '2024-02-28 14:20',
    expectedTime: '2024-02-29 10:00',
    description: '因工作需要，申请开通ERP系统的采购模块权限。',
    approver: '管理员',
    approvalTime: '2024-02-28 16:30',
    approvalComment: '已批准，权限已开通'
  },
  {
    id: 1004,
    title: 'Office软件无法激活',
    type: 'software',
    priority: 'high',
    status: 'rejected',
    applicant: '赵六',
    applyTime: '2024-02-27 11:00',
    expectedTime: '2024-02-27 15:00',
    description: '新电脑安装Office后无法激活，提示许可证无效。',
    approver: '管理员',
    approvalTime: '2024-02-27 13:00',
    approvalComment: '请先联系IT部门获取正版授权'
  },
  {
    id: 1005,
    title: 'Wi-Fi信号弱',
    type: 'network',
    priority: 'low',
    status: 'pending',
    applicant: '钱七',
    applyTime: '2024-03-01 08:45',
    expectedTime: '',
    description: '会议室A的Wi-Fi信号很弱，影响会议演示。'
  },
  {
    id: 1006,
    title: '电脑蓝屏问题',
    type: 'hardware',
    priority: 'urgent',
    status: 'pending',
    applicant: '孙八',
    applyTime: '2024-03-01 11:00',
    expectedTime: '2024-03-01 14:00',
    description: '电脑频繁蓝屏，已重启多次仍无法正常使用。'
  },
  {
    id: 1007,
    title: '邮箱无法发送邮件',
    type: 'software',
    priority: 'medium',
    status: 'approved',
    applicant: '周九',
    applyTime: '2024-02-26 09:00',
    expectedTime: '2024-02-26 12:00',
    description: 'Outlook邮箱无法发送邮件，提示服务器连接失败。',
    approver: '管理员',
    approvalTime: '2024-02-26 10:30',
    approvalComment: '已重新配置邮箱服务器'
  }
])

const filteredTickets = computed(() => {
  return tickets.value.filter(ticket => {
    const matchesSearch = ticket.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         ticket.applicant.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesStatus = !filterStatus.value || ticket.status === filterStatus.value
    const matchesType = !filterType.value || ticket.type === filterType.value
    const matchesPriority = !filterPriority.value || ticket.priority === filterPriority.value
    return matchesSearch && matchesStatus && matchesType && matchesPriority
  })
})

const totalPages = computed(() => Math.ceil(filteredTickets.value.length / pageSize))

const paginatedTickets = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return filteredTickets.value.slice(start, end)
})

const pendingCount = computed(() => tickets.value.filter(t => t.status === 'pending').length)
const approvedCount = computed(() => tickets.value.filter(t => t.status === 'approved').length)
const rejectedCount = computed(() => tickets.value.filter(t => t.status === 'rejected').length)

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

const viewTicket = (ticket: Ticket) => {
  selectedTicket.value = ticket
  showDetailModal.value = true
}

const handleApprove = (id: number) => {
  const ticket = tickets.value.find(t => t.id === id)
  if (ticket) {
    ticket.status = 'approved'
    ticket.approver = '管理员'
    ticket.approvalTime = new Date().toLocaleString('zh-CN')
    ticket.approvalComment = '已批准'
  }
}

const openRejectModal = (id: number) => {
  rejectTicketId.value = id
  showRejectModal.value = true
}

const confirmReject = () => {
  const ticket = tickets.value.find(t => t.id === rejectTicketId.value)
  if (ticket) {
    ticket.status = 'rejected'
    ticket.approver = '管理员'
    ticket.approvalTime = new Date().toLocaleString('zh-CN')
    ticket.approvalComment = rejectReason.value
  }
  showRejectModal.value = false
  rejectReason.value = ''
  rejectTicketId.value = null
}
</script>

<style scoped>
.ticket-approve {
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

.header-actions {
  display: flex;
  gap: 12px;
}

.filter-select {
  padding: 10px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  background: white;
  cursor: pointer;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  margin-bottom: 24px;
}

.stat-card {
  background: white;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 50px;
  height: 50px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
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
  font-size: 28px;
  font-weight: 700;
  color: #333;
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

.action-btn.approve {
  background: #10b981;
  color: white;
}

.action-btn.reject {
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
  max-width: 500px;
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

.form-group textarea {
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  font-family: inherit;
  resize: vertical;
}

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

.btn-approve {
  flex: 1;
  padding: 10px 20px;
  background: #10b981;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

.btn-reject {
  flex: 1;
  padding: 10px 20px;
  background: #ef4444;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary:hover,
.btn-secondary:hover,
.btn-approve:hover,
.btn-reject:hover {
  opacity: 0.9;
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
  margin-bottom: 16px;
}

.approval-result {
  background: #f8f9fa;
  padding: 16px;
  border-radius: 8px;
  margin-top: 16px;
}

.result-title {
  font-size: 14px;
  color: #333;
  margin-bottom: 12px;
  font-weight: 600;
}

.result-row {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
}

.result-label {
  font-size: 13px;
  color: #666;
  min-width: 70px;
}

.result-value {
  font-size: 13px;
  color: #333;
}
</style>
