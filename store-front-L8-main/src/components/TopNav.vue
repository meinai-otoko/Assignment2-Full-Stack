<template>
  <nav>
    <div class="logo">
      <a href="/">
        <img src="/bestbuy-logo.png" alt="Best Buy Logo">
      </a>
    </div>
    <button class="hamburger" @click="toggleNav">
      <span class="hamburger-icon"></span>
    </button>
    <ul class="nav-links" :class="{ 'nav-links--open': isNavOpen }">
      <li><router-link to="/" @click="closeNav">Products</router-link></li>
      <li><router-link to="/cart" @click="closeNav">Cart ({{ cartItemCount }})</router-link></li>
    </ul>
  </nav>
</template>

<script>
export default {
  name: 'TopNav',
  props: ['cartItemCount'],
  data() {
    return {
      isNavOpen: false
    }
  },
  methods: {
    toggleNav() {
      this.isNavOpen = !this.isNavOpen
    },
    closeNav() {
      this.isNavOpen = false
    }
  }
}
</script>

<style scoped>
nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #0046BE;
  color: #fff;
  padding: 12px 24px;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  height: 60px;
}

nav img {
  height: 24px;
  width: auto;
}

.nav-links {
  display: flex;
  list-style: none;
  font-size: 16px;
  font-weight: 500;
  margin: 0;
  padding: 0;
  gap: 24px;
}

.nav-links a {
  color: #fff;
  text-decoration: none;
  padding: 8px 0;
}

.nav-links a:hover {
  color: #FFE000;
}

.hamburger {
  display: none;
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  color: #fff;
}

.hamburger-icon {
  display: block;
  width: 24px;
  height: 2px;
  background-color: #fff;
  position: relative;
}

.hamburger-icon::before,
.hamburger-icon::after {
  content: '';
  display: block;
  width: 24px;
  height: 2px;
  background-color: #fff;
  position: absolute;
  left: 0;
}

.hamburger-icon::before {
  top: -6px;
}

.hamburger-icon::after {
  bottom: -6px;
}

@media (max-width: 768px) {
  .nav-links {
    display: none;
    position: absolute;
    top: 60px;
    left: 0;
    right: 0;
    background-color: #0046BE;
    padding: 16px;
    flex-direction: column;
    gap: 16px;
  }

  .nav-links--open {
    display: flex;
  }

  .hamburger {
    display: block;
  }
}
</style>