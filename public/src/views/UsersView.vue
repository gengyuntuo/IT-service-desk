<template>
  <div class="users-page">
    <div class="page-header">
      <h1 class="page-title">用户管理</h1>
      <button class="add-btn" @click="showAddModal = true">
        ➕ 添加用户
      </button>
    </div>
    
    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索用户..."
        class="search-input"
      />
      <select v-model="filterRole" class="filter-select">
        <option value="">所有角色</option>
        <option value="admin">管理员</option>
        <option value="user">普通用户</option>
      </select>
    </div>
    
    <div class="table-container">
      <table class="users-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>用户名</th>
            <th>邮箱</th>
            <th>角色</th>
            <th>状态</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in filteredUsers" :key="user.id">
            <td>{{ user.id }}</td>
            <td>
              <div class="user-cell">
                <div class="user-avatar">{{ user.name.charAt(0) }}</div>
                <span>{{ user.name }}</span>
              </div>
            </td>
            <td>{{ user.email }}</td>
            <td>
              <span class="role-badge" :class="user.role">{{ getRoleText(user.role) }}</span>
            </td>
            <td>
              <span class="status-badge" :class="user.status">
                {{ user.status === 'active' ? '启用' : '禁用' }}
              </span>
            </td>
            <td>{{ user.createdAt }}</td>
            <td>
              <button class="action-btn edit" @click="editUser(user)">编辑</button>
              <button class="action-btn delete" @click="deleteUser(user.id)">删除</button>
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
    
    <div v-if="showAddModal" class="modal-overlay" @click="showAddModal = false">
      <div class="modal" @click.stop>
        <h2 class="modal-title">添加用户</h2>
        <form @submit.prevent="addUser" class="modal-form">
          <div class="form-group">
            <label>用户名</label>
            <input v-model="newUser.name" type="text" required />
          </div>
          <div class="form-group">
            <label>邮箱</label>
            <input v-model="newUser.email" type="email" required />
          </div>
          <div class="form-group">
            <label>角色</label>
            <select v-model="newUser.role" required>
              <option value="user">普通用户</option>
              <option value="admin">管理员</option>
            </select>
          </div>
          <div class="form-group">
            <label>密码</label>
            <input v-model="newUser.password" type="password" required />
          </div>
          <div class="modal-actions">
            <button type="button" class="btn-secondary" @click="showAddModal = false">取消</button>
            <button type="submit" class="btn-primary">添加</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface User {
  id: number
  name: string
  email: string
  role: 'admin' | 'user'
  status: 'active' | 'inactive'
  createdAt: string
}

const searchQuery = ref('')
const filterRole = ref('')
const currentPage = ref(1)
const showAddModal = ref(false)
const newUser = ref({
  name: '',
  email: '',
  role: 'user' as 'admin' | 'user',
  password: ''
})

const users = ref<User[]>([
  { id: 1, name: '张三', email: 'zhangsan@example.com', role: 'admin', status: 'active', createdAt: '2024-01-15' },
  { id: 2, name: '李四', email: 'lisi@example.com', role: 'user', status: 'active', createdAt: '2024-01-20' },
  { id: 3, name: '王五', email: 'wangwu@example.com', role: 'user', status: 'inactive', createdAt: '2024-02-01' },
  { id: 4, name: '赵六', email: 'zhaoliu@example.com', role: 'user', status: 'active', createdAt: '2024-02-10' },
  { id: 5, name: '钱七', email: 'qianqi@example.com', role: 'admin', status: 'active', createdAt: '2024-02-15' }
])

const filteredUsers = computed(() => {
  return users.value.filter(user => {
    const matchesSearch = user.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         user.email.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesRole = !filterRole.value || user.role === filterRole.value
    return matchesSearch && matchesRole
  })
})

const totalPages = computed(() => Math.ceil(filteredUsers.value.length / 10))

const getRoleText = (role: string) => {
  return role === 'admin' ? '管理员' : '普通用户'
}

const addUser = () => {
  const id = Math.max(...users.value.map(u => u.id)) + 1
  users.value.push({
    id,
    name: newUser.value.name,
    email: newUser.value.email,
    role: newUser.value.role,
    status: 'active',
    createdAt: new Date().toISOString().split('T')[0]
  })
  showAddModal.value = false
  newUser.value = { name: '', email: '', role: 'user', password: '' }
}

const editUser = (user: User) => {
  alert(`编辑用户: ${user.name}`)
}

const deleteUser = (id: number) => {
  if (confirm('确定要删除该用户吗？')) {
    users.value = users.value.filter(u => u.id !== id)
  }
}
</script>

<style scoped>
.users-page {
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

.add-btn {
  padding: 10px 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: opacity 0.3s;
}

.add-btn:hover {
  opacity: 0.9;
}

.search-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.search-input {
  flex: 1;
  padding: 12px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: #667eea;
}

.filter-select {
  padding: 12px 16px;
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

.users-table {
  width: 100%;
  border-collapse: collapse;
}

.users-table th {
  background: #f8f9fa;
  padding: 16px;
  text-align: left;
  font-weight: 600;
  color: #333;
  border-bottom: 2px solid #eee;
}

.users-table td {
  padding: 16px;
  border-bottom: 1px solid #eee;
  color: #555;
}

.users-table tr:hover {
  background: #f8f9fa;
}

.user-cell {
  display: flex;
  align-items: center;
  gap: 10px;
}

.user-avatar {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
}

.role-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.role-badge.admin {
  background: #dbeafe;
  color: #2563eb;
}

.role-badge.user {
  background: #f3f4f6;
  color: #6b7280;
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

.action-btn.edit {
  background: #3b82f6;
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
  padding: 32px;
  border-radius: 12px;
  width: 100%;
  max-width: 480px;
}

.modal-title {
  font-size: 20px;
  color: #333;
  margin-bottom: 24px;
}

.modal-form {
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

.modal-actions {
  display: flex;
  gap: 12px;
  margin-top: 8px;
}

.btn-secondary {
  flex: 1;
  padding: 12px;
  background: #f3f4f6;
  color: #333;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary {
  flex: 1;
  padding: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary:hover,
.btn-secondary:hover {
  opacity: 0.9;
}
</style>
