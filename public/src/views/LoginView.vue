<template>
  <div class="login-container">
    <div class="login-box">
      <h1 class="login-title">IT服务台</h1>
      <p class="login-subtitle">请登录您的账户</p>
      
      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label for="username">用户名</label>
          <input
            id="username"
            v-model="form.username"
            type="text"
            placeholder="请输入用户名"
            required
          />
        </div>
        
        <div class="form-group">
          <label for="password">密码</label>
          <input
            id="password"
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            required
          />
        </div>
        
        <div class="form-options">
          <label class="remember-me">
            <input type="checkbox" v-model="form.rememberMe" />
            <span>记住我</span>
          </label>
          <a href="#" class="forgot-password">忘记密码?</a>
        </div>
        
        <button type="submit" class="login-btn" :disabled="loading">
          {{ loading ? '登录中...' : '登录' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const loading = ref(false)

const form = reactive({
  username: '',
  password: '',
  rememberMe: false
})

const handleLogin = async () => {
  loading.value = true
  
  try {
    await new Promise(resolve => setTimeout(resolve, 1000))
    console.log('登录信息:', form)
    router.push('/dashboard')
  } catch (error) {
    alert('登录失败，请重试')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-box {
  background: white;
  padding: 40px;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  width: 100%;
  max-width: 400px;
}

.login-title {
  text-align: center;
  color: #333;
  font-size: 28px;
  margin-bottom: 8px;
}

.login-subtitle {
  text-align: center;
  color: #666;
  font-size: 14px;
  margin-bottom: 30px;
}

.login-form {
  display: flex;
  flex-direction: column;
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

.form-group input {
  padding: 12px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  transition: border-color 0.3s;
}

.form-group input:focus {
  outline: none;
  border-color: #667eea;
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
}

.remember-me {
  display: flex;
  align-items: center;
  gap: 6px;
  color: #555;
  cursor: pointer;
}

.remember-me input {
  cursor: pointer;
}

.forgot-password {
  color: #667eea;
  text-decoration: none;
}

.forgot-password:hover {
  text-decoration: underline;
}

.login-btn {
  padding: 14px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.3s;
}

.login-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.login-btn:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}
</style>
